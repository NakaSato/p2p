# Makefile for P2P Energy Trading API Testing

.PHONY: help test test-unit test-integration test-postman test-performance clean setup deps

# Default target
help:
	@echo "P2P Energy Trading API - Test Commands"
	@echo "======================================"
	@echo ""
	@echo "Available targets:"
	@echo "  help              Show this help message"
	@echo "  setup             Set up development environment"
	@echo "  deps              Install dependencies"
	@echo "  test              Run all tests"
	@echo "  test-unit         Run Rust unit tests"
	@echo "  test-integration  Run integration tests"
	@echo "  test-postman      Run Postman API tests"
	@echo "  test-performance  Run performance tests"
	@echo "  start-services    Start required services (PostgreSQL, Redis)"
	@echo "  stop-services     Stop services"
	@echo "  start-api         Start API Gateway"
	@echo "  stop-api          Stop API Gateway"
	@echo "  clean             Clean build artifacts and test results"
	@echo ""
	@echo "Environment Variables:"
	@echo "  API_BASE_URL      API Gateway URL (default: http://localhost:8080)"
	@echo "  TEST_ENV          Test environment: local|production (default: local)"
	@echo ""

# Variables
API_BASE_URL ?= http://localhost:8080
TEST_ENV ?= local
POSTMAN_DIR = docs/postman
API_GATEWAY_DIR = api-gateway
DOCKER_COMPOSE = docker-compose

# Setup development environment
setup: deps
	@echo "Setting up development environment..."
	cd $(API_GATEWAY_DIR) && cargo build
	@echo "Setup complete!"

# Install dependencies
deps:
	@echo "Installing dependencies..."
	# Check if Newman is installed
	@if ! command -v newman >/dev/null 2>&1; then \
		echo "Installing Newman..."; \
		npm install -g newman; \
	fi
	# Check if jq is installed (for JSON parsing)
	@if ! command -v jq >/dev/null 2>&1; then \
		echo "Please install jq for JSON parsing"; \
		echo "macOS: brew install jq"; \
		echo "Ubuntu/Debian: sudo apt install jq"; \
	fi

# Run all tests
test: test-unit test-integration test-postman
	@echo "All tests completed!"

# Run Rust unit tests
test-unit:
	@echo "Running Rust unit tests..."
	cd $(API_GATEWAY_DIR) && cargo test

# Run integration tests
test-integration:
	@echo "Running integration tests..."
	cd $(POSTMAN_DIR) && ./integration-test.sh

# Run Postman API tests
test-postman:
	@echo "Running Postman API tests..."
	cd $(POSTMAN_DIR) && ./run-tests.sh $(TEST_ENV)

# Run performance tests
test-performance:
	@echo "Running performance tests..."
	cd $(POSTMAN_DIR) && newman run P2P_Energy_Trading_API.postman_collection.json \
		-e P2P_Energy_Trading_Local.postman_environment.json \
		--iteration-count 20 \
		--delay-request 50 \
		--reporters cli,json \
		--reporter-json-export test-results/performance-results.json

# Start required services
start-services:
	@echo "Starting required services..."
	$(DOCKER_COMPOSE) up -d postgres redis

# Stop services
stop-services:
	@echo "Stopping services..."
	$(DOCKER_COMPOSE) down

# Start API Gateway
start-api:
	@echo "Starting API Gateway..."
	cd $(API_GATEWAY_DIR) && \
	RUST_LOG=info \
	DATABASE_URL=postgres://postgres:postgres@localhost:5432/p2p_energy_trading \
	REDIS_URL=redis://default:redis_secret@localhost:6379 \
	JWT_SECRET=dev_jwt_secret_key \
	cargo run &

# Stop API Gateway
stop-api:
	@echo "Stopping API Gateway..."
	pkill -f "api-gateway" || true

# Clean build artifacts and test results
clean:
	@echo "Cleaning build artifacts and test results..."
	cd $(API_GATEWAY_DIR) && cargo clean
	rm -rf $(POSTMAN_DIR)/test-results/*
	rm -f /tmp/test_username /tmp/jwt_token

# Development workflow targets
dev-start: start-services start-api
	@echo "Development environment started!"
	@echo "API Gateway: $(API_BASE_URL)"
	@echo "Health check: curl $(API_BASE_URL)/health"

dev-stop: stop-api stop-services
	@echo "Development environment stopped!"

dev-test: test-integration test-postman
	@echo "Development tests completed!"

# CI/CD targets
ci-test: deps test-unit test-integration
	@echo "CI tests completed!"

# Quick health check
health-check:
	@echo "Checking API Gateway health..."
	@curl -s $(API_BASE_URL)/health | jq . || echo "API Gateway not responding"

# Show test results summary
test-summary:
	@echo "Test Results Summary:"
	@echo "===================="
	@if [ -f $(POSTMAN_DIR)/test-results/test-summary-*.json ]; then \
		latest_result=$$(ls -t $(POSTMAN_DIR)/test-results/test-summary-*.json | head -1); \
		echo "Latest test results from: $$latest_result"; \
		jq -r '"Total Requests: \(.run.stats.requests.total)", "Passed Tests: \(.run.stats.assertions.total - .run.stats.assertions.failed)", "Failed Tests: \(.run.stats.assertions.failed)", "Duration: \(.run.timings.completed)ms"' "$$latest_result"; \
	else \
		echo "No test results found. Run 'make test-postman' first."; \
	fi

# Docker-based testing
docker-test:
	@echo "Running tests in Docker environment..."
	$(DOCKER_COMPOSE) -f docker-compose.dev.yml up --build -d
	sleep 10
	cd $(POSTMAN_DIR) && API_BASE_URL=http://localhost:8080 ./integration-test.sh
	$(DOCKER_COMPOSE) -f docker-compose.dev.yml down

# Load testing with Artillery
load-test:
	@if ! command -v artillery >/dev/null 2>&1; then \
		echo "Installing Artillery..."; \
		npm install -g artillery; \
	fi
	@echo "Running load test..."
	# This would need an Artillery configuration file
	@echo "Load testing configuration needed. See docs for setup."