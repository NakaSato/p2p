#!/bin/bash

# P2P Energy Trading Local Network Explorer
# This script provides comprehensive local Solana network exploration

echo "🔍 P2P Energy Trading System - Local Network Explorer"
echo "=================================================="

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're connected to local network
RPC_URL=$(solana config get | grep "RPC URL" | awk '{print $3}')
if [[ "$RPC_URL" != "http://localhost:8899" ]]; then
    echo "⚠️  Warning: Not connected to local network. Current RPC: $RPC_URL"
    echo "To connect to local network: solana config set --url localhost"
    exit 1
fi

echo -e "${GREEN}✅ Connected to local Solana network${NC}"
echo

# 1. Cluster Information
echo -e "${BLUE}📊 Cluster Information${NC}"
echo "Solana Version: $(solana cluster-version)"
echo "Current Slot: $(solana slot)"
echo "RPC URL: $RPC_URL"
echo

# 2. Validator Status
echo -e "${BLUE}⚡ Validator Status${NC}"
solana validators --output json-compact | head -5
echo

# 3. Your Wallet Info
echo -e "${BLUE}💰 Your Wallet${NC}"
WALLET_ADDRESS=$(solana address)
echo "Address: $WALLET_ADDRESS"
echo "Balance: $(solana balance)"
echo

# 4. P2P Energy Trading Programs
echo -e "${BLUE}📋 P2P Energy Trading Programs${NC}"

PROGRAMS=(
    "EtmU16tPPrGZVdyd9g5zABnq8wMt9UWYNGY4uZVdpQHK:Registry"
    "HaMSXq2FPjjCVC4EaAtoSmtykoFQbX7g6cXxrbrRcDpn:Energy Token"
    "BDcRY7tRjCWWDbS3DHMje8MWgJ5G84kL19C3NjqBUwph:Trading"
    "2R68FVjvq6oRtpzJBq4Mxsw165wCL6wbFRSxzAqNkJro:Oracle"
    "D5qmDv77pmtebp3MM78HienoXWMfSa8JFzxw1Sj2rNQc:Governance"
)

for program in "${PROGRAMS[@]}"; do
    IFS=':' read -r address name <<< "$program"
    echo -e "${YELLOW}$name Program${NC} ($address)"
    solana program show $address | grep -E "(Owner|Authority|Last Deployed|Balance)" | sed 's/^/  /'
    echo
done

# 5. Recent Epoch Info
echo -e "${BLUE}📅 Epoch Information${NC}"
solana epoch-info | head -8
echo

# 6. Functions for interactive exploration
echo -e "${BLUE}🎯 Available Functions${NC}"
echo "To airdrop SOL:"
echo "  solana airdrop 10                     # 10 SOL to your wallet"
echo "  solana airdrop 5 <PUBLIC_KEY>         # 5 SOL to specific address"
echo
echo "To explore accounts:"
echo "  solana account <PUBLIC_KEY>           # Get account details"
echo "  solana balance <PUBLIC_KEY>           # Check balance"
echo
echo "To explore programs:"
echo "  solana program show <PROGRAM_ID>      # Program information"
echo "  solana program dump <PROGRAM_ID> file # Download program"
echo
echo "To check transactions:"
echo "  solana confirm <SIGNATURE>            # Transaction details"
echo "  solana transaction-history <ADDRESS>  # Recent transactions"
echo

# 7. Test Account Creation Helper
echo -e "${BLUE}🧪 Test Account Creation${NC}"
echo "To create test accounts for University users:"
echo
echo "# Create Engineering Department test accounts"
echo "mkdir -p test-accounts"
echo "cd test-accounts"
echo
echo "# Generate keypairs"
echo "solana-keygen new --outfile student.json --no-bip39-passphrase"
echo "solana-keygen new --outfile faculty.json --no-bip39-passphrase"
echo "solana-keygen new --outfile validator.json --no-bip39-passphrase"
echo
echo "# Fund accounts"
echo "solana airdrop 10 \$(solana-keygen pubkey student.json)"
echo "solana airdrop 10 \$(solana-keygen pubkey faculty.json)"
echo "solana airdrop 15 \$(solana-keygen pubkey validator.json)"
echo

# 8. AMI Meter Simulation
echo -e "${BLUE}🏭 AMI Meter Simulation${NC}"
echo "To create test accounts for 15 AMI meters (ENG_001-015):"
echo
echo "mkdir -p test-meters"
echo "for i in {1..15}; do"
echo "  meter_id=\$(printf \"ENG_%03d\" \$i)"
echo "  solana-keygen new --outfile \"./test-meters/\${meter_id}.json\" --no-bip39-passphrase"
echo "  pubkey=\$(solana-keygen pubkey \"./test-meters/\${meter_id}.json\")"
echo "  solana airdrop 5 \"\$pubkey\""
echo "  echo \"Meter \$meter_id: \$pubkey funded\""
echo "done"
echo

# 9. Web Explorer Links
echo -e "${BLUE}🌐 Web Explorers${NC}"
echo "Solana Explorer (Local): https://explorer.solana.com/?cluster=custom&customUrl=http://localhost:8899"
echo
echo "To explore your programs and accounts:"
echo "- Registry Program: https://explorer.solana.com/address/EtmU16tPPrGZVdyd9g5zABnq8wMt9UWYNGY4uZVdpQHK?cluster=custom&customUrl=http://localhost:8899"
echo "- Your Wallet: https://explorer.solana.com/address/$WALLET_ADDRESS?cluster=custom&customUrl=http://localhost:8899"
echo

echo "🎉 Local network exploration complete!"
echo "Run this script anytime to check your local Solana network status."