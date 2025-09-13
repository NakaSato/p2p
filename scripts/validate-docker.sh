#!/bin/bash

# Docker Validation Script for P2P Energy Trading Platform
# Tests the new Solana-based infrastructure

set -e

echo "🔍 P2P Energy Trading Platform - Docker Validation"
echo "=================================================="

# Check if Docker is running
if ! docker info >/dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker Desktop."
    exit 1
fi

echo "✅ Docker is running"

# Build and test core services
echo ""
echo "🏗️  Building core services..."

# Build Solana validator
echo "Building Solana validator..."
docker-compose build solana-validator

# Build API Gateway
echo "Building API Gateway..."
docker-compose build api-gateway

echo ""
echo "🚀 Starting essential services..."

# Start infrastructure services first
docker-compose up -d postgres redis kafka zookeeper

# Wait for databases to be ready
echo "⏳ Waiting for databases to initialize..."
sleep 10

# Start Solana validator
docker-compose up -d solana-validator

# Wait for validator to start
echo "⏳ Waiting for Solana validator to start..."
sleep 15

# Check if Solana validator is responding
echo "🔍 Testing Solana validator connectivity..."
if curl -s -X POST -H "Content-Type: application/json" \
   -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' \
   http://localhost:8899 | grep -q "ok"; then
    echo "✅ Solana validator is responding"
else
    echo "⚠️  Solana validator may still be starting up"
fi

# Start API Gateway
docker-compose up -d api-gateway

# Wait for API Gateway to start
echo "⏳ Waiting for API Gateway to start..."
sleep 5

# Check if API Gateway is responding
echo "🔍 Testing API Gateway connectivity..."
if curl -s http://localhost:3001/health >/dev/null 2>&1; then
    echo "✅ API Gateway is responding"
else
    echo "⚠️  API Gateway may still be starting up"
fi

echo ""
echo "📊 Service Status:"
echo "=================="
docker-compose ps

echo ""
echo "🌐 Available Endpoints:"
echo "======================="
echo "• Solana RPC:     http://localhost:8899"
echo "• API Gateway:    http://localhost:3001"
echo "• Grafana:        http://localhost:3000 (admin/admin)"
echo "• Prometheus:     http://localhost:9090"
echo "• PostgreSQL:     localhost:5432"
echo "• Redis:          localhost:6379"

echo ""
echo "🎯 Next Steps:"
echo "=============="
echo "1. Deploy Anchor programs: ./scripts/deploy-programs.sh"
echo "2. Start frontend: docker-compose --profile dev up frontend-dev"
echo "3. Run integration tests: ./scripts/test-integration.sh"
echo "4. View logs: docker-compose logs -f [service-name]"

echo ""
echo "🧹 To stop all services: docker-compose down"
echo "🗑️  To clean up: docker-compose down -v"