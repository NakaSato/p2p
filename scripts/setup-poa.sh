#!/bin/bash

# P2P Energy Trading Platform - Proof of Authority (PoA) Initialization Script
# This script sets up and initializes the PoA consensus system for the Engineering Department
# Version: 1.0 (September 2025)

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
LOG_FILE="$PROJECT_ROOT/poa-setup.log"

# PoA Configuration
AUTHORITY_NAME="Engineering Department"
NETWORK_NAME="Engineering Campus Energy Network"
VALIDATOR_NAME="engineering-validator"

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

success() {
    echo -e "${CYAN}[SUCCESS] $1${NC}"
    echo "[SUCCESS] $1" >> "$LOG_FILE"
}

# Header
print_header() {
    echo -e "${PURPLE}========================================${NC}"
    echo -e "${PURPLE}P2P Energy Trading Platform${NC}"
    echo -e "${PURPLE}Proof of Authority Initialization${NC}"
    echo -e "${PURPLE}========================================${NC}"
    echo -e "${BLUE}Authority: ${AUTHORITY_NAME}${NC}"
    echo -e "${BLUE}Network: ${NETWORK_NAME}${NC}"
    echo -e "${BLUE}Validator: ${VALIDATOR_NAME}${NC}"
    echo ""
}

# Check prerequisites
check_prerequisites() {
    log "Checking PoA setup prerequisites..."
    
    # Check if we're in the project root
    if [ ! -f "$PROJECT_ROOT/Anchor.toml" ]; then
        error "Anchor.toml not found. Please run this script from the project root."
    fi
    
    # Check Solana CLI
    if ! command -v solana &> /dev/null; then
        error "Solana CLI not found. Install from: https://docs.solana.com/cli/install-solana-cli-tools"
    fi
    
    # Check Anchor CLI
    if ! command -v anchor &> /dev/null; then
        error "Anchor CLI not found. Install from: https://www.anchor-lang.com/docs/installation"
    fi
    
    # Check if Solana validator is running
    if ! solana cluster-version --url localhost &> /dev/null; then
        warn "Local Solana validator not detected. Starting validator..."
        start_local_validator
    fi
    
    success "Prerequisites check completed"
}

# Start local validator with PoA configuration
start_local_validator() {
    log "Starting Solana test validator with PoA configuration..."
    
    # Kill any existing validator
    pkill -f solana-test-validator || true
    sleep 2
    
    # Clean previous ledger
    if [ -d "$PROJECT_ROOT/test-ledger" ]; then
        log "Cleaning previous ledger data..."
        rm -rf "$PROJECT_ROOT/test-ledger"
    fi
    
    # Create validator keys directory if it doesn't exist
    mkdir -p "$PROJECT_ROOT/validator-keys"
    
    # Generate validator keypair if it doesn't exist
    if [ ! -f "$PROJECT_ROOT/validator-keys/validator-keypair.json" ]; then
        log "Generating validator keypair..."
        solana-keygen new --no-bip39-passphrase --outfile "$PROJECT_ROOT/validator-keys/validator-keypair.json"
    fi
    
    # Generate vote account keypair if it doesn't exist
    if [ ! -f "$PROJECT_ROOT/validator-keys/vote-account-keypair.json" ]; then
        log "Generating vote account keypair..."
        solana-keygen new --no-bip39-passphrase --outfile "$PROJECT_ROOT/validator-keys/vote-account-keypair.json"
    fi
    
    # Generate stake account keypair if it doesn't exist
    if [ ! -f "$PROJECT_ROOT/validator-keys/engineering-stake-keypair.json" ]; then
        log "Generating stake account keypair..."
        solana-keygen new --no-bip39-passphrase --outfile "$PROJECT_ROOT/validator-keys/engineering-stake-keypair.json"
    fi
    
    # Start validator with custom configuration
    log "Launching PoA validator..."
    solana-test-validator \
        --ledger "$PROJECT_ROOT/test-ledger" \
        --reset \
        --quiet \
        --log > "$PROJECT_ROOT/test-ledger/validator.log" 2>&1 &
    
    local validator_pid=$!
    echo "$validator_pid" > "$PROJECT_ROOT/.validator_pid"
    
    # Wait for validator to be ready
    log "Waiting for validator to initialize..."
    local max_attempts=30
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if solana cluster-version --url localhost &> /dev/null; then
            success "Validator is ready (attempt $attempt)"
            break
        fi
        
        if [ $attempt -eq $max_attempts ]; then
            error "Validator failed to start. Check logs at: $PROJECT_ROOT/test-ledger/validator.log"
        fi
        
        sleep 2
        ((attempt++))
    done
    
    success "PoA validator started successfully (PID: $validator_pid)"
}

# Setup authority keypairs and accounts
setup_authority_accounts() {
    log "Setting up PoA authority accounts..."
    
    cd "$PROJECT_ROOT"
    
    # Create test accounts directory if it doesn't exist
    mkdir -p test-accounts
    
    # Generate REC validator keypair
    if [ ! -f "test-accounts/rec-validator.json" ]; then
        log "Generating REC validator keypair..."
        solana-keygen new --no-bip39-passphrase --outfile test-accounts/rec-validator.json
    fi
    
    # Airdrop SOL to authority accounts
    log "Funding authority accounts..."
    
    local validator_pubkey=$(solana-keygen pubkey test-accounts/rec-validator.json)
    
    solana airdrop 100 "$validator_pubkey" --url localhost
    
    info "Authority account balances:"
    echo "  • REC Validator: $(solana balance "$validator_pubkey" --url localhost)"
    
    success "Authority accounts setup completed"
}

# Initialize governance program with PoA authorities
initialize_governance() {
    log "Initializing Governance program with PoA authorities..."
    
    cd "$PROJECT_ROOT"
    
    # Check if governance program is deployed
    if [ ! -f "target/deploy/governance-keypair.json" ]; then
        warn "Governance program not found. Building and deploying..."
        anchor build
        anchor deploy --provider.cluster localnet
    fi
    
    local governance_id=$(solana-keygen pubkey target/deploy/governance-keypair.json)
    log "Governance Program ID: $governance_id"
    
    # Get authority public keys
    local validator_pubkey=$(solana-keygen pubkey test-accounts/rec-validator.json)
    
    info "PoA Authority Structure:"
    echo "  • Primary Authority (REC Validator): $validator_pubkey"
    
    # Create PoA configuration
    create_poa_config
    
    success "Governance PoA initialization completed"
}

# Create PoA configuration file
create_poa_config() {
    log "Creating PoA configuration..."
    
    local config_file="$PROJECT_ROOT/poa-config.json"
    local validator_pubkey=$(solana-keygen pubkey test-accounts/rec-validator.json)
    local governance_id=$(solana-keygen pubkey target/deploy/governance-keypair.json)
    
    cat > "$config_file" << EOF
{
  "network": {
    "name": "$NETWORK_NAME",
    "authority": "$AUTHORITY_NAME",
    "validator_name": "$VALIDATOR_NAME",
    "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  },
  "consensus": {
    "type": "proof_of_authority",
    "authorities": [
      {
        "name": "REC Validator",
        "pubkey": "$validator_pubkey",
        "role": "primary",
        "permissions": ["initialize", "manage_authorities", "emergency_stop", "validate_transactions", "oracle_data"]
      }
    ]
  },
  "programs": {
    "governance": "$governance_id",
    "registry": "$(solana-keygen pubkey target/deploy/registry-keypair.json 2>/dev/null || echo 'Not deployed')",
    "energy_token": "$(solana-keygen pubkey target/deploy/energy_token-keypair.json 2>/dev/null || echo 'Not deployed')",
    "trading": "$(solana-keygen pubkey target/deploy/trading-keypair.json 2>/dev/null || echo 'Not deployed')",
    "oracle": "$(solana-keygen pubkey target/deploy/oracle-keypair.json 2>/dev/null || echo 'Not deployed')"
  },
  "network_info": {
    "rpc_url": "http://localhost:8899",
    "ws_url": "ws://localhost:8900",
    "cluster": "localnet"
  }
}
EOF
    
    success "PoA configuration created: $config_file"
}

# Initialize all programs with PoA authorities
initialize_all_programs() {
    log "Initializing all programs with PoA authorities..."
    
    cd "$PROJECT_ROOT"
    
    # Build all programs if needed
    if [ ! -d "target/deploy" ]; then
        log "Building programs..."
        anchor build
    fi
    
    # Deploy all programs
    log "Deploying all programs..."
    anchor deploy --provider.cluster localnet
    
    # Run initialization tests
    log "Running program initialization..."
    if anchor test --skip-local-validator --skip-build --skip-deploy; then
        success "All programs initialized successfully"
    else
        warn "Some program initializations may have failed. Check test output."
    fi
}

# Verify PoA setup
verify_poa_setup() {
    log "Verifying PoA setup..."
    
    # Check validator is running
    if solana cluster-version --url localhost &> /dev/null; then
        success "✓ Validator is running"
    else
        error "✗ Validator is not accessible"
    fi
    
    # Check authority account balances
    local validator_pubkey=$(solana-keygen pubkey test-accounts/rec-validator.json)
    local validator_balance=$(solana balance "$validator_pubkey" --url localhost | cut -d' ' -f1)
    
    if (( $(echo "$validator_balance > 0" | bc -l) )); then
        success "✓ Authority accounts are funded"
    else
        error "✗ Authority accounts are not properly funded"
    fi
    
    # Check program deployments
    local programs_ok=0
    local total_programs=0
    
    for program in registry energy_token trading oracle governance; do
        total_programs=$((total_programs + 1))
        if [ -f "target/deploy/${program}-keypair.json" ]; then
            local program_id=$(solana-keygen pubkey "target/deploy/${program}-keypair.json")
            if solana program show "$program_id" --url localhost &> /dev/null; then
                success "✓ ${program} program deployed: $program_id"
                programs_ok=$((programs_ok + 1))
            else
                warn "✗ ${program} program not accessible"
            fi
        else
            warn "✗ ${program} program keypair not found"
        fi
    done
    
    if [ $programs_ok -eq $total_programs ]; then
        success "✓ All programs are deployed and accessible"
    else
        warn "✗ $((total_programs - programs_ok)) programs have issues"
    fi
    
    success "PoA verification completed"
}

# Display PoA status and information
display_poa_status() {
    echo ""
    echo -e "${PURPLE}========================================${NC}"
    echo -e "${PURPLE}PoA Setup Complete!${NC}"
    echo -e "${PURPLE}========================================${NC}"
    echo ""
    
    echo -e "${BLUE}Network Information:${NC}"
    echo "  • Name: $NETWORK_NAME"
    echo "  • Authority: $AUTHORITY_NAME"
    echo "  • Validator: $VALIDATOR_NAME"
    echo "  • RPC URL: http://localhost:8899"
    echo ""
    
    echo -e "${BLUE}Authority Accounts:${NC}"
    if [ -f "test-accounts/rec-validator.json" ]; then
        echo "  • REC Validator: $(solana-keygen pubkey test-accounts/rec-validator.json)"
    fi
    echo ""
    
    echo -e "${BLUE}Deployed Programs:${NC}"
    for program in registry energy_token trading oracle governance; do
        if [ -f "target/deploy/${program}-keypair.json" ]; then
            local program_id=$(solana-keygen pubkey "target/deploy/${program}-keypair.json")
            echo "  • ${program}: $program_id"
        fi
    done
    echo ""
    
    echo -e "${BLUE}Configuration Files:${NC}"
    echo "  • PoA Config: $PROJECT_ROOT/poa-config.json"
    echo "  • Validator Log: $PROJECT_ROOT/test-ledger/validator.log"
    echo "  • Setup Log: $PROJECT_ROOT/poa-setup.log"
    echo ""
    
    echo -e "${BLUE}Next Steps:${NC}"
    echo "  • Run integration tests: anchor test --skip-local-validator"
    echo "  • Start API Gateway: cd api-gateway && cargo run"
    echo "  • Start frontend: cd frontend && yarn dev"
    echo "  • Stop validator: kill \$(cat $PROJECT_ROOT/.validator_pid)"
    echo ""
    
    echo -e "${GREEN}Engineering Department PoA Network is ready for energy trading!${NC}"
}

# Cleanup function
cleanup() {
    log "PoA setup cleanup..."
    # Note: We intentionally don't stop the validator here as it should keep running
    # The user can manually stop it when needed
}

# Main execution
main() {
    print_header
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --reset)
                RESET_MODE=true
                shift
                ;;
            --verify-only)
                VERIFY_ONLY=true
                shift
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --reset       Reset all PoA configuration and restart"
                echo "  --verify-only Only verify existing PoA setup"
                echo "  --help        Show this help message"
                echo ""
                echo "This script initializes a Proof of Authority consensus system"
                echo "for the Engineering Department's P2P Energy Trading Platform."
                exit 0
                ;;
            *)
                error "Unknown option: $1"
                ;;
        esac
    done
    
    # Handle reset mode
    if [ "$RESET_MODE" = "true" ]; then
        warn "Resetting PoA configuration..."
        pkill -f solana-test-validator || true
        sleep 2
        rm -rf "$PROJECT_ROOT/test-ledger" || true
        rm -f "$PROJECT_ROOT/.validator_pid" || true
        rm -f "$PROJECT_ROOT/poa-config.json" || true
        log "Reset completed"
    fi
    
    # Handle verify-only mode
    if [ "$VERIFY_ONLY" = "true" ]; then
        verify_poa_setup
        exit 0
    fi
    
    # Full PoA initialization
    check_prerequisites
    setup_authority_accounts
    initialize_governance
    initialize_all_programs
    verify_poa_setup
    display_poa_status
    
    log "PoA initialization completed successfully"
}

# Trap cleanup on exit
trap cleanup EXIT

# Execute main function
main "$@"