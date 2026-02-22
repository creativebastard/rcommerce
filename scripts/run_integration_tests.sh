#!/bin/bash
#
# R Commerce Integration Test Suite Runner with Report Generation
#
# This script runs the integration tests and generates HTML/JSON reports
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default configuration
OUTPUT_DIR="${OUTPUT_DIR:-./test-reports}"
TEST_FILTER="${1:-}"
TEST_DB_URL="${TEST_DATABASE_URL:-postgres://rcommerce_test:testpass@localhost/rcommerce_test}"

# Arrays to store test results
declare -a TEST_NAMES
declare -a TEST_STATUSES

print_header() {
    echo -e "${BLUE}"
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║              R COMMERCE INTEGRATION TEST SUITE RUNNER                        ║"
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
    
    # Check if PostgreSQL is running
    if ! pg_isready -q 2>/dev/null; then
        print_error "PostgreSQL is not running"
        exit 1
    fi
    print_success "PostgreSQL is running"
    
    # Export test database URL
    export TEST_DATABASE_URL="$TEST_DB_URL"
    print_info "Test Database: $TEST_DATABASE_URL"
}

build_tests() {
    print_section "Building Integration Tests"
    
    cd "$(dirname "$0")/.."
    
    if [ -n "$TEST_FILTER" ]; then
        print_info "Running specific test: $TEST_FILTER"
    fi
    
    if cargo build --package rcommerce-api --tests 2>&1 | tee /tmp/build.log; then
        print_success "Build completed"
    else
        print_error "Build failed. See /tmp/build.log for details."
        exit 1
    fi
}

run_tests() {
    print_section "Running Integration Tests"
    
    mkdir -p "$OUTPUT_DIR"
    
    local test_args=""
    if [ -n "$TEST_FILTER" ]; then
        test_args="$TEST_FILTER"
    fi
    
    # Run tests and capture output
    local test_output="/tmp/integration_test_output.txt"
    
    print_info "Starting test execution..."
    print_info "Test threads: 1 (sequential execution for database isolation)"
    echo
    
    set +e  # Don't exit on test failure
    cargo test --package rcommerce-api --test integration_tests -- $test_args --test-threads=1 --nocapture 2>&1 | tee "$test_output"
    TEST_EXIT_CODE=${PIPESTATUS[0]}
    set -e
    
    # Parse test results
    parse_test_results "$test_output"
    
    return $TEST_EXIT_CODE
}

parse_test_results() {
    local output_file="$1"
    
    # Clear arrays
    TEST_NAMES=()
    TEST_STATUSES=()
    
    # Extract test results - cargo test outputs test name on one line and status on next
    local prev_line=""
    while IFS= read -r line; do
        # Check if previous line was a test line
        if [[ $prev_line =~ ^test\ (test_[^\ ]+)\ \.\.\. ]]; then
            local test_name="${BASH_REMATCH[1]}"
            local status="failed"
            if [[ $line == "ok" ]]; then
                status="ok"
            elif [[ $line == "ignored" ]]; then
                status="ignored"
            fi
            TEST_NAMES+=("$test_name")
            TEST_STATUSES+=("$status")
        fi
        prev_line="$line"
    done < "$output_file"
    
    # Generate reports
    generate_json_report "$OUTPUT_DIR/integration-report.json"
    generate_html_report "$OUTPUT_DIR/integration-report.html"
}

generate_json_report() {
    local output_file="$1"
    
    local total=${#TEST_NAMES[@]}
    local passed=0
    local failed=0
    
    for status in "${TEST_STATUSES[@]}"; do
        if [ "$status" = "ok" ]; then
            ((passed++))
        else
            ((failed++))
        fi
    done
    
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    
    cat > "$output_file" << EOF
{
  "summary": {
    "total": $total,
    "passed": $passed,
    "failed": $failed,
    "skipped": 0,
    "duration_ms": 0,
    "timestamp": "$timestamp"
  },
  "tests": [
EOF
    
    local i=0
    while [ $i -lt ${#TEST_NAMES[@]} ]; do
        local status="${TEST_STATUSES[$i]}"
        local status_text="passed"
        if [ "$status" != "ok" ]; then
            status_text="failed"
        fi
        
        local comma=","
        if [ $i -eq $((${#TEST_NAMES[@]} - 1)) ]; then
            comma=""
        fi
        
        cat >> "$output_file" << EOF
    {
      "name": "${TEST_NAMES[$i]}",
      "status": "$status_text",
      "duration_ms": 0,
      "description": "Integration test: ${TEST_NAMES[$i]}"
    }${comma}
EOF
        ((i++))
    done
    
    cat >> "$output_file" << EOF

  ]
}
EOF
    
    print_info "JSON Report: $output_file"
}

generate_html_report() {
    local output_file="$1"
    
    local total=${#TEST_NAMES[@]}
    local passed=0
    local failed=0
    
    for status in "${TEST_STATUSES[@]}"; do
        if [ "$status" = "ok" ]; then
            ((passed++))
        else
            ((failed++))
        fi
    done
    
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    local pass_rate=0
    if [ $total -gt 0 ]; then
        pass_rate=$((passed * 100 / total))
    fi
    
    local alert_class="success"
    local status_text="PASSED"
    if [ $failed -gt 0 ]; then
        alert_class="danger"
        status_text="FAILED"
    fi
    
    cat > "$output_file" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>R Commerce Integration Test Report</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.1.3/dist/css/bootstrap.min.css" rel="stylesheet">
    <style>
        body { padding: 20px; }
        .test-card { margin-bottom: 15px; }
        .pass { border-left: 5px solid #28a745; }
        .fail { border-left: 5px solid #dc3545; }
    </style>
</head>
<body>
    <div class="container">
        <div class="alert alert-${alert_class} text-center">
            <h1>Integration Test Suite ${status_text}</h1>
            <p class="mb-0">Generated: $timestamp</p>
        </div>
        
        <div class="row mb-4">
            <div class="col-md-3">
                <div class="card text-center">
                    <div class="card-body">
                        <h3>$total</h3>
                        <p class="text-muted">Total Tests</p>
                    </div>
                </div>
            </div>
            <div class="col-md-3">
                <div class="card text-center">
                    <div class="card-body">
                        <h3 class="text-success">$passed</h3>
                        <p class="text-muted">Passed</p>
                    </div>
                </div>
            </div>
            <div class="col-md-3">
                <div class="card text-center">
                    <div class="card-body">
                        <h3 class="text-danger">$failed</h3>
                        <p class="text-muted">Failed</p>
                    </div>
                </div>
            </div>
            <div class="col-md-3">
                <div class="card text-center">
                    <div class="card-body">
                        <h3>${pass_rate}%</h3>
                        <p class="text-muted">Pass Rate</p>
                    </div>
                </div>
            </div>
        </div>
        
        <h4>Test Details</h4>
        <hr>
EOF
    
    local i=0
    while [ $i -lt ${#TEST_NAMES[@]} ]; do
        local status="${TEST_STATUSES[$i]}"
        local card_class="pass"
        local badge_class="success"
        local badge_text="PASS"
        
        if [ "$status" != "ok" ]; then
            card_class="fail"
            badge_class="danger"
            badge_text="FAIL"
        fi
        
        cat >> "$output_file" << EOF
        <div class="card test-card ${card_class}">
            <div class="card-body d-flex justify-content-between align-items-center">
                <div>
                    <h5 class="mb-1">${TEST_NAMES[$i]}</h5>
                    <small class="text-muted">Integration test</small>
                </div>
                <span class="badge bg-${badge_class}">${badge_text}</span>
            </div>
        </div>
EOF
        ((i++))
    done
    
    cat >> "$output_file" << EOF
    </div>
</body>
</html>
EOF
    
    print_info "HTML Report: file://$(realpath "$output_file")"
}

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] [TEST_FILTER]

R Commerce Integration Test Suite Runner

OPTIONS:
    -h, --help          Show this help message
    --output DIR        Set output directory for reports (default: ./test-reports)
    
ENVIRONMENT VARIABLES:
    TEST_DATABASE_URL   PostgreSQL test database URL
                        (default: postgres://rcommerce_test:testpass@localhost/rcommerce_test)
    OUTPUT_DIR          Directory for test reports

EXAMPLES:
    # Run all tests
    $0

    # Run specific test
    $0 test_auth_flows

    # Use custom database
    TEST_DATABASE_URL="postgres://user:pass@localhost/testdb" $0

EOF
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        --output)
            OUTPUT_DIR="$2"
            shift 2
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

if run_tests; then
    echo
    print_success "Integration Test Suite completed successfully!"
    print_info "Reports generated in: $OUTPUT_DIR"
    print_info "  - JSON: $OUTPUT_DIR/integration-report.json"
    print_info "  - HTML: $OUTPUT_DIR/integration-report.html"
    exit 0
else
    echo
    print_error "Some tests failed"
    print_info "Reports generated in: $OUTPUT_DIR"
    exit 1
fi
