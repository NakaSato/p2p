# Docker Smart Contract Deployment - Quick Reference

## Essential Commands

### Contact Service - Core Operations

#### **Network Connectivity**
```bash
# Test devnet connectivity (recommended)
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/wait-for-validator.sh

# Test local validator connectivity
docker-compose run --rm contact /usr/local/bin/wait-for-validator.sh

# Test mainnet connectivity
docker-compose run --rm -e SOLANA_RPC_URL="https://api.mainnet-beta.solana.com" contact /usr/local/bin/wait-for-validator.sh
```

#### **Smart Contract Operations**
```bash
# Build all contracts
docker-compose run --rm contact /usr/local/bin/build-contracts.sh

# Deploy to devnet (recommended for development)
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/deploy-all-contracts.sh

# Deploy to local validator
docker-compose run --rm contact /usr/local/bin/deploy-all-contracts.sh

# Deploy to mainnet (production)
docker-compose run --rm -e SOLANA_RPC_URL="https://api.mainnet-beta.solana.com" contact /usr/local/bin/deploy-all-contracts.sh
```

#### **Health Monitoring**
```bash
# Quick status check
docker-compose run --rm contact /usr/local/bin/health-monitor.sh status

# Detailed health report
docker-compose run --rm contact /usr/local/bin/health-monitor.sh detailed

# Verify deployment
docker-compose run --rm contact /usr/local/bin/verify-deployment.sh
```

### Service Management

#### **Container Operations**
```bash
# Build all services
docker-compose build

# Build contact service only
docker-compose build contact

# Start all services
docker-compose up -d

# Start specific services
docker-compose up -d postgres redis api-gateway frontend

# Check service status
docker-compose ps

# View logs
docker-compose logs contact --tail=50
docker-compose logs -f contact  # Follow logs in real-time
```

#### **Environment Management**
```bash
# Development with devnet (recommended)
export SOLANA_RPC_URL="https://api.devnet.solana.com"
docker-compose run --rm -e SOLANA_RPC_URL="$SOLANA_RPC_URL" contact /usr/local/bin/deploy-all-contracts.sh

# Local development
docker-compose up -d solana-validator
docker-compose run --rm contact /usr/local/bin/deploy-all-contracts.sh

# Production deployment
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"
docker-compose run --rm -e SOLANA_RPC_URL="$SOLANA_RPC_URL" contact /usr/local/bin/deploy-all-contracts.sh
```
## Health Checks & Monitoring

### System Status
```bash
# Check all services
docker-compose ps

# Overall system health
docker-compose run --rm contact /usr/local/bin/health-monitor.sh status

# Detailed system report
docker-compose run --rm contact /usr/local/bin/health-monitor.sh detailed
```

### Container Health
```bash
# Check specific service status
docker inspect p2p-contact --format='{{.State.Status}}'
docker inspect p2p-solana-validator --format='{{.State.Health.Status}}'

# Service resource usage
docker stats p2p-contact p2p-solana-validator --no-stream
```

### Logs Monitoring
```bash
# Real-time contact service logs
docker-compose logs -f contact

# Recent logs (last 50 lines)
docker-compose logs contact --tail=50

# Validator logs (if using local)
docker-compose logs solana-validator --tail=20

# All services logs
docker-compose logs --tail=10
```

## Troubleshooting Quick Fixes

### Contact Service Issues
```bash
# Restart contact service
docker-compose restart contact

# Rebuild and restart
docker-compose build contact
docker-compose up -d contact

# Test network connectivity
docker-compose run --rm contact bash -c "ping -c 3 api.devnet.solana.com"

# Check workspace mounting
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/Anchor.toml"
```

### Validator Issues (Use Devnet Instead)
```bash
# Check validator status
docker-compose ps solana-validator

# If unhealthy, use devnet workaround
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/wait-for-validator.sh

# Deploy with devnet
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/deploy-all-contracts.sh
```

### Debug Mode
```bash
# Interactive shell access
docker-compose run --rm contact bash

# Check available tools
docker-compose run --rm contact bash -c "which anchor && which solana"

# Verify environment variables
docker-compose run --rm contact bash -c "env | grep SOLANA"

# Test script permissions
docker-compose run --rm contact bash -c "ls -la /usr/local/bin/*.sh"
```

### Recovery Procedures
```bash
# Clean restart (preserves volumes)
docker-compose restart

# Full reset (destroys volumes)
docker-compose down -v
docker-compose build --no-cache
docker-compose up -d

# Contact service only reset
docker-compose build --no-cache contact
docker-compose up -d contact
```

## Advanced Operations

### Custom Network Configuration
```bash
# Use custom RPC endpoint
export CUSTOM_RPC="https://your-custom-rpc.com"
docker-compose run --rm -e SOLANA_RPC_URL="$CUSTOM_RPC" contact /usr/local/bin/deploy-all-contracts.sh

# Override multiple environment variables
docker-compose run --rm \
  -e SOLANA_RPC_URL="https://api.devnet.solana.com" \
  -e DEPLOYMENT_TIMEOUT="600" \
  -e MAX_RETRIES="5" \
  contact /usr/local/bin/deploy-all-contracts.sh
```

### Volume Management
```bash
# Check volume usage
docker volume ls | grep contact

# Inspect volume contents
docker-compose run --rm contact bash -c "ls -la /opt/deployer/artifacts"
docker-compose run --rm contact bash -c "ls -la /opt/deployer/logs"

# Backup artifacts
docker run --rm -v p2p_contact_artifacts:/source -v $(pwd):/backup alpine tar czf /backup/artifacts-backup.tar.gz -C /source .

# Clean volumes
docker-compose down -v
docker volume prune -f
```

### Performance Monitoring
```bash
# Container resource usage
docker stats --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}"

# Network connectivity performance
docker-compose run --rm contact bash -c "time solana cluster-version --url https://api.devnet.solana.com"

# Build performance timing
time docker-compose build contact
```

## Environment-Specific Commands

### Development (Devnet - Recommended)
```bash
export SOLANA_RPC_URL="https://api.devnet.solana.com"

# Quick deployment
docker-compose run --rm -e SOLANA_RPC_URL="$SOLANA_RPC_URL" contact /usr/local/bin/deploy-all-contracts.sh

# Health check
docker-compose run --rm -e SOLANA_RPC_URL="$SOLANA_RPC_URL" contact /usr/local/bin/health-monitor.sh status
```

### Local Development (Advanced)
```bash
# Start local validator first
docker-compose up -d solana-validator

# Wait for validator to be healthy
docker-compose run --rm contact /usr/local/bin/wait-for-validator.sh

# Deploy to local validator
docker-compose run --rm contact /usr/local/bin/deploy-all-contracts.sh
```

### Production (Mainnet)
```bash
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"

# Verify connectivity first
docker-compose run --rm -e SOLANA_RPC_URL="$SOLANA_RPC_URL" contact /usr/local/bin/wait-for-validator.sh

# Deploy to mainnet
docker-compose run --rm -e SOLANA_RPC_URL="$SOLANA_RPC_URL" contact /usr/local/bin/deploy-all-contracts.sh

# Verify deployment
docker-compose run --rm -e SOLANA_RPC_URL="$SOLANA_RPC_URL" contact /usr/local/bin/verify-deployment.sh
```

## Quick Reference Links

- [Full Deployment Guide](./DOCKER_DEPLOYMENT_GUIDE.md)
- [Troubleshooting Guide](./DOCKER_TROUBLESHOOTING.md)
- [Contact Service Analysis](./CONTACT_ANALYSIS.md)
- [System Architecture](./SYSTEM_ARCHITECTURE.md)
docker-compose build --no-cache
docker-compose up -d
```

## Service URLs

### Development
- **Solana RPC**: `http://localhost:8899`
- **Frontend**: `http://localhost:3000`
- **API Gateway**: `http://localhost:3030`
- **Grafana**: `http://localhost:3001`
- **pgAdmin**: `http://localhost:5050`

### Production
- **Frontend**: `http://localhost:80`
- **API**: `http://localhost:80/api`
- **Monitoring**: `http://localhost:80/monitoring`

## Configuration Files

- **Main**: `docker-compose.yml`
- **Development**: `docker-compose.dev.yml`
- **Production**: `docker-compose.prod.yml`
- **Deployment Script**: `scripts/deploy-smart-contracts-docker.sh`

## Key Directories

### Host Machine
- **Scripts**: `./scripts/`
- **Docker configs**: `./docker/`
- **Smart contracts**: `./programs/`
- **Documentation**: `./docs/`

### Inside Containers
- **Validator**: `/opt/solana/`
- **Deployer workspace**: `/opt/deployer/workspace/`
- **Artifacts**: `/opt/deployer/artifacts/`
- **Logs**: `/opt/deployer/logs/`
- **PoA keys**: `/opt/deployer/config/poa-keys/`

## Environment Variables

```bash
# Common settings
export ENVIRONMENT=development|production
export VERBOSE=true
export SKIP_DEPLOYMENT=false
export SKIP_INFRASTRUCTURE=false
export SOLANA_RPC_URL=http://localhost:8899
```

## Quick Health Status Interpretation

- **🟢 All systems operational** - Everything working correctly
- **🟡 Warnings detected** - Non-critical issues, system functional
- **🔴 Critical failures** - Immediate attention required

---
For detailed instructions, see: `docs/DOCKER_DEPLOYMENT_GUIDE.md`