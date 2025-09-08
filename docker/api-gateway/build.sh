#!/bin/bash

# Build script for Rust API Gateway Docker containers
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Default values
MODE="development"
PUSH=false
NO_CACHE=false
PLATFORM=""

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--mode)
            MODE="$2"
            shift 2
            ;;
        -p|--push)
            PUSH=true
            shift
            ;;
        --no-cache)
            NO_CACHE=true
            shift
            ;;
        --platform)
            PLATFORM="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -m, --mode MODE      Build mode: development, production (default: development)"
            echo "  -p, --push           Push images to registry after build"
            echo "      --no-cache       Build without cache"
            echo "      --platform PLAT  Target platform (e.g., linux/amd64,linux/arm64)"
            echo "  -h, --help           Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                           # Build for development"
            echo "  $0 -m production             # Build for production"
            echo "  $0 -m production -p          # Build and push production image"
            echo "  $0 --platform linux/amd64   # Build for specific platform"
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            exit 1
            ;;
    esac
done

print_status "Building Rust API Gateway Docker containers..."
print_status "Mode: $MODE"

# Set build arguments
BUILD_ARGS=""
if [ "$NO_CACHE" = true ]; then
    BUILD_ARGS="$BUILD_ARGS --no-cache"
fi

if [ -n "$PLATFORM" ]; then
    BUILD_ARGS="$BUILD_ARGS --platform $PLATFORM"
fi

# Create network if it doesn't exist
if ! docker network ls | grep -q "p2p-network"; then
    print_status "Creating p2p-network..."
    docker network create p2p-network
fi

case $MODE in
    "development"|"dev")
        print_status "Building development image..."
        docker build $BUILD_ARGS --platform linux/amd64 -f Dockerfile.dev -t rust-api-gateway:dev ../..
        
        if [ "$PUSH" = true ]; then
            print_warning "Push not implemented for development images"
        fi
        
        print_success "Development image built successfully!"
        print_status "To run: docker compose --profile dev up"
        ;;
        
    "production"|"prod")
        print_status "Building production image..."
        docker build $BUILD_ARGS --platform linux/amd64 -f Dockerfile -t rust-api-gateway:latest ../..
        docker tag rust-api-gateway:latest rust-api-gateway:prod
        
        if [ "$PUSH" = true ]; then
            print_status "Pushing production images..."
            docker push rust-api-gateway:latest
            docker push rust-api-gateway:prod
            print_success "Images pushed successfully!"
        fi
        
        print_success "Production image built successfully!"
        print_status "To run: docker compose --profile prod up"
        ;;
        
    *)
        print_error "Invalid mode: $MODE. Use 'development' or 'production'"
        exit 1
        ;;
esac

print_status "Available images:"
docker images | grep rust-api-gateway

print_success "Build completed!"
