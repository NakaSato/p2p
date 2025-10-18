# AI Copilot Instructions for P2P Energy Trading System

## Project Overview

This is a **production-ready Solana Anchor-based P2P energy trading platform** for campus environments (92% complete). The system enables peer-to-peer renewable energy trading with:
- **5 Anchor programs** (Solana smart contracts) deployed on PoA network
- **Rust/Axum API Gateway** (23 REST endpoints)
- **React/TypeScript frontend** with Web3 integration
- **PostgreSQL + TimescaleDB** for energy time-series data
- **Docker infrastructure** with 11 containerized services

### Key Insight: Single Authority Design
This is **not** a decentralized network. The Engineering Department is the sole **Proof of Authority (PoA) validator** and system authority. This simplified design prioritizes security and operational control over decentralization.

---

## Architecture Layers

### Layer 1: Solana Smart Contracts (Programs)
**Location**: `programs/{registry,energy-token,oracle,trading,governance}/src/lib.rs`

| Program | Purpose | Key Pattern |
|---------|---------|------------|
| **Registry** | User/meter registration with UUID management | `declare_id!()` + context-based instruction handlers |
| **Energy Token** | REC-validated SPL token with automated minting | Uses token program interface + PDA seeds |
| **Trading** | Order book + automated market matching | Order state machine + settlement engine |
| **Oracle** | API Gateway-exclusive authorization layer | Validates API Gateway signatures on minting |
| **Governance** | PoA consensus (Engineering Dept = sole validator) | Authority checks via `has_one = authority` constraint |

**Pattern**: All programs use Anchor's `#[derive(Accounts)]` for account validation and `#[program]` macro for instruction handlers.

```rust
// Example pattern from registry/lib.rs
pub fn register_user(
    ctx: Context<RegisterUser>,
    user_type: UserType,
    location: String,
) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;
    // Anchor validates accounts in RegisterUser struct automatically
    user_account.authority = ctx.accounts.user_authority.key();
    // ...
}
```

### Layer 2: API Gateway
**Location**: `api-gateway/src/`

- **Framework**: Axum (Tokio async runtime) with middleware stack
- **Database**: PostgreSQL (user/transaction data) + TimescaleDB (time-series energy data)
- **Cache**: Redis for session management
- **Auth**: JWT tokens + API Keys (for AMI smart meters)

**23 Endpoints** organized by handler modules:
- `handlers/auth.rs` - Login, registration, token refresh
- `handlers/users.rs` - User profile management
- `handlers/blockchain.rs` - Direct on-chain interactions
- `handlers/trading.rs` - Order creation/matching
- `handlers/meters.rs` - Smart meter data ingestion (15 METER_001-015)
- `handlers/analytics.rs` - Trading analytics + forecasting
- `handlers/health.rs` - System health checks

**Middleware stack** (`middleware/mod.rs`):
1. `TraceLayer` - Request/response logging
2. `CorsLayer` - Cross-origin resource sharing
3. `TimeoutLayer` - 30s request timeout
4. `JwtMiddleware` - Token validation
5. `RateLimitMiddleware` - Per-user rate limiting

### Layer 3: Frontend
**Location**: `trading-frontend/`

- **Framework**: Next.js 15 with React 19 + TypeScript
- **Web3**: `@solana/web3.js` + `@coral-xyz/anchor` for program interaction
- **UI**: Tailwind CSS + Radix UI components
- **Charts**: Chart.js + Recharts for energy analytics
- **Wallet**: Phantom wallet integration via `@solana/wallet-adapter-react`

**Key routes**:
- `/` - Dashboard with portfolio overview
- `/trading` - Order book and trading interface
- `/earn` - Passive income opportunities
- `/futures` - Derivative trading
- `/analytics` - Energy consumption analytics
- `/leaderboards` - Trading competitions

### Layer 4: Infrastructure
**Location**: `docker-compose.yml` + `docker/*/`

**11 Services**:
```
solana-validator → Anchor dev chain (port 8898)
contact → Deployment service (runs once)
api-gateway → Rust service (port 3000)
postgres → User/transaction data
timescaledb → Time-series energy data
redis → Session cache
prometheus → Metrics collection
grafana → Monitoring dashboards
nginx → Reverse proxy
smart-meter-simulator → AMI mock (15 meters)
oracle-simulator → Price feed mock
```

---

## Critical Developer Workflows

### 1. Build & Deploy Smart Contracts

```bash
# Build all Anchor programs
anchor build

# Deploy to local PoA network
anchor deploy --provider.cluster localnet

# Run on-chain tests
anchor test
```

**Key files**:
- `Anchor.toml` - Program IDs and cluster configuration
- `programs.config.toml` - Per-program build settings
- `programs/*/Cargo.toml` - Dependencies (must align with anchor version)

### 2. Run Development Environment

```bash
# Automated setup (includes PoA + Docker)
./scripts/dev-setup.sh

# Manual steps:
./scripts/setup-dev.sh          # Install dependencies
./scripts/setup-poa.sh          # Initialize PoA network
docker-compose up -d            # Start all services
```

**Health checks**:
- `http://localhost:3000/health` - API Gateway
- `http://localhost:3001/metrics` - Prometheus
- `http://localhost:8899` - Solana RPC (mapped from 8898)

### 3. Test Smart Contracts

```bash
# In root directory (runs TypeScript tests)
npm test

# Or manually with Mocha
ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts
```

**Test location**: `tests/` (TypeScript + Chai assertions)
- Use `anchor.setProvider()` to connect to local validator
- Create test accounts in `test-accounts/` for deterministic testing

### 4. Test API Gateway

```bash
# From api-gateway directory
cargo test

# With logging
RUST_LOG=api_gateway=debug cargo test

# Run specific test
cargo test health_test
```

**Test files**: `api-gateway/tests/*.rs`

### 5. Smart Meter Data Integration

The system expects 15 smart meters (METER_001 through METER_015) via:
1. **HTTP POST** to `/api/v1/meters/{meter_id}/data` - Real-time updates
2. **MQTT** - Alternative ingestion (configured in docker-compose)
3. **Simulator** - `docker/smart-meter-simulator/` for dev/test

**Data format** (TimescaleDB):
```json
{
  "meter_id": "METER_001",
  "timestamp": "2025-01-15T14:30:00Z",
  "energy_consumed_kwh": 15.5,
  "solar_generated_kwh": 22.0,
  "grid_exported_kwh": 6.5
}
```

---

## Project-Specific Patterns & Conventions

### 1. Account/Address Management in Solana

**PDAs (Program Derived Addresses)**: Used for deterministic account generation
```rust
// In registry program
let (user_pda, _bump) = Pubkey::find_program_address(
    &[b"user", user_authority.key().as_ref()],
    ctx.program_id,
);
```

**UUIDs for Off-Chain Tracking**: Smart meters and users have UUIDs in database
```sql
-- api-gateway/schema.sql
CREATE TABLE meters (
  id UUID PRIMARY KEY,
  meter_code VARCHAR(20) UNIQUE,  -- e.g., "METER_001"
  user_id UUID REFERENCES users(id)
);
```

### 2. Authentication Flow

**JWT + API Key Hybrid**:
1. User logs in → receive JWT (24hr expiry)
2. AMI systems use API Keys (persistent)
3. API Gateway validates both in middleware

```rust
// From auth/jwt.rs - standard practice
pub async fn validate_token(token: &str, secret: &str) -> Result<Claims> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
```

### 3. Error Handling

**Rust Result Type** everywhere. API Gateway converts to HTTP responses:
```rust
// From api-gateway/src/error.rs
pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Blockchain error: {0}")]
    Blockchain(String),
    // ...
}

// Automatic HTTP mapping
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            // ...
        };
        // Return JSON error response
    }
}
```

### 4. Database Migrations

**SQLx** manages SQL with compile-time verification:
```bash
# Create migration
sqlx migrate add -r create_users_table

# Run migrations (automatic on API startup)
sqlx migrate run --database-url $DATABASE_URL
```

**Migrations location**: `api-gateway/migrations/`

### 5. Configuration Management

**Environment-based via `.env`**:
```env
DATABASE_URL=postgres://user:pass@localhost/p2p_energy
TIMESCALE_URL=postgres://user:pass@localhost/timescale
REDIS_URL=redis://localhost:6379
JWT_SECRET=<base64-encoded-secret>
ANCHOR_PROVIDER_URL=http://localhost:8899
```

**Loaded in**:
- Rust: `config::Config::from_env()` in `api-gateway/src/config/mod.rs`
- Frontend: `.env.local` → `next.config.ts`

---

## Cross-Component Communication

### Smart Contracts ↔ API Gateway

1. **Read from blockchain**: API Gateway queries Solana RPC for account data
   ```rust
   // In handlers/blockchain.rs
   let account_data = solana_client::rpc_client::RpcClient::new(endpoint)
       .get_account(&pubkey)?;
   ```

2. **Write to blockchain**: API Gateway signs + submits transactions
   - Uses `anchor-client` for programmatic interaction
   - Validates signatures before submission (security layer)

### API Gateway ↔ Database

- PostgreSQL stores user accounts, trading history, authentication state
- TimescaleDB stores energy time-series (append-only, high-frequency data)
- Both queried via SQLx with prepared statements

### Frontend ↔ Smart Contracts

Direct via `@coral-xyz/anchor`:
```typescript
// From trading-frontend
const program = new Program(IDL, programId, provider);
const tx = await program.methods.createOrder(amount, price).rpc();
```

### Frontend ↔ API Gateway

RESTful HTTPS:
```typescript
const response = await fetch('/api/v1/trading/orders', {
  method: 'POST',
  headers: { 'Authorization': `Bearer ${jwt_token}` },
  body: JSON.stringify(orderData),
});
```

---

## Key Files Reference

| Path | Purpose |
|------|---------|
| `Anchor.toml` | Program IDs, cluster config, snapshot settings |
| `Cargo.toml` (root) | Workspace members + release profile |
| `docker-compose.yml` | All 11 services + networking |
| `api-gateway/src/main.rs` | Server initialization, route setup |
| `programs/*/src/lib.rs` | Smart contract logic + instructions |
| `trading-frontend/app/page.tsx` | Homepage + layout structure |
| `.env.example` | Template for environment variables |
| `docs/C4_ARCHITECTURE_OVERVIEW.md` | System architecture diagrams |

---

## Common Pitfalls & Solutions

| Issue | Solution |
|-------|----------|
| "Account not found" in Solana | Ensure validator is running (`http://localhost:8898`) and program is deployed |
| JWT token validation fails | Check `JWT_SECRET` matches between API Gateway and token generation |
| TimescaleDB connection fails | Ensure TimescaleDB service is healthy: `docker-compose logs timescaledb` |
| Frontend can't connect to wallet | Verify Phantom wallet is installed + network is "Localnet" |
| Anchor CLI version mismatch | Run `anchor --version` and check `Anchor.toml` for matching version |
| Smart meter data not stored | Verify meter UUID exists in database + API key is valid |

---

## When Adding New Features

1. **New Smart Contract Instruction**: Add to `programs/X/src/lib.rs` + update `declare_id!()` after deployment
2. **New API Endpoint**: Create handler in `api-gateway/src/handlers/` + add route in `main.rs`
3. **New Database Schema**: Create migration in `api-gateway/migrations/` with `sqlx migrate add`
4. **New Frontend Page**: Add route in `trading-frontend/app/[route]/page.tsx`
5. **Docker Service**: Add to `docker-compose.yml` + create `docker/[service]/Dockerfile`

---

## Debugging Tips

- **Smart Contracts**: Enable logs with `msg!("...")` macro; view via `solana logs` or `anchor logs`
- **API Gateway**: Set `RUST_LOG=api_gateway=debug` for detailed traces
- **Frontend**: Use React DevTools + browser console for Web3.js connection issues
- **Docker**: Run `docker-compose logs -f [service-name]` to tail service logs
- **Database**: Connect directly with `psql $DATABASE_URL` to inspect schema/data

---

## Resources

- **Solana Docs**: https://docs.solana.com
- **Anchor Book**: https://book.anchor-lang.com
- **Axum**: https://docs.rs/axum/latest/axum/
- **Next.js**: https://nextjs.org/docs
- **SQLx**: https://github.com/launchbadge/sqlx

---

*Last Updated: October 2025 | Phase: Production Deployment (92%)*
