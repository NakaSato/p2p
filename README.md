# P2P Energy Trading Smart Contracts

A comprehensive suite of ink! smart contracts for peer-to-peer solar energy trading within university campuses, designed for deployment on Substrate-based blockchains with contracts pallet support.

## Overview

This platform enables prosumers (solar energy generators) to trade excess energy directly with consumers through a decentralized marketplace. The system is implemented as four interoperable ink! smart contracts that can be deployed on any Substrate chain supporting the contracts pallet.

### Key Features

- **ink! Smart Contracts**: Deployable on any Substrate chain with contracts pallet
- **PSP22 Token Standard**: GridToken (GRID) implements PSP22 for interoperability
- **Cross-Contract Communication**: Seamless interaction between contracts
- **Automated Market Clearing**: 15-minute trading epochs with oracle integration
- **Oracle Integration**: Chainlink-compatible for external data feeds
- **Energy Tokenization**: 1 kWh = 1 GRID token economics

## Architecture

The platform consists of four core smart contracts:

### 1. Registry Contract (`contracts/registry`)
- **Purpose**: Identity layer for participants and smart meters
- **Features**: User registration, meter assignment, admin management
- **Key Functions**:
  - `register_user()` - Register campus users
  - `assign_meter()` - Link smart meters to users
  - `is_user_verified()` - Verify user status
  - `is_prosumer()` - Check if user can generate energy

### 2. GridToken Contract (`contracts/grid-token`)
- **Purpose**: PSP22-compatible fungible token for energy trading
- **Standard**: Implements PSP22 with OpenBrush
- **Tokenomics**: 1 kWh solar generation = 1 GRID token
- **Key Functions**:
  - `transfer()` - PSP22 standard token transfers
  - `mint()` - Create tokens for solar generation
  - `burn()` - Remove tokens for energy consumption
  - `approve()` - Authorize spending allowances

### 3. Trading Contract (`contracts/trading`)
- **Purpose**: Order book and matching engine
- **Market Structure**: 15-minute epochs with automatic clearing
- **Key Functions**:
  - `create_sell_order()` - Prosumers sell energy
  - `create_buy_order()` - Consumers buy energy
  - `match_orders()` - Automated market clearing
  - `cancel_order()` - Cancel active orders

### 4. OracleClient Contract (`contracts/oracle-client`)
- **Purpose**: Bridge to external oracle networks
- **Features**: Chainlink Keepers integration, AMI data verification
- **Funding Requirement**: Must be funded before processing requests
- **Key Functions**:
  - `request_energy_data()` - Request meter readings (requires oracle balance)
  - `fulfill_energy_data()` - Oracle data callback
  - `fund_oracle_operations()` - Fund oracle operations (payable)
  - `check_upkeep()` - Automated market clearing trigger
  - `perform_upkeep()` - Execute automated operations

## Quick Start

### Prerequisites

- Rust (latest stable)
- `cargo-contract` CLI tool
- Substrate node with contracts pallet (e.g., Swanky Node, Canvas)

### Installation

1. **Install cargo-contract**:
```bash
cargo install --force --locked cargo-contract
```

2. **Clone and build**:
```bash
git clone <repository-url>
cd p2p
cargo contract build --manifest-path contracts/registry/Cargo.toml
cargo contract build --manifest-path contracts/grid-token/Cargo.toml
cargo contract build --manifest-path contracts/trading/Cargo.toml
cargo contract build --manifest-path contracts/oracle-client/Cargo.toml
```

3. **Start a development node**:
```bash
# Using Canvas node
canvas --dev --tmp
```

### Contract Deployment

Deploy contracts in the following order:

1. **Registry Contract**:
```bash
cargo contract instantiate \
  --constructor new \
  --args [] \
  --manifest-path contracts/registry/Cargo.toml \
  --suri //Alice
```

2. **GridToken Contract**:
```bash
cargo contract instantiate \
  --constructor new \
  --args 1000000000000000000000000 \
  --manifest-path contracts/grid-token/Cargo.toml \
  --suri //Alice
```

3. **Trading Contract**:
```bash
cargo contract instantiate \
  --constructor new \
  --args 900000 \
  --manifest-path contracts/trading/Cargo.toml \
  --suri //Alice
```

4. **OracleClient Contract**:
```bash
cargo contract instantiate \
  --constructor new \
  --args [] \
  --manifest-path contracts/oracle-client/Cargo.toml \
  --suri //Alice
```

5. **Configure Contract Addresses**:
```bash
# Set registry and token contracts in trading contract
cargo contract call \
  --contract $TRADING_ADDRESS \
  --message set_contracts \
  --args $REGISTRY_ADDRESS $TOKEN_ADDRESS \
  --suri //Alice

# Set all contract addresses in oracle client
cargo contract call \
  --contract $ORACLE_ADDRESS \
  --message set_contracts \
  --args $REGISTRY_ADDRESS $TOKEN_ADDRESS $TRADING_ADDRESS \
  --suri //Alice

# Fund oracle operations (required for oracle functionality)
cargo contract call \
  --contract $ORACLE_ADDRESS \
  --message fund_oracle_operations \
  --value 1000000000000000000 \
  --suri //Alice
```

## Usage Examples

### 1. Register Campus Users

```bash
# Register a prosumer
cargo contract call \
  --contract $REGISTRY_ADDRESS \
  --message register_user \
  --args $USER_ADDRESS Prosumer "Building A, Floor 3" \
  --suri //Alice
```

### 2. Assign Smart Meters

```bash
# Assign meter to user
cargo contract call \
  --contract $REGISTRY_ADDRESS \
  --message assign_meter \
  --args "METER_001_BUILDING_A" $USER_ADDRESS \
  --suri //Alice
```

### 3. Mint Energy Tokens

```bash
# Mint GRID tokens for solar generation
cargo contract call \
  --contract $TOKEN_ADDRESS \
  --message mint \
  --args $PROSUMER_ADDRESS 500000000000000000000 "METER_001_BUILDING_A" \
  --suri //AMI_SERVICE_ACCOUNT
```

### 4. Create Trading Orders

```bash
# Create sell order (prosumer selling energy)
cargo contract call \
  --contract $TRADING_ADDRESS \
  --message create_sell_order \
  --args 500 150000000000000000000 \
  --suri $PROSUMER_ADDRESS

# Approve tokens for trading first
cargo contract call \
  --contract $TOKEN_ADDRESS \
  --message approve \
  --args $TRADING_ADDRESS 100000000000000000000000 \
  --suri $CONSUMER_ADDRESS

# Create buy order (consumer buying energy)
cargo contract call \
  --contract $TRADING_ADDRESS \
  --message create_buy_order \
  --args 300 160000000000000000000 \
  --suri $CONSUMER_ADDRESS
```

### 5. Market Clearing

```bash
# Trigger market clearing (market maker only)
cargo contract call \
  --contract $TRADING_ADDRESS \
  --message match_orders \
  --args [] \
  --suri //MARKET_MAKER
```

## Smart Contract Integration

### Frontend Integration

Use Polkadot.js API to interact with deployed contracts:

```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';

// Connect to node
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider: wsProvider });

// Load contract
const contract = new ContractPromise(api, registryAbi, registryAddress);

// Call contract method
const result = await contract.query.isUserVerified(
  alice.address,
  { gasLimit: -1 },
  userAddress
);
```

### Cross-Contract Calls

Contracts communicate through cross-contract calls:

```rust
// Example: Trading contract calling GridToken
use ink::env::call::{build_call, ExecutionInput, Selector};

let result = build_call::<DefaultEnvironment>()
    .call(token_contract)
    .gas_limit(5000)
    .exec_input(
        ExecutionInput::new(Selector::new(ink::selector_bytes!("transfer_from")))
            .push_arg(from)
            .push_arg(to)
            .push_arg(amount)
    )
    .returns::<Result<(), PSP22Error>>()
    .invoke();
```

## Network Deployment

### Supported Networks

- **Canvas Testnet**: Public testnet for ink! contracts
- **Aleph Zero**: Substrate chain with contracts support
- **Astar Network**: Multi-VM platform supporting ink!
- **Shibuya Testnet**: Astar's testnet environment
- **Shiden Network**: Astar's canary network

### Production Deployment Checklist

1. **Security Audit**: Review all contract code
2. **Gas Optimization**: Minimize contract call costs
3. **Access Controls**: Verify admin permissions
4. **Oracle Configuration**: Set up Chainlink nodes
5. **Monitoring**: Deploy event listeners
6. **Backup Strategy**: Secure admin key management

## Testing

### Unit Tests

Run individual contract tests:

```bash
# Test Registry contract
cargo test --manifest-path contracts/registry/Cargo.toml

# Test GridToken contract
cargo test --manifest-path contracts/grid-token/Cargo.toml

# Test Trading contract
cargo test --manifest-path contracts/trading/Cargo.toml

# Test OracleClient contract
cargo test --manifest-path contracts/oracle-client/Cargo.toml
```

**Note**: Oracle client tests require proper funding setup. The tests automatically fund the oracle with sufficient balance using `ink::env::test::set_value_transferred()` to simulate real-world oracle operations funding.

### Integration Tests

Test cross-contract interactions:

```bash
# Run all tests
cargo test

# Test specific integration scenarios
cargo test --test integration_tests
```

### End-to-End Testing

1. Deploy all contracts to testnet
2. Configure contract addresses
3. Register test users and meters
4. Execute complete trading flow
5. Verify token balances and trade records

## Configuration Parameters

### Market Settings
- **Market Epoch**: 15 minutes (900,000 milliseconds)
- **Max Orders per User**: 100 orders per epoch
- **Oracle Timeout**: 100 blocks
- **Token Decimals**: 18 (standard for PSP22)

### Security Settings
- **Admin Multi-Sig**: Recommended for production
- **Oracle Operators**: Verified Chainlink node operators
- **Minter Authorization**: AMI integration services only
- **Market Maker Permissions**: Energy authorities

## Gas Optimization

### Contract Size Optimization

```bash
# Build with size optimization
cargo contract build --optimization-passes=3 --keep-debug-symbols=false

# Check contract size
ls -la target/ink/*/
```

### Call Gas Estimates

Typical gas costs for common operations:
- User Registration: ~50,000 gas
- Token Transfer: ~30,000 gas
- Order Creation: ~80,000 gas
- Oracle Data Request: ~100,000 gas
- Market Clearing: ~500,000 gas (depends on order count)

## Monitoring and Maintenance

### Event Monitoring

Set up event listeners for critical operations:

```javascript
// Monitor trading events
contract.events.TradeExecuted((result, data) => {
  console.log('Trade executed:', data);
  // Update external systems
});

// Monitor token minting
tokenContract.events.Minted((result, data) => {
  console.log('Tokens minted:', data);
  // Verify energy generation
});
```

### Health Checks

Regular monitoring tasks:
- **Contract Balance**: Ensure sufficient funds for operations
- **Oracle Balance**: Monitor oracle contract funding levels
- **Oracle Requests**: Check for expired or failed requests
- **Market Activity**: Monitor trading volume and liquidity
- **Token Supply**: Verify minting/burning balance

## Upgradeability

### Contract Migration Strategy

ink! contracts are immutable, but you can implement upgradeability:

1. **Proxy Pattern**: Use a proxy contract for logic upgrades
2. **Migration Contract**: Deploy new version and migrate state
3. **Admin Controls**: Pause old contract, redirect to new one

### State Migration

```rust
// Migration helper functions
#[ink(message)]
pub fn migrate_user_data(&mut self, old_contract: AccountId) -> Result<()> {
    // Cross-contract call to read old state
    // Write to new contract storage
}
```

## Security Considerations

### Access Control
- **Role-Based Permissions**: Separate admin, minter, market maker roles
- **Multi-Signature**: Use multi-sig for critical operations
- **Time Locks**: Implement delays for sensitive changes

### Oracle Security
- **Data Verification**: Validate all external data sources
- **Multiple Oracles**: Use consensus from multiple providers
- **Signature Verification**: Check smart meter cryptographic signatures

### Economic Security
- **Token Supply Control**: Strict minting/burning controls
- **Market Manipulation Prevention**: Order size limits and rate limiting
- **Oracle Funding**: Ensure sufficient oracle balance for continuous operations
- **Emergency Pause**: Circuit breakers for market disruption

## Troubleshooting

### Common Issues and Solutions

#### Oracle Client Test Failures

**Problem**: Tests fail with `InsufficientOracleBalance` error
```
assertion failed: oracle.request_energy_data("METER_001".to_string()).is_ok()
```

**Solution**: Oracle client requires funding before processing requests
```rust
// In tests, fund the oracle first
ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000);
oracle.fund_oracle_operations().unwrap();
```

**Production Solution**: Fund oracle during deployment
```bash
cargo contract call \
  --contract $ORACLE_ADDRESS \
  --message fund_oracle_operations \
  --value 1000000000000000000 \
  --suri //Alice
```

#### Contract Deployment Order Issues

**Problem**: Contract calls fail with cross-contract communication errors

**Solution**: Deploy contracts in correct order and configure addresses:
1. Registry Contract (no dependencies)
2. GridToken Contract (no dependencies)  
3. Trading Contract (needs Registry + GridToken addresses)
4. OracleClient Contract (needs all three addresses)

#### Token Transfer Failures

**Problem**: `transfer_from` calls fail with insufficient allowance

**Solution**: Approve trading contract before creating orders
```bash
cargo contract call \
  --contract $TOKEN_ADDRESS \
  --message approve \
  --args $TRADING_ADDRESS 1000000000000000000000 \
  --suri $USER_ADDRESS
```

#### Gas Estimation Issues

**Problem**: Transactions fail with out-of-gas errors

**Solution**: Use higher gas limits for complex operations
- Simple transfers: 50,000 gas
- Order creation: 100,000 gas  
- Market clearing: 1,000,000 gas
- Cross-contract calls: Add 20% buffer

#### Oracle Request Timeouts

**Problem**: Oracle requests never get fulfilled

**Solution**: Check oracle operator permissions and balance
```bash
# Verify oracle operators
cargo contract call \
  --contract $ORACLE_ADDRESS \
  --message is_oracle_operator \
  --args $OPERATOR_ADDRESS

# Check oracle balance
cargo contract call \
  --contract $ORACLE_ADDRESS \
  --message oracle_balance
```

#### Test Environment Issues

**Problem**: Tests pass individually but fail when run together

**Solution**: Reset test environment between tests
```rust
#[ink::test]
fn test_function() {
    // Clear any previous state
    ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
    
    // Your test logic here
}
```

### Error Code Reference

| Error | Description | Solution |
|-------|-------------|----------|
| `InsufficientOracleBalance` | Oracle has no funds | Fund oracle with `fund_oracle_operations()` |
| `NotOracleOperator` | Unauthorized oracle access | Add operator with `add_oracle_operator()` |
| `UserNotVerified` | Unregistered user | Register user with `register_user()` |
| `InsufficientBalance` | Not enough tokens | Check token balance and allowances |
| `OrderNotFound` | Invalid order ID | Verify order exists with `get_order()` |
| `MarketNotActive` | Trading outside epoch | Wait for next market epoch |

### Debug Commands

**Check Contract State:**
```bash
# Verify user registration
cargo contract call --contract $REGISTRY_ADDRESS --message is_user_verified --args $USER_ADDRESS

# Check token balances  
cargo contract call --contract $TOKEN_ADDRESS --message balance_of --args $USER_ADDRESS

# View active orders
cargo contract call --contract $TRADING_ADDRESS --message get_active_orders --args $USER_ADDRESS

# Check oracle balance
cargo contract call --contract $ORACLE_ADDRESS --message oracle_balance
```

**Monitor Events:**
```bash
# Watch for contract events
polkadot-js-api --ws ws://127.0.0.1:9944 subscribe system.events
```

**Performance Profiling:**
```bash
# Build with profiling enabled
cargo contract build --features profiling

# Run tests with detailed output
RUST_LOG=debug cargo test -- --nocapture
```

## Contributing

### Development Setup

1. Fork the repository
2. Create feature branch
3. Write comprehensive tests
4. Follow ink! best practices
5. Submit pull request

### Code Standards

- **Rust Formatting**: Use `cargo fmt`
- **Linting**: Pass `cargo clippy`
- **Documentation**: Comprehensive inline docs
- **Testing**: 100% test coverage for critical paths

### Review Process

All changes require:
- Code review by core team
- Security audit for contract changes
- Integration testing
- Gas cost analysis

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed release notes and version history.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support and Resources

### Documentation
- **ink! Documentation**: [ink.substrate.io](https://ink.substrate.io/)
- **Substrate Documentation**: [docs.substrate.io](https://docs.substrate.io/)
- **PSP Standards**: [github.com/w3f/PSPs](https://github.com/w3f/PSPs)

### Community
- **Substrate Stack Exchange**: [substrate.stackexchange.com](https://substrate.stackexchange.com/)
- **ink! Discord**: [discord.gg/wGUDt2p](https://discord.gg/wGUDt2p)
- **GitHub Issues**: [Issues](https://github.com/your-repo/issues)

### Professional Support
- **Parity Technologies**: Official Substrate support
- **Web3 Foundation Grants**: Funding opportunities
- **Substrate Builders Program**: Technical assistance

## Acknowledgments

- **Parity Technologies**: ink! smart contract framework
- **OpenBrush**: PSP22 token implementation
- **Web3 Foundation**: Polkadot ecosystem support
- **Thai Energy Authorities**: Domain expertise and governance
- **University Partners**: Campus deployment and testing