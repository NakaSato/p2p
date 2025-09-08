use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod middleware;
mod models;
mod error;

use handlers::*;
use middleware::*;

#[derive(Clone)]
pub struct AppState {
    // In a real implementation, these would be database connections
    // or service clients
}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "p2p_api_gateway=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new();

    // Build the application with middleware
    let app = create_app(state);

    // Get port from environment or default to 3000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    info!("🚀 P2P Energy Trading API Gateway listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(state: AppState) -> Router {
    Router::new()
        // Health endpoint
        .route("/health", get(health_handler))
        // Metrics endpoint
        .route("/metrics", get(metrics_handler))
        // API routes
        .nest("/api", api_routes())
        // Global middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                )
                .layer(axum::middleware::from_fn(security_headers))
                .layer(axum::middleware::from_fn(request_logging)),
        )
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .route("/meters", get(get_meters).post(create_meter))
        .route("/meters/:id", get(get_meter).put(update_meter).delete(delete_meter))
        .route("/market", get(get_market_data))
        .route("/market/orders", get(get_orders).post(create_order))
        .route("/market/orders/:id", get(get_order).put(update_order).delete(cancel_order))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_app(AppState::new());

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = tower::ServiceExt::oneshot(app, request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test] 
    async fn test_metrics_endpoint() {
        let app = create_app(AppState::new());

        let request = Request::builder()
            .uri("/metrics")
            .body(Body::empty())
            .unwrap();

        let response = tower::ServiceExt::oneshot(app, request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
