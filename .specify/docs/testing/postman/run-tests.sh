#!/bin/bash

# P2P Energy Trading API - Postman Test Runner
# This script runs the Postman API tests using Newman

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COLLECTION_FILE="$SCRIPT_DIR/P2P_Energy_Trading_API.postman_collection.json"

# Default values
ENVIRONMENT="local"
VERBOSE=false
OUTPUT_DIR="$SCRIPT_DIR/test-results"
ITERATIONS=1
DELAY=0

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage
show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] [ENVIRONMENT]

Run Postman API tests for P2P Energy Trading API Gateway

ENVIRONMENTS:
    local       Use local development environment (default)
    production  Use production environment
    custom      Use custom environment file (specify with -e)

OPTIONS:
    -e, --env-file FILE     Custom environment file path
    -o, --output DIR        Output directory for test results (default: ./test-results)
    -i, --iterations NUM    Number of test iterations (default: 1)
    -d, --delay MS          Delay between requests in milliseconds (default: 0)
    -v, --verbose           Enable verbose output
    -h, --help              Show this help message

EXAMPLES:
    $0                      # Run with local environment
    $0 production           # Run with production environment
    $0 -v local             # Run with verbose output
    $0 -e ./custom.json     # Run with custom environment file
    $0 -i 5 -d 1000 local   # Run 5 iterations with 1s delay

REQUIREMENTS:
    - Newman must be installed: npm install -g newman
    - API Gateway must be running (for local environment)
    - Environment variables must be properly configured

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -e|--env-file)
            CUSTOM_ENV_FILE="$2"
            ENVIRONMENT="custom"
            shift 2
            ;;
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -i|--iterations)
            ITERATIONS="$2"
            shift 2
            ;;
        -d|--delay)
            DELAY="$2"
            shift 2
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        local|production)
            ENVIRONMENT="$1"
            shift
            ;;
        custom)
            ENVIRONMENT="custom"
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Check if Newman is installed
if ! command -v newman &> /dev/null; then
    print_error "Newman is not installed. Please install it with: npm install -g newman"
    exit 1
fi

# Check if collection file exists
if [[ ! -f "$COLLECTION_FILE" ]]; then
    print_error "Collection file not found: $COLLECTION_FILE"
    exit 1
fi

# Determine environment file
case $ENVIRONMENT in
    "local")
        ENV_FILE="$SCRIPT_DIR/P2P_Energy_Trading_Local.postman_environment.json"
        ;;
    "production")
        ENV_FILE="$SCRIPT_DIR/P2P_Energy_Trading_Production.postman_environment.json"
        ;;
    "custom")
        if [[ -z "$CUSTOM_ENV_FILE" ]]; then
            print_error "Custom environment specified but no file provided. Use -e option."
            exit 1
        fi
        ENV_FILE="$CUSTOM_ENV_FILE"
        ;;
esac

# Check if environment file exists
if [[ ! -f "$ENV_FILE" ]]; then
    print_error "Environment file not found: $ENV_FILE"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Generate timestamp for output files
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
HTML_REPORT="$OUTPUT_DIR/test-report-$TIMESTAMP.html"
JUNIT_REPORT="$OUTPUT_DIR/test-results-$TIMESTAMP.xml"
JSON_REPORT="$OUTPUT_DIR/test-summary-$TIMESTAMP.json"

print_status "Starting P2P Energy Trading API Tests"
print_status "Environment: $ENVIRONMENT"
print_status "Collection: $(basename "$COLLECTION_FILE")"
print_status "Environment File: $(basename "$ENV_FILE")"
print_status "Iterations: $ITERATIONS"
print_status "Output Directory: $OUTPUT_DIR"

# Build Newman command
NEWMAN_CMD="newman run \"$COLLECTION_FILE\" -e \"$ENV_FILE\""
NEWMAN_CMD="$NEWMAN_CMD --reporters cli,html,junit,json"
NEWMAN_CMD="$NEWMAN_CMD --reporter-html-export \"$HTML_REPORT\""
NEWMAN_CMD="$NEWMAN_CMD --reporter-junit-export \"$JUNIT_REPORT\""
NEWMAN_CMD="$NEWMAN_CMD --reporter-json-export \"$JSON_REPORT\""
NEWMAN_CMD="$NEWMAN_CMD --iteration-count $ITERATIONS"

if [[ $DELAY -gt 0 ]]; then
    NEWMAN_CMD="$NEWMAN_CMD --delay-request $DELAY"
fi

if [[ $VERBOSE == true ]]; then
    NEWMAN_CMD="$NEWMAN_CMD --verbose"
fi

# Add bail option to stop on first failure (can be removed for complete test runs)
# NEWMAN_CMD="$NEWMAN_CMD --bail"

print_status "Running tests..."
echo

# Run the tests
if eval "$NEWMAN_CMD"; then
    print_success "All tests completed successfully!"
    
    # Show test summary if JSON report exists
    if [[ -f "$JSON_REPORT" ]] && command -v jq &> /dev/null; then
        echo
        print_status "Test Summary:"
        jq -r '
            "Total Requests: \(.run.stats.requests.total)",
            "Passed Tests: \(.run.stats.assertions.total - .run.stats.assertions.failed)",
            "Failed Tests: \(.run.stats.assertions.failed)",
            "Test Duration: \(.run.timings.completed)ms"
        ' "$JSON_REPORT"
    fi
    
    echo
    print_status "Reports generated:"
    echo "  HTML Report: $HTML_REPORT"
    echo "  JUnit XML: $JUNIT_REPORT"
    echo "  JSON Summary: $JSON_REPORT"
    
    # Open HTML report if on macOS
    if [[ "$OSTYPE" == "darwin"* ]] && command -v open &> /dev/null; then
        print_status "Opening HTML report..."
        open "$HTML_REPORT"
    fi
    
    exit 0
else
    print_error "Tests failed! Check the reports for details."
    
    echo
    print_status "Reports generated:"
    echo "  HTML Report: $HTML_REPORT"
    echo "  JUnit XML: $JUNIT_REPORT"
    echo "  JSON Summary: $JSON_REPORT"
    
    exit 1
fi