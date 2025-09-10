#!/bin/bash

echo "🚀 Deploying P2P Energy Trading Programs to Engineering Network"
echo "=============================================================="

# Set Engineering Department cluster
export ANCHOR_PROVIDER_URL="http://engineering-validator.campus.local:8899"
export ANCHOR_WALLET="/opt/campus-blockchain/admin/engineering-admin-keypair.json"

# Verify connection to Engineering Department network
echo "🔍 Verifying connection to Engineering Department blockchain..."
solana config set --url $ANCHOR_PROVIDER_URL
solana cluster-version

# Build programs
echo "🔨 Building programs for Engineering Department deployment..."
anchor build --arch x86_64

# Deploy programs to Engineering Department network
echo "📤 Deploying programs to Engineering Department blockchain..."
anchor deploy \
  --program-name registry \
  --program-name energy_token \
  --program-name trading \
  --program-name oracle \
  --program-name governance \
  --provider.cluster $ANCHOR_PROVIDER_URL \
  --provider.wallet $ANCHOR_WALLET

# Initialize programs
echo "⚙️ Initializing programs on Engineering Department network..."
anchor run initialize-production

echo "✅ Engineering Department deployment completed successfully!"
echo ""
echo "📋 Deployed Programs:"
echo "  - Registry: RegEngDeptEnergyP2P1234567890123456789"
echo "  - Energy Token: EnergyTokenEngDept1234567890123456789"  
echo "  - Trading: TradingEngDeptP2P1234567890123456789"
echo "  - Oracle: OracleEngDeptAMI1234567890123456789"
echo "  - Governance: GovernanceEngDeptPoA1234567890123456789"
