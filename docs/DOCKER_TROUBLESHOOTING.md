# Docker Smart Contract Deployment - Troubleshooting Guide

## Troubleshooting Decision Tree

```
Deployment Issue?
├── Container won't start?
│   ├── Check Docker resources → Increase memory/CPU
│   ├── Port conflicts? → Change port mappings
│   └── Image build fails? → Check Dockerfile & dependencies
├── Validator not responding?
│   ├── ARM64 Mac issues? → Use development mode
│   ├── Network issues? → Check Docker networking
│   └── Health check fails? → Restart validator container
├── Contract deployment fails?
│   ├── Build errors? → Check Anchor version & dependencies
│   ├── Deploy timeout? → Increase deployment timeout
│   └── Permission issues? → Check volume mounting
└── PoA initialization fails?
    ├── Missing governance program? → Deploy programs first
    ├── Node.js issues? → Check package dependencies
    └── Keypair issues? → Regenerate PoA keys
```

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
docker ps -a

# Check container resource usage
docker stats

# Inspect container configuration
docker inspect p2p-anchor-dev
docker inspect p2p-contact

# Check container health
docker inspect p2p-anchor-dev --format='{{.State.Health}}'
```

### Network Diagnostics
```bash
# List Docker networks
docker network ls

# Inspect network configuration
docker network inspect p2p_p2p-network

# Test network connectivity
docker exec p2p-contact ping solana-validator
docker exec p2p-contact curl -s http://solana-validator:8899
```

### Volume Diagnostics
```bash
# List volumes
docker volume ls

# Inspect volume configuration
docker volume inspect p2p_solana_ledger
docker volume inspect p2p_contract_artifacts

# Check volume contents
docker run --rm -v p2p_solana_ledger:/data alpine ls -la /data
```

## Common Error Scenarios

### 1. Container Build Failures

#### Error: "no space left on device"
```bash
# Clean up Docker system
docker system prune -a -f
docker volume prune -f

# Check available space
df -h
```

#### Error: "failed to fetch package"
```bash
# Check internet connectivity
ping 8.8.8.8

# Rebuild with no cache
docker-compose build --no-cache solana-validator
```

#### Error: "permission denied"
```bash
# Check Docker daemon status
sudo systemctl status docker

# Fix permissions (Linux)
sudo usermod -aG docker $USER
newgrp docker
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