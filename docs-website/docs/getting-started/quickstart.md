# Quick Start Guide

Get R Commerce up and running in minutes with this quick start guide.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 1.70+** - [Install from rustup.rs](https://rustup.rs/)
- **PostgreSQL 13+**
- **Redis 6+** (optional, for caching)

## Installation

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# Build the project
cargo build --release

# The binary will be at:
# target/release/rcommerce
```

### Option 2: Docker (Recommended for Quick Start)

```bash
# Clone the repository
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# Start all services
docker-compose up -d

# Check status
docker-compose ps
```

## Configuration

### Option 1: Interactive Setup Wizard (Recommended)

The easiest way to configure R Commerce is using the setup wizard:

```bash
# Run the interactive setup wizard
./target/release/rcommerce setup

# Or with a specific output file
./target/release/rcommerce setup -o config/production.toml
```

The wizard will guide you through:
- Store configuration (name, currency)
- Database setup (PostgreSQL)
- Database migrations (with handling for existing databases)
- Optional data import from WooCommerce, Shopify, Magento, or Medusa
- Server, cache, and security settings
- TLS/SSL configuration (including Let's Encrypt)
- Payment gateways and email notifications

### Option 2: Manual Configuration

Create a `config/development.toml` file:

```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
username = "rcommerce_dev"
password = "devpass"
database = "rcommerce_dev"
pool_size = 5

[cache]
cache_type = "Memory"

[payment]
test_mode = true
```

### Database Setup

**Create Database (PostgreSQL):**

```bash
# Create database
psql -U postgres -c "CREATE DATABASE rcommerce_dev;"
psql -U postgres -c "CREATE USER rcommerce_dev WITH PASSWORD 'devpass';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce_dev TO rcommerce_dev;"
```

**Run Migrations:**

```bash
# If using setup wizard, migrations run automatically
# Otherwise, run manually:
./target/release/rcommerce db migrate -c config.toml
```

## Running the Server

### Development Mode

```bash
# Run with hot reload
cargo watch -x run

# Or run directly
cargo run

# With specific config
cargo run -- --config config/development.toml
```

### Production Mode

```bash
# Build release binary
cargo build --release

# Run with production config
./target/release/rcommerce --config config/production.toml
```

## Verify Installation

### Health Check

```bash
curl http://localhost:8080/health
```

Expected response:

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "database": "connected",
  "cache": "connected",
  "timestamp": "2024-01-23T14:13:35Z"
}
```

## API Quick Start

### Step 1: Register a Customer

```bash
# Register a new customer
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepassword123",
    "first_name": "John",
    "last_name": "Doe"
  }' | jq
```

### Step 2: Login

```bash
# Login to get JWT token
JWT=$(curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepassword123"
  }' | jq -r '.access_token')

echo "JWT Token: $JWT"
```

### Step 3: Create a Guest Cart

```bash
# Create a guest cart
CART_RESPONSE=$(curl -X POST http://localhost:8080/api/v1/carts/guest \
  -H "Content-Type: application/json")

CART_ID=$(echo $CART_RESPONSE | jq -r '.id')
SESSION_TOKEN=$(echo $CART_RESPONSE | jq -r '.session_token')

echo "Cart ID: $CART_ID"
echo "Session Token: $SESSION_TOKEN"
```

### Step 4: List Products

```bash
# Get available products
PRODUCTS=$(curl -X GET http://localhost:8080/api/v1/products \
  -H "Authorization: Bearer $JWT")

PRODUCT_ID=$(echo $PRODUCTS | jq -r '.products[0].id')
echo "Product ID: $PRODUCT_ID"
```

### Step 5: Add Items to Cart

```bash
# Add a product to the cart
curl -X POST http://localhost:8080/api/v1/carts/$CART_ID/items \
  -H "Authorization: Bearer $JWT" \
  -H "Content-Type: application/json" \
  -d '{
    "product_id": "'$PRODUCT_ID'",
    "quantity": 2
  }' | jq
```

### Step 6: View Cart

```bash
# Get current cart
curl -X GET http://localhost:8080/api/v1/carts/$CART_ID | jq
```

### Step 7: Checkout

#### 7.1 Initiate Checkout

```bash
# Start checkout process to get shipping rates
curl -X POST http://localhost:8080/api/v1/checkout/initiate \
  -H "Authorization: Bearer $JWT" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "'$CART_ID'",
    "shipping_address": {
      "first_name": "John",
      "last_name": "Doe",
      "address1": "123 Main St",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "customer_email": "user@example.com"
  }' | jq
```

#### 7.2 Select Shipping

```bash
# Select a shipping rate (use rate from previous response)
curl -X POST http://localhost:8080/api/v1/checkout/shipping \
  -H "Authorization: Bearer $JWT" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "'$CART_ID'",
    "shipping_rate": {
      "provider_id": "ups",
      "carrier": "UPS",
      "service_code": "ground",
      "service_name": "UPS Ground",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    }
  }' | jq
```

#### 7.3 Complete Checkout

```bash
# Complete checkout and create order
curl -X POST http://localhost:8080/api/v1/checkout/complete \
  -H "Authorization: Bearer $JWT" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "'$CART_ID'",
    "shipping_address": {
      "first_name": "John",
      "last_name": "Doe",
      "address1": "123 Main St",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "payment_method": {
      "type": "card",
      "token": "tok_visa"
    },
    "customer_email": "user@example.com",
    "selected_shipping_rate": {
      "provider_id": "ups",
      "carrier": "UPS",
      "service_code": "ground",
      "service_name": "UPS Ground",
      "rate": "10.00",
      "currency": "USD",
      "delivery_days": 5,
      "total_cost": "10.00"
    }
  }' | jq
```

### Step 8: View Orders

```bash
# List customer orders
curl -X GET http://localhost:8080/api/v1/orders \
  -H "Authorization: Bearer $JWT" | jq
```

## Complete Example Script

Save this as `test_api.sh`:

```bash
#!/bin/bash

API_URL="http://localhost:8080"

echo "=== R Commerce API Test ==="

# 1. Register
echo "1. Registering customer..."
curl -s -X POST "$API_URL/api/v1/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "first_name": "Test",
    "last_name": "User"
  }' | jq

# 2. Login
echo "2. Logging in..."
JWT=$(curl -s -X POST "$API_URL/api/v1/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }' | jq -r '.access_token')

echo "JWT: ${JWT:0:20}..."

# 3. Create Cart
echo "3. Creating guest cart..."
CART=$(curl -s -X POST "$API_URL/api/v1/carts/guest" \
  -H "Content-Type: application/json")

CART_ID=$(echo $CART | jq -r '.id')
echo "Cart ID: $CART_ID"

echo "=== Test Complete ==="
```

Run it:

```bash
chmod +x test_api.sh
./test_api.sh
```

## Next Steps

- [Installation Guide](installation.md) - Detailed installation instructions
- [Configuration Guide](configuration.md) - Complete configuration reference
- [API Reference](../api-reference/index.md) - Start building with the API
- [Development Guide](../development/index.md) - Set up your development environment

## Troubleshooting

### Port Already in Use

```bash
# Find process using port 8080
lsof -i :8080

# Kill the process or use a different port
# Edit config/development.toml and change the port
```

### Database Connection Failed

```bash
# Check PostgreSQL is running
pg_isready -h localhost -p 5432

# Check credentials
psql -U rcommerce_dev -d rcommerce_dev -h localhost -W
```

### Build Errors

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### API Authentication Errors

```bash
# Verify JWT token is valid
curl -X GET http://localhost:8080/api/v1/customers/me \
  -H "Authorization: Bearer $JWT" -v

# Check token expiration (tokens expire after 24 hours)
```

## Getting Help

- **Documentation**: Browse the full documentation
- **GitHub Issues**: Report bugs and request features
- **Discord**: Join the community for real-time help
