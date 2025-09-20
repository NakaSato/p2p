# Docker Smart Contract Deployment - Troubleshooting Guide

## Troubleshooting Decision Tree

```
Deployment Issue?
├── Container won't start?
│   ├── Check Docker resources → Increase memory/CPU
│   ├── Port conflicts? → Change port mappings
│   └── Image build fails? → Check Dockerfile & dependencies
├── API Gateway build fails?
│   ├── SQLx compilation errors? → Regenerate query cache
│   ├── Database connection during build? → Use SQLX_OFFLINE=true
│   └── Missing .sqlx directory? → Run cargo sqlx prepare
├── Validator issues?
│   ├── Platform compatibility? → Use devnet workaround
│   ├── Missing dependencies? → Check Dockerfile for bzip2/curl
│   ├── ARM64 Mac issues? → Use --platform linux/amd64
│   └── Health check fails? → Check validator logs
├── Contact service deployment fails?
│   ├── Build errors? → Check workspace mounting
│   ├── Network connectivity? → Test RPC endpoints
│   ├── Deploy timeout? → Increase deployment timeout
│   └── Volume issues? → Check artifact directory permissions
├── Contract deployment fails?
│   ├── Anchor build errors? → Check Anchor.toml and workspace
│   ├── Program deployment fails? → Check SOL balance and RPC
│   └── Verification fails? → Check deployed program status
└── PoA initialization fails?
    ├── Missing governance program? → Deploy programs first
    ├── Authority setup? → Check SOL funding for authorities
    └── Keypair keys? → Regenerate authority keys
```

## Critical Issues & Solutions

### 1. Validator Platform Compatibility Issues

#### **Problem**: Solana validator unhealthy on Apple Silicon (M1/M2) Macs
```bash
# Symptoms
docker-compose logs solana-validator --tail=20
# Shows: missing bzip2, platform warnings

# Solution 1: Use Devnet (Recommended)
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/wait-for-validator.sh
```

#### **Problem**: Platform architecture mismatches
```bash
# Symptoms
WARNING: The requested image's platform (linux/amd64) does not match the detected host platform

# Solution: Force platform in docker-compose.yml
services:
  solana-validator:
    platform: linux/amd64
```

### 2. Contact Service Issues

#### **Problem**: Workspace mounting issues
```bash
# Symptoms
Not in workspace error from Anchor

# Check workspace mounting
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/Anchor.toml"

# Solution: Verify docker-compose.yml volumes
volumes:
  - .:/workspaces/p2p
```

#### **Problem**: Artifacts directory busy
```bash
# Symptoms
rm: cannot remove '/opt/deployer/artifacts': Device or resource busy

# Solution: Clean volumes and rebuild
docker-compose down -v
docker volume prune
docker-compose build contact
```

### 3. Network Connectivity Issues

#### **Problem**: Cannot connect to Solana network
```bash
# Test connectivity
docker-compose run --rm contact bash -c "solana cluster-version --url https://api.devnet.solana.com"

# Check DNS resolution
docker-compose run --rm contact bash -c "nslookup api.devnet.solana.com"

# Solution: Use different RPC endpoint
export SOLANA_RPC_URL="https://api.devnet.solana.com"
```

### 4. API Gateway Build Issues

#### **Problem**: SQLx compile-time query validation fails
```bash
# Error: set `DATABASE_URL` to use query macros online
# Error: failed to compile `api-gateway` (lib) due to 7 previous errors
```

**Root Cause**: SQLx macros need database connectivity during compilation to validate queries.

**Solution**: Use SQLx offline mode with query cache:

1. **Generate query cache locally**:
   ```bash
   # Start database
   docker-compose up -d postgres
   
   # Generate cache
   cd api-gateway
   export DATABASE_URL="postgresql://p2p_user:p2p_password@localhost:5432/p2p_energy_trading"
   cargo sqlx prepare
   
   # Commit the .sqlx/ directory
   git add .sqlx/
   git commit -m "Update SQLx query cache"
   ```

2. **Dockerfile includes offline mode**:
   ```dockerfile
   # Copy SQLx query cache
   COPY api-gateway/.sqlx ./.sqlx
   
   # Enable offline mode
   ENV SQLX_OFFLINE=true
   ```

3. **Verify build works**:
   ```bash
   docker-compose build api-gateway
   ```

**Prevention**: Always regenerate query cache after database schema changes.

## Diagnostic Commands

### System Information
```bash
# Check Docker resources
docker system info
docker system df

# Check available memory and CPU
free -h
nproc

# Check disk space
df -h

# Check Docker version
docker --version
docker-compose --version
```

### Container Status
```bash
# List all containers
docker-compose ps

# Check container resource usage
docker stats

# Inspect contact service configuration
docker inspect p2p-contact

# Check contact service logs
docker-compose logs contact --tail=50

# Check validator health (if using local)
docker-compose logs solana-validator --tail=20
```

### Network Diagnostics
```bash
# List Docker networks
docker network ls

# Inspect network configuration
docker network inspect p2p_p2p-network

# Test network connectivity from contact service
docker-compose run --rm contact bash -c "ping -c 3 api.devnet.solana.com"
docker-compose run --rm contact bash -c "curl -s https://api.devnet.solana.com"
```

### Volume Diagnostics
```bash
# List volumes
docker volume ls | grep contact

# Inspect volume configuration
docker volume inspect p2p_contact_artifacts
docker volume inspect p2p_deployment_logs

# Check volume contents
docker-compose run --rm contact bash -c "ls -la /opt/deployer/artifacts"
docker-compose run --rm contact bash -c "ls -la /opt/deployer/logs"
```

### Contact Service Diagnostics
```bash
# Check available scripts
docker-compose run --rm contact ls -la /usr/local/bin/

# Verify tool availability
docker-compose run --rm contact bash -c "which anchor && which solana"

# Test network connectivity
docker-compose run --rm contact /usr/local/bin/wait-for-validator.sh

# Check workspace mounting
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/Anchor.toml"

# Run health monitoring
docker-compose run --rm contact /usr/local/bin/health-monitor.sh status
```

## Common Error Scenarios

### 1. Container Build Failures

#### **Error**: "no space left on device"
```bash
# Clean up Docker system
docker system prune -a -f
docker volume prune -f

# Check available space
df -h

# If still needed, remove unused images
docker image prune -a -f
```

#### **Error**: "failed to fetch package" or "network timeout"
```bash
# Check internet connectivity
ping 8.8.8.8

# Rebuild with no cache
docker-compose build --no-cache contact

# Alternative: rebuild specific stage
docker-compose build --no-cache --pull contact
```

### 2. Validator Platform Issues

#### **Error**: Validator container unhealthy
```bash
# Check validator logs
docker-compose logs solana-validator --tail=20

# Common error: missing bzip2
# ERROR: bzip2: command not found

# Solution: Use devnet workaround
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/deploy-all-contracts.sh
```

#### **Error**: Platform warnings on Apple Silicon
```bash
# Symptoms
WARNING: The requested image's platform (linux/amd64) does not match the detected host platform (linux/arm64/v8)

# Solution: Force platform in docker-compose.yml or use devnet
export SOLANA_RPC_URL="https://api.devnet.solana.com"
```

### 3. Contact Service Deployment Issues

#### **Error**: "Not in workspace" from Anchor
```bash
# Check workspace mounting
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/Anchor.toml"

# If missing, verify docker-compose.yml volumes section
volumes:
  - .:/workspaces/p2p
```

#### **Error**: "Device or resource busy" for artifacts
```bash
# Clean volumes and restart
docker-compose down -v
docker volume prune -f
docker-compose build contact
docker-compose up -d
```

#### **Error**: Network connectivity timeout
```bash
# Test different endpoints
docker-compose run --rm contact bash -c "solana cluster-version --url https://api.devnet.solana.com"
docker-compose run --rm contact bash -c "solana cluster-version --url https://api.mainnet-beta.solana.com"

# Check firewall/proxy settings
docker-compose run --rm contact bash -c "nslookup api.devnet.solana.com"
```

### 4. Smart Contract Deployment Failures

#### **Error**: "Insufficient SOL balance"
```bash
# Check deployer balance
docker-compose run --rm contact bash -c "solana balance --url https://api.devnet.solana.com"

# Request airdrop (devnet only)
docker-compose run --rm contact bash -c "solana airdrop 1 --url https://api.devnet.solana.com"
```

#### **Error**: "Program deployment failed"
```bash
# Check program build first
docker-compose run --rm contact /usr/local/bin/build-contracts.sh

# Verify programs directory
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/programs/"

# Check for build artifacts
docker-compose run --rm contact bash -c "ls -la /workspaces/p2p/target/deploy/"
```

#### **Error**: "RPC request failed"
```bash
# Try different RPC endpoint
export SOLANA_RPC_URL="https://api.devnet.solana.com"

# Check endpoint status
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' https://api.devnet.solana.com
```

### 5. Permission and Access Issues

#### **Error**: "permission denied" for Docker commands
```bash
# Check Docker daemon status
sudo systemctl status docker

# Fix permissions (Linux)
sudo usermod -aG docker $USER
newgrp docker

# On macOS, ensure Docker Desktop is running
open -a Docker
```

#### **Error**: Volume mount permission issues
```bash
# Check volume ownership
docker-compose run --rm contact bash -c "ls -la /opt/deployer/"

# Fix with proper user mapping
docker-compose run --rm --user $(id -u):$(id -g) contact bash
```

## Recommended Workarounds

### 1. Development with Devnet (Primary Recommendation)
```bash
# For reliable development, use devnet instead of local validator
export SOLANA_RPC_URL="https://api.devnet.solana.com"

# All contact service operations with devnet
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/wait-for-validator.sh
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/build-contracts.sh
docker-compose run --rm -e SOLANA_RPC_URL="https://api.devnet.solana.com" contact /usr/local/bin/deploy-all-contracts.sh
```

### 2. Local Validator Alternatives
```bash
# If local validator needed, disable dependency in docker-compose.yml
services:
  contact:
    # depends_on:
    #   solana-validator:
    #     condition: service_healthy

# Then run contact service independently
docker-compose run --rm --no-deps contact /usr/local/bin/deploy-all-contracts.sh
```

### 3. Clean Development Environment
```bash
# Complete environment reset
docker-compose down -v
docker system prune -a -f
docker volume prune -f

# Rebuild from scratch
docker-compose build --no-cache
docker-compose up -d
```

## Performance Optimization

### 1. Docker Build Optimization
```bash
# Use BuildKit for faster builds
export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1

# Build with multiple cores
docker-compose build --parallel
```

### 2. Resource Allocation
```bash
# Increase Docker Desktop resources (macOS/Windows)
# Memory: 8GB minimum, 16GB recommended
# CPU: 4 cores minimum, 8 cores recommended
# Disk: 50GB minimum

# For Linux, ensure sufficient system resources
free -h
df -h
```

### 3. Network Optimization
```bash
# Use faster RPC endpoints for production
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"

# Consider using custom RPC providers for better performance
export SOLANA_RPC_URL="https://your-custom-rpc-provider.com"
```

## Emergency Recovery Procedures

### 1. Complete System Reset
```bash
# Stop all services
docker-compose down -v

# Remove all P2P-related containers and volumes
docker container prune -f
docker volume ls | grep p2p | awk '{print $2}' | xargs docker volume rm

# Rebuild everything
docker-compose build --no-cache
docker-compose up -d
```

### 2. Contact Service Recovery
```bash
# Rebuild contact service only
docker-compose build --no-cache contact

# Test contact service health
docker-compose run --rm contact /usr/local/bin/health-monitor.sh detailed

# Verify all scripts are available
docker-compose run --rm contact ls -la /usr/local/bin/
```

### 3. Network Connectivity Recovery
```bash
# Reset Docker networks
docker network prune -f

# Recreate networks
docker-compose down
docker-compose up -d

# Test connectivity
docker-compose run --rm contact bash -c "ping -c 3 8.8.8.8"
```

## Best Practices for Stability

### 1. Environment Configuration
- Always use environment variables for RPC URLs
- Prefer devnet for development and testing
- Use mainnet only for production deployments
- Keep deployment scripts simple and maintainable

### 2. Container Management
- Regularly clean up unused containers and volumes
- Monitor container resource usage
- Use health checks for critical services
- Implement proper logging and monitoring

### 3. Development Workflow
- Test deployments on devnet before local/mainnet
- Use version control for configuration changes
- Document any custom modifications
- Maintain separate environments for dev/test/prod

For additional support, refer to:
- [Docker Deployment Guide](./DOCKER_DEPLOYMENT_GUIDE.md)
- [Docker Quick Reference](./DOCKER_QUICK_REFERENCE.md)
- [Contact Service Analysis](./CONTACT_ANALYSIS.md)
```

### 2. Validator Issues

#### Error: "Validator not responding to RPC calls"
```bash
# Check validator container logs
docker-compose logs solana-validator

# Restart validator
docker-compose restart solana-validator

# Check if port is bound
netstat -tlnp | grep 8899
```

#### Error: "ARM64 emulation issues"
```bash
# Switch to development mode
docker-compose up -d -e START_VALIDATOR=false solana-validator

# Or use host Solana installation
export SOLANA_RPC_URL=http://host.docker.internal:8899
```

#### Error: "Validator not producing blocks"
```bash
# Check ledger directory
docker exec p2p-anchor-dev ls -la /opt/solana/ledger/

# Reset validator with clean ledger
docker-compose down solana-validator
docker volume rm p2p_solana_ledger
docker-compose up -d solana-validator
```

### 3. Contract Deployment Issues

#### Error: "anchor command not found"
```bash
# Check Anchor installation
docker exec p2p-contact anchor --version

# Rebuild container with latest Anchor
docker-compose build --no-cache contact
```

#### Error: "workspace not found"
```bash
# Check volume mounting
docker exec p2p-contact ls -la /opt/deployer/workspace/

# Verify bind mount
docker inspect p2p-contact | jq '.[0].Mounts'
```

#### Error: "deployment timeout"
```bash
# Increase timeout in deployment script
export DEPLOY_TIMEOUT=600

# Manual step-by-step deployment
docker exec -it p2p-contact bash
cd /opt/deployer/workspace
anchor build
anchor deploy --program-name registry
```

### 4. PoA Initialization Issues

#### Error: "governance program not found"
```bash
# Check if programs are deployed
docker exec p2p-contact ls -la /opt/deployer/artifacts/

# Verify governance program on-chain
docker exec p2p-contact cat /opt/deployer/artifacts/governance/program_id.txt
```

#### Error: "Node.js module not found"
```bash
# Check Node.js dependencies
docker exec p2p-contact npm list

# Reinstall dependencies
docker exec p2p-contact npm install
```

#### Error: "insufficient SOL for transaction"
```bash
# Check account balances
docker exec p2p-contact solana balance /opt/deployer/config/poa-keys/university-authority-keypair.json

# Airdrop more SOL
docker exec p2p-contact solana airdrop 1000 <PUBKEY> --url http://solana-validator:8899
```

## Recovery Procedures

### Full System Reset
```bash
# Stop all services
docker-compose down

# Remove all containers and volumes
docker-compose down -v
docker system prune -a -f

# Rebuild and restart
./scripts/deploy-smart-contracts-docker.sh deploy
```

### Partial Recovery

#### Reset Validator Only
```bash
# Stop and remove validator
docker-compose down solana-validator
docker volume rm p2p_solana_ledger

# Restart validator
docker-compose up -d solana-validator
```

#### Reset Contract Deployment
```bash
# Remove deployment artifacts
docker volume rm p2p_contract_artifacts p2p_deployer_logs

# Restart contract deployer
docker-compose up -d contact
```

#### Reset PoA Configuration
```bash
# Remove PoA configuration
docker exec p2p-contact rm -rf /opt/deployer/config/poa-keys/
docker exec p2p-contact rm -f /opt/deployer/artifacts/poa-config.json

# Reinitialize PoA
docker exec p2p-contact /usr/local/bin/setup-poa.sh
```

## Performance Troubleshooting

### Slow Container Startup
```bash
# Check resource constraints
docker stats

# Increase resource limits
# Edit docker-compose.yml:
deploy:
  resources:
    limits:
      memory: 4G
      cpus: '2'
```

### High Memory Usage
```bash
# Check memory usage per container
docker stats --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}"

# Adjust memory limits
# In docker-compose.yml:
mem_limit: 2g
```

### Slow Network Performance
```bash
# Check network latency
docker exec p2p-contact ping -c 4 solana-validator

# Use host networking for testing
docker run --network host <image>
```

## Environment-Specific Issues

### Development Environment

#### Hot Reload Not Working
```bash
# Check volume mounting
docker exec p2p-contact ls -la /opt/deployer/workspace/programs/

# Verify cached mounting
volumes:
  - .:/opt/deployer/workspace:cached
```

#### Port Conflicts
```bash
# Check which process is using port
lsof -i :8899
netstat -tlnp | grep 8899

# Change port mapping
ports:
  - "8898:8899"
```

### Production Environment

#### SSL/TLS Issues
```bash
# Check certificate validity
openssl x509 -in /path/to/cert.pem -text -noout

# Verify Nginx configuration
docker exec p2p-nginx nginx -t
```

#### Load Balancing Issues
```bash
# Check upstream health
docker exec p2p-nginx curl -f http://api-gateway:3030/health

# Verify load balancer configuration
docker exec p2p-nginx cat /etc/nginx/nginx.conf
```

## Log Analysis

### Key Log Locations
```bash
# Deployment logs
docker exec p2p-contact tail -f /opt/deployer/logs/deploy.log

# Build logs
docker exec p2p-contact tail -f /opt/deployer/logs/build.log

# PoA setup logs
docker exec p2p-contact tail -f /opt/deployer/logs/poa-setup.log

# Health monitoring logs
docker exec p2p-contact tail -f /opt/deployer/logs/health-monitor.log

# Validator logs
docker-compose logs -f solana-validator
```

### Log Analysis Commands
```bash
# Search for errors
docker exec p2p-contact grep -i error /opt/deployer/logs/*.log

# Check for warnings
docker exec p2p-contact grep -i warning /opt/deployer/logs/*.log

# Monitor real-time logs
docker exec p2p-contact tail -f /opt/deployer/logs/*.log
```

## Emergency Procedures

### Complete System Failure
1. **Stop all services**: `docker-compose down`
2. **Backup data volumes**: `docker run --rm -v p2p_postgres_data:/data -v $(pwd):/backup alpine tar czf /backup/postgres_backup.tar.gz /data`
3. **Clean system**: `docker system prune -a -f`
4. **Restore from backup**: Deploy fresh system and restore data
5. **Verify functionality**: Run health checks

### Data Recovery
```bash
# Backup critical volumes
docker run --rm -v p2p_postgres_data:/data -v $(pwd):/backup alpine tar czf /backup/postgres_backup.tar.gz /data
docker run --rm -v p2p_contract_artifacts:/data -v $(pwd):/backup alpine tar czf /backup/artifacts_backup.tar.gz /data

# Restore volumes
docker volume create p2p_postgres_data
docker run --rm -v p2p_postgres_data:/data -v $(pwd):/backup alpine tar xzf /backup/postgres_backup.tar.gz -C /
```

## Support and Escalation

### Self-Service Checklist
- [ ] Checked container logs
- [ ] Verified resource availability
- [ ] Confirmed network connectivity
- [ ] Reviewed configuration files
- [ ] Attempted restart procedures
- [ ] Consulted documentation

### When to Escalate
- Data corruption detected
- Security breach suspected
- Critical system components failing
- Multiple recovery attempts failed
- Production system down > 1 hour

### Information to Collect
```bash
# System information
docker version
docker-compose version
uname -a
free -h
df -h

# Container status
docker ps -a
docker-compose ps

# Recent logs
docker-compose logs --since 1h

# Health status
docker exec p2p-contact /usr/local/bin/health-monitor.sh detailed

# Configuration
docker-compose config
```

---
**Remember**: Always backup critical data before attempting recovery procedures in production environments.