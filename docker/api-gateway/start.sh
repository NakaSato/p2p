#!/bin/bash

# Quick start script for Rust API Gateway
set -e

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

MODE=${1:-development}

print_status "Starting Rust API Gateway in $MODE mode..."

# Create network if it doesn't exist
if ! docker network ls | grep -q "p2p-network"; then
    print_status "Creating p2p-network..."
    docker network create p2p-network
fi

# Copy environment file if it doesn't exist
if [ ! -f ../../api-gateway/.env ]; then
    print_warning ".env file not found, copying from .env.example"
    cp ../../api-gateway/.env.example ../../api-gateway/.env
fi

case $MODE in
    "development"|"dev")
        print_status "Starting in development mode with hot reload..."
        docker compose --profile dev up --build
        ;;
        
    "production"|"prod")
        print_status "Starting in production mode..."
        docker compose --profile prod up --build
        ;;
        
    *)
        print_status "Usage: $0 [development|production]"
        print_status "Default: development"
        exit 1
        ;;
esac
