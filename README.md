# P2P Energy Trading System - Solana Anchor

This project has been migrated from ink!/Substrate to Solana Anchor, implementing a permissioned Proof of Authority (PoA) consensus system with university departments as REC (Renewable Energy Certificate) validators for campus energy trading.

## Architecture Overview

### Programs

1. **Registry** (`programs/registry/`) - User and smart meter registration
2. **Energy Token** (`programs/energy-token/`) - SPL token with REC validation
3. **Trading** (`programs/trading/`) - Order book and matching engine
4. **Oracle** (`programs/oracle/`) - External data integration and market clearing
5. **Governance** (`programs/governance/`) - PoA consensus and REC validator management

### Key Features

- **Permissioned Network**: University-controlled environment with authorized participants
- **PoA Consensus**: University departments act as REC validators and certification authority
- **REC Integration**: University-issued Renewable Energy Certificate validation for token minting
- **AMI Integration**: Advanced Metering Infrastructure for real-time energy monitoring
- **Automated Market Clearing**: Oracle-driven periodic market settlement
- **University Governance**: Multi-signature validation by university REC authorities

## Quick Start

### Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/) (required)
- [Rust](https://rustup.rs/) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.17+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.29+)
- [Node.js](https://nodejs.org/) (v18+)
- [Yarn](https://yarnpkg.com/)

### Quick Setup

1. Clone and setup:
```bash
git clone <repository-url>
cd p2p
```

2. **Complete automated setup** (recommended):
```bash
./scripts/dev-setup.sh
```

3. **Or manual step-by-step setup**:
```bash
# Setup environment and start services
./scripts/setup-dev.sh

# Validate Docker infrastructure
./scripts/validate-docker.sh

# Deploy Anchor programs
./scripts/deploy-programs.sh

# Run integration tests
./scripts/test-integration.sh
```

3. Build the Anchor programs:
```bash
anchor build
```

4. Start local validator (in separate terminal):
```bash
solana-test-validator
```

5. Deploy programs:
```bash
./deploy.sh
# or manually:
anchor deploy
```

### Testing

Run integration tests:
```bash
# Run all tests
anchor test

# Run specific test suite
yarn test
```

## Program Architecture

### Registry Program
- **Purpose**: Manages user registration and smart meter onboarding
- **Key Functions**: `register_user()`, `register_meter()`, `update_status()`
- **Data**: User profiles, meter configurations, authorization states

### Energy Token Program
- **Purpose**: SPL token implementation with REC validation
- **Key Functions**: `mint_tokens()`, `validate_rec()`, `retire_tokens()`
- **Features**: REC validator integration, renewable energy verification

### Trading Program
- **Purpose**: Decentralized order book and matching engine
- **Key Functions**: `create_order()`, `match_orders()`, `settle_trade()`
- **Features**: Automated matching, escrow management, fee collection

### Oracle Program
- **Purpose**: External data integration and automated operations
- **Key Functions**: `submit_meter_reading()`, `trigger_market_clearing()`
- **Features**: AMI integration, periodic settlement, price discovery

### Governance Program
- **Purpose**: PoA consensus and REC validator management
- **Key Functions**: `add_rec_validator()`, `authorize_operation()`, `manage_consensus()`
- **Features**: Multi-signature validation, university authority control

## Deployment

### Local Development
```bash
# Start local validator
solana-test-validator --reset

# Deploy to local
anchor deploy --provider.cluster localnet
```

### Devnet Deployment
```bash
# Configure for devnet
solana config set --url devnet

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

### Production (University Network)
```bash
# Configure custom RPC endpoint
solana config set --url <university-rpc-endpoint>

# Deploy with governance approval
./deploy.sh --production
```

## Configuration

### REC Validators Setup
REC validators (university departments) must be initialized during deployment:

```typescript
// Example REC validator configuration
const recValidators = [
  { pubkey: sustainabilityValidatorKey, authority_name: "University Sustainability Office" },
  { pubkey: engineeringValidatorKey, authority_name: "University Engineering Department" },
  { pubkey: facilitiesValidatorKey, authority_name: "University Facilities Management" }
];
```

### Oracle Configuration
Configure oracle operators for different data sources:

```typescript
// Example oracle operator setup
const oracleOperators = [
  { pubkey: amiOperatorKey, source: "AMI_INTEGRATION" },
  { pubkey: weatherOperatorKey, source: "WEATHER_DATA" },
  { pubkey: priceOperatorKey, source: "EXTERNAL_PRICING" }
];
```

## Monitoring and Operations

### Health Checks
- Monitor program account states
- Verify REC validator consensus
- Check oracle data freshness
- Validate token mint/burn operations

### Governance Operations
- Add/remove REC validators requires university authority approval
- System parameter updates controlled by university administration
- Emergency pause functionality for critical issues
- All REC certifications issued and validated by university departments

## Migration Notes

This system has been migrated from ink!/Substrate with the following changes:

1. **Consensus**: Moved from Substrate's configurable consensus to Solana's PoS with PoA overlay
2. **Smart Contracts**: Converted from ink! to Anchor framework
3. **Token Standard**: Migrated from custom tokens to SPL standard
4. **Storage**: Adapted from Substrate storage to Solana account model
5. **Governance**: Enhanced with university-specific REC validator system

## Security Considerations

- All operations require appropriate authority validation
- REC validators use multi-signature consensus for critical operations
- Token minting requires REC certificate validation
- Trading operations include escrow and dispute resolution
- Oracle data includes freshness validation and consensus mechanisms

## Support and Documentation

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Developer Resources](https://docs.solana.com/)
- [SPL Token Documentation](https://spl.solana.com/token)

For university-specific deployment questions, contact the system administrators.
