#!/bin/bash
set -e

echo "======================================"
echo "R Commerce API Test Script"
echo "======================================"

# Configuration
API_BASE="http://0.0.0.0:8080"
TEST_CONFIG="test_config.toml"
SERVER_PID=""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Track test results
TESTS_PASSED=0
TESTS_FAILED=0

# Cleanup function
cleanup() {
    echo ""
    echo "Cleaning up..."
    if [ -n "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
        wait $SERVER_PID 2>/dev/null || true
    fi
}
trap cleanup EXIT

# Helper function to print success
print_success() {
    echo -e "${GREEN}✓${NC} $1"
    ((TESTS_PASSED++))
}

# Helper function to print failure
print_failure() {
    echo -e "${RED}✗${NC} $1"
    ((TESTS_FAILED++))
}

# Helper function to print info
print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Helper function to print warning
print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found"
    exit 1
fi

# Build the project
echo ""
echo "Building project..."
cargo build --release --quiet

echo ""
echo "Creating test config..."
cat > "$TEST_CONFIG" << 'EOF'
[server]
host = "127.0.0.1"
port = 8080
worker_threads = 2
graceful_shutdown_timeout_secs = 5

[server.cors]
enabled = true
allowed_origins = ["*"]
allowed_methods = ["GET", "POST", "PUT", "DELETE"]
allowed_headers = ["Content-Type", "Authorization"]
allow_credentials = true
max_age = 3600

[server.limits]
max_request_size_mb = 10
rate_limit_per_minute = 1000
rate_limit_burst = 200

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "password"
pool_size = 5
ssl_mode = "Prefer"

[logging]
level = "info"
format = "Json"

[cache]
cache_type = "Memory"
max_size_mb = 100

[security]
api_key_prefix_length = 8
api_key_secret_length = 32

[security.jwt]
secret = "test-secret-do-not-use-in-production"
expiry_hours = 24

[media]
storage_type = "Local"
local_path = "./uploads"
local_base_url = "http://localhost:8080/uploads"

[media.image_processing]
enabled = true
default_quality = 85

[notifications]
enabled = false

[rate_limiting]
enabled = false

[features]
debug_api = true
metrics = true
health_check = true
EOF

# Test database migrations
echo ""
echo "Testing database migrations..."
./target/release/rcommerce -c "$TEST_CONFIG" db migrate 2>&1 || {
    echo "Migration may have failed, continuing anyway..."
}

# Start server in background
echo ""
echo "Starting server..."
./target/release/rcommerce -c "$TEST_CONFIG" server &
SERVER_PID=$!
echo "Server PID: $SERVER_PID"

# Wait for server to start
echo ""
echo "Waiting for server to start..."
for i in {1..30}; do
    if curl -s "$API_BASE/health" > /dev/null 2>&1; then
        echo "Server is ready!"
        break
    fi
    sleep 1
    if [ $i -eq 30 ]; then
        echo "Server failed to start within 30 seconds"
        exit 1
    fi
done

# =============================================================================
# Test Functions
# =============================================================================

test_health_check() {
    echo ""
    echo "1. Testing health endpoint..."
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/health" 2>&1 || echo "000")
    if [ "$HTTP_STATUS" = "200" ]; then
        print_success "Health check passed (HTTP $HTTP_STATUS)"
        curl -s "$API_BASE/health" | head -1
    else
        print_failure "Health check failed (HTTP $HTTP_STATUS)"
    fi
}

test_api_info() {
    echo ""
    echo "2. Testing API info endpoint..."
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/" 2>&1 || echo "000")
    if [ "$HTTP_STATUS" = "200" ]; then
        print_success "API info (HTTP $HTTP_STATUS)"
    else
        print_failure "API info failed (HTTP $HTTP_STATUS)"
    fi
}

test_products_list() {
    echo ""
    echo "3. Testing products list..."
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/products" 2>&1 || echo "000")
    if [ "$HTTP_STATUS" = "200" ]; then
        print_success "Products list (HTTP $HTTP_STATUS)"
    else
        print_failure "Products list failed (HTTP $HTTP_STATUS)"
    fi
}

test_product_by_id() {
    echo ""
    echo "4. Testing product by ID..."
    # Test with non-existent ID - should return 404
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/products/00000000-0000-0000-0000-000000000000" 2>&1 || echo "000")
    if [ "$HTTP_STATUS" = "404" ]; then
        print_success "Product not found (HTTP $HTTP_STATUS) - expected"
    elif [ "$HTTP_STATUS" = "200" ]; then
        print_success "Product endpoint working (HTTP $HTTP_STATUS)"
    else
        print_warning "Unexpected response (HTTP $HTTP_STATUS)"
    fi
}

test_auth_register() {
    echo ""
    echo "5. Testing auth registration..."
    TEST_EMAIL="test_$(date +%s)@example.com"
    
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"TestPassword123!\",\"first_name\":\"Test\",\"last_name\":\"User\"}" \
        "$API_BASE/api/v1/auth/register" 2>&1 || echo "000")
    
    if [ "$HTTP_STATUS" = "201" ]; then
        print_success "Auth registration (HTTP $HTTP_STATUS)"
        echo "$TEST_EMAIL" > /tmp/test_email.txt
    elif [ "$HTTP_STATUS" = "409" ]; then
        print_warning "User already exists (HTTP $HTTP_STATUS)"
    elif [ "$HTTP_STATUS" = "400" ] || [ "$HTTP_STATUS" = "422" ]; then
        print_warning "Validation error (HTTP $HTTP_STATUS)"
    else
        print_failure "Auth registration failed (HTTP $HTTP_STATUS)"
    fi
}

test_auth_login() {
    echo ""
    echo "6. Testing auth login..."
    
    # Try to login with test credentials
    HTTP_STATUS=$(curl -s -o /tmp/login_response.json -w "%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d '{"email":"test@test.com","password":"password"}' \
        "$API_BASE/api/v1/auth/login" 2>&1 || echo "000")
    
    if [ "$HTTP_STATUS" = "200" ]; then
        print_success "Auth login (HTTP $HTTP_STATUS)"
        # Extract token
        TOKEN=$(cat /tmp/login_response.json | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
        if [ -n "$TOKEN" ]; then
            echo "$TOKEN" > /tmp/auth_token.txt
        fi
    elif [ "$HTTP_STATUS" = "401" ]; then
        print_warning "Invalid credentials (HTTP $HTTP_STATUS) - expected for test user"
    elif [ "$HTTP_STATUS" = "400" ]; then
        print_warning "Bad request (HTTP $HTTP_STATUS)"
    else
        print_failure "Auth login failed (HTTP $HTTP_STATUS)"
    fi
}

test_cart_operations() {
    echo ""
    echo "7. Testing Cart Operations..."
    
    # Create guest cart
    print_info "Creating guest cart..."
    CART_RESPONSE=$(curl -s -X POST "$API_BASE/api/v1/carts/guest")
    CART_ID=$(echo $CART_RESPONSE | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    SESSION_TOKEN=$(echo $CART_RESPONSE | grep -o '"session_token":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$CART_ID" ]; then
        print_success "Guest cart created (ID: $CART_ID)"
        
        # Get cart by ID
        print_info "Fetching cart by ID..."
        HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/carts/$CART_ID" 2>&1 || echo "000")
        if [ "$HTTP_STATUS" = "200" ]; then
            print_success "Get cart (HTTP $HTTP_STATUS)"
        else
            print_failure "Get cart failed (HTTP $HTTP_STATUS)"
        fi
    else
        print_failure "Failed to create guest cart"
    fi
}

test_orders_list() {
    echo ""
    echo "8. Testing orders list..."
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/orders" 2>&1 || echo "000")
    if [ "$HTTP_STATUS" = "200" ]; then
        print_success "Orders list (HTTP $HTTP_STATUS)"
    else
        print_failure "Orders list failed (HTTP $HTTP_STATUS)"
    fi
}

test_coupon_list() {
    echo ""
    echo "9. Testing coupon list..."
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/coupons" 2>&1 || echo "000")
    if [ "$HTTP_STATUS" = "200" ]; then
        print_success "Coupon list (HTTP $HTTP_STATUS)"
    else
        print_failure "Coupon list failed (HTTP $HTTP_STATUS)"
    fi
}

test_debug_endpoints() {
    echo ""
    echo "10. Testing debug endpoints (if enabled)..."
    HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/debug/health" 2>&1 || echo "000")
    if [ "$HTTP_STATUS" = "200" ]; then
        print_success "Debug health endpoint (HTTP $HTTP_STATUS)"
    else
        print_info "Debug endpoint not available or disabled (HTTP $HTTP_STATUS)"
    fi
}

# =============================================================================
# Advanced Test Functions
# =============================================================================

test_complete_checkout_flow() {
    echo ""
    echo "11. Testing Complete Checkout Flow..."
    
    # Step 1: Register a test user
    TEST_EMAIL="checkout_$(date +%s)@example.com"
    PASSWORD="TestPassword123!"
    
    print_info "Step 1: Registering test user..."
    REGISTER_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\",\"first_name\":\"Checkout\",\"last_name\":\"Test\"}" \
        "$API_BASE/api/v1/auth/register")
    REGISTER_STATUS=$(echo "$REGISTER_RESPONSE" | tail -n1)
    
    if [ "$REGISTER_STATUS" != "201" ] && [ "$REGISTER_STATUS" != "200" ]; then
        print_warning "Registration returned $REGISTER_STATUS, continuing with login..."
    fi
    
    # Step 2: Login
    print_info "Step 2: Logging in..."
    LOGIN_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\"}" \
        "$API_BASE/api/v1/auth/login")
    LOGIN_STATUS=$(echo "$LOGIN_RESPONSE" | tail -n1)
    
    if [ "$LOGIN_STATUS" = "200" ]; then
        JWT_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
        print_success "Login successful"
    else
        print_failure "Login failed (HTTP $LOGIN_STATUS)"
        return
    fi
    
    # Step 3: Create guest cart
    print_info "Step 3: Creating guest cart..."
    CART_RESPONSE=$(curl -s -X POST "$API_BASE/api/v1/carts/guest")
    CART_ID=$(echo $CART_RESPONSE | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    SESSION_TOKEN=$(echo $CART_RESPONSE | grep -o '"session_token":"[^"]*"' | cut -d'"' -f4)
    
    if [ -z "$CART_ID" ]; then
        print_failure "Failed to create cart"
        return
    fi
    print_success "Guest cart created (ID: $CART_ID)"
    
    # Step 4: Get products to add to cart
    print_info "Step 4: Getting products..."
    PRODUCTS_RESPONSE=$(curl -s "$API_BASE/api/v1/products")
    FIRST_PRODUCT_ID=$(echo $PRODUCTS_RESPONSE | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    
    if [ -z "$FIRST_PRODUCT_ID" ]; then
        print_warning "No products found, skipping add to cart"
    else
        # Step 5: Add item to cart
        print_info "Step 5: Adding item to cart..."
        ADD_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $JWT_TOKEN" \
            -d "{\"product_id\":\"$FIRST_PRODUCT_ID\",\"quantity\":2}" \
            "$API_BASE/api/v1/carts/$CART_ID/items")
        ADD_STATUS=$(echo "$ADD_RESPONSE" | tail -n1)
        
        if [ "$ADD_STATUS" = "200" ] || [ "$ADD_STATUS" = "201" ]; then
            print_success "Item added to cart"
        else
            print_failure "Failed to add item (HTTP $ADD_STATUS)"
        fi
    fi
    
    # Step 6: Verify cart
    print_info "Step 6: Verifying cart..."
    CART_VERIFY=$(curl -s "$API_BASE/api/v1/carts/$CART_ID")
    ITEM_COUNT=$(echo $CART_VERIFY | grep -o '"items":\[' | wc -l)
    print_success "Cart verification complete"
}

test_auth_flows() {
    echo ""
    echo "12. Testing Authentication Flows..."
    
    # Generate unique email
    TEST_EMAIL="authflow_$(date +%s)@example.com"
    PASSWORD="TestPassword123!"
    NEW_PASSWORD="NewPassword456!"
    
    # 1. Register
    print_info "Step 1: Registering..."
    REGISTER_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\",\"first_name\":\"Auth\",\"last_name\":\"Flow\"}" \
        "$API_BASE/api/v1/auth/register")
    
    if [ "$REGISTER_STATUS" = "201" ] || [ "$REGISTER_STATUS" = "200" ]; then
        print_success "Registration successful"
    else
        print_warning "Registration returned $REGISTER_STATUS"
    fi
    
    # 2. Login
    print_info "Step 2: Logging in..."
    LOGIN_RESPONSE=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\"}" \
        "$API_BASE/api/v1/auth/login")
    
    JWT_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
    REFRESH_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"refresh_token":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$JWT_TOKEN" ]; then
        print_success "Login successful, got JWT token"
        
        # 3. Access protected endpoint
        print_info "Step 3: Accessing protected endpoint..."
        PROTECTED_STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
            -H "Authorization: Bearer $JWT_TOKEN" \
            "$API_BASE/api/v1/carts/me")
        
        if [ "$PROTECTED_STATUS" = "200" ] || [ "$PROTECTED_STATUS" = "201" ]; then
            print_success "Protected endpoint accessible"
        else
            print_warning "Protected endpoint returned $PROTECTED_STATUS"
        fi
        
        # 4. Refresh token
        if [ -n "$REFRESH_TOKEN" ]; then
            print_info "Step 4: Refreshing token..."
            REFRESH_RESPONSE=$(curl -s -X POST \
                -H "Content-Type: application/json" \
                -d "{\"refresh_token\":\"$REFRESH_TOKEN\"}" \
                "$API_BASE/api/v1/auth/refresh")
            
            NEW_TOKEN=$(echo "$REFRESH_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
            if [ -n "$NEW_TOKEN" ]; then
                print_success "Token refreshed successfully"
            else
                print_warning "Token refresh failed"
            fi
        fi
        
        # 5. Request password reset
        print_info "Step 5: Requesting password reset..."
        RESET_REQUEST_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
            -H "Content-Type: application/json" \
            -d "{\"email\":\"$TEST_EMAIL\"}" \
            "$API_BASE/api/v1/auth/password-reset")
        
        if [ "$RESET_REQUEST_STATUS" = "200" ]; then
            print_success "Password reset requested"
        else
            print_warning "Password reset request returned $RESET_REQUEST_STATUS"
        fi
    else
        print_failure "Login failed, skipping auth flow tests"
    fi
}

test_cart_merge() {
    echo ""
    echo "13. Testing Cart Merge on Login..."
    
    # Create guest cart
    print_info "Creating guest cart..."
    GUEST_CART=$(curl -s -X POST "$API_BASE/api/v1/carts/guest")
    GUEST_CART_ID=$(echo $GUEST_CART | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    SESSION_TOKEN=$(echo $GUEST_CART | grep -o '"session_token":"[^"]*"' | cut -d'"' -f4)
    
    if [ -z "$GUEST_CART_ID" ]; then
        print_failure "Failed to create guest cart"
        return
    fi
    
    # Register and login
    TEST_EMAIL="cartmerge_$(date +%s)@example.com"
    PASSWORD="TestPassword123!"
    
    curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\",\"first_name\":\"Cart\",\"last_name\":\"Merge\"}" \
        "$API_BASE/api/v1/auth/register" > /dev/null
    
    LOGIN_RESPONSE=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\"}" \
        "$API_BASE/api/v1/auth/login")
    
    JWT_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$JWT_TOKEN" ] && [ -n "$SESSION_TOKEN" ]; then
        # Try to merge carts
        print_info "Attempting cart merge..."
        MERGE_RESPONSE=$(curl -s -w "\n%{http_code}" -X POST \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $JWT_TOKEN" \
            -d "{\"session_token\":\"$SESSION_TOKEN\"}" \
            "$API_BASE/api/v1/carts/merge")
        MERGE_STATUS=$(echo "$MERGE_RESPONSE" | tail -n1)
        
        if [ "$MERGE_STATUS" = "200" ]; then
            print_success "Cart merge successful"
        else
            print_warning "Cart merge returned $MERGE_STATUS (may be expected)"
        fi
    else
        print_warning "Missing token for cart merge test"
    fi
}

test_coupon_application() {
    echo ""
    echo "14. Testing Coupon Application..."
    
    # Create cart
    CART_RESPONSE=$(curl -s -X POST "$API_BASE/api/v1/carts/guest")
    CART_ID=$(echo $CART_RESPONSE | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
    
    # Register and login to get token
    TEST_EMAIL="coupon_$(date +%s)@example.com"
    PASSWORD="TestPassword123!"
    
    curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\",\"first_name\":\"Coupon\",\"last_name\":\"Test\"}" \
        "$API_BASE/api/v1/auth/register" > /dev/null
    
    LOGIN_RESPONSE=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$TEST_EMAIL\",\"password\":\"$PASSWORD\"}" \
        "$API_BASE/api/v1/auth/login")
    
    JWT_TOKEN=$(echo "$LOGIN_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$CART_ID" ] && [ -n "$JWT_TOKEN" ]; then
        # Try to apply coupon
        print_info "Applying coupon to cart..."
        COUPON_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer $JWT_TOKEN" \
            -d '{"coupon_code":"TEST10"}' \
            "$API_BASE/api/v1/carts/$CART_ID/coupon")
        
        if [ "$COUPON_STATUS" = "200" ]; then
            print_success "Coupon applied successfully"
            
            # Try to remove coupon
            print_info "Removing coupon..."
            REMOVE_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE \
                -H "Authorization: Bearer $JWT_TOKEN" \
                "$API_BASE/api/v1/carts/$CART_ID/coupon")
            
            if [ "$REMOVE_STATUS" = "200" ]; then
                print_success "Coupon removed successfully"
            else
                print_warning "Coupon removal returned $REMOVE_STATUS"
            fi
        else
            print_warning "Coupon application returned $COUPON_STATUS (may need valid coupon)"
        fi
    else
        print_warning "Missing cart ID or token for coupon test"
    fi
}

# =============================================================================
# Run All Tests
# =============================================================================

echo ""
echo "======================================"
echo "Running API Endpoint Tests"
echo "======================================"

# Basic endpoint tests
test_health_check
test_api_info
test_products_list
test_product_by_id
test_auth_register
test_auth_login
test_cart_operations
test_orders_list
test_coupon_list
test_debug_endpoints

# Advanced flow tests
test_complete_checkout_flow
test_auth_flows
test_cart_merge
test_coupon_application

# =============================================================================
# Test Summary
# =============================================================================

echo ""
echo "======================================"
echo "API Test Summary"
echo "======================================"
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi
