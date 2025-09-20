# Postman API Testing Guide

## Overview

This directory contains comprehensive Postman testing collections for the P2P Energy Trading API Gateway. The testing suite covers all major API endpoints including authentication, energy meter data, trading operations, blockchain integration, and analytics.

## Files

- `P2P_Energy_Trading_API.postman_collection.json` - Main API testing collection
- `P2P_Energy_Trading_Local.postman_environment.json` - Local development environment
- `P2P_Energy_Trading_Production.postman_environment.json` - Production environment
- `README.md` - This guide
- `run-tests.sh` - Automated test runner script

## Quick Start

### 1. Import into Postman

1. Open Postman
2. Click "Import" in the top left
3. Import all three files:
   - Collection: `P2P_Energy_Trading_API.postman_collection.json`
   - Environments: Both environment files

### 2. Set Up Environment

1. Select the appropriate environment (Local Development or Production)
2. Set the `test_password` variable if using production
3. Verify the `base_url` points to your API Gateway

### 3. Run Tests

#### Manual Testing
1. Start with the "Health Check" folder
2. Run "Register New User" to create a test account
3. Run "User Login" to get a JWT token
4. Continue with other test categories

#### Automated Testing
Use the provided shell script:
```bash
./run-tests.sh
```

## Test Categories

### 1. Health Check
- **Purpose**: Verify API Gateway is running
- **Tests**: Basic connectivity and response format
- **No authentication required**

### 2. Authentication
- **Purpose**: Test user management and JWT authentication
- **Tests**:
  - User registration with validation
  - Login and JWT token generation
  - Profile retrieval and updates
  - Password changes
- **Automated**: Username/email generation, token storage

### 3. Energy Meters
- **Purpose**: Test energy data submission and retrieval
- **Tests**:
  - Submit meter readings with realistic data
  - Retrieve readings with pagination
  - Get specific readings by ID
  - Aggregated data queries
- **Automated**: Realistic energy data generation

### 4. Trading
- **Purpose**: Test energy trading operations
- **Tests**:
  - Create buy/sell orders
  - Retrieve user orders with filtering
  - Market data and statistics
  - Trading analytics
- **Automated**: Order ID storage for follow-up tests

### 5. Blockchain
- **Purpose**: Test blockchain and smart contract integration
- **Tests**:
  - Wallet connection and verification
  - Transaction history retrieval
  - Smart contract interactions
- **Automated**: Test wallet address generation

### 6. Analytics
- **Purpose**: Test reporting and analytics endpoints
- **Tests**:
  - Energy consumption/production analytics
  - Trading performance metrics
  - Time-series data with various granularities

### 7. Negative Test Cases
- **Purpose**: Verify proper error handling
- **Tests**:
  - Unauthorized access attempts
  - Invalid credentials
  - Malformed requests
- **Expected**: Proper HTTP status codes and error messages

## Environment Variables

### Automatic Variables
These are set automatically by the test scripts:
- `jwt_token` - JWT authentication token
- `user_id` - Current user ID
- `test_username` - Generated test username
- `test_email` - Generated test email
- `reading_id` - Energy reading ID
- `buy_order_id`, `sell_order_id` - Trading order IDs
- `test_wallet_address` - Test blockchain wallet

### Manual Variables
Set these manually in your environment:
- `base_url` - API Gateway URL
- `test_password` - Password for test accounts

## Test Flow

The collection is designed to be run in sequence:

1. **Health Check** → Verify API availability
2. **Authentication** → Create account and get token
3. **Energy Meters** → Submit and retrieve energy data
4. **Trading** → Create orders and check market data
5. **Blockchain** → Connect wallet and interact with contracts
6. **Analytics** → Generate reports on energy and trading data
7. **Negative Tests** → Verify error handling

## Automated Testing

### Using Newman (Command Line)

Install Newman:
```bash
npm install -g newman
```

Run the collection:
```bash
newman run P2P_Energy_Trading_API.postman_collection.json \
  -e P2P_Energy_Trading_Local.postman_environment.json \
  --reporters cli,html \
  --reporter-html-export test-results.html
```

### Using the Test Runner Script

The `run-tests.sh` script provides automated testing with environment setup:

```bash
# Make script executable
chmod +x run-tests.sh

# Run with local environment
./run-tests.sh local

# Run with production environment
./run-tests.sh production

# Run with custom environment
./run-tests.sh custom ./path/to/custom-environment.json
```

## Test Assertions

Each request includes comprehensive test assertions:

### Standard Assertions
- HTTP status code validation
- Response time performance checks
- Required field presence validation
- Data type and format verification

### Authentication Assertions
- JWT token format and expiration
- User data consistency
- Permission-based access control

### Business Logic Assertions
- Energy data calculations
- Trading order validation
- Blockchain transaction verification
- Analytics computation accuracy

## Performance Testing

The collection includes basic performance assertions:
- Response time limits (< 500ms for health checks)
- Concurrent request handling
- Database query optimization verification

For comprehensive performance testing, consider:
- Running with increased iterations
- Load testing with Newman + Artillery
- Monitoring resource usage during tests

## CI/CD Integration

### GitHub Actions Example

```yaml
name: API Testing
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Newman
        run: npm install -g newman
      - name: Run API Tests
        run: |
          cd docs/postman
          newman run P2P_Energy_Trading_API.postman_collection.json \
            -e P2P_Energy_Trading_Local.postman_environment.json \
            --bail
```

### Jenkins Pipeline Example

```groovy
pipeline {
    agent any
    stages {
        stage('API Testing') {
            steps {
                sh '''
                    cd docs/postman
                    newman run P2P_Energy_Trading_API.postman_collection.json \
                      -e P2P_Energy_Trading_Local.postman_environment.json \
                      --reporters junit,cli \
                      --reporter-junit-export results.xml
                '''
                publishTestResults testResultsPattern: 'docs/postman/results.xml'
            }
        }
    }
}
```

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Verify API Gateway is running
   - Check `base_url` in environment
   - Confirm firewall/network access

2. **Authentication Failures**
   - Ensure user registration succeeded
   - Check JWT token is not expired
   - Verify token is properly formatted

3. **Data Validation Errors**
   - Check request body format
   - Verify required fields are present
   - Ensure data types match schema

4. **Test Script Errors**
   - Review Postman console for JavaScript errors
   - Check environment variable availability
   - Verify response format matches expectations

### Debug Mode

Enable verbose logging in environment:
```json
{
  "key": "debug_mode",
  "value": "true"
}
```

This will output detailed request/response information in the Postman console.

## Contributing

When adding new tests:

1. Follow existing naming conventions
2. Include comprehensive test assertions
3. Add proper error handling tests
4. Update environment variables as needed
5. Document new test categories in this guide

## Security Considerations

- Never commit real credentials to version control
- Use environment-specific variables for sensitive data
- Regularly rotate test account credentials
- Monitor test account usage in production

## Support

For issues or questions:
- Check the API Gateway logs
- Review the Postman console output
- Consult the main project documentation
- Open an issue in the project repository