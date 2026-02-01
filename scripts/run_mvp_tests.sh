#!/bin/bash
#
# MVP Test Runner - Starts server and runs comprehensive tests
#
# Usage: ./run_mvp_tests.sh

set -e

cd "$(dirname "$0")/.."

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}R Commerce MVP Test Runner${NC}"
echo -e "${BLUE}==========================================${NC}"
echo ""

# Check if server is already running
echo -e "${YELLOW}Checking if server is running...${NC}"
if curl -s http://localhost:8080/health > /dev/null 2>&1; then
    echo -e "${GREEN}Server is already running on port 8080${NC}"
    SERVER_PID=""
else
    echo -e "${YELLOW}Starting test server...${NC}"
    
    # Build the project first
    echo "Building project..."
    cargo build --release -p rcommerce-cli 2>&1 | tail -5
    
    # Start server in background
    ./target/release/rcommerce server --port 8080 &
    SERVER_PID=$!
    
    # Wait for server to be ready
    echo "Waiting for server to start..."
    for i in {1..30}; do
        if curl -s http://localhost:8080/health > /dev/null 2>&1; then
            echo -e "${GREEN}Server is ready!${NC}"
            break
        fi
        sleep 1
        echo -n "."
    done
    
    # Check if server started successfully
    if ! curl -s http://localhost:8080/health > /dev/null 2>&1; then
        echo -e "${RED}Failed to start server!${NC}"
        exit 1
    fi
fi

echo ""
echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}Running API Tests${NC}"
echo -e "${BLUE}==========================================${NC}"
echo ""

# Run the API tests
./scripts/test_api_mvp.sh http://localhost:8080
TEST_RESULT=$?

echo ""
echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}Running Unit Tests${NC}"
echo -e "${BLUE}==========================================${NC}"
echo ""

# Run cargo tests
cargo test --workspace 2>&1 | tail -20

echo ""
echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}Test Summary${NC}"
echo -e "${BLUE}==========================================${NC}"
echo ""

if [ "$TEST_RESULT" -eq 0 ]; then
    echo -e "${GREEN}MVP Tests: PASSED${NC}"
else
    echo -e "${RED}MVP Tests: FAILED${NC}"
fi

# Cleanup: Stop server if we started it
if [ -n "$SERVER_PID" ]; then
    echo ""
    echo -e "${YELLOW}Stopping test server...${NC}"
    kill $SERVER_PID 2>/dev/null || true
    wait $SERVER_PID 2>/dev/null || true
    echo -e "${GREEN}Server stopped${NC}"
fi

echo ""
echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}MVP Test Run Complete${NC}"
echo -e "${BLUE}==========================================${NC}"

exit $TEST_RESULT
