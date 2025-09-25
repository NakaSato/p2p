# P2P Energy Trading System - Solana Anchor

![Alt](https://repobeats.axiom.co/api/embed/5fa20d94a0ea053095c632762cb654fc41cdbd81.svg "Repobeats analytics image")

[![Production Ready](https://img.shields.io/badge/Production-92%25_Complete-brightgreen)](https://github.com/NakaSato/p2p)
[![Phase](https://img.shields.io/badge/Phase-Production_Deployment-blue)](https://github.com/NakaSato/p2p)
[![Last Updated](https://img.shields.io/badge/Updated-September_2025-orange)](https://github.com/NakaSato/p2p)

**Advanced P2P Energy Trading Platform** - A production-ready Solana Anchor implementation featuring a permissioned Proof of Authority (PoA) consensus system with Engineering Department as the primary validator for campus energy trading.

## Project Status

**Current Phase**: Production Deployment (92% Complete)
- **All 5 Anchor Programs**: Deployed and tested
- **API Gateway**: 23 endpoints with blockchain integration
- **Oracle Security**: Enhanced with API Gateway authorization
- **AMI Integration**: UUID-based smart meter management
- **Frontend**: React/TypeScript Web3 application
- **Final Monitoring**: Prometheus/Grafana integration
- **Security Audit**: Scheduled for completion

## Architecture Overview

### Smart Contract Programs (Production Ready)

1. **Registry Program** (`programs/registry/`) - User and smart meter registration with UUID management
2. **Energy Token Program** (`programs/energy-token/`) - SPL token with REC validation and automated minting
3. **Trading Program** (`programs/trading/`) - Automated order book and matching engine
4. **Oracle Program** (`programs/oracle/`) - API Gateway exclusive authorization for AMI integration
5. **Governance Program** (`programs/governance/`) - Proof of Authority consensus with Engineering Department validation

### System Components

- **API Gateway** (Rust/Axum): 23 REST endpoints with blockchain integration
- **Database Layer**: PostgreSQL + TimescaleDB for energy time-series data
- **Redis Cache**: Session management and performance optimization
- **Monitoring Stack**: Prometheus + Grafana for system observability
- **Frontend**: React/TypeScript with Web3 Solana integration
- **Docker Infrastructure**: Production-ready containerized deployment

### Key Features

- **Permissioned Network**: Engineering Department-controlled environment with authorized participants
- **PoA Consensus**: Engineering Department acts as primary validator and certification authority
- **REC Integration**: University-issued Renewable Energy Certificate validation for token minting
- **AMI Integration**: Advanced Metering Infrastructure with UUID-based smart meter management
- **Automated Market Clearing**: Oracle-driven periodic market settlement with API Gateway security
- **University Governance**: Multi-signature validation by Engineering Department authorities
- **Production Ready**: 92% complete with comprehensive testing and documentation

## Quick Start

### Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) *(Required - Main deployment method)*
- [Rust](https://rustup.rs/) (latest stable) + [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.17+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.29+) *(For program development)*
- [Node.js](https://nodejs.org/) (v18+) + [Yarn](https://yarnpkg.com/) *(For frontend)*

### Production Setup (Recommended)

**1. Complete Automated Setup:**
```bash
git clone https://github.com/NakaSato/p2p
cd p2p

# Production-ready setup with all components
./scripts/dev-setup.sh
```

**2. Manual Production Deployment:**
```bash
# Environment setup
./scripts/setup-dev.sh

# Infrastructure validation  
./scripts/validate-docker.sh

# Deploy all Anchor programs
./scripts/deploy-programs.sh

# Run comprehensive tests
./scripts/test-integration.sh
```

### Development & Testing

```bash
# Build all programs
anchor build

# Start local Solana validator
solana-test-validator --reset

# Deploy programs to local network
anchor deploy --provider.cluster localnet

# Run integration test suite
anchor test
yarn test
```

## Program Architecture (Production Ready)

### Registry Program
- **Purpose**: Manages user registration and UUID-based smart meter onboarding
- **Key Functions**: `register_user()`, `register_meter()`, `update_status()`
- **Data**: User profiles, meter configurations, authorization states
- **Status**: Production Ready

### Energy Token Program
- **Purpose**: SPL token implementation with automated REC validation
- **Key Functions**: `mint_tokens()`, `validate_rec()`, `retire_tokens()`
- **Features**: REC validator integration, renewable energy verification
- **Status**: Production Ready

### Trading Program
- **Purpose**: Decentralized order book with automated matching engine
- **Key Functions**: `create_order()`, `match_orders()`, `settle_trade()`
- **Features**: Real-time matching, escrow management, fee collection
- **Status**: Production Ready

### Oracle Program (Enhanced September 2025)
- **Purpose**: **API Gateway exclusive** external data integration
- **Key Functions**: `submit_meter_reading()`, `trigger_market_clearing()`, `update_api_gateway()`
- **Features**: Enhanced security, AMI integration, periodic settlement, price discovery
- **Security**: Only authorized API Gateway can submit data
- **Status**: Production Ready with Security Enhancements

### Governance Program
- **Purpose**: PoA consensus and Engineering Department validator management
- **Key Functions**: `add_rec_validator()`, `authorize_operation()`, `manage_consensus()`
- **Features**: Multi-signature validation, university authority control
- **Status**: Production Ready

## Deployment

### Local Development
```bash
# Start local validator with reset
solana-test-validator --reset

# Deploy to local network
anchor deploy --provider.cluster localnet

# Access services
# • Solana Validator: http://localhost:8899
# • API Gateway: http://localhost:8080
# • Frontend: http://localhost:5173
# • Grafana: http://localhost:3000 (admin/admin)
```

### Devnet Deployment
```bash
# Configure for Solana devnet
solana config set --url devnet

# Deploy all programs to devnet
anchor deploy --provider.cluster devnet

# Verify deployment
solana program show <program-id> --url devnet
```

### Production (Engineering Department Network)
```bash
# Configure custom RPC endpoint
solana config set --url <engineering-rpc-endpoint>

# Deploy with governance approval
./scripts/deploy-programs.sh --production

# Initialize with Engineering Department as primary validator
./scripts/initialize-production.sh
```

## Configuration

### Engineering Department Validator Setup
Engineering Department serves as the primary PoA validator:

```typescript
// Engineering Department validator configuration
const engineeringValidator = {
  pubkey: engineeringValidatorKey,
  authority_name: "Engineering Department",
  validator_type: "PRIMARY_POA",
  rec_authority: true,
  governance_weight: 100
};
```

### Oracle Configuration (Enhanced Security)
API Gateway exclusive configuration for secure data submission:

```typescript
// Enhanced oracle configuration with API Gateway authorization
const oracleConfig = {
  authority: engineeringDepartmentKey,
  api_gateway: authorizedApiGatewayKey,    // Only this gateway can submit data
  operators: [
    { pubkey: amiOperatorKey, source: "AMI_INTEGRATION" },
    { pubkey: meterOperatorKey, source: "SMART_METERS" },
    { pubkey: marketOperatorKey, source: "MARKET_DATA" }
  ]
};
```

## Monitoring and Operations

### System Health Monitoring
- **Prometheus Metrics**: Real-time performance monitoring at http://localhost:9090
- **Grafana Dashboards**: Visual analytics at http://localhost:3000 (admin/admin)
- **API Gateway Health**: Endpoint monitoring and blockchain integration status
- **Program Account States**: Automated validation of all program accounts
- **Oracle Data Freshness**: Real-time validation of AMI data submission

### Production Operations
- **Engineering Department Governance**: All system parameters controlled by Engineering Department
- **Smart Meter Management**: UUID-based registration and monitoring
- **Energy Token Operations**: Automated minting/burning with REC validation
- **Trading Operations**: Real-time order matching with automated settlement
- **Security Monitoring**: Comprehensive audit trails and access control

### Emergency Procedures
- **Emergency Pause**: System-wide halt functionality for critical issues
- **Governance Override**: Engineering Department emergency authority
- **Data Recovery**: Comprehensive backup and restoration procedures
- **Network Maintenance**: Coordinated maintenance windows and updates

## Security Architecture

### Multi-Layer Security
- **Proof of Authority**: Engineering Department as primary validator
- **API Gateway Authorization**: Oracle program restricted to authorized gateway only
- **Multi-signature Validation**: Critical operations require Engineering Department approval
- **Hardware Security Module**: TPM 2.0 integration for smart meter authentication
- **Cryptographic Verification**: RSA-4096/ECDSA-P256 for all device communications

### Access Control
- **Role-Based Permissions**: Student, Faculty, Admin, and System roles
- **Device Authentication**: X.509 certificates for smart meter validation
- **Transaction Signing**: Ed25519 signatures for all blockchain operations
- **Audit Logging**: Comprehensive logging of all system operations

## Documentation and Resources

### Project Documentation
- **[Comprehensive Development Guide](docs/COMPREHENSIVE_DEVELOPMENT_GUIDE.md)**: Complete development documentation with 92% completion status
- **[Architecture Guide](docs/COMPREHENSIVE_ARCHITECTURE_GUIDE.md)**: System architecture and component interactions
- **[Blockchain Guide](docs/COMPREHENSIVE_BLOCKCHAIN_GUIDE.md)**: Solana/Anchor implementation with Oracle security enhancements
- **[AMI Integration Plan](docs/AMI_INTEGRATION_PLAN.md)**: Advanced Metering Infrastructure with UUID-based meters

### External Resources
- **[Anchor Framework](https://www.anchor-lang.com/)**: Solana smart contract framework
- **[Solana Developer Docs](https://docs.solana.com/)**: Blockchain development resources
- **[SPL Token Documentation](https://spl.solana.com/token)**: Token standard implementation

### Support
- **Engineering Department**: Primary system administrator and validator
- **Technical Issues**: See comprehensive documentation in `/docs` directory
- **Production Deployment**: Contact Engineering Department for university-specific configuration

## Project Achievements

### **Phase 1**: Foundation Complete (Q3 2025)
- Core Anchor program architecture
- Docker containerization
- Database integration

### **Phase 2**: Blockchain Integration Complete (Q3 2025)
- All 5 programs deployed and tested
- Oracle security enhancements
- API Gateway blockchain integration

### **Phase 3**: Trading System Complete (Q3 2025)
- Automated order matching
- Energy token management
- AMI integration with UUID meters

### **Phase 4**: Production Deployment (92% Complete)
- **In Progress**: Final monitoring setup
- **Pending**: Security audit completion
- **Target**: December 2025 full deployment

---

**Ready for Engineering Department Production Deployment**
