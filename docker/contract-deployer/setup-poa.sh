#!/bin/bash

# PoA Setup for Docker Container Environment
# This script is called from within the contract-deployer container

set -e

# Container environment paths
WORKSPACE_DIR="/opt/deployer/workspace"
ARTIFACTS_DIR="/opt/deployer/artifacts"
CONFIG_DIR="/opt/deployer/config"
LOG_FILE="/opt/deployer/logs/poa-setup.log"
VALIDATOR_URL="${SOLANA_RPC_URL:-http://solana-validator:8899}"

# Ensure log file exists
mkdir -p "$(dirname "$LOG_FILE")"
touch "$LOG_FILE"

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [POA-DOCKER] $1" | tee -a "$LOG_FILE"
}

# Function to setup PoA keypairs in container
setup_poa_keypairs() {
    log "Setting up PoA keypairs in container environment..."
    
    local keys_dir="$CONFIG_DIR/poa-keys"
    mkdir -p "$keys_dir"
    
    # Generate PoA authority keypairs
    local keypairs=(
        "university-authority-keypair.json"
        "sustainability-validator-keypair.json"
        "engineering-validator-keypair.json"
        "facilities-validator-keypair.json"
    )
    
    for keypair in "${keypairs[@]}"; do
        local keypair_path="$keys_dir/$keypair"
        if [ ! -f "$keypair_path" ]; then
            log "Generating keypair: $keypair"
            solana-keygen new --no-bip39-passphrase --outfile "$keypair_path"
        fi
        
        local pubkey
        pubkey=$(solana-keygen pubkey "$keypair_path")
        log "$(basename "$keypair" .json) public key: $pubkey"
        
        # Airdrop SOL to the account
        log "Airdropping SOL to $(basename "$keypair" .json)..."
        solana airdrop 1000 "$pubkey" --url "$VALIDATOR_URL" 2>&1 | tee -a "$LOG_FILE" || true
    done
    
    echo "$keys_dir"
}

# Function to create PoA initialization script
create_poa_init_script() {
    local keys_dir="$1"
    local init_script="$CONFIG_DIR/init-poa.js"
    
    log "Creating PoA initialization script..."
    
    cat > "$init_script" << 'EOF'
const anchor = require('@coral-xyz/anchor');
const { Connection, Keypair, PublicKey, SystemProgram } = require('@solana/web3.js');
const fs = require('fs');
const path = require('path');

async function initializePoA() {
    try {
        console.log('=== PoA Governance Initialization ===');
        
        const rpcUrl = process.env.SOLANA_RPC_URL || 'http://solana-validator:8899';
        const keysDir = process.env.POA_KEYS_DIR;
        const governanceProgramId = process.env.GOVERNANCE_PROGRAM_ID;
        
        console.log('RPC URL:', rpcUrl);
        console.log('Keys Directory:', keysDir);
        console.log('Governance Program ID:', governanceProgramId);
        
        // Setup connection
        const connection = new Connection(rpcUrl, 'confirmed');
        
        // Load keypairs
        const loadKeypair = (filename) => {
            const keypairPath = path.join(keysDir, filename);
            console.log('Loading keypair from:', keypairPath);
            const secretKey = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
            return Keypair.fromSecretKey(new Uint8Array(secretKey));
        };
        
        const universityAuthority = loadKeypair('university-authority-keypair.json');
        const sustainabilityValidator = loadKeypair('sustainability-validator-keypair.json');
        const engineeringValidator = loadKeypair('engineering-validator-keypair.json');
        const facilitiesValidator = loadKeypair('facilities-validator-keypair.json');
        
        console.log('University Authority:', universityAuthority.publicKey.toString());
        console.log('Sustainability Validator:', sustainabilityValidator.publicKey.toString());
        console.log('Engineering Validator:', engineeringValidator.publicKey.toString());
        console.log('Facilities Validator:', facilitiesValidator.publicKey.toString());
        
        // Setup Anchor provider
        const provider = new anchor.AnchorProvider(
            connection,
            new anchor.Wallet(universityAuthority),
            { commitment: 'confirmed' }
        );
        anchor.setProvider(provider);
        
        // Load governance program
        const programId = new PublicKey(governanceProgramId);
        const idlPath = '/opt/deployer/workspace/target/idl/governance.json';
        
        if (!fs.existsSync(idlPath)) {
            throw new Error(`IDL file not found: ${idlPath}`);
        }
        
        const idl = JSON.parse(fs.readFileSync(idlPath, 'utf8'));
        const program = new anchor.Program(idl, programId, provider);
        
        // Find PoA config PDA
        const [poaConfigPda] = PublicKey.findProgramAddressSync(
            [Buffer.from('poa_config')],
            programId
        );
        
        console.log('PoA Config PDA:', poaConfigPda.toString());
        
        // Check if already initialized
        try {
            const existingConfig = await program.account.poAConfig.fetch(poaConfigPda);
            console.log('PoA already initialized with authority:', existingConfig.universityAuthority.toString());
            console.log('REC Validators count:', existingConfig.authorizedRecValidators.length);
            return;
        } catch (error) {
            console.log('PoA not initialized, proceeding with setup...');
        }
        
        // Initialize PoA governance
        console.log('Initializing PoA governance...');
        
        const tx = await program.methods
            .initializePoaWithRec()
            .accounts({
                poaConfig: poaConfigPda,
                universityAuthority: universityAuthority.publicKey,
                sustainabilityValidator: sustainabilityValidator.publicKey,
                engineeringValidator: engineeringValidator.publicKey,
                facilitiesValidator: facilitiesValidator.publicKey,
                systemProgram: SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([universityAuthority])
            .rpc();
        
        console.log('PoA governance initialized successfully!');
        console.log('Transaction signature:', tx);
        
        // Verify initialization
        const poaConfig = await program.account.poAConfig.fetch(poaConfigPda);
        console.log('\n=== PoA Configuration ===');
        console.log('University Authority:', poaConfig.universityAuthority.toString());
        console.log('REC Validators:', poaConfig.authorizedRecValidators.length);
        console.log('Min Validators Required:', poaConfig.minRecValidators);
        console.log('Created At:', new Date(poaConfig.createdAt * 1000).toISOString());
        
        // Save configuration for later use
        const configData = {
            universityAuthority: poaConfig.universityAuthority.toString(),
            poaConfigPda: poaConfigPda.toString(),
            validators: poaConfig.authorizedRecValidators.map(v => ({
                pubkey: v.pubkey.toString(),
                authorityName: v.authorityName,
                active: v.active
            })),
            transactionSignature: tx,
            timestamp: new Date().toISOString()
        };
        
        fs.writeFileSync('/opt/deployer/artifacts/poa-config.json', JSON.stringify(configData, null, 2));
        console.log('PoA configuration saved to artifacts');
        
    } catch (error) {
        console.error('Error initializing PoA governance:', error);
        if (error.logs) {
            console.error('Program logs:', error.logs);
        }
        process.exit(1);
    }
}

// Execute if this script is run directly
if (require.main === module) {
    initializePoA().catch(console.error);
}

module.exports = { initializePoA };
EOF
    
    echo "$init_script"
}

# Function to run PoA initialization
run_poa_initialization() {
    local keys_dir="$1"
    local init_script="$2"
    
    # Get governance program ID
    local governance_program_id=""
    if [ -f "$ARTIFACTS_DIR/governance/program_id.txt" ]; then
        governance_program_id=$(cat "$ARTIFACTS_DIR/governance/program_id.txt")
        log "Found governance program ID: $governance_program_id"
    else
        log "ERROR: Governance program ID not found in artifacts"
        return 1
    fi
    
    # Check if we're in the workspace with package.json
    cd "$WORKSPACE_DIR"
    
    if [ ! -f "package.json" ]; then
        log "ERROR: package.json not found in workspace"
        return 1
    fi
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        log "Installing Node.js dependencies..."
        npm install 2>&1 | tee -a "$LOG_FILE"
    fi
    
    # Set environment variables
    export SOLANA_RPC_URL="$VALIDATOR_URL"
    export POA_KEYS_DIR="$keys_dir"
    export GOVERNANCE_PROGRAM_ID="$governance_program_id"
    
    # Run the initialization script
    log "Running PoA initialization..."
    node "$init_script" 2>&1 | tee -a "$LOG_FILE"
    
    if [ $? -eq 0 ]; then
        log "PoA initialization completed successfully"
        
        # Create a simple status file
        echo "SUCCESS" > "$CONFIG_DIR/poa-status.txt"
        echo "$(date)" >> "$CONFIG_DIR/poa-status.txt"
        echo "Governance Program: $governance_program_id" >> "$CONFIG_DIR/poa-status.txt"
        
        return 0
    else
        log "ERROR: PoA initialization failed"
        echo "FAILED" > "$CONFIG_DIR/poa-status.txt"
        return 1
    fi
}

# Main function
main() {
    log "=== PoA Docker Setup Starting ==="
    
    # Setup keypairs
    keys_dir=$(setup_poa_keypairs)
    
    # Create initialization script
    init_script=$(create_poa_init_script "$keys_dir")
    
    # Run PoA initialization
    if run_poa_initialization "$keys_dir" "$init_script"; then
        log "=== PoA Docker Setup Completed Successfully ==="
    else
        log "=== PoA Docker Setup Failed ==="
        exit 1
    fi
}

# Execute main function
main "$@"