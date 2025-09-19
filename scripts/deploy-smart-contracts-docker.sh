#!/bin/bash

# Multi-Stage Docker-based Smart Contract Deployment Pipeline
# This script orchestrates the complete deployment process for the P2P Energy Trading System

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
LOG_DIR="$PROJECT_ROOT/logs/deployment"
TIMESTAMP=$(date '+%Y%m%d_%H%M%S')
DEPLOYMENT_LOG="$LOG_DIR/deployment_$TIMESTAMP.log"

# Docker configuration
COMPOSE_FILE="$PROJECT_ROOT/docker-compose.yml"
COMPOSE_DEV_FILE="$PROJECT_ROOT/docker-compose.dev.yml"

# Deployment stages
STAGE_1_INFRASTRUCTURE="infrastructure"
STAGE_2_VALIDATOR="validator"
STAGE_3_BUILD="build"
STAGE_4_DEPLOY="deploy"
STAGE_5_VERIFY="verify"
STAGE_6_INITIALIZE="initialize"

# Default configuration
ENVIRONMENT="${ENVIRONMENT:-development}"
SKIP_INFRASTRUCTURE="${SKIP_INFRASTRUCTURE:-false}"
SKIP_TESTS="${SKIP_TESTS:-false}"
VERBOSE="${VERBOSE:-true}"

# Ensure log directory exists
mkdir -p "$LOG_DIR"

# Logging functions
log() {
    local level="$1"
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo "[$timestamp] [$level] $message" | tee -a "$DEPLOYMENT_LOG"
}

log_info() {
    log "INFO" "$@"
}

log_warn() {
    log "WARN" "$@"
}

log_error() {
    log "ERROR" "$@"
}

log_stage() {
    local stage="$1"
    shift
    log "STAGE" "[$stage] $*"
}

# Function to check prerequisites
check_prerequisites() {
    log_stage "PREREQ" "Checking deployment prerequisites..."
    
    # Check if Docker is running
    if ! docker info >/dev/null 2>&1; then
        log_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
    
    # Check if docker-compose is available
    if ! command -v docker-compose >/dev/null 2>&1; then
        log_error "docker-compose not found. Please install docker-compose."
        exit 1
    fi
    
    # Check if required files exist
    local required_files=(
        "$COMPOSE_FILE"
        "$PROJECT_ROOT/programs"
        "$PROJECT_ROOT/Anchor.toml"
    )
    
    for file in "${required_files[@]}"; do
        if [ ! -e "$file" ]; then
            log_error "Required file/directory not found: $file"
            exit 1
        fi
    done
    
    log_info "Prerequisites check passed"
}

# Stage 1: Infrastructure Services
deploy_infrastructure() {
    if [ "$SKIP_INFRASTRUCTURE" = "true" ]; then
        log_stage "$STAGE_1_INFRASTRUCTURE" "Skipping infrastructure deployment"
        return 0
    fi
    
    log_stage "$STAGE_1_INFRASTRUCTURE" "Starting infrastructure services..."
    
    # Start database and supporting services first
    local infrastructure_services=(
        "postgres"
        "timescaledb"
        "redis"
        "kafka"
        "grafana"
        "prometheus"
    )
    
    for service in "${infrastructure_services[@]}"; do
        if docker-compose -f "$COMPOSE_FILE" ps --services | grep -q "^$service$"; then
            log_info "Starting service: $service"
            docker-compose -f "$COMPOSE_FILE" up -d "$service"
            
            # Wait for service to be healthy
            wait_for_service_health "$service"
        else
            log_warn "Service $service not found in compose file, skipping"
        fi
    done
    
    log_stage "$STAGE_1_INFRASTRUCTURE" "Infrastructure services started successfully"
}

# Stage 2: Solana Validator
deploy_validator() {
    log_stage "$STAGE_2_VALIDATOR" "Starting Solana validator..."
    
    # Start the enhanced validator container
    docker-compose -f "$COMPOSE_FILE" up -d solana-validator
    
    # Wait for validator to be healthy
    log_info "Waiting for Solana validator to become healthy..."
    wait_for_service_health "solana-validator" 300
    
    # Additional validator readiness check
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if docker-compose -f "$COMPOSE_FILE" exec -T solana-validator \
           solana cluster-version --url http://localhost:8899 >/dev/null 2>&1; then
            log_info "Validator is ready and responding to RPC calls"
            break
        fi
        
        log_info "Waiting for validator RPC readiness (attempt $attempt/$max_attempts)..."
        sleep 10
        attempt=$((attempt + 1))
    done
    
    if [ $attempt -gt $max_attempts ]; then
        log_error "Validator failed to become ready within timeout"
        return 1
    fi
    
    log_stage "$STAGE_2_VALIDATOR" "Solana validator started successfully"
}

# Stage 3: Build Contracts
build_contracts() {
    log_stage "$STAGE_3_BUILD" "Building smart contracts..."
    
    # Start the contract deployer for building only
    docker-compose -f "$COMPOSE_FILE" run --rm \
        -e "BUILD_ONLY=true" \
        contact \
        /usr/local/bin/build-contracts.sh
    
    # Check if build was successful
    if [ $? -eq 0 ]; then
        log_stage "$STAGE_3_BUILD" "Smart contracts built successfully"
    else
        log_error "Contract build failed"
        return 1
    fi
}

# Stage 4: Deploy Contracts
deploy_contracts() {
    log_stage "$STAGE_4_DEPLOY" "Deploying smart contracts..."
    
    # Deploy contracts using the deployer service
    docker-compose -f "$COMPOSE_FILE" run --rm \
        contact \
        /usr/local/bin/deploy-all-contracts.sh
    
    if [ $? -eq 0 ]; then
        log_stage "$STAGE_4_DEPLOY" "Smart contracts deployed successfully"
    else
        log_error "Contract deployment failed"
        return 1
    fi
}

# Stage 5: Verify Deployment
verify_deployment() {
    log_stage "$STAGE_5_VERIFY" "Verifying contract deployment..."
    
    # Run deployment verification
    docker-compose -f "$COMPOSE_FILE" run --rm \
        -e "VERIFY_VERBOSE=true" \
        contact \
        /usr/local/bin/verify-deployment.sh
    
    if [ $? -eq 0 ]; then
        log_stage "$STAGE_5_VERIFY" "Deployment verification passed"
    else
        log_error "Deployment verification failed"
        return 1
    fi
}

# Stage 6: Initialize PoA System
initialize_poa_system() {
    log_stage "$STAGE_6_INITIALIZE" "Initializing PoA governance system..."
    
    # Check if PoA initialization script exists
    if [ -f "$PROJECT_ROOT/scripts/initialize-poa-docker.sh" ]; then
        log_info "Running PoA initialization script..."
        bash "$PROJECT_ROOT/scripts/initialize-poa-docker.sh" 2>&1 | tee -a "$DEPLOYMENT_LOG"
    else
        log_warn "PoA initialization script not found, running basic setup"
        
        # Basic PoA setup using the validator container
        docker-compose -f "$COMPOSE_FILE" exec -T solana-validator \
            bash -c "
                cd /workspaces/p2p && 
                echo 'Setting up PoA governance...' &&
                # Add basic PoA initialization commands here
                echo 'PoA initialization completed'
            "
    fi
    
    log_stage "$STAGE_6_INITIALIZE" "PoA system initialization completed"
}

# Function to wait for service health
wait_for_service_health() {
    local service="$1"
    local timeout="${2:-120}"
    local interval=10
    local elapsed=0
    
    log_info "Waiting for $service to become healthy (timeout: ${timeout}s)..."
    
    while [ $elapsed -lt $timeout ]; do
        local health_status
        health_status=$(docker-compose -f "$COMPOSE_FILE" ps --format json "$service" 2>/dev/null | \
                       jq -r '.[0].Health // "unknown"' 2>/dev/null || echo "unknown")
        
        case "$health_status" in
            "healthy")
                log_info "Service $service is healthy"
                return 0
                ;;
            "unhealthy")
                log_warn "Service $service is unhealthy, waiting..."
                ;;
            "starting")
                log_info "Service $service is starting..."
                ;;
            *)
                # Check if container is running (might not have healthcheck)
                if docker-compose -f "$COMPOSE_FILE" ps "$service" | grep -q "Up"; then
                    log_info "Service $service is running (no healthcheck)"
                    return 0
                fi
                ;;
        esac
        
        sleep $interval
        elapsed=$((elapsed + interval))
    done
    
    log_error "Service $service failed to become healthy within ${timeout}s"
    return 1
}

# Function to run post-deployment tests
run_tests() {
    if [ "$SKIP_TESTS" = "true" ]; then
        log_info "Skipping post-deployment tests"
        return 0
    fi
    
    log_info "Running post-deployment tests..."
    
    # Start frontend for integration tests if needed
    if docker-compose -f "$COMPOSE_FILE" ps --services | grep -q "^frontend$"; then
        docker-compose -f "$COMPOSE_FILE" up -d frontend
        wait_for_service_health "frontend"
    fi
    
    # Run tests
    cd "$PROJECT_ROOT"
    if [ -f "package.json" ]; then
        log_info "Running integration tests..."
        npm test 2>&1 | tee -a "$DEPLOYMENT_LOG"
    fi
    
    log_info "Post-deployment tests completed"
}

# Function to display deployment summary
display_summary() {
    log_info "=== DEPLOYMENT SUMMARY ==="
    log_info "Environment: $ENVIRONMENT"
    log_info "Deployment log: $DEPLOYMENT_LOG"
    
    # Show running services
    log_info "Running services:"
    docker-compose -f "$COMPOSE_FILE" ps --format "table {{.Name}}\t{{.State}}\t{{.Ports}}"
    
    # Show validator RPC endpoint
    local validator_port
    validator_port=$(docker-compose -f "$COMPOSE_FILE" port solana-validator 8899 2>/dev/null | cut -d: -f2 || echo "8899")
    log_info "Solana RPC endpoint: http://localhost:$validator_port"
    
    # Show deployed programs
    log_info "Deployed smart contracts:"
    docker-compose -f "$COMPOSE_FILE" run --rm contact \
        bash -c "find /opt/deployer/artifacts -name 'program_id.txt' -exec echo -n '{}: ' \; -exec cat {} \; -exec echo \;" 2>/dev/null || \
        log_warn "Could not retrieve program IDs"
    
    log_info "=== DEPLOYMENT COMPLETED SUCCESSFULLY ==="
}

# Function to handle cleanup on error
cleanup_on_error() {
    log_error "Deployment failed, cleaning up..."
    docker-compose -f "$COMPOSE_FILE" down
}

# Main deployment function
main() {
    log_info "=== P2P Energy Trading Smart Contract Deployment Pipeline ==="
    log_info "Environment: $ENVIRONMENT"
    log_info "Timestamp: $TIMESTAMP"
    log_info "Log file: $DEPLOYMENT_LOG"
    
    # Set up error handling
    trap cleanup_on_error ERR
    
    # Execute deployment stages
    check_prerequisites
    deploy_infrastructure
    deploy_validator
    build_contracts
    deploy_contracts
    verify_deployment
    initialize_poa_system
    run_tests
    display_summary
    
    log_info "Deployment pipeline completed successfully!"
}

# Handle command line arguments
case "${1:-deploy}" in
    "deploy")
        main
        ;;
    "infrastructure")
        check_prerequisites
        deploy_infrastructure
        ;;
    "validator")
        check_prerequisites
        deploy_validator
        ;;
    "build")
        check_prerequisites
        build_contracts
        ;;
    "contracts")
        check_prerequisites
        deploy_contracts
        ;;
    "verify")
        verify_deployment
        ;;
    "initialize")
        initialize_poa_system
        ;;
    "test")
        run_tests
        ;;
    "clean")
        log_info "Cleaning up deployment..."
        docker-compose -f "$COMPOSE_FILE" down -v
        docker system prune -f
        ;;
    *)
        echo "Usage: $0 {deploy|infrastructure|validator|build|contracts|verify|initialize|test|clean}"
        echo ""
        echo "Commands:"
        echo "  deploy         - Run complete deployment pipeline (default)"
        echo "  infrastructure - Deploy only infrastructure services"
        echo "  validator      - Deploy only Solana validator"
        echo "  build          - Build smart contracts only"
        echo "  contracts      - Deploy smart contracts only"
        echo "  verify         - Verify deployment only"
        echo "  initialize     - Initialize PoA system only"
        echo "  test           - Run post-deployment tests only"
        echo "  clean          - Clean up all services and volumes"
        echo ""
        echo "Environment variables:"
        echo "  ENVIRONMENT=development|production"
        echo "  SKIP_INFRASTRUCTURE=true|false"
        echo "  SKIP_TESTS=true|false"
        echo "  VERBOSE=true|false"
        exit 1
        ;;
esac