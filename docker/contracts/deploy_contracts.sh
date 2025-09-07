#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
NODE_URL="ws://localhost:9944"
DEPLOYER_ACCOUNT="//Alice"
OUTPUT_DIR="/tmp/contract_addresses"

mkdir -p $OUTPUT_DIR

echo -e "${GREEN}Starting Substrate Contracts Node...${NC}"
substrate-contracts-node --dev --tmp &
NODE_PID=$!

# Wait for node to start
echo -e "${YELLOW}Waiting for node to be ready...${NC}"
sleep 15

# Check if node is running
if ! curl -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' http://localhost:9933 > /dev/null; then
    echo -e "${RED}Node is not responding. Exiting...${NC}"
    exit 1
fi

echo -e "${GREEN}Node is ready! Starting contract deployment...${NC}"

# Function to deploy contract
deploy_contract() {
    local contract_name=$1
    local constructor_name=$2
    local constructor_args=$3
    
    echo -e "${YELLOW}Deploying $contract_name...${NC}"
    cd /contracts/contracts/$contract_name
    
    # First, upload the code
    echo "Uploading contract code..."
    cargo contract upload \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm \
        --output-json > /tmp/${contract_name}_upload.json
    
    # Extract code hash
    CODE_HASH=$(cat /tmp/${contract_name}_upload.json | grep -o '"code_hash":"[^"]*"' | cut -d'"' -f4)
    echo "Code hash: $CODE_HASH"
    
    # Instantiate the contract
    echo "Instantiating contract..."
    cargo contract instantiate \
        --constructor $constructor_name \
        --args "$constructor_args" \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm \
        --output-json > /tmp/${contract_name}_deployment.json
    
    # Extract contract address
    CONTRACT_ADDRESS=$(cat /tmp/${contract_name}_deployment.json | grep -o '"contract":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$CONTRACT_ADDRESS" ]; then
        echo -e "${GREEN}✅ $contract_name deployed successfully!${NC}"
        echo "Contract Address: $CONTRACT_ADDRESS"
        echo "Code Hash: $CODE_HASH"
        
        # Save to file
        echo "$CONTRACT_ADDRESS" > $OUTPUT_DIR/${contract_name}_address.txt
        echo "$CODE_HASH" > $OUTPUT_DIR/${contract_name}_code_hash.txt
        
        # Create deployment info JSON
        cat > $OUTPUT_DIR/${contract_name}_info.json << EOF
{
  "contract_name": "$contract_name",
  "contract_address": "$CONTRACT_ADDRESS",
  "code_hash": "$CODE_HASH",
  "constructor": "$constructor_name",
  "constructor_args": "$constructor_args",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "deployer": "$DEPLOYER_ACCOUNT"
}
EOF
    else
        echo -e "${RED}❌ Failed to deploy $contract_name${NC}"
        cat /tmp/${contract_name}_deployment.json
        return 1
    fi
    
    echo "---"
}

# Function to setup contract connections
setup_contract_connections() {
    echo -e "${YELLOW}Setting up contract connections...${NC}"
    
    # Read contract addresses
    REGISTRY_ADDRESS=$(cat $OUTPUT_DIR/registry_address.txt)
    TOKEN_ADDRESS=$(cat $OUTPUT_DIR/grid-token_address.txt)
    TRADING_ADDRESS=$(cat $OUTPUT_DIR/trading_address.txt)
    ORACLE_ADDRESS=$(cat $OUTPUT_DIR/oracle-client_address.txt)
    
    echo "Registry: $REGISTRY_ADDRESS"
    echo "Token: $TOKEN_ADDRESS"
    echo "Trading: $TRADING_ADDRESS"
    echo "Oracle: $ORACLE_ADDRESS"
    
    # Set registry contract in token contract
    echo "Connecting token contract to registry..."
    cd /contracts/contracts/grid-token
    cargo contract call \
        --contract $TOKEN_ADDRESS \
        --message set_registry_contract \
        --args $REGISTRY_ADDRESS \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm
    
    # Set contracts in trading contract
    echo "Connecting trading contract to registry and token..."
    cd /contracts/contracts/trading
    cargo contract call \
        --contract $TRADING_ADDRESS \
        --message set_contracts \
        --args $REGISTRY_ADDRESS $TOKEN_ADDRESS \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm
    
    # Set contracts in oracle contract
    echo "Connecting oracle contract to other contracts..."
    cd /contracts/contracts/oracle-client
    cargo contract call \
        --contract $ORACLE_ADDRESS \
        --message set_contracts \
        --args $REGISTRY_ADDRESS $TOKEN_ADDRESS $TRADING_ADDRESS \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm
    
    echo -e "${GREEN}✅ Contract connections established!${NC}"
}

# Deploy contracts in order
echo -e "${GREEN}=== STARTING CONTRACT DEPLOYMENT ===${NC}"

deploy_contract "registry" "new" ""
deploy_contract "grid-token" "new" "1000000000000000000000"  # 1000 GRID initial supply
deploy_contract "trading" "new" "900000"  # 15 minutes market epoch
deploy_contract "oracle-client" "new" ""

echo -e "${GREEN}=== ALL CONTRACTS DEPLOYED ===${NC}"

# Setup contract connections
setup_contract_connections

# Create summary file
cat > $OUTPUT_DIR/deployment_summary.json << EOF
{
  "deployment_summary": {
    "network": "substrate-contracts-node (dev)",
    "deployer": "$DEPLOYER_ACCOUNT",
    "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "node_url": "$NODE_URL",
    "contracts": {
      "registry": {
        "address": "$(cat $OUTPUT_DIR/registry_address.txt)",
        "code_hash": "$(cat $OUTPUT_DIR/registry_code_hash.txt)"
      },
      "grid_token": {
        "address": "$(cat $OUTPUT_DIR/grid-token_address.txt)",
        "code_hash": "$(cat $OUTPUT_DIR/grid-token_code_hash.txt)"
      },
      "trading": {
        "address": "$(cat $OUTPUT_DIR/trading_address.txt)",
        "code_hash": "$(cat $OUTPUT_DIR/trading_code_hash.txt)"
      },
      "oracle_client": {
        "address": "$(cat $OUTPUT_DIR/oracle-client_address.txt)",
        "code_hash": "$(cat $OUTPUT_DIR/oracle-client_code_hash.txt)"
      }
    }
  }
}
EOF

echo -e "${GREEN}✅ Deployment completed successfully!${NC}"
echo -e "${YELLOW}Contract addresses and deployment info saved to: $OUTPUT_DIR${NC}"
cat $OUTPUT_DIR/deployment_summary.json

echo -e "${GREEN}🚀 Substrate node is running at $NODE_URL${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop the node${NC}"

# Keep the node running
wait $NODE_PID
