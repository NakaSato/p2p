use std::net::SocketAddr;

use anyhow::Result;
use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer, timeout::TimeoutLayer};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod database;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;
mod error;

use config::Config;
use handlers::health;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Loaded configuration for environment: {}", config.environment);

    // Setup database connection
    let db_pool = database::setup_database(&config.database_url).await?;
    info!("Database connection established");

    // Run database migrations
    database::run_migrations(&db_pool).await?;
    info!("Database migrations completed");

    // Setup Redis connection
    let redis_client = redis::Client::open(config.redis_url.as_str())?;
    info!("Redis connection established");

    // Build application router
    let app = Router::new()
        .route("/health", get(health::health_check))
        .route("/health/ready", get(health::readiness_check))
        .route("/health/live", get(health::liveness_check))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
                .layer(CorsLayer::permissive()) // TODO: Configure proper CORS in production
        )
        .with_state(config.clone());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Starting API Gateway server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}