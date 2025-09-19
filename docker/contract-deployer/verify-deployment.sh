#!/bin/bash

# Verify deployment status and health of all deployed contracts
# Used by Docker healthcheck and monitoring

set -e

ARTIFACTS_DIR="/opt/deployer/artifacts"
VALIDATOR_URL="${SOLANA_RPC_URL:-http://solana-validator:8899}"
LOG_FILE="/opt/deployer/logs/verify.log"

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [VERIFY] $1" | tee -a "$LOG_FILE"
}

# Function to check if validator is healthy
check_validator() {
    if solana cluster-version --url "$VALIDATOR_URL" >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to verify a single program deployment
verify_program() {
    local program_name="$1"
    local program_id_file="$ARTIFACTS_DIR/$program_name/program_id.txt"
    
    if [ ! -f "$program_id_file" ]; then
        log "ERROR: Program ID file not found for $program_name"
        return 1
    fi
    
    local program_id
    program_id=$(cat "$program_id_file")
    
    # Check if program exists on-chain
    if solana account "$program_id" --url "$VALIDATOR_URL" >/dev/null 2>&1; then
        log "✓ Program $program_name ($program_id) is deployed and accessible"
        return 0
    else
        log "✗ Program $program_name ($program_id) is not accessible"
        return 1
    fi
}

# Function to verify all program deployments
verify_all_programs() {
    local programs=("registry" "energy-token" "governance" "oracle" "trading")
    local verified_programs=()
    local failed_programs=()
    
    log "Verifying all deployed programs..."
    
    for program in "${programs[@]}"; do
        if verify_program "$program"; then
            verified_programs+=("$program")
        else
            failed_programs+=("$program")
        fi
    done
    
    log "Verification Summary:"
    log "Successfully verified (${#verified_programs[@]}): ${verified_programs[*]}"
    
    if [ ${#failed_programs[@]} -gt 0 ]; then
        log "Failed verification (${#failed_programs[@]}): ${failed_programs[*]}"
        return 1
    fi
    
    log "All programs verified successfully"
    return 0
}

# Function to check deployment artifacts
check_artifacts() {
    log "Checking deployment artifacts..."
    
    if [ ! -d "$ARTIFACTS_DIR" ]; then
        log "ERROR: Artifacts directory not found"
        return 1
    fi
    
    local programs=("registry" "energy-token" "governance" "oracle" "trading")
    local missing_artifacts=()
    
    for program in "${programs[@]}"; do
        if [ ! -d "$ARTIFACTS_DIR/$program" ]; then
            missing_artifacts+=("$program")
        fi
    done
    
    if [ ${#missing_artifacts[@]} -gt 0 ]; then
        log "Missing artifacts for programs: ${missing_artifacts[*]}"
        return 1
    fi
    
    log "All program artifacts found"
    return 0
}

# Main verification function
main() {
    # For verbose output (manual runs)
    if [ "${VERIFY_VERBOSE:-false}" = "true" ]; then
        log "=== Starting deployment verification ==="
        /usr/local/bin/health-monitor.sh detailed
        return $?
    else
        # Silent check for Docker healthcheck
        /usr/local/bin/health-monitor.sh status >/dev/null 2>&1
        return $?
    fi
}

# Execute main function
main "$@"