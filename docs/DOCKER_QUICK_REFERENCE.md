# Docker Smart Contract Deployment - Quick Reference

## Essential Commands

### Development Environment
```bash
# Start development environment
./scripts/deploy-smart-contracts-docker.sh deploy
# or
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d

# View logs
docker-compose logs -f solana-validator
docker-compose logs -f contract-deployer

# Check health status
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh check
```

### Production Environment
```bash
# Start production environment
ENVIRONMENT=production ./scripts/deploy-smart-contracts-docker.sh deploy
# or
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d

# Production health check
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh detailed
```

### Service Management
```bash
# Infrastructure only
./scripts/deploy-smart-contracts-docker.sh infrastructure

# Smart contracts only
./scripts/deploy-smart-contracts-docker.sh contracts

# Verify deployment
./scripts/deploy-smart-contracts-docker.sh verify

# Clean up everything
./scripts/deploy-smart-contracts-docker.sh clean
```

## Health Checks & Monitoring

### Quick Health Check
```bash
# Overall system status
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh status

# Detailed report
docker exec p2p-contract-deployer /usr/local/bin/health-monitor.sh detailed
```

### Container Status
```bash
# View all containers
docker-compose ps

# Check specific service health
docker inspect p2p-anchor-dev --format='{{.State.Health.Status}}'
docker inspect p2p-contract-deployer --format='{{.State.Health.Status}}'
```

### Logs
```bash
# Deployment logs
docker exec p2p-contract-deployer tail -f /opt/deployer/logs/deploy.log

# PoA setup logs
docker exec p2p-contract-deployer tail -f /opt/deployer/logs/poa-setup.log

# Health monitoring logs
docker exec p2p-contract-deployer tail -f /opt/deployer/logs/health-monitor.log
```

## Troubleshooting

### Common Issues
```bash
# Validator not responding
docker-compose restart solana-validator
docker exec p2p-anchor-dev solana cluster-version --url http://localhost:8899

# Contract deployment failed
docker exec p2p-contract-deployer /usr/local/bin/build-contracts.sh
docker exec p2p-contract-deployer /usr/local/bin/verify-deployment.sh

# PoA initialization failed
docker exec p2p-contract-deployer /usr/local/bin/setup-poa.sh
```

### Debug Mode
```bash
# Enter container shell
docker exec -it p2p-contract-deployer bash
docker exec -it p2p-anchor-dev bash

# Check environment
docker exec p2p-contract-deployer env | grep -E "SOLANA|ANCHOR|RUST"

# Verify file permissions
docker exec p2p-contract-deployer ls -la /usr/local/bin/
```

### Recovery
```bash
# Full reset (destroys data)
./scripts/deploy-smart-contracts-docker.sh clean
./scripts/deploy-smart-contracts-docker.sh deploy

# Restart services (preserves data)
docker-compose restart

# Rebuild containers
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