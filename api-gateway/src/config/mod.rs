use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub environment: String,
    pub port: u16,
    pub database_url: String,
    pub timescale_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub solana_rpc_url: String,
    pub solana_ws_url: String,
    pub engineering_api_key: String,
    pub max_connections: u32,
    pub redis_pool_size: u32,
    pub request_timeout: u64,
    pub rate_limit_window: u64,
    pub log_level: String,
    pub audit_log_enabled: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok(); // Load .env file if it exists

        Ok(Config {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/energy_trading".to_string()),
            timescale_url: env::var("TIMESCALE_URL")
                .unwrap_or_else(|_| "postgresql://timescale_user:timescale_password@localhost:5433/p2p_timeseries".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key".to_string()),
            solana_rpc_url: env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "http://localhost:8899".to_string()),
            solana_ws_url: env::var("SOLANA_WS_URL")
                .unwrap_or_else(|_| "ws://localhost:8900".to_string()),
            engineering_api_key: env::var("ENGINEERING_API_KEY")
                .unwrap_or_else(|_| "engineering-department-api-key-2025".to_string()),
            max_connections: env::var("MAX_CONNECTIONS")
                .unwrap_or_else(|_| "50".to_string())
                .parse()?,
            redis_pool_size: env::var("REDIS_POOL_SIZE")
                .unwrap_or_else(|_| "20".to_string())
                .parse()?,
            request_timeout: env::var("REQUEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()?,
            rate_limit_window: env::var("RATE_LIMIT_WINDOW")
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
            audit_log_enabled: env::var("AUDIT_LOG_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()?,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            environment: "development".to_string(),
            port: 8080,
            database_url: "postgresql://user:password@localhost:5432/energy_trading".to_string(),
            timescale_url: "postgresql://timescale_user:timescale_password@localhost:5433/p2p_timeseries".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            jwt_secret: "your-super-secret-jwt-key".to_string(),
            solana_rpc_url: "http://localhost:8899".to_string(),
            solana_ws_url: "ws://localhost:8900".to_string(),
            engineering_api_key: "engineering-department-api-key-2025".to_string(),
            max_connections: 50,
            redis_pool_size: 20,
            request_timeout: 30,
            rate_limit_window: 60,
            log_level: "info".to_string(),
            audit_log_enabled: true,
        }
    }
}