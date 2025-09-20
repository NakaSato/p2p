# API Gateway Specification for P2P Energy Trading System

## Overview

This document specifies the design of the API Gateway that serves as the bridge between external applications and the Solana blockchain infrastructure for the P2P Energy Trading System. The API Gateway handles all external interactions, authentication, data validation, and blockchain communication.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Technology Stack](#technology-stack)
3. [API Endpoints Specification](#api-endpoints-specification)
4. [Authentication & Authorization](#authentication--authorization)
5. [Blockchain Integration Layer](#blockchain-integration-layer)
6. [Data Models](#data-models)
7. [Error Handling](#error-handling)
8. [Performance & Scalability](#performance--scalability)
9. [Security Considerations](#security-considerations)
10. [Deployment Configuration](#deployment-configuration)

---

## Architecture Overview

### System Architecture Diagram

```mermaid
graph TB
    subgraph "External Systems"
        AMI["Engineering AMI<br/>Smart Meters"]
        Frontend["React Frontend<br/>Engineering Portal"]
        Mobile["Mobile App<br/>(Future)"]
        External["External APIs<br/>Weather, Grid Data"]
    end
    
    subgraph "API Gateway Layer"
        LB["Load Balancer<br/>Nginx"]
        Auth["Authentication<br/>Service"]
        Gateway["API Gateway<br/>Rust/Axum"]
        Cache["Redis Cache"]
        Queue["Message Queue<br/>Redis Streams"]
    end
    
    subgraph "Business Logic Layer"
        Validator["Data Validator"]
        Processor["Transaction Processor"]
        Monitor["Event Monitor"]
        Scheduler["Market Scheduler"]
    end
    
    subgraph "Blockchain Layer"
        Solana["Engineering Dept<br/>Solana Validator"]
        Programs["Anchor Programs"]
    end
    
    subgraph "Data Layer"
        TimescaleDB["TimescaleDB<br/>Time-series Data"]
        PostgreSQL["PostgreSQL<br/>Application Data"]
    end
    
    AMI --> LB
    Frontend --> LB
    Mobile --> LB
    External --> LB
    
    LB --> Auth
    Auth --> Gateway
    Gateway --> Cache
    Gateway --> Queue
    
    Gateway --> Validator
    Gateway --> Processor
    Gateway --> Monitor
    
    Processor --> Solana
    Monitor --> Solana
    Scheduler --> Programs
    
    Gateway --> TimescaleDB
    Gateway --> PostgreSQL
```

### Core Components

1. **API Gateway Core**: Rust/Axum-based HTTP server
2. **Authentication Service**: JWT-based auth with Engineering Department integration
3. **Blockchain Client**: Solana RPC client with transaction management
4. **Event Processor**: Real-time blockchain event monitoring
5. **Data Validator**: Input validation and sanitization
6. **Cache Layer**: Redis for performance optimization
7. **Message Queue**: Async processing for blockchain transactions

---

## Technology Stack

### Backend Framework
- **Language**: Rust (Edition 2021)
- **Framework**: Axum 0.7+ (async web framework)
- **Database**: PostgreSQL 15+ with TimescaleDB extension
- **Cache**: Redis 7+
- **Message Queue**: Redis Streams
- **Blockchain Client**: Solana Web3.js (Rust bindings)

### Development & Deployment
- **Containerization**: Docker & Docker Compose
- **Reverse Proxy**: Nginx
- **Monitoring**: Prometheus + Grafana
- **Logging**: Structured logging with serde_json

### Dependencies
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "chrono", "uuid"] }
redis = { version = "0.24", features = ["tokio-comp"] }
solana-client = "1.18"
solana-sdk = "1.18"
anchor-client = "0.31"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
jsonwebtoken = "9.0"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
thiserror = "1.0"
```

---

## API Endpoints Specification

### Base Configuration
- **Base URL**: `https://api.engineering-energy.campus.local/v1`
- **Content-Type**: `application/json`
- **Authentication**: Bearer Token (JWT)

### 1. Authentication Endpoints

#### POST `/auth/login`
Engineering Department user authentication.

**Request:**
```json
{
  "username": "eng_student_001",
  "password": "secure_password",
  "department": "engineering",
  "role": "student" | "faculty" | "admin"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "user": {
    "id": "uuid",
    "username": "eng_student_001",
    "role": "student",
    "department": "engineering",
    "wallet_address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"
  }
}
```

#### POST `/auth/refresh`
Refresh JWT token.

**Request:**
```json
{
  "refresh_token": "refresh_token_here"
}
```

### 2. User Management Endpoints

#### GET `/users/profile`
Get current user profile and blockchain registration status.

**Response:**
```json
{
  "user": {
    "id": "uuid",
    "username": "eng_student_001",
    "role": "student",
    "department": "engineering",
    "wallet_address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
    "blockchain_registered": true,
    "meter_assignments": ["ENG_001", "ENG_002"]
  },
  "balances": {
    "grid_tokens": "125.500000000",
    "pending_trades": "25.000000000"
  }
}
```

#### POST `/users/register-blockchain`
Register user on Solana blockchain through Registry Program.

**Request:**
```json
{
  "wallet_address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
  "user_type": "prosumer" | "consumer",
  "location": "Engineering Building A, Room 301"
}
```

**Response:**
```json
{
  "transaction_signature": "2pEGQr8G7Pq9WYm3...blockchain_tx_signature",
  "status": "confirmed",
  "user_account_pda": "Generated_PDA_Address",
  "registration_time": "2025-09-13T10:30:00Z"
}
```

### 3. Smart Meter Integration Endpoints

#### POST `/meters/reading`
Submit energy meter reading from Engineering AMI systems.

**Authentication**: Engineering Department API Key + Digital Signature

**Request:**
```json
{
  "meter_id": "ENG_001",
  "timestamp": "2025-09-13T10:15:00Z",
  "energy_generated": "15.750000000",  // kWh (9 decimal precision)
  "energy_consumed": "12.250000000",   // kWh
  "solar_irradiance": "850.5",         // W/m²
  "temperature": "32.5",               // °C
  "engineering_authority_signature": "hex_signature",
  "metadata": {
    "location": "Engineering Building Rooftop",
    "panel_efficiency": "22.1",
    "weather_conditions": "sunny"
  }
}
```

**Response:**
```json
{
  "status": "accepted",
  "processing_id": "uuid",
  "oracle_submission": {
    "transaction_signature": "oracle_tx_signature",
    "status": "pending",
    "estimated_confirmation": "2025-09-13T10:16:30Z"
  },
  "token_minting": {
    "net_energy": "3.500000000",       // Generated - Consumed
    "grid_tokens_to_mint": "3.500000000",
    "recipient_wallet": "meter_owner_wallet"
  }
}
```

#### GET `/meters/{meter_id}/history`
Get historical data for specific meter.

**Query Parameters:**
- `start_time`: ISO 8601 timestamp
- `end_time`: ISO 8601 timestamp
- `interval`: `15min` | `1hour` | `1day`

**Response:**
```json
{
  "meter_id": "ENG_001",
  "data_points": [
    {
      "timestamp": "2025-09-13T10:00:00Z",
      "energy_generated": "14.250000000",
      "energy_consumed": "11.750000000",
      "net_energy": "2.500000000",
      "blockchain_tx": "tx_signature_if_submitted"
    }
  ],
  "total_records": 96,
  "page": 1,
  "page_size": 50
}
```

### 4. Trading Endpoints

#### GET `/trading/market`
Get current market status and recent trades.

**Response:**
```json
{
  "market_epoch": {
    "current_epoch": 1234,
    "epoch_start": "2025-09-13T10:00:00Z",
    "epoch_end": "2025-09-13T10:15:00Z",
    "next_clearing": "2025-09-13T10:15:00Z",
    "status": "active" | "clearing" | "closed"
  },
  "order_book": {
    "sell_orders": [
      {
        "order_id": "uuid",
        "seller": "wallet_address",
        "energy_amount": "10.000000000",
        "price_per_kwh": "0.250000000",
        "timestamp": "2025-09-13T10:05:00Z"
      }
    ],
    "buy_orders": [
      {
        "order_id": "uuid",
        "buyer": "wallet_address",
        "energy_amount": "5.000000000",
        "price_per_kwh": "0.240000000",
        "timestamp": "2025-09-13T10:07:00Z"
      }
    ]
  },
  "recent_trades": [
    {
      "trade_id": "uuid",
      "seller": "wallet_address",
      "buyer": "wallet_address",
      "energy_amount": "2.500000000",
      "price_per_kwh": "0.245000000",
      "total_cost": "0.612500000",
      "timestamp": "2025-09-13T10:10:00Z",
      "transaction_signature": "trade_tx_signature"
    }
  ]
}
```

#### POST `/trading/sell-order`
Create a sell order for energy tokens.

**Request:**
```json
{
  "energy_amount": "10.000000000",      // GRID tokens to sell
  "price_per_kwh": "0.250000000",       // Price in USDC per kWh
  "expiry_time": "2025-09-13T11:00:00Z", // Optional
  "order_type": "market" | "limit"
}
```

**Response:**
```json
{
  "order_id": "uuid",
  "status": "submitted",
  "transaction_signature": "order_creation_tx",
  "order_details": {
    "seller": "user_wallet_address",
    "energy_amount": "10.000000000",
    "price_per_kwh": "0.250000000",
    "total_value": "2.500000000",
    "created_at": "2025-09-13T10:12:00Z",
    "expires_at": "2025-09-13T11:00:00Z"
  }
}
```

#### POST `/trading/buy-order`
Create a buy order for energy tokens.

**Request:**
```json
{
  "energy_amount": "5.000000000",
  "max_price_per_kwh": "0.260000000",
  "expiry_time": "2025-09-13T11:00:00Z",
  "order_type": "market" | "limit"
}
```

#### DELETE `/trading/orders/{order_id}`
Cancel an existing order.

**Response:**
```json
{
  "status": "cancelled",
  "transaction_signature": "cancellation_tx",
  "refund_details": {
    "tokens_returned": "10.000000000",
    "fees_refunded": "0.001000000"
  }
}
```

### 5. Blockchain Interaction Endpoints

#### POST `/blockchain/transaction`
Submit raw transaction to Solana blockchain.

**Request:**
```json
{
  "transaction": "base64_encoded_transaction",
  "program_id": "registry" | "trading" | "oracle" | "governance",
  "priority_fee": "0.000001",  // SOL
  "compute_units": 200000
}
```

#### GET `/blockchain/transaction/{signature}`
Get transaction status and details.

**Response:**
```json
{
  "signature": "transaction_signature",
  "status": "confirmed" | "finalized" | "failed",
  "block_height": 123456789,
  "confirmation_status": "confirmed",
  "fee": "0.000005",
  "compute_units_consumed": 150000,
  "logs": ["Program log: Instruction executed successfully"],
  "program_interactions": [
    {
      "program_id": "Registry Program",
      "instruction": "register_user",
      "accounts_modified": ["user_pda", "registry_state"]
    }
  ]
}
```

### 6. Analytics & Reporting Endpoints

#### GET `/analytics/energy-flow`
Get energy flow analytics for Engineering Department.

**Query Parameters:**
- `timeframe`: `1hour` | `1day` | `1week` | `1month`
- `meter_ids`: Comma-separated list of meter IDs

**Response:**
```json
{
  "timeframe": "1day",
  "total_generation": "450.750000000",  // kWh
  "total_consumption": "380.250000000", // kWh
  "net_surplus": "70.500000000",        // kWh
  "trades_executed": 25,
  "total_trade_volume": "65.250000000", // GRID tokens
  "average_price": "0.248000000",       // Per kWh
  "top_producers": [
    {
      "meter_id": "ENG_001",
      "location": "Engineering Rooftop South",
      "generation": "45.250000000"
    }
  ],
  "carbon_offset": {
    "co2_reduced": "32.5",  // kg CO2
    "trees_equivalent": "1.5"
  }
}
```

#### GET `/analytics/market-performance`
Get trading market performance metrics.

**Response:**
```json
{
  "market_metrics": {
    "total_volume_24h": "125.750000000",
    "trades_count_24h": 45,
    "average_price_24h": "0.252000000",
    "price_volatility": "0.025",  // 2.5%
    "liquidity_ratio": "0.85"
  },
  "price_history": [
    {
      "timestamp": "2025-09-13T09:00:00Z",
      "price": "0.245000000",
      "volume": "12.500000000"
    }
  ]
}
```

---

## Authentication & Authorization

### JWT Token Structure
```json
{
  "sub": "user_uuid",
  "username": "eng_student_001",
  "role": "student" | "faculty" | "admin",
  "department": "engineering",
  "wallet_address": "solana_wallet_address",
  "permissions": [
    "energy:read",
    "energy:trade",
    "meters:read",
    "analytics:read"
  ],
  "iat": 1694601600,
  "exp": 1694605200
}
```

### Permission Levels

#### Student Role
- `energy:read` - View energy data and balances
- `energy:trade` - Create buy/sell orders
- `meters:read` - View assigned meter data
- `analytics:read` - Basic analytics access

#### Faculty Role
- All student permissions
- `meters:manage` - Assign meters to students
- `analytics:advanced` - Advanced analytics
- `system:monitor` - System health monitoring

#### Admin Role
- All faculty permissions
- `system:manage` - System configuration
- `blockchain:admin` - Blockchain administration
- `users:manage` - User management

### API Key Authentication (AMI Systems)
```http
Authorization: ApiKey engineering-dept-key-2025
X-Signature: HMAC-SHA256 signature of request body
X-Timestamp: Unix timestamp (for replay protection)
```

---

## Blockchain Integration Layer

### Solana Client Configuration
```rust
pub struct BlockchainClient {
    pub rpc_client: RpcClient,
    pub pubsub_client: PubsubClient,
    pub programs: ProgramClients,
    pub keypair: Keypair,  // Engineering Department authority
}

pub struct ProgramClients {
    pub registry: Program<Registry>,
    pub energy_token: Program<EnergyToken>,
    pub trading: Program<Trading>,
    pub oracle: Program<Oracle>,
    pub governance: Program<Governance>,
}
```

### Transaction Management
```rust
pub struct TransactionBuilder {
    instructions: Vec<Instruction>,
    signers: Vec<&Keypair>,
    priority_fee: u64,
    compute_units: u32,
}

impl TransactionBuilder {
    pub async fn submit_with_retry(&self) -> Result<Signature, BlockchainError> {
        // Implement transaction submission with retry logic
        // Include priority fee estimation
        // Handle transaction confirmation polling
    }
}
```

### Event Monitoring
```rust
pub struct EventMonitor {
    subscriptions: HashMap<String, Subscription>,
    event_handlers: HashMap<ProgramId, Box<dyn EventHandler>>,
}

#[async_trait]
pub trait EventHandler {
    async fn handle_event(&self, event: ProgramEvent) -> Result<(), EventError>;
}
```

---

## Data Models

### Database Schema (PostgreSQL + TimescaleDB)

#### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(100) UNIQUE NOT NULL,
    email VARCHAR(255),
    role user_role NOT NULL,
    department VARCHAR(100) NOT NULL,
    wallet_address VARCHAR(44),
    blockchain_registered BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TYPE user_role AS ENUM ('student', 'faculty', 'admin');
```

#### Energy Readings Table (TimescaleDB Hypertable)
```sql
CREATE TABLE energy_readings (
    id UUID DEFAULT gen_random_uuid(),
    meter_id VARCHAR(20) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    energy_generated DECIMAL(18,9) NOT NULL,
    energy_consumed DECIMAL(18,9) NOT NULL,
    net_energy DECIMAL(18,9) GENERATED ALWAYS AS (energy_generated - energy_consumed) STORED,
    solar_irradiance DECIMAL(8,2),
    temperature DECIMAL(5,2),
    blockchain_submitted BOOLEAN DEFAULT FALSE,
    transaction_signature VARCHAR(88),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create hypertable for time-series optimization
SELECT create_hypertable('energy_readings', 'timestamp');
```

#### Trading Orders Table
```sql
CREATE TABLE trading_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    order_type order_type_enum NOT NULL,
    side order_side_enum NOT NULL,
    energy_amount DECIMAL(18,9) NOT NULL,
    price_per_kwh DECIMAL(18,9) NOT NULL,
    status order_status_enum DEFAULT 'pending',
    blockchain_signature VARCHAR(88),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    filled_amount DECIMAL(18,9) DEFAULT 0,
    filled_at TIMESTAMPTZ
);

CREATE TYPE order_type_enum AS ENUM ('market', 'limit');
CREATE TYPE order_side_enum AS ENUM ('buy', 'sell');
CREATE TYPE order_status_enum AS ENUM ('pending', 'active', 'filled', 'cancelled', 'expired');
```

### Rust Data Models
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: UserRole,
    pub department: String,
    pub wallet_address: Option<String>,
    pub blockchain_registered: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyReading {
    pub meter_id: String,
    pub timestamp: DateTime<Utc>,
    pub energy_generated: Decimal,
    pub energy_consumed: Decimal,
    pub net_energy: Decimal,
    pub metadata: Option<EnergyMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingOrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub energy_amount: Decimal,
    pub price_per_kwh: Decimal,
    pub status: OrderStatus,
}
```

---

## Error Handling

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Authorization failed: insufficient permissions")]
    Authorization,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Blockchain error: {0}")]
    Blockchain(#[from] BlockchainError),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("External service error: {0}")]
    ExternalService(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Internal server error")]
    Internal,
}

#[derive(Debug, thiserror::Error)]
pub enum BlockchainError {
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Program error: {0}")]
    ProgramError(String),
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Network error: {0}")]
    Network(String),
}
```

### Error Response Format
```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid energy amount: must be positive",
    "details": {
      "field": "energy_amount",
      "provided_value": "-5.0",
      "constraint": "must be > 0"
    },
    "request_id": "uuid",
    "timestamp": "2025-09-13T10:15:00Z"
  }
}
```

---

## Performance & Scalability

### Caching Strategy
```rust
pub struct CacheManager {
    redis: redis::Client,
    ttl_config: TtlConfig,
}

pub struct TtlConfig {
    user_sessions: Duration,      // 1 hour
    market_data: Duration,        // 5 minutes
    energy_readings: Duration,    // 15 minutes
    analytics: Duration,          // 30 minutes
}
```

### Rate Limiting
```rust
pub struct RateLimiter {
    redis: redis::Client,
    limits: HashMap<String, RateLimit>,
}

pub struct RateLimit {
    requests_per_minute: u32,
    burst_size: u32,
    window_size: Duration,
}

// Configuration
let rate_limits = HashMap::from([
    ("default", RateLimit { requests_per_minute: 60, burst_size: 10, window_size: Duration::from_secs(60) }),
    ("trading", RateLimit { requests_per_minute: 30, burst_size: 5, window_size: Duration::from_secs(60) }),
    ("ami_submission", RateLimit { requests_per_minute: 240, burst_size: 20, window_size: Duration::from_secs(60) }),
]);
```

### Connection Pooling
```rust
pub struct DatabaseConfig {
    max_connections: u32,      // 20
    min_connections: u32,      // 5
    connect_timeout: Duration, // 10 seconds
    idle_timeout: Duration,    // 300 seconds
    max_lifetime: Duration,    // 3600 seconds
}
```

---

## Security Considerations

### Input Validation
```rust
pub fn validate_energy_reading(reading: &EnergyReading) -> Result<(), ValidationError> {
    // Validate energy amounts (must be non-negative)
    if reading.energy_generated < Decimal::ZERO {
        return Err(ValidationError::NegativeEnergy);
    }
    
    // Validate timestamp (must be recent, within 1 hour)
    let now = Utc::now();
    if (now - reading.timestamp).num_hours() > 1 {
        return Err(ValidationError::StaleTimestamp);
    }
    
    // Validate meter ID format
    if !reading.meter_id.starts_with("ENG_") {
        return Err(ValidationError::InvalidMeterFormat);
    }
    
    Ok(())
}
```

### CORS Configuration
```rust
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("https://energy.engineering.campus.local".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            HeaderName::from_static("x-api-key"),
        ])
        .max_age(Duration::from_secs(3600))
}
```

### Audit Logging
```rust
#[derive(Debug, Serialize)]
pub struct AuditLog {
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource: String,
    pub timestamp: DateTime<Utc>,
    pub ip_address: IpAddr,
    pub user_agent: Option<String>,
    pub request_id: Uuid,
    pub success: bool,
    pub error_message: Option<String>,
}
```

---

## Deployment Configuration

### Docker Configuration
```dockerfile
# Dockerfile for API Gateway
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api-gateway /usr/local/bin/
EXPOSE 8080
CMD ["api-gateway"]
```

### Docker Compose Service
```yaml
# docker-compose.yml
version: '3.8'
services:
  api-gateway:
    build: 
      context: ./api-gateway
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://user:pass@postgres:5432/energy_trading
      - REDIS_URL=redis://redis:6379
      - SOLANA_RPC_URL=http://solana-validator:8899
      - JWT_SECRET=${JWT_SECRET}
      - ENGINEERING_DEPT_KEYPAIR_PATH=/keys/engineering-dept.json
    volumes:
      - ./keys:/keys:ro
      - ./logs:/app/logs
    depends_on:
      - postgres
      - redis
      - solana-validator
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

### Environment Variables
```bash
# Production Environment
DATABASE_URL=postgresql://user:password@postgres:5432/energy_trading
REDIS_URL=redis://redis:6379
SOLANA_RPC_URL=http://engineering-validator.campus.local:8899
SOLANA_WS_URL=ws://engineering-validator.campus.local:8900

# Security
JWT_SECRET=your-super-secret-jwt-key
ENGINEERING_API_KEY=engineering-department-api-key-2025
HMAC_SECRET=hmac-signature-secret

# Engineering Department Authority
ENGINEERING_DEPT_KEYPAIR_PATH=/keys/engineering-department-authority.json

# Performance
MAX_CONNECTIONS=50
REDIS_POOL_SIZE=20
REQUEST_TIMEOUT=30
RATE_LIMIT_WINDOW=60

# Logging
LOG_LEVEL=info
LOG_FORMAT=json
AUDIT_LOG_ENABLED=true
```

### Nginx Configuration
```nginx
upstream api_gateway {
    server api-gateway-1:8080;
    server api-gateway-2:8080;
    keepalive 32;
}

server {
    listen 443 ssl http2;
    server_name api.engineering-energy.campus.local;
    
    ssl_certificate /etc/ssl/certs/engineering-energy.crt;
    ssl_certificate_key /etc/ssl/private/engineering-energy.key;
    
    location /v1/ {
        proxy_pass http://api_gateway;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
        
        # Rate limiting
        limit_req zone=api burst=10 nodelay;
        
        # Timeouts
        proxy_connect_timeout 10s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;
    }
    
    location /health {
        proxy_pass http://api_gateway;
        access_log off;
    }
}

# Rate limiting zone
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
```

---

## Monitoring & Observability

### Health Check Endpoint
```rust
pub async fn health_check() -> Json<HealthStatus> {
    let mut status = HealthStatus::new();
    
    // Check database connectivity
    status.database = check_database_health().await;
    
    // Check Redis connectivity
    status.cache = check_redis_health().await;
    
    // Check Solana validator connectivity
    status.blockchain = check_solana_health().await;
    
    // Check service dependencies
    status.dependencies = check_dependencies_health().await;
    
    Json(status)
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub database: ServiceHealth,
    pub cache: ServiceHealth,
    pub blockchain: ServiceHealth,
    pub dependencies: Vec<ServiceHealth>,
}
```

### Metrics Export (Prometheus)
```rust
pub struct Metrics {
    pub requests_total: IntCounterVec,
    pub request_duration: HistogramVec,
    pub blockchain_transactions: IntCounterVec,
    pub energy_readings_processed: IntCounter,
    pub active_connections: IntGauge,
}

pub fn init_metrics() -> Metrics {
    let requests_total = register_int_counter_vec!(
        "api_requests_total",
        "Total number of API requests",
        &["method", "endpoint", "status"]
    ).expect("metric can be created");
    
    let request_duration = register_histogram_vec!(
        "api_request_duration_seconds",
        "Request duration in seconds",
        &["method", "endpoint"]
    ).expect("metric can be created");
    
    Metrics {
        requests_total,
        request_duration,
        // ... other metrics
    }
}
```

This comprehensive API Gateway specification provides a complete blueprint for implementing the external interface to your Solana blockchain-based P2P energy trading system. The design focuses on security, performance, and maintainability while providing all necessary endpoints for Engineering Department operations.