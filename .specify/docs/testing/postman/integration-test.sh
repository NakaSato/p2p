#!/bin/bash

# CI/CD Integration Test Script for P2P Energy Trading API
# This script runs basic API health checks and core functionality tests

set -e

# Configuration
API_BASE_URL="${API_BASE_URL:-http://localhost:8080}"
MAX_RETRIES=30
RETRY_INTERVAL=2

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${YELLOW}[TEST]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Wait for API to be ready
wait_for_api() {
    print_status "Waiting for API Gateway to be ready at $API_BASE_URL..."
    
    for i in $(seq 1 $MAX_RETRIES); do
        if curl -s -f "$API_BASE_URL/health" > /dev/null 2>&1; then
            print_success "API Gateway is ready!"
            return 0
        fi
        
        echo "Attempt $i/$MAX_RETRIES failed, retrying in ${RETRY_INTERVAL}s..."
        sleep $RETRY_INTERVAL
    done
    
    print_error "API Gateway failed to start within timeout"
    return 1
}

# Test API health endpoint
test_health_endpoint() {
    print_status "Testing health endpoint..."
    
    response=$(curl -s -w "%{http_code}" "$API_BASE_URL/health")
    http_code="${response: -3}"
    
    if [[ "$http_code" == "200" ]]; then
        print_success "Health check passed (HTTP $http_code)"
        return 0
    else
        print_error "Health check failed (HTTP $http_code)"
        return 1
    fi
}

# Test user registration
test_user_registration() {
    print_status "Testing user registration..."
    
    # Generate unique test data
    timestamp=$(date +%s)
    test_user="testuser_$timestamp"
    test_email="test_$timestamp@engineering.edu"
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE_URL/auth/register" \
        -H "Content-Type: application/json" \
        -d "{
            \"username\": \"$test_user\",
            \"email\": \"$test_email\",
            \"password\": \"SecureTest123!\",
            \"first_name\": \"Test\",
            \"last_name\": \"User\",
            \"role\": \"student\",
            \"department\": \"Computer Engineering\"
        }")
    
    http_code="${response: -3}"
    
    if [[ "$http_code" == "200" ]]; then
        print_success "User registration passed (HTTP $http_code)"
        # Store user for login test
        echo "$test_user" > /tmp/test_username
        return 0
    else
        print_error "User registration failed (HTTP $http_code)"
        echo "Response: ${response%???}"
        return 1
    fi
}

# Test user login
test_user_login() {
    print_status "Testing user login..."
    
    if [[ ! -f /tmp/test_username ]]; then
        print_error "No test user available for login test"
        return 1
    fi
    
    test_user=$(cat /tmp/test_username)
    
    response=$(curl -s -w "%{http_code}" -X POST "$API_BASE_URL/auth/login" \
        -H "Content-Type: application/json" \
        -d "{
            \"username\": \"$test_user\",
            \"password\": \"SecureTest123!\"
        }")
    
    http_code="${response: -3}"
    
    if [[ "$http_code" == "200" ]]; then
        print_success "User login passed (HTTP $http_code)"
        
        # Extract and store JWT token for further tests
        jwt_token=$(echo "${response%???}" | jq -r '.access_token' 2>/dev/null || echo "")
        if [[ -n "$jwt_token" && "$jwt_token" != "null" ]]; then
            echo "$jwt_token" > /tmp/jwt_token
            print_success "JWT token extracted successfully"
        else
            print_error "Failed to extract JWT token from response"
            return 1
        fi
        
        return 0
    else
        print_error "User login failed (HTTP $http_code)"
        echo "Response: ${response%???}"
        return 1
    fi
}

# Test authenticated endpoint
test_authenticated_endpoint() {
    print_status "Testing authenticated endpoint..."
    
    if [[ ! -f /tmp/jwt_token ]]; then
        print_error "No JWT token available for authenticated test"
        return 1
    fi
    
    jwt_token=$(cat /tmp/jwt_token)
    
    response=$(curl -s -w "%{http_code}" -X GET "$API_BASE_URL/auth/profile" \
        -H "Authorization: Bearer $jwt_token")
    
    http_code="${response: -3}"
    
    if [[ "$http_code" == "200" ]]; then
        print_success "Authenticated endpoint test passed (HTTP $http_code)"
        return 0
    else
        print_error "Authenticated endpoint test failed (HTTP $http_code)"
        echo "Response: ${response%???}"
        return 1
    fi
}

# Test error handling
test_error_handling() {
    print_status "Testing error handling (unauthorized access)..."
    
    response=$(curl -s -w "%{http_code}" -X GET "$API_BASE_URL/auth/profile")
    http_code="${response: -3}"
    
    if [[ "$http_code" == "401" ]]; then
        print_success "Error handling test passed (HTTP $http_code)"
        return 0
    else
        print_error "Error handling test failed - expected 401, got $http_code"
        return 1
    fi
}

# Cleanup temporary files
cleanup() {
    rm -f /tmp/test_username /tmp/jwt_token
}

# Main test execution
main() {
    echo "===========================================" 
    echo "P2P Energy Trading API Integration Tests"
    echo "API Base URL: $API_BASE_URL"
    echo "==========================================="
    echo
    
    # Ensure cleanup on exit
    trap cleanup EXIT
    
    # Run tests
    wait_for_api || exit 1
    echo
    
    test_health_endpoint || exit 1
    echo
    
    test_user_registration || exit 1
    echo
    
    test_user_login || exit 1
    echo
    
    test_authenticated_endpoint || exit 1
    echo
    
    test_error_handling || exit 1
    echo
    
    print_success "All integration tests passed!"
    echo
    echo "Next steps:"
    echo "- Run full Postman test suite: ./run-tests.sh"
    echo "- Check API Gateway logs for any warnings"
    echo "- Verify database connections and data persistence"
    
    return 0
}

# Run main function
main "$@"