# Docker-Based Smart Contract Deployment Guide

## Overview

This guide provides comprehensive instructions for deploying and managing the P2P Energy Trading System's smart contracts using Docker. The system uses the `contact` service that automates the entire process from building containers to deploying smart contracts on various Solana networks including local validator, devnet, and mainnet.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Architecture Overview](#architecture-overview)
3. [Contact Service](#contact-service)
4. [Quick Start](#quick-start)
5. [Deployment Environments](#deployment-environments)
6. [Deployment Commands](#deployment-commands)
7. [Monitoring and Health Checks](#monitoring-and-health-checks)
8. [Troubleshooting](#troubleshooting)
9. [Advanced Configuration](#advanced-configuration)
10. [Security Considerations](#security-considerations)

## Prerequisites

### System Requirements

- **Docker**: Version 20.10+ with Docker Compose V2
- **Hardware**: Minimum 8GB RAM, 4 CPU cores, 50GB storage
- **Operating System**: Linux (Ubuntu 20.04+), macOS (Intel/Apple Silicon), Windows with WSL2
- **Network**: Unrestricted outbound internet access for initial setup

### Required Tools

```bash
# Verify Docker installation
docker --version
docker-compose --version

# Verify system resources
docker system info
```

### Initial Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd p2p-energy-trading
   ```

2. **Verify project structure**:
   ```bash
   ls -la docker/contact/
   ls -la Anchor.toml
   ls -la programs/
   ```

## Architecture Overview

### Container Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    P2P Energy Trading System               │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌──────────────────┐  ┌─────────────┐ │
│  │ Solana Validator│  │ Contact Service  │  │  Frontend   │ │
│  │   Container     │  │    Container     │  │  Container  │ │
│  │                 │  │                  │  │             │ │
│  │ • Test Validator│  │ • Anchor Build   │  │ • React App │ │
│  │ • Health checks │  │ • Smart Contract │  │ • Production │ │
│  │ • Platform deps │  │   Deployment     │  │   Optimized │ │
│  └─────────────────┘  └──────────────────┘  └─────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────┐  ┌───────┐  ┌──────────────┐ │
│  │ PostgreSQL  │  │  Redis   │  │API GW │  │   Grafana    │ │
│  │ TimescaleDB │  │  Cache   │  │ Rust  │  │  Monitoring  │ │
│  └─────────────┘  └──────────┘  └───────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Smart Contract Deployment Flow

```
1. Network Selection
   ├── Local Validator (development)
   ├── Solana Devnet (testing)
   └── Solana Mainnet (production)

2. Contact Service Deployment
   ├── Build optimized container
   ├── Mount workspace and programs
   ├── Configure target network
   └── Execute deployment scripts

3. Contract Build & Deploy
   ├── Build all 5 Anchor programs
   ├── Deploy in dependency order
   ├── Verify program accessibility
   └── Save deployment artifacts

4. Health Monitoring
   ├── Validator connectivity checks
   ├── Program deployment verification
   └── Continuous health monitoring
```

## Contact Service

The `contact` service is the core deployment automation component that handles:

### Key Features
- **Multi-Network Support**: Local validator, devnet, mainnet
- **Optimized Container**: Multi-stage Docker build with minimal runtime
- **Simplified Scripts**: Clean, maintainable deployment automation
- **Health Monitoring**: Comprehensive service and deployment verification
- **Flexible Configuration**: Environment-based network switching

### Available Scripts
- `wait-for-validator.sh` - Network connectivity verification
- `build-contracts.sh` - Anchor smart contract compilation
- `deploy-all-contracts.sh` - Complete deployment pipeline
- `verify-deployment.sh` - Post-deployment verification
- `setup-poa.sh` - Proof of Authority initialization
- `health-monitor.sh` - Comprehensive health checks
   ├── Initialize governance program
   └── Configure REC validators

5. Post-Deployment
   ├── Run health checks
   ├── Verify program functionality
   ├── Initialize monitoring
   └── Generate deployment report
```

## Quick Start

### 1. Build All Services
```bash
# Build all containers including contact service
docker-compose build

# Build only contact service
docker-compose build contact
```

### 2. Development with Devnet (Recommended)
```bash
# Test network connectivity
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/wait-for-validator.sh

# Build smart contracts
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/build-contracts.sh

# Deploy all contracts to devnet
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/deploy-all-contracts.sh
```

### 3. Local Development (Advanced)
```bash
# Start local validator (if healthy)
docker-compose up -d solana-validator

# Wait for validator to be ready
docker-compose run --rm contact /usr/local/bin/wait-for-validator.sh

# Deploy to local validator
docker-compose run --rm contact /usr/local/bin/deploy-all-contracts.sh
```

### 4. Start All Services
```bash
# Start complete infrastructure
docker-compose up -d

# Check service status
docker-compose ps
```

# Or use docker-compose with production overrides
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Infrastructure Only

```bash
# Start only infrastructure services (no smart contracts)
./scripts/deploy-smart-contracts-docker.sh infrastructure
```

## Deployment Environments

### Environment Configuration

| Environment | RPC URL | Use Case | Configuration |
|-------------|---------|----------|---------------|
| **Local** | `http://solana-validator:8899` | Development | Default, requires healthy validator |
| **Devnet** | `https://api.devnet.solana.com` | Testing | Recommended for testing |
| **Mainnet** | `https://api.mainnet-beta.solana.com` | Production | Production deployment |

### Environment Variables

```bash
# Core Configuration
SOLANA_RPC_URL="<network-endpoint>"
ANCHOR_PROVIDER_URL="<network-endpoint>"
ANCHOR_WALLET="/opt/deployer/config/deployer-keypair.json"

# Deployment Settings
RUST_LOG="info"
DEPLOYMENT_TIMEOUT="300"
MAX_RETRIES="3"
```

## Deployment Commands

### Basic Operations

```bash
# Health check
docker-compose run --rm contact /usr/local/bin/health-monitor.sh status

# Network connectivity test
docker-compose run --rm -e SOLANA_RPC_URL="<endpoint>" contact /usr/local/bin/wait-for-validator.sh

# Build contracts only
docker-compose run --rm contact /usr/local/bin/build-contracts.sh

# Full deployment pipeline
docker-compose run --rm -e SOLANA_RPC_URL="<endpoint>" contact /usr/local/bin/deploy-all-contracts.sh
```

### Network-Specific Deployments

```bash
# Devnet deployment (recommended for testing)
docker-compose run --rm \
  -e SOLANA_RPC_URL="https://api.devnet.solana.com" \
  contact /usr/local/bin/deploy-all-contracts.sh

# Local validator deployment
docker-compose run --rm \
  -e SOLANA_RPC_URL="http://solana-validator:8899" \
  contact /usr/local/bin/deploy-all-contracts.sh

# Mainnet deployment (production)
docker-compose run --rm \
  -e SOLANA_RPC_URL="https://api.mainnet-beta.solana.com" \
  contact /usr/local/bin/deploy-all-contracts.sh
```

### Advanced Operations

```bash
# Interactive container access
docker-compose run --rm contact bash

# Custom script execution
docker-compose run --rm contact bash -c "your-custom-command"

# Deployment verification
docker-compose run --rm contact /usr/local/bin/verify-deployment.sh

# Detailed health monitoring
docker-compose run --rm contact /usr/local/bin/health-monitor.sh detailed
```
## Monitoring and Health Checks

### Service Health Status

```bash
# Check all services
docker-compose ps

# Check contact service logs
docker-compose logs contact

# Check validator status
docker-compose logs solana-validator --tail=20

# Real-time monitoring
docker-compose logs -f contact
```

### Health Check Scripts

```bash
# Quick status check
docker-compose run --rm contact /usr/local/bin/health-monitor.sh status

# Detailed health report
docker-compose run --rm contact /usr/local/bin/health-monitor.sh detailed

# Network connectivity verification
docker-compose run --rm contact /usr/local/bin/wait-for-validator.sh
```

### Deployment Verification

```bash
# Verify successful deployment
docker-compose run --rm contact /usr/local/bin/verify-deployment.sh

# Check deployed programs
docker-compose run --rm contact bash -c "solana program show --programs"

# Verify program accessibility
docker-compose run --rm contact bash -c "anchor test --skip-local-validator"
```

## Troubleshooting

### Common Issues

#### 1. Validator Platform Issues
**Problem**: Local validator unhealthy due to platform compatibility
```bash
# Check validator logs
docker-compose logs solana-validator --tail=20

# Workaround: Use devnet
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/wait-for-validator.sh
```

#### 2. Volume Mount Issues
**Problem**: Artifacts directory busy or permission errors
```bash
# Clean volumes
docker-compose down -v
docker volume prune

# Rebuild and restart
docker-compose build contact
docker-compose up -d
```

#### 3. Network Connectivity
**Problem**: Cannot connect to Solana network
```bash
# Test network connectivity
docker-compose run --rm contact bash -c "solana cluster-version --url https://api.devnet.solana.com"

# Check DNS resolution
docker-compose run --rm contact bash -c "nslookup api.devnet.solana.com"
```

#### 4. Build Failures
**Problem**: Anchor build fails
```bash
# Check workspace structure
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/Anchor.toml"

# Verify programs directory
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/programs/"

# Clean and rebuild
docker-compose run --rm contact bash -c "cd /workspaces/p2p && anchor clean && anchor build"
```

### Debug Commands

```bash
# Interactive debugging session
docker-compose run --rm contact bash

# Check available tools
docker-compose run --rm contact bash -c "which anchor && which solana"

# Verify environment variables
docker-compose run --rm contact bash -c "env | grep SOLANA"

# Test script permissions
docker-compose run --rm contact bash -c "ls -la /usr/local/bin/*.sh"
```

## Advanced Configuration

### Custom Network Configuration

```yaml
# docker-compose.override.yml
services:
  contact:
    environment:
      SOLANA_RPC_URL: "https://your-custom-endpoint.com"
      ANCHOR_PROVIDER_URL: "https://your-custom-endpoint.com"
      DEPLOYMENT_TIMEOUT: "600"
      MAX_RETRIES: "5"
```

### Persistent Storage Configuration

```yaml
# Custom volume configuration
volumes:
  contact_artifacts:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: /host/path/to/artifacts
```

### Production Deployment

```bash
# Production environment setup
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"
export ANCHOR_WALLET="/path/to/production/keypair.json"

# Deploy to production
docker-compose run --rm \
  -e SOLANA_RPC_URL="$SOLANA_RPC_URL" \
  -e ANCHOR_WALLET="$ANCHOR_WALLET" \
  contact /usr/local/bin/deploy-all-contracts.sh
```

## Security Considerations

### Keypair Management

1. **Development**: Use generated keypairs for local/devnet testing
2. **Production**: Use secure, externally managed keypairs
3. **Storage**: Never commit keypairs to version control
4. **Access**: Limit container access to necessary keypairs only

### Network Security

```bash
# Secure RPC endpoints
export SOLANA_RPC_URL="https://secure-rpc-endpoint.com"

# Use environment-specific configurations
export RUST_LOG="warn"  # Reduce log verbosity in production
```

### Container Security

```bash
# Run with specific user (non-root)
docker-compose run --rm --user $(id -u):$(id -g) contact <command>

# Limit container capabilities
docker-compose run --rm --cap-drop=ALL contact <command>
```

## Conclusion

The Docker-based deployment system provides a robust, scalable solution for managing smart contract deployments across different Solana networks. The `contact` service offers simplified deployment automation while maintaining flexibility for various deployment scenarios.

For additional support and troubleshooting, refer to:
- [Docker Troubleshooting Guide](./DOCKER_TROUBLESHOOTING.md)
- [Docker Quick Reference](./DOCKER_QUICK_REFERENCE.md)
- [System Architecture](./SYSTEM_ARCHITECTURE.md)
# Start contract deployer
docker-compose up -d contact

# Monitor deployment progress
docker exec p2p-contact /usr/local/bin/health-monitor.sh detailed

# Check deployment artifacts
docker exec p2p-contact ls -la /opt/deployer/artifacts/
```

#### 4. PoA Initialization

```bash
# Run PoA setup
docker exec p2p-contact /usr/local/bin/setup-poa.sh

# Verify PoA configuration
docker exec p2p-contact cat /opt/deployer/artifacts/poa-config.json
```

## Monitoring and Health Checks

### Health Check System

The deployment includes comprehensive health monitoring with three levels:

1. **Container Health**: Docker healthcheck directives
2. **Service Health**: Application-level health endpoints
3. **System Health**: Resource and performance monitoring

### Health Check Levels

- **🟢 HEALTHY (0)**: All systems operational
- **🟡 WARNING (1)**: Non-critical issues detected
- **🔴 CRITICAL (2)**: Critical failures requiring attention

### Health Monitoring Commands

```bash
# Check overall system health
docker exec p2p-contact /usr/local/bin/health-monitor.sh check

# Detailed health report
docker exec p2p-contact /usr/local/bin/health-monitor.sh detailed

# Monitor and alert on warnings
docker exec p2p-contact /usr/local/bin/health-monitor.sh monitor

# Get health status code
docker exec p2p-contact /usr/local/bin/health-monitor.sh status
```

### Health Check Components

1. **Validator Health**:
   - RPC endpoint responsiveness
   - Block production status
   - Network connectivity

2. **Deployment Status**:
   - Program deployment verification
   - On-chain program accessibility
   - Artifact integrity

3. **PoA Governance**:
   - Governance program initialization
   - REC validator configuration
   - Authority account setup

4. **System Resources**:
   - Disk space utilization
   - Memory usage
   - Network connectivity

### Monitoring Dashboard

Access the Grafana monitoring dashboard:

**Development**: `http://localhost:3001`
**Production**: `http://localhost:80/monitoring`

**Default credentials**:
- Username: `admin`
- Password: `dev_admin` (development) / `secure_admin_password` (production)

### Log Monitoring

```bash
# View all service logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f solana-validator
docker-compose logs -f contact

# View deployment logs
docker exec p2p-contact tail -f /opt/deployer/logs/deploy.log

# View health monitoring logs
docker exec p2p-contact tail -f /opt/deployer/logs/health-monitor.log
```

## Troubleshooting

### Common Issues

#### 1. Validator Not Starting

**Symptoms**:
- Health check fails with "Validator not responding"
- Container exits immediately

**Solutions**:
```bash
# Check container logs
docker-compose logs solana-validator

# Verify ARM64 compatibility
docker exec p2p-anchor-dev uname -m

# Try development mode
docker-compose up -d -e START_VALIDATOR=false solana-validator
```

#### 2. Contract Deployment Fails

**Symptoms**:
- Deployment hangs or fails
- Missing program artifacts

**Solutions**:
```bash
# Check deployer logs
docker exec p2p-contact tail -f /opt/deployer/logs/deploy.log

# Verify workspace mounting
docker exec p2p-contact ls -la /opt/deployer/workspace/programs/

# Rebuild contracts
docker exec p2p-contact /usr/local/bin/build-contracts.sh

# Manual deployment
docker exec -it p2p-contact bash
cd /opt/deployer/workspace
anchor build
anchor deploy
```

#### 3. PoA Initialization Fails

**Symptoms**:
- PoA status shows "FAILED"
- Missing PoA configuration

**Solutions**:
```bash
# Check PoA logs
docker exec p2p-contact cat /opt/deployer/logs/poa-setup.log

# Verify governance program
docker exec p2p-contact cat /opt/deployer/artifacts/governance/program_id.txt

# Manual PoA setup
docker exec p2p-contact /usr/local/bin/setup-poa.sh

# Check Node.js dependencies
docker exec p2p-contact npm list @coral-xyz/anchor
```

#### 4. Resource Issues

**Symptoms**:
- Containers killed by OOM
- Slow performance

**Solutions**:
```bash
# Check resource usage
docker stats

# Increase Docker resources
# Docker Desktop: Settings > Resources > Advanced

# Clean up unused resources
docker system prune -a
docker volume prune
```

### Debugging Commands

```bash
# Enter container shell
docker exec -it p2p-anchor-dev bash
docker exec -it p2p-contact bash

# Check container environment
docker exec p2p-contact env

# Verify file permissions
docker exec p2p-contact ls -la /usr/local/bin/

# Test network connectivity
docker exec p2p-contact curl -s http://solana-validator:8899

# Check disk space
docker exec p2p-contact df -h

# Verify Anchor installation
docker exec p2p-contact anchor --version
```

### Recovery Procedures

#### 1. Restart Deployment

```bash
# Clean restart
./scripts/deploy-smart-contracts-docker.sh clean
./scripts/deploy-smart-contracts-docker.sh deploy

# Partial restart (keep data)
docker-compose restart solana-validator contact
```

#### 2. Reset Validator

```bash
# Reset validator with clean ledger
docker-compose down solana-validator
docker volume rm p2p_solana_ledger
docker-compose up -d solana-validator
```

#### 3. Rebuild Containers

```bash
# Rebuild specific container
docker-compose build --no-cache solana-validator
docker-compose up -d solana-validator

# Rebuild all containers
docker-compose build --no-cache
docker-compose up -d
```

## Advanced Configuration

### Custom Environment Variables

Create a `.env` file in the project root:

```bash
# .env file
ENVIRONMENT=production
POSTGRES_PASSWORD=secure_password
GRAFANA_ADMIN_PASSWORD=secure_admin_password
SOLANA_RPC_URL=http://custom-validator:8899
RUST_LOG=debug
SKIP_DEPLOYMENT=false
SKIP_INFRASTRUCTURE=false
VERBOSE=true
```

### Volume Mounting

For development with hot reload:

```yaml
# docker-compose.override.yml
services:
  solana-validator:
    volumes:
      - ./programs:/workspaces/programs:cached
      - ./target:/workspaces/target:cached
  
  contact:
    volumes:
      - .:/opt/deployer/workspace:cached
      - ./custom-scripts:/opt/deployer/custom:ro
```

### Network Configuration

For custom networking:

```yaml
# docker-compose.override.yml
networks:
  p2p-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.25.0.0/16
          gateway: 172.25.0.1
```

### Resource Limits

For production optimization:

```yaml
# docker-compose.override.yml
services:
  solana-validator:
    deploy:
      resources:
        limits:
          memory: 8G
          cpus: '4'
        reservations:
          memory: 4G
          cpus: '2'
```

## Security Considerations

### Container Security

1. **Non-root users**: All containers run as non-privileged users
2. **Read-only filesystems**: Application directories are read-only where possible
3. **Resource limits**: CPU and memory limits prevent resource exhaustion
4. **Network isolation**: Services communicate through internal Docker networks

### Secret Management

```bash
# Use Docker secrets for sensitive data
echo "secure_password" | docker secret create postgres_password -

# Or use external secret management
export POSTGRES_PASSWORD=$(vault kv get -field=password secret/postgres)
```

### Network Security

```bash
# Restrict external access in production
# Only expose necessary ports
ports:
  - "127.0.0.1:8899:8899"  # Bind to localhost only
```

### PoA Security

1. **Key management**: Store PoA keypairs securely
2. **Access control**: Limit validator node access
3. **Audit logging**: Enable comprehensive logging
4. **Regular rotation**: Rotate keys periodically

### Production Checklist

- [ ] Change default passwords
- [ ] Configure SSL/TLS certificates
- [ ] Enable firewall rules
- [ ] Set up log rotation
- [ ] Configure backup procedures
- [ ] Enable monitoring alerts
- [ ] Review security policies

## Conclusion

This Docker-based deployment system provides a robust, scalable, and maintainable solution for deploying the P2P Energy Trading System's smart contracts. The automated pipeline ensures consistent deployments across development and production environments while providing comprehensive monitoring and health checking capabilities.

For additional support or troubleshooting assistance, refer to the project's main documentation or contact the development team.

---

**Version**: 1.0.0  
**Last Updated**: September 18, 2025  
**Maintainer**: P2P Energy Trading Development Team