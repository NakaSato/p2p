#!/bin/bash

# Deploy all Anchor programs for the P2P Energy Trading System
# This is the main deployment orchestration script

set -e

WORKSPACE_DIR="/opt/deployer/workspace"
ARTIFACTS_DIR="/opt/deployer/artifacts"
LOG_FILE="/opt/deployer/logs/deploy.log"
CONFIG_DIR="/opt/deployer/config"
VALIDATOR_URL="${SOLANA_RPC_URL:-http://solana-validator:8899}"
DEPLOYER_KEYPAIR="$CONFIG_DIR/deployer-keypair.json"

# Ensure directories exist
mkdir -p /opt/deployer/logs "$CONFIG_DIR"

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [DEPLOY] $1" | tee -a "$LOG_FILE"
}

# Function to setup deployer account
setup_deployer_account() {
    log "Setting up deployer account..."
    
    # Generate keypair if it doesn't exist
    if [ ! -f "$DEPLOYER_KEYPAIR" ]; then
        log "Generating deployer keypair..."
        solana-keygen new --no-bip39-passphrase --outfile "$DEPLOYER_KEYPAIR"
    fi
    
    # Get deployer public key
    local deployer_pubkey
    deployer_pubkey=$(solana-keygen pubkey "$DEPLOYER_KEYPAIR")
    log "Deployer account: $deployer_pubkey"
    
    # Check balance and airdrop if needed
    local balance
    balance=$(solana balance "$deployer_pubkey" --url "$VALIDATOR_URL" 2>/dev/null | awk '{print $1}' || echo "0")
    log "Current balance: $balance SOL"
    
    # Airdrop SOL if balance is low
    if [ "${balance%.*}" -lt 100 ]; then
        log "Airdropping SOL to deployer account..."
        solana airdrop 1000 "$deployer_pubkey" --url "$VALIDATOR_URL" 2>&1 | tee -a "$LOG_FILE"
        
        # Verify airdrop
        balance=$(solana balance "$deployer_pubkey" --url "$VALIDATOR_URL" 2>/dev/null | awk '{print $1}' || echo "0")
        log "Updated balance: $balance SOL"
    fi
    
    # Set as default keypair
    solana config set --keypair "$DEPLOYER_KEYPAIR" --url "$VALIDATOR_URL"
}

# Function to deploy a single program
deploy_program() {
    local program_name="$1"
    local program_path="$WORKSPACE_DIR/programs/$program_name"
    
    log "Deploying program: $program_name"
    
    if [ ! -d "$program_path" ]; then
        log "ERROR: Program directory not found: $program_path"
        return 1
    fi
    
    # Change to program directory or workspace root
    if [ -f "$program_path/Anchor.toml" ]; then
        cd "$program_path"
    else
        cd "$WORKSPACE_DIR"
    fi
    
    # Deploy the program
    log "Running anchor deploy for $program_name"
    if ! anchor deploy --program-name "$program_name" --provider.cluster "$VALIDATOR_URL" 2>&1 | tee -a "$LOG_FILE"; then
        log "ERROR: Failed to deploy $program_name"
        return 1
    fi
    
    # Verify deployment
    local program_id
    program_id=$(anchor keys list | grep "$program_name" | awk '{print $2}' 2>/dev/null || echo "")
    if [ -n "$program_id" ]; then
        log "Successfully deployed $program_name with program ID: $program_id"
        
        # Save program ID to artifacts
        echo "$program_id" > "$ARTIFACTS_DIR/$program_name/program_id.txt"
    else
        log "WARNING: Could not verify program ID for $program_name"
    fi
    
    return 0
}

# Function to initialize PoA governance
initialize_poa() {
    log "Initializing PoA governance system..."
    
    # Run the PoA setup script
    if /usr/local/bin/setup-poa.sh 2>&1 | tee -a "$LOG_FILE"; then
        log "PoA governance initialized successfully"
        return 0
    else
        log "WARNING: PoA initialization failed, but continuing deployment"
        return 0  # Don't fail the entire deployment for PoA issues
    fi
}

# Function to run post-deployment tests
run_post_deployment_tests() {
    log "Running post-deployment verification tests..."
    
    cd "$WORKSPACE_DIR"
    
    # Check if test script exists
    if [ -f "package.json" ] && npm list --depth=0 2>/dev/null | grep -q "@coral-xyz/anchor"; then
        log "Running Anchor tests..."
        if npm test 2>&1 | tee -a "$LOG_FILE"; then
            log "Post-deployment tests passed"
        else
            log "WARNING: Some post-deployment tests failed"
        fi
    else
        log "No test configuration found, skipping tests"
    fi
}

# Main deployment function
main() {
    log "=== Starting deployment of all contracts ==="
    
    # Wait for validator to be ready
    log "Waiting for Solana validator..."
    /usr/local/bin/wait-for-validator.sh
    
    # Setup deployer account
    setup_deployer_account
    
    # Build all contracts first
    log "Building all contracts..."
    /usr/local/bin/build-contracts.sh
    
    # Define deployment order (dependencies first)
    local programs=("registry" "energy-token" "governance" "oracle" "trading")
    local deployed_programs=()
    local failed_programs=()
    
    # Deploy each program
    for program in "${programs[@]}"; do
        if deploy_program "$program"; then
            deployed_programs+=("$program")
        else
            failed_programs+=("$program")
        fi
    done
    
    # Report deployment results
    log "Deployment Summary:"
    log "Successfully deployed (${#deployed_programs[@]}): ${deployed_programs[*]}"
    
    if [ ${#failed_programs[@]} -gt 0 ]; then
        log "Failed to deploy (${#failed_programs[@]}): ${failed_programs[*]}"
        log "=== Deployment failed ==="
        exit 1
    fi
    
    # Initialize PoA governance
    initialize_poa
    
    # Run post-deployment tests
    run_post_deployment_tests
    
    log "All programs deployed and configured successfully"
    log "=== Deployment completed successfully ==="
    
    # Keep container running for monitoring
    log "Deployment service ready for monitoring..."
    while true; do
        sleep 3600
    done
}

# Execute main function
main "$@"