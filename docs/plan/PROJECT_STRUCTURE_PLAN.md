# API Gateway Project Structure Plan
## P2P Energy Trading System - Engineering Department

**Document Version**: 1.0  
**Created**: September 13, 2025  
**Last Updated**: September 13, 2025  

---

## Table of Contents

1. [Project Directory Structure](#project-directory-structure)
2. [Module Architecture](#module-architecture)
3. [Dependency Management](#dependency-management)
4. [Configuration Management](#configuration-management)
5. [Development Environment Setup](#development-environment-setup)
6. [Build System](#build-system)
7. [Code Organization Principles](#code-organization-principles)

---

## Project Directory Structure

### Complete Directory Layout
```
api-gateway/
├── Cargo.toml                     # Main project manifest
├── Cargo.lock                     # Dependency lock file
├── README.md                      # Project documentation
├── .env.example                   # Environment variables template
├── .gitignore                     # Git ignore rules
├── docker-compose.yml             # Development environment
├── Dockerfile                     # Production container
├── Dockerfile.dev                 # Development container
├── .github/                       # CI/CD workflows
│   └── workflows/
│       ├── ci.yml                 # Continuous integration
│       ├── security.yml           # Security scanning
│       └── deploy.yml             # Deployment pipeline
├── config/                        # Configuration files
│   ├── development.toml           # Development config
│   ├── staging.toml               # Staging config
│   ├── production.toml            # Production config
│   └── local.toml                 # Local overrides
├── migrations/                    # Database migrations
│   ├── 001_create_users.sql
│   ├── 002_create_energy_readings.sql
│   ├── 003_create_trading_orders.sql
│   ├── 004_create_audit_logs.sql
│   └── 005_create_blockchain_events.sql
├── scripts/                       # Utility scripts
│   ├── setup-dev.sh               # Development setup
│   ├── run-tests.sh               # Test runner
│   ├── docker-build.sh            # Docker build script
│   └── migrate.sh                 # Database migration runner
├── docs/                          # Additional documentation
│   ├── api.md                     # API documentation
│   ├── deployment.md              # Deployment guide
│   └── development.md             # Development guide
├── src/                           # Source code
│   ├── main.rs                    # Application entry point
│   ├── lib.rs                     # Library root
│   ├── config/                    # Configuration module
│   │   ├── mod.rs
│   │   ├── app.rs                 # Application configuration
│   │   ├── database.rs            # Database configuration
│   │   ├── blockchain.rs          # Blockchain configuration
│   │   └── auth.rs                # Authentication configuration
│   ├── handlers/                  # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── auth.rs                # Authentication endpoints
│   │   ├── users.rs               # User management endpoints
│   │   ├── energy.rs              # Energy data endpoints
│   │   ├── trading.rs             # Trading endpoints
│   │   ├── blockchain.rs          # Blockchain interaction endpoints
│   │   ├── analytics.rs           # Analytics endpoints
│   │   └── health.rs              # Health check endpoints
│   ├── services/                  # Business logic layer
│   │   ├── mod.rs
│   │   ├── auth_service.rs        # Authentication service
│   │   ├── user_service.rs        # User management service
│   │   ├── energy_service.rs      # Energy data service
│   │   ├── trading_service.rs     # Trading service
│   │   ├── blockchain_service.rs  # Blockchain service
│   │   ├── analytics_service.rs   # Analytics service
│   │   └── notification_service.rs # Notification service
│   ├── models/                    # Data models and DTOs
│   │   ├── mod.rs
│   │   ├── user.rs                # User models
│   │   ├── energy.rs              # Energy data models
│   │   ├── trading.rs             # Trading models
│   │   ├── blockchain.rs          # Blockchain models
│   │   ├── auth.rs                # Authentication models
│   │   └── api.rs                 # API request/response models
│   ├── database/                  # Database layer
│   │   ├── mod.rs
│   │   ├── connection.rs          # Database connection management
│   │   ├── repositories/          # Repository implementations
│   │   │   ├── mod.rs
│   │   │   ├── user_repository.rs
│   │   │   ├── energy_repository.rs
│   │   │   ├── trading_repository.rs
│   │   │   └── audit_repository.rs
│   │   └── migrations.rs          # Migration runner
│   ├── blockchain/                # Blockchain integration
│   │   ├── mod.rs
│   │   ├── client.rs              # Solana client wrapper
│   │   ├── programs/              # Anchor program clients
│   │   │   ├── mod.rs
│   │   │   ├── registry.rs        # Registry program client
│   │   │   ├── energy_token.rs    # Energy token program client
│   │   │   ├── trading.rs         # Trading program client
│   │   │   ├── oracle.rs          # Oracle program client
│   │   │   └── governance.rs      # Governance program client
│   │   ├── events.rs              # Blockchain event processing
│   │   ├── transactions.rs        # Transaction building utilities
│   │   └── monitoring.rs          # Blockchain monitoring
│   ├── middleware/                # HTTP middleware
│   │   ├── mod.rs
│   │   ├── auth.rs                # Authentication middleware
│   │   ├── cors.rs                # CORS middleware
│   │   ├── rate_limit.rs          # Rate limiting middleware
│   │   ├── logging.rs             # Request logging middleware
│   │   ├── metrics.rs             # Metrics collection middleware
│   │   └── error.rs               # Error handling middleware
│   ├── cache/                     # Caching layer
│   │   ├── mod.rs
│   │   ├── redis.rs               # Redis client wrapper
│   │   ├── strategies.rs          # Caching strategies
│   │   └── keys.rs                # Cache key management
│   ├── security/                  # Security utilities
│   │   ├── mod.rs
│   │   ├── jwt.rs                 # JWT token management
│   │   ├── encryption.rs          # Encryption utilities
│   │   ├── validation.rs          # Input validation
│   │   └── audit.rs               # Audit logging
│   ├── integrations/              # External integrations
│   │   ├── mod.rs
│   │   ├── ami.rs                 # AMI system integration
│   │   ├── notifications.rs       # Notification services
│   │   └── external_apis.rs       # External API clients
│   ├── utils/                     # Utility functions
│   │   ├── mod.rs
│   │   ├── time.rs                # Time utilities
│   │   ├── decimal.rs             # Decimal utilities
│   │   ├── crypto.rs              # Cryptographic utilities
│   │   └── formatting.rs          # Data formatting utilities
│   ├── monitoring/                # Observability
│   │   ├── mod.rs
│   │   ├── metrics.rs             # Prometheus metrics
│   │   ├── tracing.rs             # Distributed tracing
│   │   └── health.rs              # Health check implementations
│   └── error.rs                   # Error types and handling
├── tests/                         # Test suites
│   ├── integration/               # Integration tests
│   │   ├── mod.rs
│   │   ├── auth_tests.rs
│   │   ├── energy_tests.rs
│   │   ├── trading_tests.rs
│   │   └── blockchain_tests.rs
│   ├── unit/                      # Unit tests
│   │   ├── mod.rs
│   │   ├── services/
│   │   ├── handlers/
│   │   └── utils/
│   ├── fixtures/                  # Test data fixtures
│   │   ├── users.json
│   │   ├── energy_readings.json
│   │   └── trading_orders.json
│   └── common/                    # Test utilities
│       ├── mod.rs
│       ├── test_db.rs             # Test database setup
│       ├── mock_blockchain.rs     # Blockchain mocking
│       └── test_server.rs         # Test server setup
├── benches/                       # Performance benchmarks
│   ├── api_benchmarks.rs
│   ├── database_benchmarks.rs
│   └── blockchain_benchmarks.rs
└── examples/                      # Usage examples
    ├── simple_client.rs           # Basic API client example
    ├── trading_bot.rs             # Trading bot example
    └── energy_monitor.rs          # Energy monitoring example
```

---

## Module Architecture

### Core Application Structure

#### Main Application (`src/main.rs`)
```rust
// Application entry point and server setup
use api_gateway::{
    config::AppConfig,
    database::establish_connection,
    blockchain::BlockchainClient,
    cache::RedisClient,
    monitoring::setup_metrics,
    server::create_app,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();
    
    // Load configuration
    let config = AppConfig::from_env()?;
    
    // Setup database connection
    let db_pool = establish_connection(&config.database).await?;
    
    // Setup blockchain client
    let blockchain_client = BlockchainClient::new(&config.blockchain).await?;
    
    // Setup cache
    let cache_client = RedisClient::new(&config.redis).await?;
    
    // Setup metrics
    let metrics = setup_metrics()?;
    
    // Create and run server
    let app = create_app(db_pool, blockchain_client, cache_client, metrics).await?;
    let listener = tokio::net::TcpListener::bind(&config.server.bind_address).await?;
    
    tracing::info!("Starting API Gateway on {}", config.server.bind_address);
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

#### Library Root (`src/lib.rs`)
```rust
// Public API and module declarations
pub mod config;
pub mod handlers;
pub mod services;
pub mod models;
pub mod database;
pub mod blockchain;
pub mod middleware;
pub mod cache;
pub mod security;
pub mod integrations;
pub mod utils;
pub mod monitoring;
pub mod error;

// Re-exports for convenience
pub use error::{ApiError, Result};
pub use config::AppConfig;

// Server setup function
pub mod server {
    use axum::{Router, routing::get};
    use tower::ServiceBuilder;
    use crate::{
        handlers,
        middleware,
        database::DbPool,
        blockchain::BlockchainClient,
        cache::RedisClient,
        monitoring::Metrics,
    };
    
    pub async fn create_app(
        db_pool: DbPool,
        blockchain_client: BlockchainClient,
        cache_client: RedisClient,
        metrics: Metrics,
    ) -> Result<Router, crate::error::ApiError> {
        let app = Router::new()
            // Health endpoints
            .route("/health", get(handlers::health::health_check))
            .route("/ready", get(handlers::health::readiness_check))
            .route("/metrics", get(handlers::health::metrics))
            
            // API routes
            .nest("/v1", create_v1_routes())
            
            // Middleware stack
            .layer(
                ServiceBuilder::new()
                    .layer(middleware::metrics::metrics_layer(metrics))
                    .layer(middleware::logging::logging_layer())
                    .layer(middleware::cors::cors_layer())
                    .layer(middleware::rate_limit::rate_limit_layer(cache_client.clone()))
                    .layer(middleware::auth::auth_layer())
            )
            
            // Application state
            .with_state(AppState {
                db_pool,
                blockchain_client,
                cache_client,
            });
        
        Ok(app)
    }
    
    fn create_v1_routes() -> Router {
        Router::new()
            .nest("/auth", handlers::auth::routes())
            .nest("/users", handlers::users::routes())
            .nest("/energy", handlers::energy::routes())
            .nest("/trading", handlers::trading::routes())
            .nest("/blockchain", handlers::blockchain::routes())
            .nest("/analytics", handlers::analytics::routes())
    }
}

// Application state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: database::DbPool,
    pub blockchain_client: blockchain::BlockchainClient,
    pub cache_client: cache::RedisClient,
}
```

### Handler Module Structure

#### Example Handler (`src/handlers/energy.rs`)
```rust
use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::energy::{EnergyReading, EnergyReadingSubmission},
    services::energy_service::EnergyService,
    security::auth::Claims,
    error::{ApiError, Result},
    AppState,
};

// Route configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/readings", post(submit_reading))
        .route("/readings/:meter_id", get(get_meter_readings))
        .route("/readings/:meter_id/history", get(get_meter_history))
        .route("/meters/:meter_id/status", get(get_meter_status))
}

// Handler implementations
pub async fn submit_reading(
    State(state): State<AppState>,
    claims: Claims,
    Json(submission): Json<EnergyReadingSubmission>,
) -> Result<Json<EnergyReadingResponse>> {
    let service = EnergyService::new(
        state.db_pool.clone(),
        state.blockchain_client.clone(),
        state.cache_client.clone(),
    );
    
    let result = service.submit_energy_reading(claims.user_id, submission).await?;
    Ok(Json(result))
}

pub async fn get_meter_readings(
    State(state): State<AppState>,
    Path(meter_id): Path<String>,
    Query(params): Query<ReadingQueryParams>,
    claims: Claims,
) -> Result<Json<Vec<EnergyReading>>> {
    let service = EnergyService::new(
        state.db_pool.clone(),
        state.blockchain_client.clone(),
        state.cache_client.clone(),
    );
    
    let readings = service.get_meter_readings(&meter_id, params, claims.user_id).await?;
    Ok(Json(readings))
}

// Request/Response models
#[derive(Deserialize)]
pub struct ReadingQueryParams {
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub interval: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Serialize)]
pub struct EnergyReadingResponse {
    pub id: Uuid,
    pub status: String,
    pub processing_id: Uuid,
    pub oracle_submission: OracleSubmissionStatus,
    pub token_minting: TokenMintingInfo,
}
```

### Service Module Structure

#### Example Service (`src/services/energy_service.rs`)
```rust
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::{
    models::energy::{EnergyReading, EnergyReadingSubmission},
    database::{DbPool, repositories::EnergyRepository},
    blockchain::BlockchainClient,
    cache::RedisClient,
    error::{ApiError, Result},
    security::validation::validate_energy_reading,
    integrations::ami::AMIClient,
};

pub struct EnergyService {
    db_pool: DbPool,
    blockchain_client: BlockchainClient,
    cache_client: RedisClient,
    energy_repo: EnergyRepository,
    ami_client: AMIClient,
}

impl EnergyService {
    pub fn new(
        db_pool: DbPool,
        blockchain_client: BlockchainClient,
        cache_client: RedisClient,
    ) -> Self {
        let energy_repo = EnergyRepository::new(db_pool.clone());
        let ami_client = AMIClient::new();
        
        Self {
            db_pool,
            blockchain_client,
            cache_client,
            energy_repo,
            ami_client,
        }
    }
    
    pub async fn submit_energy_reading(
        &self,
        user_id: Uuid,
        submission: EnergyReadingSubmission,
    ) -> Result<EnergyReadingResponse> {
        // Validate input
        validate_energy_reading(&submission)?;
        
        // Verify AMI signature
        self.ami_client.verify_signature(&submission).await?;
        
        // Store in database
        let reading = self.energy_repo.create_reading(user_id, submission).await?;
        
        // Submit to blockchain oracle
        let oracle_result = self.blockchain_client
            .submit_to_oracle(&reading)
            .await?;
        
        // Update cache
        self.cache_client
            .invalidate_meter_cache(&reading.meter_id)
            .await?;
        
        // Prepare response
        Ok(EnergyReadingResponse {
            id: reading.id,
            status: "accepted".to_string(),
            oracle_submission: oracle_result,
            // ... other fields
        })
    }
    
    pub async fn get_meter_readings(
        &self,
        meter_id: &str,
        params: ReadingQueryParams,
        user_id: Uuid,
    ) -> Result<Vec<EnergyReading>> {
        // Check authorization
        self.verify_meter_access(user_id, meter_id).await?;
        
        // Check cache first
        if let Some(cached) = self.cache_client
            .get_meter_readings(meter_id, &params)
            .await? {
            return Ok(cached);
        }
        
        // Query database
        let readings = self.energy_repo
            .get_readings_by_meter(meter_id, params)
            .await?;
        
        // Cache results
        self.cache_client
            .cache_meter_readings(meter_id, &readings)
            .await?;
        
        Ok(readings)
    }
    
    async fn verify_meter_access(&self, user_id: Uuid, meter_id: &str) -> Result<()> {
        // Implementation for access control
        todo!()
    }
}
```

---

## Dependency Management

### Main Cargo.toml Configuration
```toml
[package]
name = "api-gateway"
version = "0.1.0"
edition = "2021"
authors = ["Engineering Department <energy@engineering.local>"]
description = "API Gateway for P2P Energy Trading System"
license = "MIT"
readme = "README.md"
repository = "https://github.com/engineering-dept/p2p-energy-trading"

[dependencies]
# Web Framework
axum = { version = "0.7", features = ["macros", "multipart"] }
axum-extra = { version = "0.9", features = ["typed-header"] }
tower = { version = "0.4", features = ["full"] }
tower-http = { version = "0.5", features = ["add-extension", "auth", "compression-gzip", "cors", "fs", "limit", "request-id", "sensitive-headers", "timeout", "trace", "util"] }
hyper = { version = "1.0", features = ["full"] }

# Async Runtime
tokio = { version = "1.35", features = ["full"] }
tokio-util = { version = "0.7", features = ["codec", "io"] }
futures = "0.3"
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_with = "3.4"

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid", "decimal", "migrate"] }
sea-query = "0.30"
sea-query-postgres = "0.4"

# Blockchain
solana-client = "1.18"
solana-sdk = "1.18"
solana-program = "1.18"
anchor-client = "0.31"
anchor-lang = "0.31"
anchor-spl = "0.31"

# Authentication & Security
jsonwebtoken = "9.2"
bcrypt = "0.15"
argon2 = "0.5"
ring = "0.17"
rsa = "0.9"

# Caching
redis = { version = "0.24", features = ["tokio-comp", "connection-manager", "streams"] }

# Configuration
config = "0.14"
figment = { version = "0.10", features = ["toml", "env"] }

# Logging & Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt", "json"] }
tracing-opentelemetry = "0.22"
opentelemetry = { version = "0.21", features = ["rt-tokio"] }
opentelemetry-jaeger = "0.20"

# Metrics
metrics = "0.22"
metrics-exporter-prometheus = { version = "0.13", default-features = false }

# HTTP Client
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }

# Time & Date
chrono = { version = "0.4", features = ["serde", "clock"] }
time = { version = "0.3", features = ["serde", "formatting", "parsing"] }

# UUID
uuid = { version = "1.6", features = ["v4", "serde"] }

# Decimal Math
rust_decimal = { version = "1.33", features = ["serde-float", "db-postgres"] }
bigdecimal = { version = "0.4", features = ["serde"] }

# Error Handling
anyhow = "1.0"
thiserror = "1.0"
color-eyre = "0.6"

# Validation
validator = { version = "0.17", features = ["derive"] }
garde = "0.17"

# Utilities
lazy_static = "1.4"
once_cell = "1.19"
regex = "1.10"
url = { version = "2.5", features = ["serde"] }
base64 = "0.21"
hex = "0.4"

# Rate Limiting
governor = "0.6"
nonzero_ext = "0.3"

[dev-dependencies]
# Testing
tokio-test = "0.4"
proptest = "1.4"
quickcheck = "1.0"
quickcheck_macros = "1.0"
mockall = "0.12"
wiremock = "0.6"
httpmock = "0.7"

# Test Containers
testcontainers = "0.15"
testcontainers-modules = { version = "0.3", features = ["postgres", "redis"] }

# Benchmarking
criterion = { version = "0.5", features = ["html_reports"] }

# Development Tools
cargo-watch = "8.4"
cargo-nextest = "0.9"

[profile.dev]
debug = true
opt-level = 0

[profile.release]
debug = false
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"

[profile.test]
debug = true
opt-level = 1

[[bin]]
name = "api-gateway"
path = "src/main.rs"

[[bench]]
name = "api_benchmarks"
harness = false

[[bench]]
name = "database_benchmarks"
harness = false

[[bench]]
name = "blockchain_benchmarks"
harness = false
```

### Feature Configuration
```toml
[features]
default = ["postgres", "redis", "metrics"]
postgres = ["sqlx/postgres"]
redis = ["dep:redis"]
metrics = ["metrics-exporter-prometheus"]
blockchain = ["solana-client", "anchor-client"]
security-audit = ["ring", "rsa"]
development = ["tokio/test-util"]
```

---

## Configuration Management

### Configuration Structure (`src/config/mod.rs`)
```rust
use figment::{Figment, providers::{Format, Toml, Env}};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub blockchain: BlockchainConfig,
    pub auth: AuthConfig,
    pub monitoring: MonitoringConfig,
    pub integrations: IntegrationConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("config/development.toml"))
            .merge(Env::prefixed("API_GATEWAY_"))
            .extract()
    }
    
    pub fn from_file(path: &str) -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file(path))
            .merge(Env::prefixed("API_GATEWAY_"))
            .extract()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub keep_alive: Duration,
    pub client_timeout: Duration,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub auto_migrate: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockchainConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub commitment: String,
    pub keypair_path: String,
    pub programs: ProgramConfig,
    pub timeout: Duration,
    pub retry_attempts: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProgramConfig {
    pub registry: String,
    pub energy_token: String,
    pub trading: String,
    pub oracle: String,
    pub governance: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration: Duration,
    pub refresh_token_expiration: Duration,
    pub bcrypt_cost: u32,
    pub api_key_header: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub tracing_enabled: bool,
    pub jaeger_endpoint: Option<String>,
    pub log_level: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IntegrationConfig {
    pub ami: AMIConfig,
    pub notifications: NotificationConfig,
}
```

### Environment Configuration Files

#### Development Config (`config/development.toml`)
```toml
[server]
bind_address = "0.0.0.0:8080"
port = 8080
workers = 4
keep_alive = "60s"
client_timeout = "30s"
max_connections = 100

[database]
url = "postgresql://api_user:api_password@localhost:5432/energy_trading_dev"
max_connections = 10
min_connections = 2
connect_timeout = "10s"
idle_timeout = "300s"
max_lifetime = "3600s"
auto_migrate = true

[redis]
url = "redis://localhost:6379"
pool_size = 10
timeout = "5s"

[blockchain]
rpc_url = "http://localhost:8899"
ws_url = "ws://localhost:8900"
commitment = "confirmed"
keypair_path = "/keys/development-authority.json"
timeout = "30s"
retry_attempts = 3

[blockchain.programs]
registry = "11111111111111111111111111111111"
energy_token = "22222222222222222222222222222222"
trading = "33333333333333333333333333333333"
oracle = "44444444444444444444444444444444"
governance = "55555555555555555555555555555555"

[auth]
jwt_secret = "development-secret-key-change-in-production"
jwt_expiration = "3600s"
refresh_token_expiration = "86400s"
bcrypt_cost = 10
api_key_header = "X-API-Key"

[monitoring]
metrics_enabled = true
metrics_port = 9090
tracing_enabled = true
jaeger_endpoint = "http://localhost:14268"
log_level = "debug"

[integrations.ami]
base_url = "http://localhost:3001"
api_key = "development-ami-key"
timeout = "10s"

[integrations.notifications]
enabled = false
webhook_url = ""
```

#### Production Config (`config/production.toml`)
```toml
[server]
bind_address = "0.0.0.0:8080"
port = 8080
workers = 8
keep_alive = "300s"
client_timeout = "60s"
max_connections = 1000

[database]
url = "${DATABASE_URL}"
max_connections = 50
min_connections = 10
connect_timeout = "10s"
idle_timeout = "600s"
max_lifetime = "3600s"
auto_migrate = false

[redis]
url = "${REDIS_URL}"
pool_size = 50
timeout = "5s"

[blockchain]
rpc_url = "${SOLANA_RPC_URL}"
ws_url = "${SOLANA_WS_URL}"
commitment = "finalized"
keypair_path = "${ENGINEERING_DEPT_KEYPAIR_PATH}"
timeout = "60s"
retry_attempts = 5

[auth]
jwt_secret = "${JWT_SECRET}"
jwt_expiration = "3600s"
refresh_token_expiration = "86400s"
bcrypt_cost = 12
api_key_header = "X-API-Key"

[monitoring]
metrics_enabled = true
metrics_port = 9090
tracing_enabled = true
jaeger_endpoint = "${JAEGER_ENDPOINT}"
log_level = "info"
```

---

## Development Environment Setup

### Development Setup Script (`scripts/setup-dev.sh`)
```bash
#!/bin/bash

set -e

echo "🚀 Setting up API Gateway development environment..."

# Check prerequisites
check_prerequisites() {
    echo "📋 Checking prerequisites..."
    
    if ! command -v rust &> /dev/null; then
        echo "❌ Rust not found. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    if ! command -v docker &> /dev/null; then
        echo "❌ Docker not found. Please install Docker"
        exit 1
    fi
    
    if ! command -v docker-compose &> /dev/null; then
        echo "❌ Docker Compose not found. Please install Docker Compose"
        exit 1
    fi
    
    if ! command -v solana &> /dev/null; then
        echo "⚠️  Solana CLI not found. Installing..."
        sh -c "$(curl -sSfL https://release.solana.com/v1.18.17/install)"
        export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
    fi
    
    if ! command -v anchor &> /dev/null; then
        echo "⚠️  Anchor CLI not found. Installing..."
        cargo install --git https://github.com/coral-xyz/anchor --tag v0.31.1 anchor-cli
    fi
    
    echo "✅ Prerequisites check completed"
}

# Setup Rust toolchain
setup_rust() {
    echo "🦀 Setting up Rust toolchain..."
    
    rustup update stable
    rustup default stable
    rustup component add rustfmt clippy
    
    # Install additional tools
    cargo install cargo-watch cargo-nextest cargo-audit
    
    echo "✅ Rust toolchain setup completed"
}

# Create environment file
setup_environment() {
    echo "⚙️  Setting up environment configuration..."
    
    if [ ! -f .env ]; then
        cp .env.example .env
        echo "📝 Created .env file from template"
        echo "🔧 Please edit .env file with your specific configuration"
    else
        echo "📁 .env file already exists"
    fi
}

# Setup development database
setup_database() {
    echo "🗄️  Setting up development database..."
    
    docker-compose up -d postgres redis
    
    # Wait for database to be ready
    echo "⏳ Waiting for database to be ready..."
    sleep 10
    
    # Run migrations
    sqlx database create --database-url "postgresql://api_user:api_password@localhost:5432/energy_trading_dev"
    sqlx migrate run --database-url "postgresql://api_user:api_password@localhost:5432/energy_trading_dev"
    
    echo "✅ Database setup completed"
}

# Setup Solana test validator
setup_blockchain() {
    echo "⛓️  Setting up Solana test validator..."
    
    # Create keypair directory
    mkdir -p keys
    
    # Generate development keypair if it doesn't exist
    if [ ! -f keys/development-authority.json ]; then
        solana-keygen new --outfile keys/development-authority.json --no-bip39-passphrase
        echo "🔑 Generated development authority keypair"
    fi
    
    # Start test validator (in background)
    pkill -f "solana-test-validator" || true
    solana-test-validator --reset --quiet &
    
    echo "⏳ Waiting for validator to start..."
    sleep 5
    
    # Set config to use local validator
    solana config set --url localhost
    solana config set --keypair keys/development-authority.json
    
    echo "✅ Blockchain setup completed"
}

# Build the project
build_project() {
    echo "🔨 Building project..."
    
    cargo build
    cargo test --no-run
    
    echo "✅ Project build completed"
}

# Main setup process
main() {
    echo "🎯 Starting API Gateway development setup..."
    echo "📍 Working directory: $(pwd)"
    
    check_prerequisites
    setup_rust
    setup_environment
    setup_database
    setup_blockchain
    build_project
    
    echo ""
    echo "🎉 Development environment setup completed!"
    echo ""
    echo "📋 Next steps:"
    echo "   1. Edit .env file with your configuration"
    echo "   2. Run 'cargo run' to start the development server"
    echo "   3. Run 'cargo test' to execute tests"
    echo "   4. Visit http://localhost:8080/health for health check"
    echo ""
    echo "🔧 Development commands:"
    echo "   cargo run              - Start development server"
    echo "   cargo test             - Run tests"
    echo "   cargo watch -x run     - Auto-reload development server"
    echo "   cargo clippy           - Run linter"
    echo "   cargo fmt              - Format code"
    echo "   docker-compose up      - Start supporting services"
    echo ""
}

main "$@"
```

### Docker Compose for Development (`docker-compose.yml`)
```yaml
version: '3.8'

services:
  # PostgreSQL Database
  postgres:
    image: timescale/timescaledb:2.12.1-pg15
    container_name: api-gateway-postgres
    environment:
      POSTGRES_DB: energy_trading_dev
      POSTGRES_USER: api_user
      POSTGRES_PASSWORD: api_password
      POSTGRES_HOST_AUTH_METHOD: trust
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U api_user -d energy_trading_dev"]
      interval: 10s
      timeout: 5s
      retries: 5

  # Redis Cache
  redis:
    image: redis:7-alpine
    container_name: api-gateway-redis
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    command: redis-server --appendonly yes
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 3

  # Solana Test Validator (for integration testing)
  solana-validator:
    build:
      context: ../docker/solana-validator
      dockerfile: Dockerfile
    container_name: api-gateway-solana
    ports:
      - "8899:8899"
      - "8900:8900"
    volumes:
      - solana_data:/solana-data
      - ./keys:/keys
    environment:
      SOLANA_RUN_SH_VALIDATOR_ARGS: "--reset --bind-address 0.0.0.0"
    healthcheck:
      test: ["CMD", "solana", "cluster-version"]
      interval: 30s
      timeout: 10s
      retries: 3

  # Prometheus (for metrics)
  prometheus:
    image: prom/prometheus:latest
    container_name: api-gateway-prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./config/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'

  # Grafana (for visualization)
  grafana:
    image: grafana/grafana:latest
    container_name: api-gateway-grafana
    ports:
      - "3000:3000"
    volumes:
      - grafana_data:/var/lib/grafana
      - ./config/grafana:/etc/grafana/provisioning
    environment:
      GF_SECURITY_ADMIN_PASSWORD: admin
      GF_USERS_ALLOW_SIGN_UP: false

  # Jaeger (for tracing)
  jaeger:
    image: jaegertracing/all-in-one:1.50
    container_name: api-gateway-jaeger
    ports:
      - "16686:16686"
      - "14268:14268"
    environment:
      COLLECTOR_OTLP_ENABLED: true

volumes:
  postgres_data:
  redis_data:
  solana_data:
  prometheus_data:
  grafana_data:

networks:
  default:
    name: api-gateway-network
```

---

## Build System

### Makefile for Development
```makefile
# Makefile for API Gateway Development

.PHONY: help build test lint format check clean setup dev docker-build docker-run

# Default target
help:
	@echo "API Gateway Development Commands"
	@echo "================================"
	@echo "setup          - Setup development environment"
	@echo "dev            - Start development server with auto-reload"
	@echo "build          - Build the project"
	@echo "test           - Run all tests"
	@echo "test-unit      - Run unit tests only"
	@echo "test-integration - Run integration tests only"
	@echo "lint           - Run clippy linter"
	@echo "format         - Format code with rustfmt"
	@echo "check          - Run all checks (build, test, lint, format)"
	@echo "clean          - Clean build artifacts"
	@echo "docker-build   - Build Docker image"
	@echo "docker-run     - Run in Docker container"
	@echo "benchmark      - Run performance benchmarks"
	@echo "docs           - Generate documentation"
	@echo "security-audit - Run security audit"

# Development environment setup
setup:
	@./scripts/setup-dev.sh

# Development server with auto-reload
dev:
	@echo "🚀 Starting development server with auto-reload..."
	@cargo watch -x "run"

# Build the project
build:
	@echo "🔨 Building project..."
	@cargo build

# Build release version
build-release:
	@echo "🔨 Building release version..."
	@cargo build --release

# Run all tests
test:
	@echo "🧪 Running all tests..."
	@cargo nextest run

# Run unit tests only
test-unit:
	@echo "🧪 Running unit tests..."
	@cargo nextest run --lib

# Run integration tests only
test-integration:
	@echo "🧪 Running integration tests..."
	@cargo nextest run --test '*'

# Run tests with coverage
test-coverage:
	@echo "🧪 Running tests with coverage..."
	@cargo tarpaulin --out Html

# Lint with clippy
lint:
	@echo "🔍 Running clippy linter..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Format code
format:
	@echo "🎨 Formatting code..."
	@cargo fmt

# Check formatting
format-check:
	@echo "🎨 Checking code formatting..."
	@cargo fmt --check

# Run all checks
check: build test lint format-check
	@echo "✅ All checks passed!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean

# Docker build
docker-build:
	@echo "🐳 Building Docker image..."
	@docker build -t api-gateway:latest .

# Docker run
docker-run:
	@echo "🐳 Running Docker container..."
	@docker-compose up --build

# Run benchmarks
benchmark:
	@echo "⚡ Running performance benchmarks..."
	@cargo bench

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	@cargo doc --no-deps --open

# Security audit
security-audit:
	@echo "🔒 Running security audit..."
	@cargo audit
	@cargo deny check

# Database operations
db-migrate:
	@echo "🗄️ Running database migrations..."
	@sqlx migrate run

db-reset:
	@echo "🗄️ Resetting database..."
	@sqlx database drop -y
	@sqlx database create
	@sqlx migrate run

# Load testing
load-test:
	@echo "⚡ Running load tests..."
	@./scripts/run-load-tests.sh

# Start all services
services-up:
	@echo "🚀 Starting all services..."
	@docker-compose up -d

# Stop all services
services-down:
	@echo "🛑 Stopping all services..."
	@docker-compose down

# View logs
logs:
	@echo "📋 Viewing logs..."
	@docker-compose logs -f

# Run specific test by name
test-specific:
	@echo "🧪 Running specific test: $(TEST_NAME)"
	@cargo nextest run $(TEST_NAME)

# Install development dependencies
install-deps:
	@echo "📦 Installing development dependencies..."
	@cargo install cargo-watch cargo-nextest cargo-audit cargo-deny cargo-tarpaulin

# Pre-commit hook
pre-commit: format lint test
	@echo "✅ Pre-commit checks completed!"
```

### Build Scripts

#### Docker Build Script (`scripts/docker-build.sh`)
```bash
#!/bin/bash

set -e

echo "🐳 Building API Gateway Docker image..."

# Build arguments
BUILD_ARGS=""
if [ ! -z "$BUILD_TARGET" ]; then
    BUILD_ARGS="$BUILD_ARGS --target $BUILD_TARGET"
fi

if [ ! -z "$REGISTRY" ]; then
    TAG="$REGISTRY/api-gateway:$TAG"
else
    TAG="api-gateway:${TAG:-latest}"
fi

# Build the image
docker build $BUILD_ARGS \
    --tag "$TAG" \
    --build-arg BUILD_DATE=$(date -u +'%Y-%m-%dT%H:%M:%SZ') \
    --build-arg VCS_REF=$(git rev-parse --short HEAD) \
    --build-arg VERSION=${VERSION:-dev} \
    .

echo "✅ Docker image built: $TAG"

# Run security scan if requested
if [ "$SCAN" = "true" ]; then
    echo "🔒 Running security scan..."
    docker scan "$TAG"
fi

# Push if requested
if [ "$PUSH" = "true" ]; then
    echo "📤 Pushing image to registry..."
    docker push "$TAG"
fi
```

---

## Code Organization Principles

### 1. Separation of Concerns
- **Handlers**: HTTP request/response handling only
- **Services**: Business logic and orchestration
- **Repositories**: Data access abstraction
- **Models**: Data structures and DTOs
- **Utils**: Pure functions and utilities

### 2. Dependency Injection
```rust
// Example of dependency injection pattern
pub struct EnergyService {
    energy_repo: Arc<dyn EnergyRepository>,
    blockchain_client: Arc<dyn BlockchainClient>,
    cache_client: Arc<dyn CacheClient>,
}

impl EnergyService {
    pub fn new(
        energy_repo: Arc<dyn EnergyRepository>,
        blockchain_client: Arc<dyn BlockchainClient>,
        cache_client: Arc<dyn CacheClient>,
    ) -> Self {
        Self {
            energy_repo,
            blockchain_client,
            cache_client,
        }
    }
}
```

### 3. Error Handling Strategy
```rust
// Centralized error handling
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Blockchain error: {0}")]
    Blockchain(#[from] BlockchainError),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication failed")]
    Authentication,
    
    #[error("Authorization failed")]
    Authorization,
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Authentication => (StatusCode::UNAUTHORIZED, "Authentication failed".to_string()),
            ApiError::Authorization => (StatusCode::FORBIDDEN, "Authorization failed".to_string()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };
        
        let body = Json(serde_json::json!({
            "error": {
                "message": error_message,
                "code": status.as_u16()
            }
        }));
        
        (status, body).into_response()
    }
}
```

### 4. Testing Strategy
```rust
// Example test organization
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_energy_service_submit_reading() {
        // Arrange
        let mut mock_repo = MockEnergyRepository::new();
        let mut mock_blockchain = MockBlockchainClient::new();
        let mut mock_cache = MockCacheClient::new();
        
        mock_repo
            .expect_create_reading()
            .times(1)
            .returning(|_, _| Ok(EnergyReading::default()));
        
        mock_blockchain
            .expect_submit_to_oracle()
            .times(1)
            .returning(|_| Ok(OracleResult::default()));
        
        let service = EnergyService::new(
            Arc::new(mock_repo),
            Arc::new(mock_blockchain),
            Arc::new(mock_cache),
        );
        
        // Act
        let result = service.submit_energy_reading(
            Uuid::new_v4(),
            EnergyReadingSubmission::default(),
        ).await;
        
        // Assert
        assert!(result.is_ok());
    }
}
```

### 5. Configuration Management
```rust
// Layered configuration approach
impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start with default configuration
            .add_source(File::with_name("config/default"))
            // Layer environment-specific config
            .add_source(File::with_name(&format!("config/{}", env::var("ENV").unwrap_or_else(|_| "development".into()))))
            // Layer local config (for local overrides)
            .add_source(File::with_name("config/local").required(false))
            // Layer environment variables
            .add_source(Environment::with_prefix("API_GATEWAY"))
            .build()?;
            
        config.try_deserialize()
    }
}
```

This comprehensive project structure plan provides a solid foundation for implementing the API Gateway with clear organization, robust dependency management, and comprehensive development tooling. The structure supports scalability, maintainability, and testing while following Rust best practices.