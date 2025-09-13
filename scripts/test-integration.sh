#!/bin/bash

# Integration Testing Script for P2P Energy Trading Platform

set -e

echo "🧪 P2P Energy Trading Platform - Integration Tests"
echo "================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
run_test() {
    local test_name="$1"
    local test_command="$2"
    
    echo -e "\n${BLUE}🔍 Testing: $test_name${NC}"
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if eval "$test_command" >/dev/null 2>&1; then
        echo -e "${GREEN}✅ PASS: $test_name${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}❌ FAIL: $test_name${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

# Check if Docker is running
if ! docker info >/dev/null 2>&1; then
    echo -e "${RED}❌ Docker is not running. Please start Docker Desktop.${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Docker is running${NC}"

# Test 1: Docker services health
echo -e "\n${YELLOW}📦 Testing Docker Services${NC}"

run_test "PostgreSQL Container" "docker-compose ps postgres | grep -q 'Up'"
run_test "Redis Container" "docker-compose ps redis | grep -q 'Up'"
run_test "Kafka Container" "docker-compose ps kafka | grep -q 'Up'"
run_test "Solana Validator Container" "docker-compose ps solana-validator | grep -q 'Up'"

# Test 2: Service connectivity
echo -e "\n${YELLOW}🌐 Testing Service Connectivity${NC}"

run_test "PostgreSQL Connection" "docker-compose exec -T postgres pg_isready -U p2p_user -d p2p_energy_trading"
run_test "Redis Connection" "docker-compose exec -T redis redis-cli ping | grep -q PONG"
run_test "Solana RPC Health" "curl -s -X POST -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getHealth\"}' http://localhost:8899 | grep -q ok"

# Test 3: API Gateway (if running)
echo -e "\n${YELLOW}🚀 Testing API Gateway${NC}"

if docker-compose ps api-gateway | grep -q 'Up'; then
    run_test "API Gateway Health" "curl -s http://localhost:3001/health"
    run_test "API Gateway Metrics" "curl -s http://localhost:3001/metrics"
else
    echo -e "${YELLOW}⚠️  API Gateway not running, skipping API tests${NC}"
fi

# Test 4: Solana validator functionality
echo -e "\n${YELLOW}⛓️  Testing Solana Validator${NC}"

run_test "Solana Cluster Info" "curl -s -X POST -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getClusterNodes\"}' http://localhost:8899 | grep -q result"
run_test "Solana Slot" "curl -s -X POST -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getSlot\"}' http://localhost:8899 | grep -q result"

# Test 5: Monitoring endpoints
echo -e "\n${YELLOW}📊 Testing Monitoring Services${NC}"

if docker-compose ps grafana | grep -q 'Up'; then
    run_test "Grafana API" "curl -s http://localhost:3000/api/health"
else
    echo -e "${YELLOW}⚠️  Grafana not running, skipping Grafana tests${NC}"
fi

if docker-compose ps prometheus | grep -q 'Up'; then
    run_test "Prometheus API" "curl -s http://localhost:9090/-/healthy"
    run_test "Prometheus Targets" "curl -s http://localhost:9090/api/v1/targets"
else
    echo -e "${YELLOW}⚠️  Prometheus not running, skipping Prometheus tests${NC}"
fi

# Test 6: Program deployment (if programs exist)
echo -e "\n${YELLOW}📄 Testing Anchor Programs${NC}"

if [ -f "deployed_programs.json" ]; then
    # Check if jq is available for parsing JSON
    if command -v jq &> /dev/null; then
        # Test each deployed program
        while IFS= read -r program_id; do
            if [ -n "$program_id" ] && [ "$program_id" != "null" ]; then
                program_name=$(echo "$program_id" | cut -d: -f1 | tr -d '"')
                program_address=$(echo "$program_id" | cut -d: -f2 | tr -d '" ')
                run_test "Program $program_name Account" "solana account $program_address --url http://localhost:8899"
            fi
        done < <(jq -r '.localnet | to_entries[] | "\(.key):\(.value)"' deployed_programs.json)
    else
        echo -e "${YELLOW}⚠️  jq not available, skipping program verification${NC}"
    fi
else
    echo -e "${YELLOW}⚠️  No deployed_programs.json found, skipping program tests${NC}"
fi

# Test 7: Volume mounts and data persistence
echo -e "\n${YELLOW}💾 Testing Data Persistence${NC}"

run_test "PostgreSQL Data Volume" "docker volume inspect p2p_postgres_data"
run_test "Solana Ledger Volume" "docker volume inspect p2p_solana_ledger"
run_test "Program Deployments Volume" "docker volume inspect p2p_program_deployments"

# Test 8: Log accessibility
echo -e "\n${YELLOW}📝 Testing Log Access${NC}"

run_test "Solana Validator Logs" "docker-compose logs --tail=1 solana-validator"
if docker-compose ps api-gateway | grep -q 'Up'; then
    run_test "API Gateway Logs" "docker-compose logs --tail=1 api-gateway"
fi

# Test 9: Environment configuration
echo -e "\n${YELLOW}⚙️  Testing Environment Configuration${NC}"

if [ -f ".env" ]; then
    run_test "Environment File Exists" "test -f .env"
    run_test "Solana RPC URL Set" "grep -q 'SOLANA_RPC_URL' .env"
    run_test "Database URL Set" "grep -q 'DATABASE_URL' .env"
else
    echo -e "${YELLOW}⚠️  No .env file found, skipping environment tests${NC}"
fi

# Summary
echo -e "\n${BLUE}📊 Test Summary${NC}"
echo "==============="
echo -e "Tests Run:    $TESTS_RUN"
echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All tests passed! The system is ready for development.${NC}"
    exit 0
else
    echo -e "\n${RED}❌ Some tests failed. Please check the services and try again.${NC}"
    echo -e "\n${YELLOW}🔧 Troubleshooting tips:${NC}"
    echo "1. Restart services: docker-compose restart"
    echo "2. Check logs: docker-compose logs [service-name]"
    echo "3. Rebuild images: docker-compose build"
    echo "4. Clean restart: docker-compose down && docker-compose up -d"
    exit 1
fi