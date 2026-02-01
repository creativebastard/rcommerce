#!/bin/bash
#
# MVP API Test Script for R Commerce
#
# This script tests the complete MVP flow:
# 1. Health check
# 2. Customer registration
# 3. Customer login
# 4. Product listing
# 5. Order creation
# 6. Payment methods
# 7. Webhook handling
#
# Usage: ./test_api_mvp.sh [BASE_URL]
# Default: http://localhost:8080

# set -e  # Disabled for debugging

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
BASE_URL="${1:-http://localhost:8080}"
API_V1="$BASE_URL/api/v1"

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
log_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((TESTS_PASSED++))
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((TESTS_FAILED++))
}

make_request() {
    local method=$1
    local url=$2
    local data=$3
    local headers=$4
    
    if [ -n "$data" ]; then
        if [ -n "$headers" ]; then
            curl -s --max-time 10 -X "$method" "$url" -H "Content-Type: application/json" -H "$headers" -d "$data" -w "\n%{http_code}"
        else
            curl -s --max-time 10 -X "$method" "$url" -H "Content-Type: application/json" -d "$data" -w "\n%{http_code}"
        fi
    else
        if [ -n "$headers" ]; then
            curl -s --max-time 10 -X "$method" "$url" -H "$headers" -w "\n%{http_code}"
        else
            curl -s --max-time 10 -X "$method" "$url" -w "\n%{http_code}"
        fi
    fi
}

check_status() {
    local expected=$1
    local actual=$2
    local test_name=$3
    
    if [ "$actual" -eq "$expected" ]; then
        log_success "$test_name (HTTP $actual)"
        return 0
    else
        log_error "$test_name (expected HTTP $expected, got $actual)"
        return 1
    fi
}

# Generate unique test data
TEST_EMAIL="test-$(date +%s)@example.com"
TEST_PASSWORD="TestPassword123!"

echo "=========================================="
echo "R Commerce MVP API Test Suite"
echo "=========================================="
echo "Base URL: $BASE_URL"
echo "Test Email: $TEST_EMAIL"
echo ""

# Test 1: Health Check
echo "--- Test 1: Health Check ---"
RESPONSE=$(make_request "GET" "$BASE_URL/health")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

check_status 200 "$HTTP_CODE" "Health check"
echo "Response: $BODY"
echo ""

# Test 2: API Info
echo "--- Test 2: API Info ---"
RESPONSE=$(make_request "GET" "$BASE_URL/")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

check_status 200 "$HTTP_CODE" "API info"
echo "Response: $BODY"
echo ""

# Test 3: Customer Registration
echo "--- Test 3: Customer Registration ---"
REGISTER_DATA="{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\",\"first_name\":\"Test\",\"last_name\":\"User\"}"
RESPONSE=$(make_request "POST" "$API_V1/auth/register" "$REGISTER_DATA")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

check_status 201 "$HTTP_CODE" "Customer registration"
echo "Response: $BODY"

# Extract customer ID if registration successful
if [ "$HTTP_CODE" -eq 201 ]; then
    CUSTOMER_ID=$(echo "$BODY" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    log_info "Created customer ID: $CUSTOMER_ID"
fi
echo ""

# Test 4: Customer Login
echo "--- Test 4: Customer Login ---"
LOGIN_DATA="{\"email\":\"$TEST_EMAIL\",\"password\":\"$TEST_PASSWORD\"}"
RESPONSE=$(make_request "POST" "$API_V1/auth/login" "$LOGIN_DATA")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

check_status 200 "$HTTP_CODE" "Customer login"
echo "Response: $BODY"

# Extract access token if login successful
if [ "$HTTP_CODE" -eq 200 ]; then
    ACCESS_TOKEN=$(echo "$BODY" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
    log_info "Received access token"
fi
echo ""

# Test 5: Get Customer Profile (requires auth)
echo "--- Test 5: Get Customer Profile ---"
if [ -n "$ACCESS_TOKEN" ]; then
    RESPONSE=$(make_request "GET" "$API_V1/customers/me" "" "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')
    
    check_status 200 "$HTTP_CODE" "Get customer profile"
    echo "Response: $BODY"
else
    log_error "Get customer profile (no access token)"
fi
echo ""

# Test 6: Product Listing
echo "--- Test 6: Product Listing ---"
RESPONSE=$(make_request "GET" "$API_V1/products")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

check_status 200 "$HTTP_CODE" "Product listing"
echo "Response: $(echo "$BODY" | head -c 200)..."

# Extract first product ID if available
if [ "$HTTP_CODE" -eq 200 ]; then
    PRODUCT_ID=$(echo "$BODY" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    if [ -n "$PRODUCT_ID" ]; then
        log_info "Found product ID: $PRODUCT_ID"
    fi
fi
echo ""

# Test 7: Create Order (requires auth)
echo "--- Test 7: Create Order ---"
if [ -n "$PRODUCT_ID" ] && [ -n "$ACCESS_TOKEN" ]; then
    ORDER_DATA="{\"customer_email\":\"$TEST_EMAIL\",\"items\":[{\"product_id\":\"$PRODUCT_ID\",\"quantity\":2}]}"
    RESPONSE=$(make_request "POST" "$API_V1/orders" "$ORDER_DATA" "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')
    
    check_status 201 "$HTTP_CODE" "Create order"
    echo "Response: $BODY"
    
    # Extract order ID if creation successful
    if [ "$HTTP_CODE" -eq 201 ]; then
        ORDER_ID=$(echo "$BODY" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
        ORDER_NUMBER=$(echo "$BODY" | grep -o '"order_number":"[^"]*"' | cut -d'"' -f4)
        log_info "Created order: $ORDER_NUMBER (ID: $ORDER_ID)"
    fi
else
    log_error "Create order (no product available or no auth token)"
fi
echo ""

# Test 8: Get Order by ID (requires auth)
echo "--- Test 8: Get Order by ID ---"
if [ -n "$ORDER_ID" ] && [ -n "$ACCESS_TOKEN" ]; then
    RESPONSE=$(make_request "GET" "$API_V1/orders/$ORDER_ID" "" "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')
    
    check_status 200 "$HTTP_CODE" "Get order by ID"
    echo "Response: $(echo "$BODY" | head -c 200)..."
else
    log_error "Get order by ID (no order created or no auth token)"
fi
echo ""

# Test 9: List Orders (requires auth)
echo "--- Test 9: List Orders ---"
if [ -n "$ACCESS_TOKEN" ]; then
    RESPONSE=$(make_request "GET" "$API_V1/orders" "" "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')
    
    check_status 200 "$HTTP_CODE" "List orders"
    echo "Response: $(echo "$BODY" | head -c 200)..."
else
    log_error "List orders (no auth token)"
fi
echo ""

# Test 10: Payment Methods (requires auth)
echo "--- Test 10: Payment Methods ---"
if [ -n "$ACCESS_TOKEN" ]; then
    PAYMENT_DATA="{\"currency\":\"USD\",\"amount\":\"100.00\"}"
    RESPONSE=$(make_request "POST" "$API_V1/payments/methods" "$PAYMENT_DATA" "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')
    
    check_status 200 "$HTTP_CODE" "Get payment methods"
    echo "Response: $(echo "$BODY" | head -c 200)..."
else
    log_error "Get payment methods (no auth token)"
fi
echo ""

# Test 11: Stripe Webhook (no auth required)
echo "--- Test 11: Stripe Webhook ---"
WEBHOOK_BODY='{"id":"evt_test","type":"payment_intent.succeeded","data":{"object":{"id":"pi_test","amount":2000,"currency":"usd"}}}'
RESPONSE=$(curl -s --max-time 10 -X "POST" "$API_V1/webhooks/stripe" \
    -H "Content-Type: application/json" \
    -H "Stripe-Signature: test_signature" \
    -d "$WEBHOOK_BODY" \
    -w "\n%{http_code}")
HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

# Webhook should accept or return bad request (invalid signature)
if [ "$HTTP_CODE" -eq 200 ] || [ "$HTTP_CODE" -eq 400 ]; then
    log_success "Stripe webhook (HTTP $HTTP_CODE)"
else
    log_error "Stripe webhook (expected 200 or 400, got $HTTP_CODE)"
fi
echo "Response: $BODY"
echo ""

# Test 12: 404 Handling (requires auth)
echo "--- Test 12: 404 Handling ---"
FAKE_ID="550e8400-e29b-41d4-a716-446655440000"
if [ -n "$ACCESS_TOKEN" ]; then
    RESPONSE=$(make_request "GET" "$API_V1/orders/$FAKE_ID" "" "Authorization: Bearer $ACCESS_TOKEN")
    HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
    BODY=$(echo "$RESPONSE" | sed '$d')
    
    check_status 404 "$HTTP_CODE" "404 for non-existent order"
else
    log_error "404 handling (no auth token)"
fi
echo ""

# Summary
echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
echo ""

if [ "$TESTS_FAILED" -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
fi
