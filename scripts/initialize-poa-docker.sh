#!/bin/bash

# Initialize PoA (Proof of Authority) governance system for P2P Energy Trading
# This script sets up the university authority and REC validators in Docker environment

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
KEYS_DIR="$PROJECT_ROOT/validator-keys"
VALIDATOR_URL="${SOLANA_RPC_URL:-http://localhost:8899}"
LOG_FILE="/tmp/poa-init.log"

# PoA Configuration
UNIVERSITY_AUTHORITY_KEYPAIR="$KEYS_DIR/university-authority-keypair.json"
SUSTAINABILITY_VALIDATOR_KEYPAIR="$KEYS_DIR/sustainability-validator-keypair.json"
ENGINEERING_VALIDATOR_KEYPAIR="$KEYS_DIR/engineering-validator-keypair.json"
FACILITIES_VALIDATOR_KEYPAIR="$KEYS_DIR/facilities-validator-keypair.json"

# Ensure log file exists
touch "$LOG_FILE"

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [POA-INIT] $1" | tee -a "$LOG_FILE"
}

# Function to check if Solana validator is ready
wait_for_validator() {
    local max_attempts=60
    local attempt=1
    
    log "Waiting for Solana validator at $VALIDATOR_URL..."
    
    while [ $attempt -le $max_attempts ]; do
        if solana cluster-version --url "$VALIDATOR_URL" >/dev/null 2>&1; then
            log "Validator is ready"
            return 0
        fi
        
        log "Waiting for validator (attempt $attempt/$max_attempts)..."
        sleep 5
        attempt=$((attempt + 1))
    done
    
    log "ERROR: Validator failed to become ready"
    return 1
}

# Function to generate or load keypair
ensure_keypair() {
    local keypair_path="$1"
    local description="$2"
    
    if [ ! -f "$keypair_path" ]; then
        log "Generating $description keypair: $keypair_path"
        mkdir -p "$(dirname "$keypair_path")"
        solana-keygen new --no-bip39-passphrase --outfile "$keypair_path"
    else
        log "Using existing $description keypair: $keypair_path"
    fi
    
    # Get public key
    local pubkey
    pubkey=$(solana-keygen pubkey "$keypair_path")
    log "$description public key: $pubkey"
    echo "$pubkey"
}

# Function to airdrop SOL to an account
airdrop_sol() {
    local keypair_path="$1"
    local amount="${2:-1000}"
    local description="$3"
    
    local pubkey
    pubkey=$(solana-keygen pubkey "$keypair_path")
    
    log "Airdropping $amount SOL to $description ($pubkey)"
    
    # Check current balance
    local balance
    balance=$(solana balance "$pubkey" --url "$VALIDATOR_URL" 2>/dev/null | awk '{print $1}' || echo "0")
    log "Current balance for $description: $balance SOL"
    
    # Airdrop if balance is low
    if [ "${balance%.*}" -lt 100 ]; then
        solana airdrop "$amount" "$pubkey" --url "$VALIDATOR_URL" 2>&1 | tee -a "$LOG_FILE"
        
        # Verify airdrop
        balance=$(solana balance "$pubkey" --url "$VALIDATOR_URL" 2>/dev/null | awk '{print $1}' || echo "0")
        log "Updated balance for $description: $balance SOL"
    else
        log "$description has sufficient balance, skipping airdrop"
    fi
}

# Function to check if governance program is deployed
check_governance_program() {
    log "Checking if governance program is deployed..."
    
    # Try to get the program ID from deployed artifacts
    local program_id=""
    
    # Check if running in Docker container with deployer artifacts
    if [ -f "/opt/deployer/artifacts/governance/program_id.txt" ]; then
        program_id=$(cat "/opt/deployer/artifacts/governance/program_id.txt" 2>/dev/null || echo "")
    fi
    
    # Check in workspace
    if [ -z "$program_id" ] && [ -f "$PROJECT_ROOT/target/deploy/governance-keypair.json" ]; then
        program_id=$(solana-keygen pubkey "$PROJECT_ROOT/target/deploy/governance-keypair.json" 2>/dev/null || echo "")
    fi
    
    # Check using anchor
    if [ -z "$program_id" ]; then
        cd "$PROJECT_ROOT"
        program_id=$(anchor keys list 2>/dev/null | grep governance | awk '{print $2}' || echo "")
    fi
    
    if [ -n "$program_id" ]; then
        log "Governance program found with ID: $program_id"
        
        # Verify program exists on-chain
        if solana account "$program_id" --url "$VALIDATOR_URL" >/dev/null 2>&1; then
            log "Governance program is deployed and accessible"
            echo "$program_id"
            return 0
        else
            log "WARNING: Governance program ID found but not accessible on-chain"
        fi
    fi
    
    log "ERROR: Governance program not found or not deployed"
    return 1
}

# Function to initialize PoA governance
initialize_poa_governance() {
    local governance_program_id="$1"
    
    log "Initializing PoA governance with program ID: $governance_program_id"
    
    # Set Solana configuration
    solana config set --url "$VALIDATOR_URL" --keypair "$UNIVERSITY_AUTHORITY_KEYPAIR"
    
    cd "$PROJECT_ROOT"
    
    # Create a simple initialization script using anchor
    cat > "/tmp/poa_init.js" << 'EOF'
const anchor = require('@coral-xyz/anchor');
const { Connection, Keypair, PublicKey } = require('@solana/web3.js');
const fs = require('fs');

async function initializePoA() {
    try {
        console.log('Initializing PoA governance...');
        
        // Connection setup
        const connection = new Connection(process.env.SOLANA_RPC_URL || 'http://localhost:8899', 'confirmed');
        
        // Load keypairs
        const universityAuthority = Keypair.fromSecretKey(
            new Uint8Array(JSON.parse(fs.readFileSync(process.env.UNIVERSITY_AUTHORITY_KEYPAIR || 'validator-keys/university-authority-keypair.json')))
        );
        
        const sustainabilityValidator = Keypair.fromSecretKey(
            new Uint8Array(JSON.parse(fs.readFileSync(process.env.SUSTAINABILITY_VALIDATOR_KEYPAIR || 'validator-keys/sustainability-validator-keypair.json')))
        );
        
        const engineeringValidator = Keypair.fromSecretKey(
            new Uint8Array(JSON.parse(fs.readFileSync(process.env.ENGINEERING_VALIDATOR_KEYPAIR || 'validator-keys/engineering-validator-keypair.json')))
        );
        
        const facilitiesValidator = Keypair.fromSecretKey(
            new Uint8Array(JSON.parse(fs.readFileSync(process.env.FACILITIES_VALIDATOR_KEYPAIR || 'validator-keys/facilities-validator-keypair.json')))
        );
        
        console.log('University Authority:', universityAuthority.publicKey.toString());
        console.log('Sustainability Validator:', sustainabilityValidator.publicKey.toString());
        console.log('Engineering Validator:', engineeringValidator.publicKey.toString());
        console.log('Facilities Validator:', facilitiesValidator.publicKey.toString());
        
        // Load the governance program
        const programId = new PublicKey(process.env.GOVERNANCE_PROGRAM_ID);
        
        // Set up provider
        const provider = new anchor.AnchorProvider(
            connection,
            new anchor.Wallet(universityAuthority),
            anchor.AnchorProvider.defaultOptions()
        );
        anchor.setProvider(provider);
        
        // Load program
        const idl = JSON.parse(fs.readFileSync('target/idl/governance.json'));
        const program = new anchor.Program(idl, programId, provider);
        
        // Find PoA config PDA
        const [poaConfigPda] = PublicKey.findProgramAddressSync(
            [Buffer.from('poa_config')],
            programId
        );
        
        console.log('PoA Config PDA:', poaConfigPda.toString());
        
        // Check if PoA is already initialized
        try {
            const poaConfig = await program.account.poAConfig.fetch(poaConfigPda);
            console.log('PoA governance already initialized with authority:', poaConfig.universityAuthority.toString());
            return;
        } catch (error) {
            console.log('PoA not initialized yet, proceeding with initialization...');
        }
        
        // Initialize PoA governance
        const tx = await program.methods
            .initializePoaWithRec()
            .accounts({
                poaConfig: poaConfigPda,
                universityAuthority: universityAuthority.publicKey,
                sustainabilityValidator: sustainabilityValidator.publicKey,
                engineeringValidator: engineeringValidator.publicKey,
                facilitiesValidator: facilitiesValidator.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([universityAuthority])
            .rpc();
        
        console.log('PoA governance initialized successfully!');
        console.log('Transaction signature:', tx);
        
        // Verify initialization
        const poaConfig = await program.account.poAConfig.fetch(poaConfigPda);
        console.log('PoA Configuration:');
        console.log('- University Authority:', poaConfig.universityAuthority.toString());
        console.log('- REC Validators:', poaConfig.authorizedRecValidators.length);
        console.log('- Min Validators:', poaConfig.minRecValidators);
        console.log('- Created At:', new Date(poaConfig.createdAt * 1000).toISOString());
        
    } catch (error) {
        console.error('Error initializing PoA governance:', error);
        process.exit(1);
    }
}

initializePoA();
EOF
    
    # Set environment variables for the script
    export SOLANA_RPC_URL="$VALIDATOR_URL"
    export GOVERNANCE_PROGRAM_ID="$governance_program_id"
    export UNIVERSITY_AUTHORITY_KEYPAIR="$UNIVERSITY_AUTHORITY_KEYPAIR"
    export SUSTAINABILITY_VALIDATOR_KEYPAIR="$SUSTAINABILITY_VALIDATOR_KEYPAIR"
    export ENGINEERING_VALIDATOR_KEYPAIR="$ENGINEERING_VALIDATOR_KEYPAIR"
    export FACILITIES_VALIDATOR_KEYPAIR="$FACILITIES_VALIDATOR_KEYPAIR"
    
    # Check if we have Node.js and required packages
    if command -v node >/dev/null 2>&1 && [ -f "$PROJECT_ROOT/package.json" ]; then
        log "Running PoA initialization with Node.js..."
        
        # Install dependencies if needed
        if [ ! -d "$PROJECT_ROOT/node_modules" ]; then
            log "Installing Node.js dependencies..."
            npm install
        fi
        
        # Run initialization
        node /tmp/poa_init.js 2>&1 | tee -a "$LOG_FILE"
    else
        log "Node.js not available, using basic Solana CLI initialization..."
        
        # Basic initialization using Solana CLI
        # This would involve calling the governance program directly
        log "PoA governance program deployed at: $governance_program_id"
        log "University Authority: $(solana-keygen pubkey "$UNIVERSITY_AUTHORITY_KEYPAIR")"
        log "Manual PoA initialization required using Anchor/TypeScript client"
    fi
}

# Main function
main() {
    log "=== Starting PoA Governance Initialization ==="
    
    # Wait for validator
    wait_for_validator
    
    # Generate or load keypairs for PoA authorities
    log "Setting up PoA authority keypairs..."
    
    university_pubkey=$(ensure_keypair "$UNIVERSITY_AUTHORITY_KEYPAIR" "University Authority")
    sustainability_pubkey=$(ensure_keypair "$SUSTAINABILITY_VALIDATOR_KEYPAIR" "Sustainability Validator")
    engineering_pubkey=$(ensure_keypair "$ENGINEERING_VALIDATOR_KEYPAIR" "Engineering Validator")
    facilities_pubkey=$(ensure_keypair "$FACILITIES_VALIDATOR_KEYPAIR" "Facilities Validator")
    
    # Airdrop SOL to all authorities
    log "Airdropping SOL to PoA authorities..."
    airdrop_sol "$UNIVERSITY_AUTHORITY_KEYPAIR" 1000 "University Authority"
    airdrop_sol "$SUSTAINABILITY_VALIDATOR_KEYPAIR" 500 "Sustainability Validator"
    airdrop_sol "$ENGINEERING_VALIDATOR_KEYPAIR" 500 "Engineering Validator"
    airdrop_sol "$FACILITIES_VALIDATOR_KEYPAIR" 500 "Facilities Validator"
    
    # Check if governance program is deployed
    if governance_program_id=$(check_governance_program); then
        # Initialize PoA governance
        initialize_poa_governance "$governance_program_id"
    else
        log "WARNING: Governance program not deployed, skipping PoA initialization"
        log "Deploy the governance program first, then run this script again"
    fi
    
    log "=== PoA Governance Initialization Summary ==="
    log "University Authority: $university_pubkey"
    log "Sustainability Validator: $sustainability_pubkey"
    log "Engineering Validator: $engineering_pubkey"
    log "Facilities Validator: $facilities_pubkey"
    log "Validator RPC: $VALIDATOR_URL"
    log "Keypairs stored in: $KEYS_DIR"
    log "Log file: $LOG_FILE"
    log "=== PoA Initialization Completed ==="
}

# Execute main function if script is run directly
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
    main "$@"
fi