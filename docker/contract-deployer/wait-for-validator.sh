#!/bin/bash

# Wait for Solana validator to be ready
# This script ensures the validator is healthy before attempting deployment

set -e

VALIDATOR_URL="${SOLANA_RPC_URL:-http://solana-validator:8899}"
MAX_ATTEMPTS=60
ATTEMPT_DELAY=5

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') [WAIT-VALIDATOR] $1"
}

log "Waiting for Solana validator at $VALIDATOR_URL..."

attempt=1
while [ $attempt -le $MAX_ATTEMPTS ]; do
    log "Attempt $attempt/$MAX_ATTEMPTS: Checking validator health..."
    
    # Check if validator is responding
    if solana cluster-version --url "$VALIDATOR_URL" >/dev/null 2>&1; then
        log "Validator is healthy and responding"
        
        # Additional check: ensure validator has some block height
        block_height=$(solana block-height --url "$VALIDATOR_URL" 2>/dev/null || echo "0")
        if [ "$block_height" -gt 0 ]; then
            log "Validator is producing blocks (height: $block_height)"
            exit 0
        else
            log "Validator responding but no blocks produced yet"
        fi
    else
        log "Validator not responding yet"
    fi
    
    if [ $attempt -lt $MAX_ATTEMPTS ]; then
        log "Waiting $ATTEMPT_DELAY seconds before next attempt..."
        sleep $ATTEMPT_DELAY
    fi
    
    attempt=$((attempt + 1))
done

log "ERROR: Validator failed to become ready within $((MAX_ATTEMPTS * ATTEMPT_DELAY)) seconds"
exit 1