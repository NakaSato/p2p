#!/bin/bash

# Anchor Program Deployment Script for P2P Energy Trading Platform

set -e

echo "🚀 Deploying Anchor Programs to Local Solana Validator"
echo "======================================================"

# Check if Anchor is installed
if ! command -v anchor &> /dev/null; then
    echo "❌ Anchor is not installed. Please install Anchor CLI first:"
    echo "   npm install -g @coral-xyz/anchor-cli"
    exit 1
fi

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo "❌ Solana CLI is not installed. Please install Solana CLI first:"
    echo "   sh -c \"\$(curl -sSfL https://release.solana.com/v1.17.0/install)\""
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Anchor.toml" ]; then
    echo "❌ Anchor.toml not found. Please run this script from the project root."
    exit 1
fi

# Check if Solana validator is running
echo "🔍 Checking Solana validator connectivity..."
if ! solana cluster-version --url http://localhost:8899 >/dev/null 2>&1; then
    echo "❌ Solana validator is not accessible at http://localhost:8899"
    echo "   Please start the Docker services first: docker-compose up -d solana-validator"
    exit 1
fi

echo "✅ Solana validator is accessible"

# Set Solana config for local development
echo "⚙️  Configuring Solana for local development..."
solana config set --url http://localhost:8899
solana config set --keypair ~/.config/solana/id.json

# Create keypair if it doesn't exist
if [ ! -f ~/.config/solana/id.json ]; then
    echo "🔑 Creating new Solana keypair..."
    solana-keygen new --no-bip39-passphrase --silent --outfile ~/.config/solana/id.json
fi

# Show current configuration
echo "📋 Current Solana configuration:"
solana config get

# Airdrop SOL for deployment
echo "💰 Requesting SOL for deployment..."
solana airdrop 10 --url http://localhost:8899 || echo "⚠️  Airdrop may have failed, but continuing..."

# Show balance
echo "💳 Current SOL balance:"
solana balance --url http://localhost:8899

# Build programs
echo "🏗️  Building Anchor programs..."
anchor build

# Deploy programs
echo "🚀 Deploying programs..."
anchor deploy --provider.cluster localnet

# Get program IDs
echo "📝 Extracting program IDs..."

# Create program IDs file
cat > deployed_programs.json << 'EOF'
{
  "localnet": {
EOF

# Extract program IDs from Anchor.toml and target/deploy
programs_dir="target/deploy"
if [ -d "$programs_dir" ]; then
    first=true
    for keypair in "$programs_dir"/*.json; do
        if [ -f "$keypair" ]; then
            program_name=$(basename "$keypair" -keypair.json)
            program_id=$(solana-keygen pubkey "$keypair")
            
            if [ "$first" = true ]; then
                first=false
            else
                echo "," >> deployed_programs.json
            fi
            
            echo "    \"$program_name\": \"$program_id\"" >> deployed_programs.json
            echo "✅ $program_name: $program_id"
        fi
    done
fi

cat >> deployed_programs.json << 'EOF'
  }
}
EOF

echo ""
echo "📄 Program IDs saved to deployed_programs.json"

# Update environment file with program IDs
if [ -f ".env" ]; then
    echo "🔧 Updating .env file with program IDs..."
    
    # Read program IDs and update .env
    if command -v jq &> /dev/null; then
        energy_token_id=$(jq -r '.localnet["energy_token"] // empty' deployed_programs.json)
        governance_id=$(jq -r '.localnet["governance"] // empty' deployed_programs.json)
        oracle_id=$(jq -r '.localnet["oracle"] // empty' deployed_programs.json)
        registry_id=$(jq -r '.localnet["registry"] // empty' deployed_programs.json)
        trading_id=$(jq -r '.localnet["trading"] // empty' deployed_programs.json)
        
        # Update .env file
        sed -i.bak "s/ENERGY_TOKEN_PROGRAM_ID=.*/ENERGY_TOKEN_PROGRAM_ID=\"$energy_token_id\"/" .env
        sed -i.bak "s/GOVERNANCE_PROGRAM_ID=.*/GOVERNANCE_PROGRAM_ID=\"$governance_id\"/" .env
        sed -i.bak "s/ORACLE_PROGRAM_ID=.*/ORACLE_PROGRAM_ID=\"$oracle_id\"/" .env
        sed -i.bak "s/REGISTRY_PROGRAM_ID=.*/REGISTRY_PROGRAM_ID=\"$registry_id\"/" .env
        sed -i.bak "s/TRADING_PROGRAM_ID=.*/TRADING_PROGRAM_ID=\"$trading_id\"/" .env
        
        echo "✅ Environment file updated with program IDs"
    else
        echo "⚠️  jq not found. Please manually update .env with program IDs from deployed_programs.json"
    fi
else
    echo "⚠️  .env file not found. Please create one or run setup-dev.sh first"
fi

# Test deployment with a simple RPC call
echo ""
echo "🧪 Testing program deployment..."
for keypair in target/deploy/*.json; do
    if [ -f "$keypair" ]; then
        program_name=$(basename "$keypair" -keypair.json)
        program_id=$(solana-keygen pubkey "$keypair")
        
        if solana account "$program_id" --url http://localhost:8899 >/dev/null 2>&1; then
            echo "✅ $program_name ($program_id) is deployed and accessible"
        else
            echo "❌ $program_name ($program_id) deployment verification failed"
        fi
    fi
done

echo ""
echo "🎉 Anchor program deployment completed!"
echo ""
echo "📋 Next steps:"
echo "  1. Restart API Gateway to pick up new program IDs: docker-compose restart api-gateway"
echo "  2. Test program interactions through the API"
echo "  3. Initialize program accounts if needed"
echo "  4. Run integration tests: npm test"
echo ""
echo "🔍 Useful commands:"
echo "  - View program logs: solana logs --url http://localhost:8899"
echo "  - Check program account: solana account <PROGRAM_ID> --url http://localhost:8899"
echo "  - Anchor test: anchor test --skip-local-validator"
echo ""
echo "📁 Program IDs are saved in:"
echo "  - deployed_programs.json (structured)"
echo "  - .env (environment variables)"