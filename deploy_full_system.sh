#!/bin/bash

# P2P Energy Trading Platform - Complete Deployment Script
# This script deploys all smart contracts and starts the full system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🏗️  P2P Energy Trading Platform - Complete Deployment${NC}"
echo -e "${BLUE}=================================================${NC}"

# Function to check if a service is healthy
check_service_health() {
    local service_name=$1
    local max_attempts=30
    local attempt=1
    
    echo -e "${YELLOW}Checking health of $service_name...${NC}"
    
    while [ $attempt -le $max_attempts ]; do
        if docker-compose ps $service_name | grep -q "healthy\|Up"; then
            echo -e "${GREEN}✅ $service_name is healthy${NC}"
            return 0
        fi
        
        echo "Attempt $attempt/$max_attempts - waiting for $service_name to be healthy..."
        sleep 10
        attempt=$((attempt + 1))
    done
    
    echo -e "${RED}❌ $service_name failed to become healthy${NC}"
    return 1
}

# Function to wait for contract deployment
wait_for_contract_deployment() {
    echo -e "${YELLOW}⏳ Waiting for smart contracts to be deployed...${NC}"
    
    local max_attempts=60
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if docker-compose exec -T substrate-node test -f /tmp/contract_addresses/deployment_summary.json 2>/dev/null; then
            echo -e "${GREEN}✅ Smart contracts are deployed!${NC}"
            
            # Display contract addresses
            echo -e "${BLUE}📋 Contract Deployment Summary:${NC}"
            docker-compose exec -T substrate-node cat /tmp/contract_addresses/deployment_summary.json | jq .
            
            return 0
        fi
        
        echo "Attempt $attempt/$max_attempts - waiting for contract deployment..."
        sleep 10
        attempt=$((attempt + 1))
    done
    
    echo -e "${RED}❌ Contract deployment failed or timed out${NC}"
    return 1
}

# Function to display system status
show_system_status() {
    echo -e "${BLUE}📊 System Status${NC}"
    echo -e "${BLUE}===============${NC}"
    
    echo -e "${YELLOW}Docker Containers:${NC}"
    docker-compose ps
    
    echo -e "\n${YELLOW}Service Endpoints:${NC}"
    echo "🌐 API Gateway: http://localhost:8080"
    echo "📊 Grafana: http://localhost:3000 (admin/admin)"
    echo "📈 Prometheus: http://localhost:9090"
    echo "🔗 Substrate Node: ws://localhost:9944"
    echo "🗄️  PostgreSQL: localhost:5432"
    echo "📊 TimescaleDB: localhost:5433"
    echo "🔄 Redis: localhost:6379"
    echo "📡 Kafka: localhost:9092"
    
    echo -e "\n${YELLOW}Testing API Gateway:${NC}"
    if curl -s http://localhost:8080/health | jq . 2>/dev/null; then
        echo -e "${GREEN}✅ API Gateway is responding${NC}"
    else
        echo -e "${RED}❌ API Gateway is not responding${NC}"
    fi
}

# Step 1: Stop any existing services
echo -e "${YELLOW}🛑 Stopping existing services...${NC}"
docker-compose down -v || true

# Step 2: Build and start infrastructure services first
echo -e "${YELLOW}🚀 Starting infrastructure services...${NC}"
docker-compose up -d postgres timescaledb redis zookeeper kafka

# Step 3: Wait for infrastructure to be ready
echo -e "${YELLOW}⏳ Waiting for infrastructure services...${NC}"
check_service_health "postgres"
check_service_health "redis" 
check_service_health "kafka"

# Step 4: Start monitoring services
echo -e "${YELLOW}📊 Starting monitoring services...${NC}"
docker-compose up -d grafana prometheus

# Step 5: Build and start smart meter simulator
echo -e "${YELLOW}🔋 Starting smart meter simulator...${NC}"
docker-compose up -d smart-meter-simulator

# Step 6: Build and start the substrate node with contracts
echo -e "${YELLOW}⛓️  Starting Substrate node and deploying contracts...${NC}"
docker-compose up -d substrate-node

# Step 7: Wait for contract deployment
wait_for_contract_deployment

# Step 8: Start oracle simulator (depends on contracts)
echo -e "${YELLOW}🔮 Starting oracle simulator...${NC}"
docker-compose up -d oracle-simulator

# Step 9: Start API Gateway (depends on contracts)
echo -e "${YELLOW}🌐 Starting API Gateway...${NC}"
docker-compose up -d api-gateway

# Step 10: Final health checks
echo -e "${YELLOW}🏥 Performing final health checks...${NC}"
sleep 30

check_service_health "api-gateway"
check_service_health "oracle-simulator"

# Step 11: Display system status
show_system_status

echo -e "\n${GREEN}🎉 Deployment completed successfully!${NC}"
echo -e "${GREEN}🌟 P2P Energy Trading Platform is now running!${NC}"

echo -e "\n${BLUE}📚 Quick Start:${NC}"
echo "1. 🌐 Access the API Gateway at: http://localhost:8080"
echo "2. 📊 View metrics in Grafana at: http://localhost:3000"
echo "3. 🔋 Smart meter simulator is generating energy data every 30 seconds"
echo "4. 🔮 Oracle is performing market clearing every 60 seconds"
echo "5. 💾 All data is persisted in PostgreSQL and TimescaleDB"

echo -e "\n${YELLOW}📖 Useful Commands:${NC}"
echo "• View logs: docker-compose logs -f [service-name]"
echo "• Stop system: docker-compose down"
echo "• View contract addresses: docker-compose exec substrate-node cat /tmp/contract_addresses/deployment_summary.json"
echo "• Execute market clearing: docker-compose exec oracle-simulator ./interact_contracts.sh clear-market"

echo -e "\n${GREEN}✨ Happy trading! ✨${NC}"
