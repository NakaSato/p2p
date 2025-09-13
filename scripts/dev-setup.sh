#!/bin/bash

# Master Development Setup Script for P2P Energy Trading Platform
# This script orchestrates the complete development environment setup

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${PURPLE}🚀 P2P Energy Trading Platform - Complete Setup${NC}"
echo "=================================================="
echo ""

# Check prerequisites
echo -e "${BLUE}🔍 Checking Prerequisites${NC}"

# Check Docker
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed. Please install Docker Desktop first.${NC}"
    exit 1
fi

if ! docker info >/dev/null 2>&1; then
    echo -e "${RED}❌ Docker is not running. Please start Docker Desktop.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Docker is available and running${NC}"

# Check if we're in the right directory
if [ ! -f "Anchor.toml" ]; then
    echo -e "${RED}❌ Anchor.toml not found. Please run this script from the project root.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Running from project root${NC}"

# Step 1: Initial environment setup
echo ""
echo -e "${YELLOW}📋 Step 1: Setting up development environment${NC}"
./scripts/setup-dev.sh

# Step 2: Validate Docker setup
echo ""
echo -e "${YELLOW}🐳 Step 2: Validating Docker infrastructure${NC}"
./scripts/validate-docker.sh

# Step 3: Deploy Anchor programs
echo ""
echo -e "${YELLOW}⚓ Step 3: Deploying Anchor programs${NC}"
read -p "Do you want to deploy Anchor programs now? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Check if Anchor and Solana CLI are installed
    if command -v anchor &> /dev/null && command -v solana &> /dev/null; then
        ./scripts/deploy-programs.sh
    else
        echo -e "${YELLOW}⚠️  Anchor CLI or Solana CLI not found. Skipping program deployment.${NC}"
        echo "Please install them and run: ./scripts/deploy-programs.sh"
    fi
else
    echo -e "${YELLOW}⏭️  Skipping program deployment. Run ./scripts/deploy-programs.sh when ready.${NC}"
fi

# Step 4: Run integration tests
echo ""
echo -e "${YELLOW}🧪 Step 4: Running integration tests${NC}"
read -p "Do you want to run integration tests now? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    ./scripts/test-integration.sh
else
    echo -e "${YELLOW}⏭️  Skipping integration tests. Run ./scripts/test-integration.sh when ready.${NC}"
fi

# Step 5: Frontend setup (optional)
echo ""
echo -e "${YELLOW}🌐 Step 5: Frontend development server${NC}"
read -p "Do you want to start the frontend development server? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}🚀 Starting frontend development server...${NC}"
    docker-compose --profile dev up -d frontend-dev
    echo -e "${GREEN}✅ Frontend available at: http://localhost:5173${NC}"
else
    echo -e "${YELLOW}⏭️  Skipping frontend server. Start with: docker-compose --profile dev up frontend-dev${NC}"
fi

# Final summary
echo ""
echo -e "${GREEN}🎉 Setup Complete!${NC}"
echo "=================="
echo ""
echo -e "${BLUE}📋 Available Services:${NC}"
echo "• Solana Validator: http://localhost:8899"
echo "• Grafana: http://localhost:3000 (admin/admin)"
echo "• Prometheus: http://localhost:9090"
echo "• PostgreSQL: localhost:5432"
echo "• Redis: localhost:6379"
if docker-compose ps frontend-dev | grep -q 'Up'; then
    echo "• Frontend (Dev): http://localhost:5173"
fi

echo ""
echo -e "${BLUE}🛠️  Development Commands:${NC}"
echo "• View all services: docker-compose ps"
echo "• View logs: docker-compose logs -f [service-name]"
echo "• Restart service: docker-compose restart [service-name]"
echo "• Deploy programs: ./scripts/deploy-programs.sh"
echo "• Run tests: ./scripts/test-integration.sh"
echo "• Stop all: docker-compose down"

echo ""
echo -e "${BLUE}📚 Next Development Steps:${NC}"
echo "1. 🔧 Implement program logic in programs/"
echo "2. 🌐 Develop frontend components in frontend/src/"
echo "3. 🔌 Create API Gateway in api-gateway/"
echo "4. 📊 Configure monitoring dashboards"
echo "5. 🧪 Write comprehensive tests"

echo ""
echo -e "${GREEN}Happy coding! 🚀${NC}"