#!/bin/bash

echo "Starting Anchor Development Environment..."

# Source environment
export SOLANA_VERSION=1.18.17
export PATH=/home/solana/.local/share/solana/install/active_release/bin:/home/solana/.cargo/bin:$PATH

# Verify installations
echo "Verifying installations..."
echo "Solana version: $(solana --version 2>/dev/null || echo 'Not found')"
echo "Anchor version: $(anchor --version 2>/dev/null || echo 'Not found')"
echo "Rust version: $(rustc --version 2>/dev/null || echo 'Not found')"
echo "Node version: $(node --version 2>/dev/null || echo 'Not found')"

# Create directories for validator if needed
mkdir -p /opt/solana/config /opt/solana/ledger

# Check if we should start the validator (for backward compatibility)
if [ "${START_VALIDATOR:-true}" = "true" ]; then
    echo "Attempting to start Solana Test Validator..."
    
    # Try to generate keypair if it doesn't exist
    if [ ! -f /opt/solana/config/validator-keypair.json ]; then
        echo "Generating validator keypair..."
        timeout 10 solana-keygen new --no-bip39-passphrase --outfile /opt/solana/config/validator-keypair.json 2>/dev/null || {
            echo "Failed to generate keypair due to emulation issues"
            echo "Switching to development mode..."
            export START_VALIDATOR=false
        }
    fi

    # Try to start the validator with timeout
    if [ "${START_VALIDATOR:-true}" = "true" ]; then
        echo "Starting validator with timeout..."
        timeout 30 solana-test-validator \
            --ledger /opt/solana/ledger \
            --rpc-port 8899 \
            --bind-address 0.0.0.0 \
            --gossip-port 8001 \
            --gossip-host 0.0.0.0 \
            --dynamic-port-range 8002-8020 \
            --log \
            --reset 2>/dev/null || {
                echo "Validator failed to start due to emulation issues"
                echo "Switching to development mode..."
                export START_VALIDATOR=false
            }
    fi
fi

if [ "${START_VALIDATOR:-true}" != "true" ]; then
  echo ""
  echo "Anchor Development Environment Ready!"
  echo "Available tools:"
  echo "   - anchor: $(anchor --version 2>/dev/null)"
  echo "   - rust: $(rustc --version 2>/dev/null)"  
  echo "   - node: $(node --version 2>/dev/null)"
  echo ""
  echo "Solana CLI has emulation issues on ARM64 Mac"
  echo "   Consider using Solana Playground or native installation for validator"
  echo ""
  echo "Container running in development mode"
  echo "   Access with: docker exec -it p2p-anchor-dev bash"
  
  # Keep container running
  while true; do
    sleep 3600
  done
fi