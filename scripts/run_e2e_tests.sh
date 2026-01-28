#!/bin/bash
#
# R Commerce End-to-End Test Suite Runner
#
# This script runs the comprehensive E2E test suite which:
# 1. Spins up an instance with a fresh SQLite database
# 2. Creates dummy data across all tables
# 3. Tests order workflows and lifecycle
# 4. Tests Redis caching functionality
# 5. Tests SSL/TLS certificate generation
# 6. Generates comprehensive reports
# 7. Cleans up test data
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default configuration
DB_PATH="/tmp/rcommerce_e2e_test.db"
REDIS_URL="${REDIS_URL:-redis://127.0.0.1:6379}"
RUN_SSL_TESTS="${RUN_SSL_TESTS:-0}"
KEEP_DATA="${KEEP_DATA:-0}"
OUTPUT_DIR="${OUTPUT_DIR:-./test-reports}"
TEST_FILTER="${1:-}"

print_header() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                   R COMMERCE E2E TEST SUITE RUNNER                           ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_section() {
    echo -e "${YELLOW}"
    echo -e "▶ $1"
    echo -e "${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

check_prerequisites() {
    print_section "Checking Prerequisites"
    
    # Check if cargo is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust."
        exit 1
    fi
    print_success "Cargo installed"
    
    # Check if Redis is available
    if command -v redis-cli &> /dev/null; then
        if redis-cli ping > /dev/null 2>&1; then
            print_success "Redis is running"
            export REDIS_URL="$REDIS_URL"
        else
            print_info "Redis not running, cache tests will be skipped"
            unset REDIS_URL
        fi
    else
        print_info "Redis not installed, cache tests will be skipped"
        unset REDIS_URL
    fi
    
    # Clean up any existing test database
    if [ -f "$DB_PATH" ]; then
        print_info "Removing existing test database"
        rm -f "$DB_PATH" "$DB_PATH-shm" "$DB_PATH-wal"
    fi
}

build_tests() {
    print_section "Building Test Suite"
    
    cd "$(dirname "$0")/.."
    
    if [ -n "$TEST_FILTER" ]; then
        print_info "Running specific test: $TEST_FILTER"
    fi
    
    cargo build --package rcommerce-api --tests --release 2>&1 | tee /tmp/build.log
    
    if [ ${PIPESTATUS[0]} -ne 0 ]; then
        print_error "Build failed. See /tmp/build.log for details."
        exit 1
    fi
    
    print_success "Test suite built successfully"
}

run_tests() {
    print_section "Running E2E Tests"
    
    export RCOMMERCE_TEST_DB_PATH="$DB_PATH"
    export RCOMMERCE_TEST_REDIS_URL="$REDIS_URL"
    export RUN_SSL_TESTS="$RUN_SSL_TESTS"
    export RUST_BACKTRACE=1
    
    # Create output directory
    mkdir -p "$OUTPUT_DIR"
    
    local test_args=""
    if [ -n "$TEST_FILTER" ]; then
        test_args="$TEST_FILTER"
    else
        test_args="run_e2e_test_suite"
    fi
    
    print_info "Starting test execution..."
    print_info "Database: $DB_PATH"
    print_info "Redis: ${REDIS_URL:-disabled}"
    print_info "SSL Tests: $([ "$RUN_SSL_TESTS" = "1" ] && echo "enabled" || echo "disabled")"
    echo
    
    # Run tests
    if cargo test --package rcommerce-api --test e2e_test_suite -- "$test_args" --nocapture; then
        TEST_RESULT=0
        print_success "All tests passed!"
    else
        TEST_RESULT=1
        print_error "Some tests failed"
    fi
    
    return $TEST_RESULT
}

generate_reports() {
    print_section "Generating Reports"
    
    # Reports are saved to workspace root test-reports/
    local reports_dir="$(dirname "$0")/../test-reports"
    
    if [ -d "$reports_dir" ]; then
        print_info "Reports available in: $reports_dir"
        
        if [ -f "$reports_dir/report.html" ]; then
            print_info "HTML Report: file://$(realpath "$reports_dir/report.html")"
        fi
        
        if [ -f "$reports_dir/report.json" ]; then
            print_info "JSON Report: $reports_dir/report.json"
        fi
    else
        print_info "No reports directory found at $reports_dir"
    fi
}

cleanup() {
    if [ "$KEEP_DATA" = "1" ]; then
        print_info "Keeping test data (KEEP_DATA=1)"
        print_info "Database: $DB_PATH"
        return
    fi
    
    print_section "Cleaning Up"
    
    # Remove test database
    if [ -f "$DB_PATH" ]; then
        rm -f "$DB_PATH" "$DB_PATH-shm" "$DB_PATH-wal"
        print_success "Test database removed"
    fi
    
    # Clean up temp files
    rm -f /tmp/test_cert.pem /tmp/test_key.pem
    
    print_success "Cleanup complete"
}

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] [TEST_FILTER]

R Commerce End-to-End Test Suite Runner

OPTIONS:
    -h, --help          Show this help message
    --keep-data         Keep test database after tests
    --with-ssl          Run SSL/TLS tests (requires DNS setup)
    --output DIR        Set output directory for reports (default: ./test-reports)
    --no-redis          Skip Redis cache tests

ENVIRONMENT VARIABLES:
    REDIS_URL           Redis connection URL (default: redis://127.0.0.1:6379)
    RUN_SSL_TESTS       Set to 1 to enable SSL tests
    KEEP_DATA           Set to 1 to preserve test database
    OUTPUT_DIR          Directory for test reports

EXAMPLES:
    # Run all tests
    $0

    # Run specific test
    $0 test_create_order

    # Run with SSL tests enabled
    RUN_SSL_TESTS=1 $0

    # Keep test data for debugging
    $0 --keep-data

    # Run without Redis
    $0 --no-redis

EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        --keep-data)
            KEEP_DATA=1
            shift
            ;;
        --with-ssl)
            RUN_SSL_TESTS=1
            shift
            ;;
        --output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --no-redis)
            unset REDIS_URL
            shift
            ;;
        -*)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
        *)
            TEST_FILTER="$1"
            shift
            ;;
    esac
done

# Main execution
print_header
check_prerequisites
build_tests

# Set trap for cleanup on exit
trap cleanup EXIT

if run_tests; then
    generate_reports
    echo
    print_success "E2E Test Suite completed successfully!"
    exit 0
else
    generate_reports
    echo
    print_error "E2E Test Suite completed with failures"
    exit 1
fi
