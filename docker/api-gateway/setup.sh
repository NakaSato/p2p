#!/bin/bash

# Quick setup script for the Rust API Gateway Docker environment
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

print_status "Setting up Rust API Gateway Docker environment..."

# Check if we're in the right directory
if [ ! -f "../../api-gateway/Cargo.toml" ]; then
    print_warning "This script should be run from docker/api-gateway/ directory"
    print_status "Current directory: $(pwd)"
    print_status "Looking for: ../../api-gateway/Cargo.toml"
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f "../../api-gateway/.env" ]; then
    print_status "Creating .env file from example..."
    cp ../../api-gateway/.env.example ../../api-gateway/.env
    print_success ".env file created"
else
    print_status ".env file already exists"
fi

# Create Docker network
if ! docker network ls | grep -q "p2p-network"; then
    print_status "Creating p2p-network..."
    docker network create p2p-network
    print_success "Network created"
else
    print_status "Network p2p-network already exists"
fi

print_success "Setup complete!"
print_status ""
print_status "Available commands:"
print_status "  ./start.sh development   - Start development environment with hot reload"
print_status "  ./start.sh production    - Start production environment"
print_status "  ./build.sh --mode dev    - Build development image"
print_status "  ./build.sh --mode prod   - Build production image"
print_status ""
print_status "Docker compose commands:"
print_status "  docker compose --profile dev up   - Development mode"
print_status "  docker compose --profile prod up  - Production mode"
