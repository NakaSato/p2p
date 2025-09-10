# Smart Contract Deployment Guide

This guide explains how to deploy the P2P Energy Trading smart contracts to Docker.

## 🏗️ Architecture Overview

The system consists of 4 main smart contracts:

1. **Registry Contract** - User and meter management
2. **Grid Token Contract** - PSP22 token for energy trading
3. **Trading Contract** - Order book and market clearing
4. **Oracle Client Contract** - Bridge between blockchain and external data

## 🚀 Quick Start

### Option 1: Full System Deployment (Recommended)

Deploy everything with one command:

```bash
./deploy_full_system.sh
```

This will:
- ✅ Start all infrastructure services
- ✅ Deploy smart contracts to Solana
- ✅ Setup demo data and users
- ✅ Start oracle for automated market clearing
- ✅ Launch API Gateway and monitoring

### Option 2: Manual Step-by-Step Deployment

1. **Start infrastructure**:
```bash
docker-compose up -d postgres timescaledb redis kafka zookeeper
```

2. **Deploy contracts only**:
```bash
docker-compose up -d solana-validator
```

3. **Start remaining services**:
```bash
docker-compose up -d oracle-simulator api-gateway smart-meter-simulator grafana prometheus
```

## 📋 Contract Details

### Registry Contract
- **Purpose**: Manages user registration and smart meter assignments
- **Key Functions**:
  - `register_user()` - Register new users as Prosumer/Consumer
  - `assign_meter()` - Assign smart meters to users
  - `is_user_verified()` - Check user verification status

### Grid Token Contract (GRID)
- **Purpose**: PSP22 token representing energy (1 kWh = 1 GRID)
- **Key Functions**:
  - `mint()` - Create tokens when energy is generated
  - `transfer()` - Standard PSP22 transfer
  - `approve()` - Standard PSP22 approval

### Trading Contract
- **Purpose**: Manages energy trading orders and market clearing
- **Key Functions**:
  - `create_sell_order()` - Prosumers sell energy
  - `create_buy_order()` - Consumers buy energy
  - `match_orders()` - Execute market clearing

### Oracle Client Contract
- **Purpose**: Bridge between blockchain and external systems
- **Key Functions**:
  - Request energy data from AMI systems
  - Trigger automated market operations

## 🔧 Contract Interaction

### Using the Shell Script

The oracle container includes an interactive script:

```bash
# Setup demo data
docker-compose exec oracle-simulator ./interact_contracts.sh setup

# Check system status
docker-compose exec oracle-simulator ./interact_contracts.sh status

# Execute market clearing
docker-compose exec oracle-simulator ./interact_contracts.sh clear-market
```

### Manual Contract Calls

You can also interact with contracts directly:

```bash
# Enter the substrate container
docker-compose exec solana-validator bash

# Example: Check user balance
cargo contract call \
  --contract <TOKEN_ADDRESS> \
  --message balance_of \
  --args <USER_ADDRESS> \
  --suri //Alice \
  --url ws://localhost:9944 \
  --dry-run
```

## 📊 Monitoring and Logs

### View Container Logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f solana-validator
docker-compose logs -f oracle-simulator
docker-compose logs -f api-gateway
```

### Access Monitoring
- **Grafana**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9090
- **API Gateway**: http://localhost:8080

### View Contract Addresses
```bash
docker-compose exec solana-validator cat /tmp/contract_addresses/deployment_summary.json
```

## 🔄 System Workflow

1. **Initialization**:
   - Contracts are deployed to Solana node
   - Demo users (Alice, Bob, Charlie) are registered
   - Smart meters are assigned to users
   - Initial GRID tokens are minted

2. **Energy Generation**:
   - Smart meter simulator generates energy readings
   - Oracle mints GRID tokens for energy producers
   - Tokens represent actual energy generated

3. **Energy Trading**:
   - Users create buy/sell orders via API Gateway
   - Oracle performs market clearing every 60 seconds
   - Successful trades transfer GRID tokens

4. **Settlement**:
   - Completed trades are recorded on blockchain
   - Energy delivery is tracked off-chain
   - All transactions are immutable and transparent

## 🛠️ Development Commands

### Rebuild Contracts
```bash
# Rebuild specific contract
docker-compose build solana-validator

# Rebuild and restart with fresh data
docker-compose down -v
docker-compose up -d
```

### Reset System
```bash
# Stop everything and remove volumes
docker-compose down -v

# Start fresh
./deploy_full_system.sh
```

### Access Container Shell
```bash
# Solana node
docker-compose exec solana-validator bash

# Oracle simulator
docker-compose exec oracle-simulator bash
```

## 🔐 Security Notes

- This is a **development setup** with default keys
- Uses `//Alice` account for all operations
- Private blockchain with controlled access
- All demo accounts are pre-funded

## 🐛 Troubleshooting

### Common Issues

1. **Contracts not deploying**:
   - Check solana-validator logs: `docker-compose logs solana-validator`
   - Ensure sufficient wait time for node startup

2. **Oracle not working**:
   - Verify contract addresses are available
   - Check oracle logs: `docker-compose logs oracle-simulator`

3. **API Gateway errors**:
   - Ensure all dependencies are running
   - Check database connectivity

### Health Checks
```bash
# Check all container status
docker-compose ps

# Test API Gateway
curl http://localhost:8080/health

# Check Solana node
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
  http://localhost:9933
```

## 📚 Additional Resources

- [ink! Documentation](https://use.ink/)
- [Solana Documentation](https://docs.substrate.io/)
- [Polkadot API Documentation](https://polkadot.js.org/docs/)
- [Docker Compose Reference](https://docs.docker.com/compose/)

---

For support or questions, check the logs first, then refer to this documentation.
