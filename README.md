# P2P Energy Trading Blockchain

A private Substrate-based blockchain implementation for peer-to-peer solar energy trading within university campuses, built according to the technical specification for the Thai energy market.

## Overview

This blockchain platform enables prosumers (solar energy generators) to trade excess energy directly with consumers through a decentralized marketplace. The system is designed as a private permissioned network with Proof of Authority (PoA) consensus, operated by Thai energy authorities.

### Key Features

- **Private Permissioned Network**: Only authorized participants can join
- **Proof of Authority Consensus**: Fast finality with 2-4 second block times
- **PSP22-Compatible Token (GRID)**: 1 kWh = 1 GRID token
- **Automated Market Clearing**: 15-minute trading epochs with automatic matching
- **Oracle Integration**: Chainlink-compatible for external data feeds
- **Thai Energy Authority Governance**: ERC, MEA, PEA, and EGAT as validators

## Architecture

The blockchain consists of four core pallets (smart contracts):

### 1. Registry Pallet (`pallet-registry`)
- **Purpose**: Identity layer for participants and smart meters
- **Features**: User registration, meter assignment, admin management
- **Key Functions**:
  - `register_user()` - Register campus users
  - `assign_meter()` - Link smart meters to users
  - `is_user_verified()` - Verify user status

### 2. GridToken Pallet (`pallet-grid-token`)
- **Purpose**: PSP22-compatible fungible token for energy trading
- **Tokenomics**: 1 kWh solar generation = 1 GRID token
- **Key Functions**:
  - `transfer()` - Transfer tokens between users
  - `mint()` - Create tokens for solar generation
  - `burn()` - Remove tokens for energy consumption
  - `approve()` - Authorize spending allowances

### 3. Trading Pallet (`pallet-trading`)
- **Purpose**: Order book and matching engine
- **Market Structure**: 15-minute epochs with automatic clearing
- **Key Functions**:
  - `create_sell_order()` - Prosumers sell energy
  - `create_buy_order()` - Consumers buy energy
  - `match_orders()` - Automated market clearing
  - `cancel_order()` - Cancel active orders

### 4. OracleClient Pallet (`pallet-oracle-client`)
- **Purpose**: Bridge to external oracle networks
- **Features**: Chainlink Keepers integration, AMI data verification
- **Key Functions**:
  - `request_energy_data()` - Request meter readings
  - `fulfill_energy_data()` - Oracle data callback
  - `check_upkeep()` - Automated market clearing trigger
  - `perform_upkeep()` - Execute automated operations

## Quick Start

### Prerequisites

- Rust (latest stable)
- Substrate development environment
- Node.js (for frontend integration)

### Installation

1. **Clone the repository**:
```bash
git clone <repository-url>
cd p2p
```

2. **Build the project**:
```bash
cargo build --release
```

3. **Run a development node**:
```bash
./target/release/p2p-node --dev
```

4. **Run with custom validators** (Production):
```bash
./target/release/p2p-node \
  --chain local \
  --alice \
  --port 30333 \
  --rpc-port 9933 \
  --rpc-cors all \
  --rpc-methods unsafe
```

### Network Configuration

The network is configured with Thai energy authorities as validators:

- **ERC** (Energy Regulatory Commission)
- **MEA** (Metropolitan Electricity Authority) 
- **PEA** (Provincial Electricity Authority)
- **EGAT** (Electricity Generating Authority of Thailand)

## Usage Examples

### 1. Register a Campus User

```rust
// Register a prosumer (can generate and consume energy)
let user_info = UserInfo {
    user_type: UserType::Prosumer,
    location: b"Building A, Floor 3".to_vec(),
    status: UserStatus::Active,
    registered_at: current_timestamp(),
};

Registry::register_user(
    origin,
    user_account,
    user_info.user_type,
    user_info.location
)?;
```

### 2. Assign Smart Meter

```rust
// Assign a simulated smart meter to a user
Registry::assign_meter(
    origin,
    b"METER_001_BUILDING_A".to_vec(),
    user_account
)?;
```

### 3. Mint GRID Tokens for Solar Generation

```rust
// Mint tokens when solar energy is generated
GridToken::mint(
    origin,
    prosumer_account,
    1000 * 10_u128.pow(18), // 1000 kWh worth of tokens
    Some(b"METER_001_BUILDING_A".to_vec())
)?;
```

### 4. Create Energy Trading Orders

```rust
// Prosumer creates sell order
Trading::create_sell_order(
    origin,
    500, // 500 kWh
    150 * 10_u128.pow(18) // 150 GRID per kWh
)?;

// Consumer creates buy order
Trading::create_buy_order(
    origin,
    300, // 300 kWh
    160 * 10_u128.pow(18) // Max 160 GRID per kWh
)?;
```

### 5. Automated Market Clearing

```rust
// Oracle triggers market clearing every 15 minutes
Trading::match_orders(oracle_account)?;
```

## Network Participants

### Roles

1. **Admins**: University IT, facility management, academic supervisors
2. **Prosumers**: Users with solar panels who can generate and consume energy
3. **Consumers**: Users who only consume energy
4. **Validators**: Thai energy authorities (ERC, MEA, PEA, EGAT)
5. **Oracle Operators**: Chainlink node operators, AMI integration services
6. **Market Makers**: Authorized accounts that can trigger market clearing

### Permissions

- **User Registration**: Admin only
- **Meter Assignment**: Admin only
- **Token Minting**: Authorized minters (AMI service, Oracle client)
- **Market Clearing**: Market makers (Oracle client, AMI service)
- **Order Creation**: Verified users only

## Configuration Parameters

### Network Settings
- **Block Time**: 6 seconds (configurable down to 2-4 seconds)
- **Market Epoch**: 15 minutes (900,000 milliseconds)
- **Consensus**: Aura (PoA) + GRANDPA (finality)

### Limits
- **Max Meters per User**: 10
- **Max Orders per User per Epoch**: 100
- **Max Orders per Epoch**: 10,000
- **Oracle Timeout**: 100 blocks

### Token Configuration
- **Token Symbol**: GRID
- **Decimals**: 18
- **Base Unit**: 1 GRID = 1 kWh of solar energy

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific pallet tests
cargo test -p pallet-registry
cargo test -p pallet-grid-token
cargo test -p pallet-trading
cargo test -p pallet-oracle-client
```

### Runtime Benchmarking

```bash
# Enable benchmarking features
cargo build --release --features runtime-benchmarks

# Run benchmarks
./target/release/p2p-node benchmark pallet \
  --chain dev \
  --pallet pallet_registry \
  --extrinsic "*" \
  --steps 50 \
  --repeat 20
```

### Integration with Frontend

The node exposes JSON-RPC endpoints for frontend integration:

- **WebSocket**: `ws://localhost:9944`
- **HTTP**: `http://localhost:9933`

Use Polkadot.js API or custom clients to interact with the blockchain.

## Production Deployment

### Validator Setup

1. **Generate Authority Keys**:
```bash
./target/release/p2p-node key generate --scheme Sr25519 --password-interactive
./target/release/p2p-node key generate-node-key
```

2. **Configure Chain Spec**:
```bash
./target/release/p2p-node build-spec --disable-default-bootnode --chain local > customSpec.json
./target/release/p2p-node build-spec --chain customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
```

3. **Start Validator Nodes**:
```bash
# ERC node
./target/release/p2p-node \
  --base-path /tmp/erc \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --rpc-port 9933 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name ERC-Node \
  --password-interactive
```

### Oracle Integration

Configure Chainlink nodes or custom oracle services to:

1. **Monitor AMI Head-End APIs** for energy data
2. **Verify smart meter signatures** for data integrity
3. **Trigger automated market clearing** every 15 minutes
4. **Submit energy generation data** for token minting

### Security Considerations

- **Key Management**: Use hardware security modules (HSMs) for validator keys
- **Network Security**: Implement VPN or private networking for validator communication
- **Oracle Security**: Verify data signatures and use multiple oracle sources
- **Access Control**: Regularly audit admin and operator permissions

## Monitoring and Maintenance

### Metrics

The node exposes Prometheus metrics on `/metrics` endpoint:

- Block production times
- Transaction pool size
- Network peer count
- Custom pallet metrics

### Logging

Configure logging levels:
```bash
RUST_LOG=info,pallet_trading=debug,pallet_oracle_client=debug ./target/release/p2p-node --dev
```

### Health Checks

- **Node Status**: Check if node is syncing and producing blocks
- **Market Clearing**: Monitor if orders are being matched regularly
- **Oracle Activity**: Verify oracle requests are being fulfilled
- **Token Supply**: Track GRID token minting and burning

## Contributing

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

### Code Standards

- Follow Rust naming conventions
- Add comprehensive documentation
- Include unit tests for all public functions
- Use `cargo fmt` and `cargo clippy`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For technical support and questions:

- **Documentation**: [Substrate Documentation](https://docs.substrate.io/)
- **Community**: [Substrate Stack Exchange](https://substrate.stackexchange.com/)
- **Issues**: [GitHub Issues](https://github.com/your-repo/issues)

## Acknowledgments

- **Substrate Framework**: Parity Technologies
- **Thai Energy Authorities**: ERC, MEA, PEA, EGAT
- **Academic Partners**: University research teams
- **Chainlink**: Oracle infrastructure