#!/bin/bash

# P2P Energy Trading Platform - Local Development Setup Script

set -e

echo "Setting up P2P Energy Trading Platform for local development..."

# Check if Docker and Docker Compose are installed
if ! command -v docker &> /dev/null; then
  echo "Docker is not installed. Please install Docker first."
  exit 1
fi

if ! command -v docker-compose &> /dev/null; then
  echo "Docker Compose is not installed. Please install Docker Compose first."
  exit 1
fi

# Create necessary directories
echo "Creating directories..."
mkdir -p docker/grafana/dashboards
mkdir -p docker/grafana/datasources
mkdir -p logs

# Set environment variables for development
echo "Setting up environment variables..."
cat > .env << 'EOF'
# Blockchain Configuration
SUBSTRATE_WS_URL=ws://localhost:9944
SUBSTRATE_HTTP_URL=http://localhost:9933

# Database Configuration
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DB=p2p_energy_trading
POSTGRES_USER=p2p_user
POSTGRES_PASSWORD=p2p_password
DATABASE_URL=postgresql://p2p_user:p2p_password@localhost:5432/p2p_energy_trading

# TimescaleDB Configuration
TIMESCALE_HOST=localhost
TIMESCALE_PORT=5433
TIMESCALE_DB=p2p_timeseries
TIMESCALE_USER=timescale_user
TIMESCALE_PASSWORD=timescale_password
TIMESCALE_URL=postgresql://timescale_user:timescale_password@localhost:5433/p2p_timeseries

# Redis Configuration
REDIS_HOST=localhost
REDIS_PORT=6379
REDIS_PASSWORD=redis_password_change_in_production
REDIS_URL=redis://:redis_password_change_in_production@localhost:6379

# Kafka Configuration
KAFKA_BOOTSTRAP_SERVERS=localhost:9092

# API Configuration
API_PORT=8080
JWT_SECRET=dev-secret-key-change-in-production

# Smart Contract Addresses (will be updated after deployment)
REGISTRY_CONTRACT_ADDRESS=""
GRID_TOKEN_CONTRACT_ADDRESS=""
TRADING_CONTRACT_ADDRESS=""
ORACLE_CLIENT_CONTRACT_ADDRESS=""

# Oracle Configuration
ORACLE_ACCOUNT_SEED="//Alice"
PROCESSING_INTERVAL=60

# Simulation Configuration
SIMULATION_INTERVAL=30
NUM_METERS=10

# Monitoring
GRAFANA_ADMIN_PASSWORD=admin
EOF

echo "Environment file created"

# Create Grafana datasources configuration
echo "Setting up Grafana datasources..."
cat > docker/grafana/datasources/datasources.yml << 'EOF'
apiVersion: 1

datasources:
  - name: PostgreSQL
  type: postgres
  url: postgres:5432
  database: p2p_energy_trading
  user: p2p_user
  secureJsonData:
    password: p2p_password
  jsonData:
    sslmode: disable
    maxOpenConns: 0
    maxIdleConns: 2
    connMaxLifetime: 14400

  - name: TimescaleDB
  type: postgres
  url: timescaledb:5432
  database: p2p_timeseries
  user: timescale_user
  secureJsonData:
    password: timescale_password
  jsonData:
    sslmode: disable
    timescaledb: true

  - name: Prometheus
  type: prometheus
  url: http://prometheus:9090
  access: proxy
  isDefault: true
EOF

# Create basic dashboard configuration
cat > docker/grafana/dashboards/dashboard.yml << 'EOF'
apiVersion: 1

providers:
  - name: 'default'
  orgId: 1
  folder: ''
  type: file
  disableDeletion: false
  updateIntervalSeconds: 10
  allowUiUpdates: true
  options:
    path: /etc/grafana/provisioning/dashboards
EOF

echo "Starting Docker containers..."

# Pull images first to show progress
docker-compose pull

# Start the services
docker-compose up -d

echo "Waiting for services to start..."
sleep 30

# Check service health
echo "Checking service health..."

services=("substrate-node:9944" "postgres:5432" "timescaledb:5432" "redis:6379" "kafka:9092")

for service in "${services[@]}"; do
  IFS=':' read -r host port <<< "$service"
  if docker-compose exec -T $host sh -c "nc -z localhost $port" 2>/dev/null; then
    echo "$host is running on port $port"
  else
    echo "$host might not be ready yet on port $port"
  fi
done

echo ""
echo "P2P Energy Trading Platform setup complete!"
echo ""
echo "Service URLs:"
echo "  Substrate Node (WebSocket): ws://localhost:9944"
echo "  Substrate Node (HTTP): http://localhost:9933"
echo "  PostgreSQL: localhost:5432"
echo "  TimescaleDB: localhost:5433"
echo "  Redis: localhost:6379"
echo "  Kafka: localhost:9092"
echo "  API Gateway: http://localhost:8080"
echo "  Grafana: http://localhost:3000 (admin/admin)"
echo "  Prometheus: http://localhost:9090"
echo ""
echo "Next steps:"
echo "  1. Deploy smart contracts to the local Substrate node"
echo "  2. Update contract addresses in .env file"
echo "  3. Access Grafana dashboard for monitoring"
echo "  4. Check API health at http://localhost:8080/health"
echo ""
echo "Useful commands:"
echo "  View logs: docker-compose logs -f [service-name]"
echo "  Restart services: docker-compose restart"
echo "  Stop all services: docker-compose down"
echo "  Clean up: docker-compose down -v --remove-orphans"
echo ""
echo "Happy coding!"
