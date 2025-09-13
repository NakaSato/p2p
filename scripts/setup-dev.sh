#!/bin/bash

# P2P Energy Trading Platform - Local Development Setup Script

set -e

echo "Setting up P2P Energy Trading Platform for local development..."

# Check if Docker and Docker Compose are installed
if ! command -v docker &> /dev/null; then
  echo "Docker is not installed. Please install Docker first."
  exit 1
fi

echo "Docker is available"

# Create necessary directories
echo "Creating directories..."
mkdir -p docker/grafana/dashboards
mkdir -p docker/grafana/datasources  
mkdir -p logs

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "Environment file not found, but should exist from user setup"
else
    echo "Environment file exists, using existing configuration"
fi

echo "Starting Docker containers..."
docker-compose up -d

echo "P2P Energy Trading Platform setup complete!"

