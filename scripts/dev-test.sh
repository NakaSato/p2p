#!/bin/bash

# P2P Energy Trading System - Development & Testing Script
# Comprehensive development environment setup and testing suite
# Version: 6.0 (September 2025)

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LOG_FILE="$PROJECT_ROOT/dev-test.log"

# Functions
log() {
    echo -e "${GREEN}[$(date +'%Y-%m-%d %H:%M:%S')] $1${NC}"
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1" >> "$LOG_FILE"
}

warn() {
    echo -e "${YELLOW}[WARNING] $1${NC}"
    echo "[WARNING] $1" >> "$LOG_FILE"
}

error() {
    echo -e "${RED}[ERROR] $1${NC}"
    echo "[ERROR] $1" >> "$LOG_FILE"
    exit 1
}

info() {
    echo -e "${BLUE}[INFO] $1${NC}"
    echo "[INFO] $1" >> "$LOG_FILE"
}

# Check prerequisites
check_prerequisites() {
    log "Checking development prerequisites..."
    
    # Check Rust and Cargo
    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo not found. Install from: https://rustup.rs/"
    fi
    
    # Check Solana CLI
    if ! command -v solana &> /dev/null; then
        error "Solana CLI not found. Install from: https://docs.solana.com/cli/install-solana-cli-tools"
    fi
    
    # Check Anchor CLI
    if ! command -v anchor &> /dev/null; then
        error "Anchor CLI not found. Install from: https://www.anchor-lang.com/docs/installation"
    fi
    
    # Check Node.js and Yarn
    if ! command -v node &> /dev/null; then
        warn "Node.js not found. Some frontend tests may fail."
    fi
    
    if ! command -v yarn &> /dev/null; then
        warn "Yarn not found. Using npm for frontend operations."
    fi
    
    # Check Docker (optional for local development)
    if ! command -v docker &> /dev/null; then
        warn "Docker not found. Container tests will be skipped."
    fi
    
    log "Prerequisites check completed"
}

# Setup development environment
setup_dev_environment() {
    log "Setting up development environment..."
    
    cd "$PROJECT_ROOT"
    
    # Configure Solana for local development
    log "Configuring Solana for local development..."
    solana config set --url localhost
    solana config set --keypair ~/.config/solana/id.json
    
    # Create keypair if it doesn't exist
    if [ ! -f ~/.config/solana/id.json ]; then
        log "Creating new Solana keypair..."
        solana-keygen new --no-bip39-passphrase --silent
    fi
    
    # Install frontend dependencies if Node.js is available
    if command -v node &> /dev/null; then
        log "Installing frontend dependencies..."
        cd "$PROJECT_ROOT/frontend"
        if command -v yarn &> /dev/null; then
            yarn install
        else
            npm install
        fi
        cd "$PROJECT_ROOT"
    fi
    
    log "Development environment setup completed"
}

# Build all programs
build_programs() {
    log "Building all Anchor programs..."
    
    cd "$PROJECT_ROOT"
    
    # Clean previous builds
    log "Cleaning previous builds..."
    anchor clean
    
    # Build all programs
    log "Building programs..."
    anchor build
    
    # Verify build artifacts
    if [ -d "target/deploy" ]; then
        log "Build artifacts created successfully:"
        ls -la target/deploy/*.so | while read -r line; do
            info "  $line"
        done
    else
        error "Build failed - no deployment artifacts found"
    fi
    
    log "Program build completed successfully"
}

# Start local Solana validator
start_local_validator() {
    log "Starting local Solana validator..."
    
    # Kill any existing validator processes
    pkill -f solana-test-validator || true
    sleep 2
    
    # Clean ledger data
    if [ -d "test-ledger" ]; then
        log "Cleaning previous ledger data..."
        rm -rf test-ledger
    fi
    
    # Create test-ledger directory for validator logs
    mkdir -p test-ledger
    
    # Start validator in background
    log "Launching Solana test validator..."
    solana-test-validator \
        --reset \
        --ledger test-ledger \
        --log > test-ledger/validator.log 2>&1 &
    
    VALIDATOR_PID=$!
    
    # Wait for validator to be ready
    log "Waiting for validator to be ready..."
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if solana cluster-version &> /dev/null; then
            log "Validator is ready (attempt $attempt)"
            break
        fi
        
        if [ $attempt -eq $max_attempts ]; then
            error "Validator failed to start after $max_attempts attempts. Check logs:"
            if [ -f "test-ledger/validator.log" ]; then
                echo "=== Validator Log ==="
                cat test-ledger/validator.log
                echo "===================="
            fi
            exit 1
        fi
        
        sleep 2
        ((attempt++))
    done
    
    # Airdrop SOL for testing
    log "Airdropping SOL for testing..."
    solana airdrop 100 --url localhost
    
    info "Local validator started successfully (PID: $VALIDATOR_PID)"
    echo "$VALIDATOR_PID" > "$PROJECT_ROOT/.validator_pid"
}

# Deploy programs to local network
deploy_programs() {
    log "Deploying programs to local network..."
    
    cd "$PROJECT_ROOT"
    
    # Deploy all programs
    log "Deploying Anchor programs..."
    anchor deploy --provider.cluster localnet
    
    # Verify deployments
    log "Verifying program deployments..."
    local programs_deployed=0
    
    if [ -f "target/deploy/registry-keypair.json" ]; then
        local registry_id=$(solana-keygen pubkey target/deploy/registry-keypair.json)
        if solana program show "$registry_id" --url localhost &> /dev/null; then
            info "  ✓ Registry Program: $registry_id"
            ((programs_deployed++))
        fi
    fi
    
    if [ -f "target/deploy/energy_token-keypair.json" ]; then
        local energy_token_id=$(solana-keygen pubkey target/deploy/energy_token-keypair.json)
        if solana program show "$energy_token_id" --url localhost &> /dev/null; then
            info "  ✓ Energy Token Program: $energy_token_id"
            ((programs_deployed++))
        fi
    fi
    
    if [ -f "target/deploy/trading-keypair.json" ]; then
        local trading_id=$(solana-keygen pubkey target/deploy/trading-keypair.json)
        if solana program show "$trading_id" --url localhost &> /dev/null; then
            info "  ✓ Trading Program: $trading_id"
            ((programs_deployed++))
        fi
    fi
    
    if [ -f "target/deploy/oracle-keypair.json" ]; then
        local oracle_id=$(solana-keygen pubkey target/deploy/oracle-keypair.json)
        if solana program show "$oracle_id" --url localhost &> /dev/null; then
            info "  ✓ Oracle Program: $oracle_id"
            ((programs_deployed++))
        fi
    fi
    
    if [ -f "target/deploy/governance-keypair.json" ]; then
        local governance_id=$(solana-keygen pubkey target/deploy/governance-keypair.json)
        if solana program show "$governance_id" --url localhost &> /dev/null; then
            info "  ✓ Governance Program: $governance_id"
            ((programs_deployed++))
        fi
    fi
    
    log "Successfully deployed $programs_deployed programs"
}

# Run comprehensive tests
run_tests() {
    log "Running comprehensive test suite..."
    
    cd "$PROJECT_ROOT"
    
    # Run Anchor tests
    log "Running Anchor integration tests..."
    if ! anchor test --skip-local-validator; then
        error "Anchor tests failed"
    fi
    
    # Run frontend tests if available
    if [ -f "frontend/package.json" ] && command -v node &> /dev/null; then
        log "Running frontend tests..."
        cd "$PROJECT_ROOT/frontend"
        
        if command -v yarn &> /dev/null; then
            yarn test --run
        else
            npm test
        fi
        
        cd "$PROJECT_ROOT"
    fi
    
    # Run performance tests if available
    if [ -f "tests/performance.test.ts" ]; then
        log "Running performance tests..."
        if command -v yarn &> /dev/null; then
            yarn test:performance
        else
            npm run test:performance
        fi
    fi
    
    log "All tests completed successfully"
}

# Start development services
start_dev_services() {
    log "Starting development services..."
    
    # API Gateway (if available)
    if [ -d "api-gateway" ]; then
        log "Starting API Gateway in background..."
        cd "$PROJECT_ROOT/api-gateway"
        cargo run &
        API_GATEWAY_PID=$!
        echo "$API_GATEWAY_PID" > "$PROJECT_ROOT/.api_gateway_pid"
        cd "$PROJECT_ROOT"
        info "API Gateway started (PID: $API_GATEWAY_PID)"
        sleep 3
    fi
    
    # Frontend development server
    if [ -f "frontend/package.json" ] && command -v node &> /dev/null; then
        log "Starting frontend development server..."
        cd "$PROJECT_ROOT/frontend"
        if command -v yarn &> /dev/null; then
            yarn dev &
        else
            npm run dev &
        fi
        FRONTEND_PID=$!
        echo "$FRONTEND_PID" > "$PROJECT_ROOT/.frontend_pid"
        cd "$PROJECT_ROOT"
        info "Frontend server started (PID: $FRONTEND_PID)"
    fi
    
    # Docker services (if available and requested)
    if [ "$START_DOCKER" = "true" ] && command -v docker &> /dev/null; then
        log "Starting Docker services..."
        docker-compose up -d
        info "Docker services started"
    fi
}

# Display service URLs
show_service_urls() {
    log "Development environment is ready!"
    echo ""
    info "Available services:"
    info "  • Solana Validator: http://localhost:8899"
    info "  • Solana WebSocket: ws://localhost:8900"
    
    if [ -f "$PROJECT_ROOT/.api_gateway_pid" ]; then
        info "  • API Gateway: http://localhost:8080"
    fi
    
    if [ -f "$PROJECT_ROOT/.frontend_pid" ]; then
        info "  • Frontend: http://localhost:5173"
    fi
    
    if [ "$START_DOCKER" = "true" ]; then
        info "  • Grafana: http://localhost:3000 (admin/admin)"
        info "  • Prometheus: http://localhost:9090"
    fi
    
    echo ""
    warn "Press Ctrl+C to stop all services"
}

# Cleanup function
cleanup() {
    log "Cleaning up development environment..."
    
    # Stop validator
    if [ -f "$PROJECT_ROOT/.validator_pid" ]; then
        local validator_pid=$(cat "$PROJECT_ROOT/.validator_pid")
        kill "$validator_pid" 2>/dev/null || true
        rm -f "$PROJECT_ROOT/.validator_pid"
    fi
    
    # Stop API Gateway
    if [ -f "$PROJECT_ROOT/.api_gateway_pid" ]; then
        local api_gateway_pid=$(cat "$PROJECT_ROOT/.api_gateway_pid")
        kill "$api_gateway_pid" 2>/dev/null || true
        rm -f "$PROJECT_ROOT/.api_gateway_pid"
    fi
    
    # Stop frontend
    if [ -f "$PROJECT_ROOT/.frontend_pid" ]; then
        local frontend_pid=$(cat "$PROJECT_ROOT/.frontend_pid")
        kill "$frontend_pid" 2>/dev/null || true
        rm -f "$PROJECT_ROOT/.frontend_pid"
    fi
    
    # Stop Docker services
    if [ "$START_DOCKER" = "true" ] && command -v docker &> /dev/null; then
        docker-compose down
    fi
    
    log "Cleanup completed"
}

# Trap cleanup on exit
trap cleanup EXIT INT TERM

# Main execution
main() {
    log "P2P Energy Trading System - Development & Testing"
    log "=================================================="
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --build-only)
                BUILD_ONLY=true
                shift
                ;;
            --test-only)
                TEST_ONLY=true
                shift
                ;;
            --with-docker)
                START_DOCKER=true
                shift
                ;;
            --no-frontend)
                NO_FRONTEND=true
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --build-only     Only build programs, don't start services"
                echo "  --test-only      Only run tests, don't start services"
                echo "  --with-docker    Start Docker services (database, monitoring)"
                echo "  --no-frontend    Skip frontend development server"
                echo "  --help           Show this help message"
                echo ""
                echo "Examples:"
                echo "  $0                    # Full development environment"
                echo "  $0 --build-only      # Build and deploy programs only"
                echo "  $0 --test-only       # Run test suite only"
                echo "  $0 --with-docker     # Include Docker services"
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done
    
    # Execute steps based on options
    check_prerequisites
    setup_dev_environment
    build_programs
    start_local_validator
    deploy_programs
    
    if [ "$TEST_ONLY" = "true" ]; then
        run_tests
        log "Test execution completed"
        exit 0
    fi
    
    if [ "$BUILD_ONLY" = "true" ]; then
        log "Build process completed"
        exit 0
    fi
    
    # Full development environment
    run_tests
    start_dev_services
    show_service_urls
    
    # Wait for interrupt
    while true; do
        sleep 1
    done
}

# Execute main function
main "$@"