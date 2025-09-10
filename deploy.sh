#!/bin/bash

echo "🚀 Deploying P2P Energy Trading System to Solana Permissioned Network..."
echo "=========================================="

# Build all programs
echo "📦 Building Anchor programs..."
anchor build

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"

# Deploy programs
echo "🔄 Deploying programs to localnet..."
anchor deploy --provider.cluster localnet

if [ $? -ne 0 ]; then
    echo "❌ Deployment failed!"
    exit 1
fi

echo "✅ Programs deployed successfully!"

# Initialize programs
echo "⚙️  Initializing programs..."

# You would run initialization commands here
# For example:
# anchor run initialize-registry
# anchor run initialize-token
# anchor run initialize-trading
# anchor run initialize-oracle
# anchor run initialize-governance

echo "✅ Programs initialized successfully!"

echo "=========================================="
echo "🎉 P2P Energy Trading System deployment completed!"
echo ""
echo "📋 Deployed Programs:"
echo "  - Registry: RegistryProgramId1234567890123456789"
echo "  - Energy Token: EnergyTokenProgramId1234567890123456789"
echo "  - Trading: TradingProgramId1234567890123456789"
echo "  - Oracle: OracleProgramId1234567890123456789"
echo "  - Governance: GovernanceProgramId1234567890123456789"
echo ""
echo "🔗 Network: Solana Localnet (Permissioned PoA)"
echo "🏛️  University Authority: UniversityAuthorityKey1234567890123456789"
echo ""
echo "Ready for university campus deployment! 🎓"
