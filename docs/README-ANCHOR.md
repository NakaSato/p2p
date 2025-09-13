# P2P Energy Trading System - Solana Anchor

This project implements a P2P energy trading platform using Solana Anchor framework with Engineering Department single validator authority for campus energy trading within the Engineering Complex.

## Architecture Overview

### Programs

1. **Registry** (`programs/registry/`) - User and smart meter registration under Engineering Department authority
2. **Energy Token** (`programs/energy-token/`) - SPL token with Engineering Department mint authority
3. **Trading** (`programs/trading/`) - Order book and automated market clearing
4. **Oracle** (`programs/oracle/`) - AMI data integration and automated operations
5. **Governance** (`programs/governance/`) - Engineering Department system administration

### Key Features

- **Single Validator Network**: Engineering Department operated Solana validator
- **Proof of Stake Consensus**: Single validator Proof of Stake with Engineering Department authority
- **SPL Token Standard**: Native Solana token standard for energy representation
- **AMI Integration**: Advanced Metering Infrastructure for Engineering Complex smart meters
- **Automated Market Clearing**: 15-minute epoch-based market settlement
- **Engineering Department Governance**: Complete system control by Engineering Department

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.17+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.29+)
- [Node.js](https://nodejs.org/) (v18+)
- [Yarn](https://yarnpkg.com/)

### Installation

1. Clone and setup:
```bash
git clone <repository-url>
cd p2p
```

2. Install dependencies:
```bash
# Install Rust dependencies
cargo build

# Install Node.js dependencies for testing
yarn install
```

3. Start local Engineering Department validator (in separate terminal):
```bash
solana-test-validator --reset
```

4. Deploy programs to Engineering Department validator:
```bash
./deploy.sh
# or manually:
anchor deploy --provider.cluster localnet
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
- **Purpose**: Manages user registration and smart meter onboarding within Engineering Complex
- **Key Functions**: `register_user()`, `assign_meter()`, `verify_user()`
- **Authority**: Engineering Department controls all registrations
- **Data**: Engineering student/faculty profiles, Engineering Complex meter configurations

### Energy Token Program
- **Purpose**: SPL token implementation with Engineering Department mint authority
- **Key Functions**: `mint_to()`, `transfer()`, `burn()` (standard SPL token operations)
- **Features**: Engineering Department controlled minting, 9 decimal precision, associated token accounts

### Trading Program
- **Purpose**: Automated order book and market clearing within Engineering Complex
- **Key Functions**: `create_sell_order()`, `create_buy_order()`, `match_orders()`
- **Features**: 15-minute epoch clearing, Engineering Department oversight, automated settlement

### Oracle Program
- **Purpose**: AMI data integration and automated market operations
- **Key Functions**: `submit_meter_data()`, `trigger_market_clearing()`, `validate_energy_data()`
- **Features**: Engineering Complex AMI integration, automated token minting, market triggers

### Governance Program
- **Purpose**: Engineering Department system administration and parameter management
- **Key Functions**: `update_parameters()`, `emergency_pause()`, `manage_authorities()`
- **Features**: Engineering Department exclusive control, system configuration, emergency operations

## Deployment

### Local Development
```bash
# Start local Engineering Department validator
solana-test-validator --reset

# Deploy to local validator
anchor deploy --provider.cluster localnet
```

### Devnet Testing
```bash
# Configure for devnet
solana config set --url devnet

# Deploy to devnet for testing
anchor deploy --provider.cluster devnet
```

### Production (Engineering Department Validator)
```bash
# Configure Engineering Department RPC endpoint
solana config set --url <engineering-department-rpc-endpoint>

# Deploy with Engineering Department authority
./deploy.sh --production --authority engineering-department
```

## Configuration

### Engineering Department Authority Setup
Engineering Department authority must be configured during deployment:

```typescript
// Engineering Department authority configuration
const engineeringAuthority = {
  pubkey: engineeringDepartmentKey,
  authority_name: "Engineering Department",
  validator_endpoint: "http://engineering-validator.campus.local:8899",
  mint_authority: true,
  governance_authority: true
};
```

### Oracle Configuration
Configure oracle operators for Engineering Complex AMI data:

```typescript
// Engineering Complex oracle operator setup
const oracleOperators = [
  { 
    pubkey: amiOperatorKey, 
    source: "ENGINEERING_AMI_INTEGRATION",
    meters: ["ENG_001", "ENG_002", "...", "ENG_015"]
  },
  { 
    pubkey: engineeringWeatherKey, 
    source: "ENGINEERING_WEATHER_DATA",
    location: "Engineering Complex Rooftop"
  }
];
```

## Monitoring and Operations

### Health Checks
- Monitor Engineering Department validator status
- Verify Engineering Department authority operations
- Check oracle data freshness from Engineering Complex AMI
- Validate SPL token mint/burn operations
- Monitor 15-minute market clearing cycles

### Engineering Department Operations
- Engineering Department has exclusive control over all system parameters
- User registration and meter assignment controlled by Engineering Department
- System upgrades and maintenance managed by Engineering Department
- Emergency pause functionality available to Engineering Department
- All energy token minting authorized by Engineering Department

## Migration Notes

This system implements a Solana Anchor-based architecture with the following design:

1. **Consensus**: Single validator Proof of Stake operated by Engineering Department
2. **Smart Contracts**: Anchor framework programs with Engineering Department authority
3. **Token Standard**: SPL Token standard with Engineering Department mint authority
4. **Storage**: Solana account model with Program Derived Addresses (PDAs)
5. **Governance**: Engineering Department exclusive control and administration

## Security Considerations

- All operations require Engineering Department authority validation
- Engineering Department controls all critical system functions
- SPL token minting requires Engineering Department authorization
- Trading operations include automated settlement and Engineering Department oversight
- Oracle data includes freshness validation and Engineering Department consensus
- Emergency controls available exclusively to Engineering Department

## Academic Integration

### Educational Benefits
- Real-world Solana blockchain implementation for Engineering students
- Hands-on experience with Anchor framework development
- SPL Token standard integration and usage
- Campus energy systems and blockchain intersection
- Research opportunities in decentralized energy trading

### Engineering Department Advantages
- Complete operational control and system governance
- Integration with Engineering curriculum and research programs
- Demonstration of blockchain technology in practical applications
- Cost-effective single validator operation
- Direct control over energy trading policies and parameters

## Support and Documentation

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Developer Resources](https://docs.solana.com/)
- [SPL Token Documentation](https://spl.solana.com/token)
- [Engineering Department Technical Documentation](./TECHNICAL_SUMMARY.md)
- [System Architecture Documentation](./SYSTEM_ARCHITECTURE.md)
- [Smart Meter Simulation Guide](./SMART_METER_SIMULATION.md)

For Engineering Department specific deployment questions and system administration, contact the Engineering Department IT administrators.
