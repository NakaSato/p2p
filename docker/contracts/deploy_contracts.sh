#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
NODE_URL="ws://localhost:9944"
HTTP_URL="http://localhost:9933"
DEPLOYER_ACCOUNT="//Alice"
OUTPUT_DIR="/tmp/contract_addresses"

# Ensure output directory exists
mkdir -p $OUTPUT_DIR

echo -e "${GREEN}Starting Substrate Contracts Node...${NC}"
substrate-contracts-node --dev --tmp --ws-external --rpc-external &
NODE_PID=$!

# Enhanced wait for node with retries
echo -e "${YELLOW}Waiting for node to be ready...${NC}"
for i in {1..30}; do
    sleep 2
    if curl -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' $HTTP_URL > /dev/null 2>&1; then
        echo -e "${GREEN}Node is ready after ${i} attempts!${NC}"
        break
    fi
    if [ $i -eq 30 ]; then
        echo -e "${RED}Node failed to start after 60 seconds. Exiting...${NC}"
        kill $NODE_PID 2>/dev/null || true
        exit 1
    fi
    echo "Attempt $i/30: Waiting for node..."
done

echo -e "${GREEN}Node is ready! Starting contract deployment...${NC}"

# Function to deploy contract
deploy_contract() {
    local contract_name=$1
    local constructor_name=$2
    local constructor_args=$3
    
    echo -e "${YELLOW}Deploying $contract_name...${NC}"
    cd /contracts/contracts/$contract_name
    
    # Check if contract build exists
    if [ ! -f "target/ink/$contract_name.contract" ]; then
        echo -e "${RED}Contract build not found! Building $contract_name...${NC}"
        cargo contract build --release
    fi
    
    # First, upload the code
    echo "Uploading contract code..."
    if ! cargo contract upload \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm \
        --output-json > /tmp/${contract_name}_upload.json 2>/tmp/${contract_name}_upload_error.log; then
        echo -e "${RED}Failed to upload $contract_name code${NC}"
        cat /tmp/${contract_name}_upload_error.log
        return 1
    fi
    
    # Extract code hash with better error handling
    CODE_HASH=$(jq -r '.code_hash // empty' /tmp/${contract_name}_upload.json 2>/dev/null)
    if [ -z "$CODE_HASH" ] || [ "$CODE_HASH" = "null" ]; then
        echo -e "${RED}Failed to extract code hash for $contract_name${NC}"
        cat /tmp/${contract_name}_upload.json
        return 1
    fi
    echo "Code hash: $CODE_HASH"
    
    # Instantiate the contract
    echo "Instantiating contract..."
    local instantiate_cmd="cargo contract instantiate --constructor $constructor_name --suri $DEPLOYER_ACCOUNT --url $NODE_URL --skip-confirm --output-json"
    
    if [ -n "$constructor_args" ]; then
        instantiate_cmd="$instantiate_cmd --args $constructor_args"
    fi
    
    if ! eval $instantiate_cmd > /tmp/${contract_name}_deployment.json 2>/tmp/${contract_name}_deploy_error.log; then
        echo -e "${RED}Failed to instantiate $contract_name${NC}"
        cat /tmp/${contract_name}_deploy_error.log
        return 1
    fi
    
    # Extract contract address with better error handling
    CONTRACT_ADDRESS=$(jq -r '.contract // empty' /tmp/${contract_name}_deployment.json 2>/dev/null)
    if [ -z "$CONTRACT_ADDRESS" ] || [ "$CONTRACT_ADDRESS" = "null" ]; then
        echo -e "${RED}Failed to extract contract address for $contract_name${NC}"
        cat /tmp/${contract_name}_deployment.json
        return 1
    fi
    
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
    
    # Check if all contract addresses exist
    for contract in registry grid-token trading oracle-client; do
        if [ ! -f "$OUTPUT_DIR/${contract}_address.txt" ]; then
            echo -e "${RED}Missing contract address file for $contract${NC}"
            return 1
        fi
    done
    
    # Read contract addresses
    REGISTRY_ADDRESS=$(cat $OUTPUT_DIR/registry_address.txt)
    TOKEN_ADDRESS=$(cat $OUTPUT_DIR/grid-token_address.txt)
    TRADING_ADDRESS=$(cat $OUTPUT_DIR/trading_address.txt)
    ORACLE_ADDRESS=$(cat $OUTPUT_DIR/oracle-client_address.txt)
    
    echo "Registry: $REGISTRY_ADDRESS"
    echo "Token: $TOKEN_ADDRESS"
    echo "Trading: $TRADING_ADDRESS"
    echo "Oracle: $ORACLE_ADDRESS"
    
    # Validate addresses are not empty
    for addr in "$REGISTRY_ADDRESS" "$TOKEN_ADDRESS" "$TRADING_ADDRESS" "$ORACLE_ADDRESS"; do
        if [ -z "$addr" ]; then
            echo -e "${RED}One or more contract addresses are empty${NC}"
            return 1
        fi
    done
    
    # Set registry contract in token contract
    echo "Connecting token contract to registry..."
    cd /contracts/contracts/grid-token
    if ! cargo contract call \
        --contract $TOKEN_ADDRESS \
        --message set_registry_contract \
        --args $REGISTRY_ADDRESS \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm > /tmp/token_registry_connection.log 2>&1; then
        echo -e "${RED}Failed to connect token to registry${NC}"
        cat /tmp/token_registry_connection.log
        return 1
    fi
    
    # Set contracts in trading contract
    echo "Connecting trading contract to registry and token..."
    cd /contracts/contracts/trading
    if ! cargo contract call \
        --contract $TRADING_ADDRESS \
        --message set_contracts \
        --args $REGISTRY_ADDRESS $TOKEN_ADDRESS \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm > /tmp/trading_connections.log 2>&1; then
        echo -e "${RED}Failed to connect trading contract${NC}"
        cat /tmp/trading_connections.log
        return 1
    fi
    
    # Set contracts in oracle contract
    echo "Connecting oracle contract to other contracts..."
    cd /contracts/contracts/oracle-client
    if ! cargo contract call \
        --contract $ORACLE_ADDRESS \
        --message set_contracts \
        --args $REGISTRY_ADDRESS $TOKEN_ADDRESS $TRADING_ADDRESS \
        --suri $DEPLOYER_ACCOUNT \
        --url $NODE_URL \
        --skip-confirm > /tmp/oracle_connections.log 2>&1; then
        echo -e "${RED}Failed to connect oracle contract${NC}"
        cat /tmp/oracle_connections.log
        return 1
    fi
    
    echo -e "${GREEN}✅ Contract connections established!${NC}"
}

# Deploy contracts in order
echo -e "${GREEN}=== STARTING CONTRACT DEPLOYMENT ===${NC}"

# Deploy contracts with error handling
deploy_contract "registry" "new" "" || { echo -e "${RED}Registry deployment failed${NC}"; exit 1; }
deploy_contract "grid-token" "new" "1000000000000000000000" || { echo -e "${RED}Grid token deployment failed${NC}"; exit 1; }  # 1000 GRID initial supply
deploy_contract "trading" "new" "900000" || { echo -e "${RED}Trading contract deployment failed${NC}"; exit 1; }  # 15 minutes market epoch
deploy_contract "oracle-client" "new" "" || { echo -e "${RED}Oracle client deployment failed${NC}"; exit 1; }

echo -e "${GREEN}=== ALL CONTRACTS DEPLOYED ===${NC}"

# Setup contract connections
if ! setup_contract_connections; then
    echo -e "${RED}Failed to setup contract connections${NC}"
    exit 1
fi

# Add authorization setup
echo -e "${YELLOW}Setting up contract authorizations...${NC}"

# Add Alice as REC regulator in registry
echo "Adding Alice as REC regulator..."
cd /contracts/contracts/registry
cargo contract call \
    --contract $(cat $OUTPUT_DIR/registry_address.txt) \
    --message add_rec_regulator \
    --args "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
    --suri $DEPLOYER_ACCOUNT \
    --url $NODE_URL \
    --skip-confirm

# Add Alice as minter in grid-token
echo "Adding Alice as minter..."
cd /contracts/contracts/grid-token
cargo contract call \
    --contract $(cat $OUTPUT_DIR/grid-token_address.txt) \
    --message add_minter \
    --args "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
    --suri $DEPLOYER_ACCOUNT \
    --url $NODE_URL \
    --skip-confirm

echo -e "${GREEN}✅ Authorization setup completed!${NC}"

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

# Display deployment summary
echo -e "${GREEN}=== DEPLOYMENT SUMMARY ===${NC}"
cat $OUTPUT_DIR/deployment_summary.json | jq '.'

echo -e "${GREEN}🚀 Substrate node is running at $NODE_URL${NC}"
echo -e "${GREEN}🌐 HTTP RPC endpoint: $HTTP_URL${NC}"
echo -e "${YELLOW}📁 Contract files and addresses: $OUTPUT_DIR${NC}"
echo -e "${YELLOW}Press Ctrl+C to stop the node${NC}"

# Setup signal handler for graceful shutdown
cleanup() {
    echo -e "\n${YELLOW}Shutting down substrate node...${NC}"
    kill $NODE_PID 2>/dev/null || true
    wait $NODE_PID 2>/dev/null || true
    echo -e "${GREEN}Substrate node stopped.${NC}"
    exit 0
}

trap cleanup SIGINT SIGTERM

# Keep the node running
wait $NODE_PID
