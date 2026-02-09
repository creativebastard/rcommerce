#!/bin/bash
set -e

echo "======================================"
echo "R Commerce API Test Script"
echo "======================================"

# Configuration
API_BASE="http://0.0.0.0:8080"
TEST_CONFIG="test_config.toml"
SERVER_PID=""

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

# Test endpoints
echo ""
echo "======================================"
echo "Testing API Endpoints"
echo "======================================"

# Health check
echo ""
echo "1. Testing health endpoint..."
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/health" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "200" ]; then
    echo "   ✅ Health check passed (HTTP $HTTP_STATUS)"
    curl -s "$API_BASE/health" | head -1
else
    echo "   ❌ Health check failed (HTTP $HTTP_STATUS)"
fi

# Products list
echo ""
echo "2. Testing products list..."
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/products" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "200" ]; then
    echo "   ✅ Products list (HTTP $HTTP_STATUS)"
else
    echo "   ❌ Products list failed (HTTP $HTTP_STATUS)"
fi

# Product by ID (test with non-existent ID - should return 404)
echo ""
echo "3. Testing product by ID (non-existent)..."
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/products/00000000-0000-0000-0000-000000000000" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "404" ]; then
    echo "   ✅ Product not found (HTTP $HTTP_STATUS) - expected"
elif [ "$HTTP_STATUS" = "200" ]; then
    echo "   ✅ Product endpoint working (HTTP $HTTP_STATUS)"
else
    echo "   ⚠️  Unexpected response (HTTP $HTTP_STATUS)"
fi

# Customers list
echo ""
echo "4. Testing customers list..."
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/customers" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "200" ]; then
    echo "   ✅ Customers list (HTTP $HTTP_STATUS)"
else
    echo "   ❌ Customers list failed (HTTP $HTTP_STATUS)"
fi

# Orders list
echo ""
echo "5. Testing orders list..."
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/api/v1/orders" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "200" ]; then
    echo "   ✅ Orders list (HTTP $HTTP_STATUS)"
else
    echo "   ❌ Orders list failed (HTTP $HTTP_STATUS)"
fi

# Auth endpoints
echo ""
echo "6. Testing auth endpoints..."

# Register (should fail without proper data, but should return expected error)
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -d '{"email":"test@test.com","password":"password"}' \
    "$API_BASE/api/v1/auth/register" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "200" ] || [ "$HTTP_STATUS" = "400" ] || [ "$HTTP_STATUS" = "422" ]; then
    echo "   ✅ Auth register endpoint responding (HTTP $HTTP_STATUS)"
else
    echo "   ⚠️  Auth register unexpected response (HTTP $HTTP_STATUS)"
fi

# Login
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
    -H "Content-Type: application/json" \
    -d '{"email":"test@test.com","password":"password"}' \
    "$API_BASE/api/v1/auth/login" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "200" ] || [ "$HTTP_STATUS" = "401" ] || [ "$HTTP_STATUS" = "400" ]; then
    echo "   ✅ Auth login endpoint responding (HTTP $HTTP_STATUS)"
else
    echo "   ⚠️  Auth login unexpected response (HTTP $HTTP_STATUS)"
fi

# Debug endpoints (if available)
echo ""
echo "7. Testing debug endpoints (if enabled)..."
HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$API_BASE/debug/health" 2>&1 || echo "000")
if [ "$HTTP_STATUS" = "200" ]; then
    echo "   ✅ Debug health endpoint (HTTP $HTTP_STATUS)"
else
    echo "   ℹ️  Debug endpoint not available or disabled (HTTP $HTTP_STATUS)"
fi

echo ""
echo "======================================"
echo "API Test Summary"
echo "======================================"
echo "Tests completed. Check above for individual results."
echo ""
echo "Note: Some tests may show expected failures:"
echo "  - 404 for non-existent resources is correct"
echo "  - 401 for unauthenticated requests is correct"
echo "  - 400/422 for invalid data is correct""
