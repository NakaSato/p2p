# Docker-Based Smart Contract Deployment Guide

## Overview

This guide provides comprehensive instructions for deploying and managing the P2P Energy Trading System's smart contracts using Docker. The system uses an enhanced deployment pipeline that automates the entire process from building containers to initializing the PoA governance system.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Architecture Overview](#architecture-overview)
3. [Quick Start](#quick-start)
4. [Deployment Environments](#deployment-environments)
5. [Deployment Pipeline](#deployment-pipeline)
6. [Monitoring and Health Checks](#monitoring-and-health-checks)
7. [Troubleshooting](#troubleshooting)
8. [Advanced Configuration](#advanced-configuration)
9. [Security Considerations](#security-considerations)

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
   ls -la scripts/deploy-smart-contracts-docker.sh
   ls -la docker/contract-deployer/
   ls -la docker/solana-validator/
   ```

## Architecture Overview

### Container Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    P2P Energy Trading System               │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌──────────────────┐  ┌─────────────┐ │
│  │ Solana Validator│  │ Contract Deployer│  │  Frontend   │ │
│  │   Container     │  │    Container     │  │  Container  │ │
│  │                 │  │                  │  │             │ │
│  │ • Validator     │  │ • Anchor Build   │  │ • React App │ │
│  │ • Auto-deploy  │  │ • Program Deploy │  │ • Vite Dev  │ │
│  │ • Health checks │  │ • PoA Init       │  │ • Hot reload│ │
│  └─────────────────┘  └──────────────────┘  └─────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────┐  ┌───────┐  ┌──────────────┐ │
│  │ PostgreSQL  │  │  Redis   │  │ Kafka │  │   Grafana    │ │
│  │ Database    │  │  Cache   │  │ Queue │  │  Monitoring  │ │
│  └─────────────┘  └──────────┘  └───────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Smart Contract Deployment Flow

```
1. Infrastructure Setup
   ├── Start PostgreSQL, Redis, Kafka
   ├── Initialize TimescaleDB
   └── Start monitoring services

2. Solana Validator
   ├── Build enhanced validator container
   ├── Start Solana test validator
   ├── Generate validator keypairs
   └── Begin health monitoring

3. Contract Deployment
   ├── Build all 5 Anchor programs
   ├── Deploy in dependency order
   ├── Verify program accessibility
   └── Save deployment artifacts

4. PoA Initialization
   ├── Generate PoA authority keypairs
   ├── Airdrop SOL to authorities
   ├── Initialize governance program
   └── Configure REC validators

5. Post-Deployment
   ├── Run health checks
   ├── Verify program functionality
   ├── Initialize monitoring
   └── Generate deployment report
```

## Quick Start

### Development Environment

```bash
# Start complete development environment
./scripts/deploy-smart-contracts-docker.sh deploy

# Or use docker-compose directly
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d
```

### Production Environment

```bash
# Deploy production environment
ENVIRONMENT=production ./scripts/deploy-smart-contracts-docker.sh deploy

# Or use docker-compose with production overrides
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Infrastructure Only

```bash
# Start only infrastructure services (no smart contracts)
./scripts/deploy-smart-contracts-docker.sh infrastructure
```

## Deployment Environments

### Development Environment

**Configuration**: `docker-compose.dev.yml`

**Features**:
- Hot reload for frontend and API
- Debug logging enabled
- Development database settings
- Direct port mappings for easy access
- Auto-deployment of smart contracts
- Enhanced monitoring and verbose health checks

**Services**:
- Solana Validator: `localhost:8899` (RPC), `localhost:8900` (WebSocket)
- Frontend: `localhost:3000`
- API Gateway: `localhost:3030`
- PostgreSQL: `localhost:5433`
- Redis: `localhost:6380`
- Grafana: `localhost:3001`
- pgAdmin: `localhost:5050`

**Usage**:
```bash
# Start development environment
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d

# View logs
docker-compose -f docker-compose.yml -f docker-compose.dev.yml logs -f

# Rebuild and restart specific service
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d --build solana-validator
```

### Production Environment

**Configuration**: `docker-compose.prod.yml`

**Features**:
- Optimized resource allocation
- Production security settings
- Nginx reverse proxy
- Persistent data volumes
- Resource limits and reservations
- Automated restart policies

**Services**:
- All services behind Nginx: `localhost:80`
- Monitoring dashboard: `localhost:80/monitoring`
- API endpoints: `localhost:80/api`

**Usage**:
```bash
# Start production environment
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d

# Scale services
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d --scale api-gateway=3

# Update without downtime
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d --no-deps api-gateway
```

## Deployment Pipeline

### Automated Deployment Script

The main deployment script `scripts/deploy-smart-contracts-docker.sh` provides several commands:

#### Commands

1. **`deploy`** (default): Complete deployment pipeline
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh deploy
   ```

2. **`infrastructure`**: Deploy only supporting services
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh infrastructure
   ```

3. **`validator`**: Deploy only Solana validator
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh validator
   ```

4. **`build`**: Build smart contracts only
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh build
   ```

5. **`contracts`**: Deploy smart contracts only
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh contracts
   ```

6. **`verify`**: Verify deployment status
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh verify
   ```

7. **`initialize`**: Initialize PoA system
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh initialize
   ```

8. **`test`**: Run post-deployment tests
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh test
   ```

9. **`clean`**: Clean up all services and volumes
   ```bash
   ./scripts/deploy-smart-contracts-docker.sh clean
   ```

#### Environment Variables

```bash
# Environment selection
export ENVIRONMENT=development|production

# Skip infrastructure deployment
export SKIP_INFRASTRUCTURE=true

# Skip post-deployment tests
export SKIP_TESTS=true

# Enable verbose logging
export VERBOSE=true

# Custom validator URL
export SOLANA_RPC_URL=http://custom-validator:8899
```

### Manual Deployment Steps

#### 1. Infrastructure Setup

```bash
# Start infrastructure services
docker-compose up -d postgres redis kafka zookeeper timescaledb grafana prometheus

# Wait for services to be healthy
docker-compose ps
```

#### 2. Solana Validator

```bash
# Build and start validator
docker-compose up -d solana-validator

# Check validator health
docker exec p2p-anchor-dev solana cluster-version --url http://localhost:8899

# View validator logs
docker-compose logs -f solana-validator
```

#### 3. Contract Deployment

```bash
# Start contract deployer
docker-compose up -d contract-deployer

# Monitor deployment progress
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh detailed

# Check deployment artifacts
docker exec p2p-contract-deployer ls -la /opt/deployer/artifacts/
```

#### 4. PoA Initialization

```bash
# Run PoA setup
docker exec p2p-contract-deployer /usr/local/bin/setup-poa.sh

# Verify PoA configuration
docker exec p2p-contract-deployer cat /opt/deployer/artifacts/poa-config.json
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
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh check

# Detailed health report
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh detailed

# Monitor and alert on warnings
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh monitor

# Get health status code
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh status
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
docker-compose logs -f contract-deployer

# View deployment logs
docker exec p2p-contract-deployer tail -f /opt/deployer/logs/deploy.log

# View health monitoring logs
docker exec p2p-contract-deployer tail -f /opt/deployer/logs/health-monitor.log
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
docker exec p2p-contract-deployer tail -f /opt/deployer/logs/deploy.log

# Verify workspace mounting
docker exec p2p-contract-deployer ls -la /opt/deployer/workspace/programs/

# Rebuild contracts
docker exec p2p-contract-deployer /usr/local/bin/build-contracts.sh

# Manual deployment
docker exec -it p2p-contract-deployer bash
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
docker exec p2p-contract-deployer cat /opt/deployer/logs/poa-setup.log

# Verify governance program
docker exec p2p-contract-deployer cat /opt/deployer/artifacts/governance/program_id.txt

# Manual PoA setup
docker exec p2p-contract-deployer /usr/local/bin/setup-poa.sh

# Check Node.js dependencies
docker exec p2p-contract-deployer npm list @coral-xyz/anchor
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
docker exec -it p2p-contract-deployer bash

# Check container environment
docker exec p2p-contract-deployer env

# Verify file permissions
docker exec p2p-contract-deployer ls -la /usr/local/bin/

# Test network connectivity
docker exec p2p-contract-deployer curl -s http://solana-validator:8899

# Check disk space
docker exec p2p-contract-deployer df -h

# Verify Anchor installation
docker exec p2p-contract-deployer anchor --version
```

### Recovery Procedures

#### 1. Restart Deployment

```bash
# Clean restart
./scripts/deploy-smart-contracts-docker.sh clean
./scripts/deploy-smart-contracts-docker.sh deploy

# Partial restart (keep data)
docker-compose restart solana-validator contract-deployer
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
  
  contract-deployer:
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