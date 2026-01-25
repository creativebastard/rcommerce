# R commerce CLI Reference

The R commerce CLI provides a powerful command-line interface for managing your headless ecommerce platform, automating tasks, and interacting with the API directly from your terminal.

## Installation

### Install from Source

```bash
# Clone repository
git clone https://gitee.com/captainjez/gocart.git
cd gocart

# Build release version
cargo build --release

# Move binary to system path
sudo cp target/release/rcommerce /usr/local/bin/

# Verify installation
rcommerce --version
rcommerce help
```

### Install via Cargo

```bash
# Install from crates.io (when published)
cargo install rcommerce-cli

# Verify installation
rcommerce --version
```

### Install via Package Manager

**Linux/macOS:**
```bash
# Using Homebrew (when available)
brew tap rcommerce/tap
brew install rcommerce

# Using apt (Debian/Ubuntu) - future
sudo apt-add-repository 'deb https://apt.rcommerce.app stable main'
sudo apt install rcommerce
```

**FreeBSD:**
```bash
# Using pkg
sudo pkg install rcommerce

# From ports
cd /usr/ports/commerce/rcommerce && sudo make install
```

## Global Options

```bash
# Display help
rcommerce --help
rcommerce help
rcommerce [command] --help

# Version information
rcommerce --version
rcommerce -V

# Configuration file
rcommerce --config /path/to/rcommerce.toml
rcommerce -c /etc/rcommerce/config.toml

# Verbose output
rcommerce --verbose
rcommerce -v

# API endpoint (for remote management)
rcommerce --api-url https://api.yourstore.com
rcommerce --api-key sk_live_xxx

# Output format
rcommerce --format json    # Options: json, table, yaml, csv
rcommerce --no-color       # Disable colors

# Dry run (test without making changes)
rcommerce --dry-run

# Batch mode (disable interactive prompts)
rcommerce --batch
```

## Core Commands

### Server Management

```bash
# Start server
rcommerce server start
rcommerce server start --config /etc/rcommerce/production.toml
rcommerce server start --port 3000 --host 0.0.0.0

# Stop server
rcommerce server stop
rcommerce server stop --graceful --timeout 30s

# Restart server
rcommerce server restart

# Server status
rcommerce server status
rcommerce server status --json

# Server logs
rcommerce server logs
rcommerce server logs --follow --lines 100
rcommerce server logs --since "1 hour ago" --level error

# Server health check
rcommerce server health
rcommerce server health --watch --interval 30s

# Server metrics
rcommerce server metrics
rcommerce server metrics --prometheus

# Server configuration show
rcommerce server config show
rcommerce server config validate
rcommerce server config reload  # Reload without restart
```

### Product Management

```bash
# List products
rcommerce product list
rcommerce product list --limit 50 --page 2
rcommerce product list --status active --sort price:desc
rcommerce product list --format json | jq '.products[].name'

# Get product details
rcommerce product get <product-id>
rcommerce product get prod_123 --include variants,images
rcommerce product get --sku TSHIRT-001

# Create product
rcommerce product create \
  --name "Premium T-Shirt" \
  --price 29.99 \
  --sku TSHIRT-001 \
  --description "High quality cotton t-shirt" \
  --status active

# Interactive product creation
rcommerce product create --interactive
rcommerce product create --template template.json

# Update product
rcommerce product update <product-id> \
  --price 24.99 \
  --status on_sale

# Delete product
rcommerce product delete <product-id>
rcommerce product delete <product-id> --force --reason "Discontinued"

# Upload product images
rcommerce product image upload <product-id> image1.jpg image2.jpg
rcommerce product image upload <product-id> --source https://example.com/image.jpg
rcommerce product image upload <product-id> --variant sku-red-large

# Manage product variants
rcommerce product variant list <product-id>
rcommerce product variant create <product-id> \
  --title "Red Large" \
  --price 29.99 \
  --sku TSHIRT-RED-L \
  --inventory 100

rcommerce product variant update <variant-id> --inventory 50

# Bulk product operations
rcommerce product bulk create products.csv --dry-run
rcommerce product bulk update products.json --format json
rcommerce product bulk delete --filter "status:draft" --confirm

# Product inventory management
rcommerce product inventory set <product-id> --quantity 50 --location "warehouse-1"
rcommerce product inventory adjust <product-id> --delta -5 --reason "sold"
rcommerce product inventory list <product-id>

# Product collections/categories
rcommerce collection list
rcommerce collection create --name "Summer Collection" --handle summer
rcommerce collection add-product <collection-id> <product-id>

# Product search
rcommerce product search "t-shirt"
rcommerce product search --query "category:outdoor price_range:20-50"
rcommerce product search --advanced '{"category": "clothing", "status": "active"}'
```

### Order Management

```bash
# List orders
rcommerce order list
rcommerce order list --status pending --limit 20
rcommerce order list --customer cus_123 --sort created_at:desc
rcommerce order list --created-after "2024-01-01" --total-min 100

# Get order details
rcommerce order get <order-id>
rcommerce order get ord_123 --include items,payments,fulfillments
rcommerce order get --number "ORD-1001"

# Create order (manual/admin)
rcommerce order create \
  --customer cus_123 \
  --line-items '[{"product_id": "prod_1", "quantity": 2}]' \
  --billing-address '{...}' \
  --shipping-address '{...}'

# Update order
rcommerce order update <order-id> --status processing
rcommerce order update <order-id> --add-note "Priority shipping requested"
rcommerce order update <order-id> --tags "vip,holiday"

# Cancel order
rcommerce order cancel <order-id> --reason "Customer requested" --refund full

# Order fulfillment
rcommerce order fulfill <order-id> --items '{"var_1": 2}' \
  --tracking-number "1Z999AA10123456784" \
  --carrier ups

rcommerce order fulfill <order-id> --partial --items '{"var_1": 1}'

# Order refunds
rcommerce order refund <order-id> --amount 25.00 --reason "Damaged item" \
  --items '[{"line_item_id": "item_1", "quantity": 1}]'

rcommerce order refund <order-id> --full --reason "Order cancellation"

# Order editing
rcommerce order edit <order-id> add-item --product-id prod_123 --quantity 2
rcommerce order edit <order-id> remove-item --item-id item_456
rcommerce order edit <order-id> update-quantity --item-id item_789 --quantity 3

# Order split/combine
rcommerce order split <order-id> --by-availability
rcommerce order split <order-id> --by-destination
rcommerce order combine <order-id-1> <order-id-2>

# Order fraud review
rcommerce order review <order-id> --fraud-score 85 --reason "High value + new customer"
rcommerce order approve <order-id> --fraud-review
rcommerce order hold <order-id> --reason "Manual review required"

# Bulk order operations
rcommerce order bulk update --filter "status:pending" --set "status:on_hold"
rcommerce order bulk export --start-date 2024-01-01 --end-date 2024-01-31 --format csv
```

### Customer Management

```bash
# List customers
rcommerce customer list
rcommerce customer list --limit 100 --page 1 --sort created_at:desc
rcommerce customer list --query "email:@example.com"
rcommerce customer list --group wholesale

# Get customer details
rcommerce customer get <customer-id>
rcommerce customer get --email "customer@example.com"
rcommerce customer get cus_123 --include orders,addresses

# Create customer
rcommerce customer create \
  --email "new@example.com" \
  --first-name "John" \
  --last-name "Doe" \
  --phone "+1234567890"

# Update customer
rcommerce customer update <customer-id> \
  --first-name "Jane" \
  --accepts-marketing true

# Customer addresses
rcommerce customer address add <customer-id> \
  --first-name "John" \
  --last-name "Doe" \
  --street "123 Main St" \
  --city "New York" \
  --state "NY" \
  --postal-code "10001" \
  --country "US" \
  --is-default

rcommerce customer address list <customer-id>
rcommerce customer address update <address-id> --is-default

# Customer groups
rcommerce customer group create --name "VIP Customers" --metadata '{"discount": 10}'
rcommerce customer group add-customer <group-id> <customer-id>
rcommerce customer group list

# Customer orders
rcommerce customer orders <customer-id>
rcommerce customer orders <customer-id> --status completed
```

### Payment Management

```bash
# Process payment
rcommerce payment process <order-id> \
  --gateway stripe \
  --payment-method pm_card_visa \
  --amount 99.99 \
  --currency USD

# Capture authorized payment
rcommerce payment capture <payment-id>
rcommerce payment capture <payment-id> --amount 50.00

# Refund payment
rcommerce payment refund <payment-id> --amount 25.00 --reason "partial refund"
rcommerce payment refund <payment-id> --full --reason "order cancellation"

# List payments
rcommerce payment list --order <order-id>
rcommerce payment list --customer <customer-id> --limit 20

# Payment methods
rcommerce payment methods list <customer-id>
rcommerce payment methods add <customer-id> --type card --token tok_visa

# Payment gateway management
rcommerce gateway enable stripe --config '{"secret_key": "sk_..."}'
rcommerce gateway disable paypal
rcommerce gateway list
rcommerce gateway test stripe

# Handle webhook
rcommerce gateway webhook <gateway> --endpoint https://api.yourstore.com/webhooks
```

### Shipping & Fulfillment

```bash
# Create shipping label
rcommerce shipping label create <order-id> \
  --provider shipstation \
  --service-code usps_priority \
  --package-weight 1.2 \
  --package-units lb

# Get shipping rates
rcommerce shipping rates --from 10001 --to 90210 \
  --weight 1.2 --unit lb \
  --package-type small

# Fulfillment providers
rcommerce fulfillment providers list
rcommerce fulfillment enable shipstation --api-key key --api-secret secret

# Track shipments
rcommerce tracking create <shipment-id> --tracking-number 1Z999AA \
  --carrier ups

rcommerce tracking update <tracking-number> --status delivered
```

### Inventory Management

```bash
# Check inventory
rcommerce inventory check <product-id> --location "warehouse-1"
rcommerce inventory check --sku TSHIRT-RED-L --quantity 10

# Update inventory
rcommerce inventory adjust <product-id> --delta -5 --reason "sold"
rcommerce inventory adjust <product-id> --set 100 --location "warehouse-1"

# Inventory locations
rcommerce location create --name "Main Warehouse" --address '{...}'
rcommerce location list
rcommerce location update <location-id> --priority high

# Stock transfers
rcommerce transfer create --from warehouse-1 --to warehouse-2 \
  --product <product-id> --quantity 50

rcommerce transfer complete <transfer-id>
```

### Catalog Management

```bash
# Categories
rcommerce category list
rcommerce category create --name "Clothing" --slug clothing --parent-id cat_parent
rcommerce category update <category-id> --name "Premium Clothing"
rcommerce category tree

# Collections
rcommerce collection create --name "Summer Sale" --type automated \
  --filters '{"price": {"gte": 20, "lte": 100}}'

rcommerce collection add-product <collection-id> <product-id>

# Product types
rcommerce type create --name "T-Shirt" --attributes '[{"name": "size", "values": ["S","M","L"]}]'

# Product tags
rcommerce tag create --value "summer2024"
rcommerce tag list
```

### Discounts & Promotions

```bash
# Create discount code
rcommerce discount create \
  --code SAVE20 \
  --type percentage \
  --value 20 \
  --min-cart-value 50.00 \
  --usage-limit 100 \
  --starts-at "2024-01-01T00:00:00Z" \
  --ends-at "2024-01-31T23:59:59Z"

# Create automatic discount
rcommerce discount create-auto \
  --name "Free Shipping Over $75" \
  --type free_shipping \
  --min-cart-value 75.00 \
  --regions "US,CA"

# Discount rules
rcommerce discount rule add <discount-id> \
  --type "product" \
  --operator "in" \
  --value '["prod_1", "prod_2"]'

# Apply discount to order
rcommerce order discount add <order-id> --code SAVE20

# List active discounts
rcommerce discount list --active
rcommerce discount validate <code> --cart-value 100.00
```

### Reports & Analytics

```bash
# Sales reports
rcommerce report sales --period today
rcommerce report sales --period week --format json
rcommerce report sales --start 2024-01-01 --end 2024-01-31 > january_sales.json

# Order reports
rcommerce report orders --status shipped --by-day
rcommerce report fulfillment --by-provider

# Customer reports
rcommerce report customers --segment new --period month
rcommerce report customers --segment returning --by-country

# Inventory reports
rcommerce report inventory --low-stock --threshold 10
rcommerce report inventory --by-location
rcommerce report inventory-movement --product <product-id> --period week

# Export data
rcommerce export products --format csv > products.csv
rcommerce export orders --start-date 2024-01-01 --format json > orders.json
rcommerce export customers --include-addresses --format csv > customers.csv
```

### User Management

```bash
# Create admin user
rcommerce user create --email "admin@yourstore.com" \
  --first-name "Admin" \
  --role admin \
  --permissions "orders:write,products:write,customers:write"

# List users
rcommerce user list
rcommerce user list --role admin

# Manage API keys
rcommerce api-key create --user-id usr_123 --name "Production Key" --permissions "*"
rcommerce api-key list --user usr_123
rcommerce api-key revoke <key-id>
rcommerce api-key rotate <key-id>

# Manage permissions
rcommerce permission list
docker permission assign <user-id> "orders:write"
rcommerce permission revoke <user-id> "products:delete"
```

### Store Configuration

```bash
# View configuration
rcommerce config show
rcommerce config show --section payments.stripe

# Edit configuration
rcommerce config set payments.stripe.secret_key sk_live_xxx
rcommerce config set shipping.default_provider shipstation

# Validate configuration
rcommerce config validate
rcommerce config test --section database

# Reload configuration without restart
rcommerce config reload

# Export/import configuration
rcommerce config export > config.json
rcommerce config import config.json --dry-run
```

### Database Management

```bash
# Run migrations
rcommerce db migrate status
rcommerce db migrate run
rcommerce db migrate rollback --steps 1
rcommerce db migrate create add_customer_note_field

# Database health
rcommerce db check
rcommerce db connections
rcommerce db slow-queries --threshold 1000

# Backup/restore
rcommerce db backup --output backup.sql
rcommerce db restore backup.sql

# Seed database
rcommerce db seed --environment development
rcommerce db seed --template starter

# Database console
rcommerce db console
rcommerce db query "SELECT COUNT(*) FROM products"
```

### Webhook Management

```bash
# List webhooks
rcommerce webhook list
rcommerce webhook list --event order.created

# Create webhook
rcommerce webhook create \
  --url https://your-app.com/webhooks/orders \
  --events order.created,order.updated \
  --secret whsec_xxx

# Test webhook
rcommerce webhook test <webhook-id> --event order.created
rcommerce webhook test <webhook-id> --payload '{"test": true}'

# Trigger webhook manually
rcommerce webhook trigger <webhook-id> --data <json-data>

# Webhook deliveries
rcommerce webhook deliveries <webhook-id> --limit 20
```

### Cache Management

```bash
# View cache stats
rcommerce cache stats
rcommerce cache keys --pattern "products:*"

# Clear cache
rcommerce cache clear
rcommerce cache clear --pattern "products:*"
rcommerce cache clear --pattern "orders:*"

# Warm cache
rcommerce cache warm-products
rcommerce cache warm all
```

## Bulk Operations

```bash
# Process CSV/JSON files
rcommerce bulk import products products.csv --format csv
rcommerce bulk import customers customers.json --format json
rcommerce bulk import orders orders.csv --auto-process

# Export in bulk
rcommerce bulk export products --filter "status:active" --format csv > active_products.csv

# Batch processing
rcommerce bulk update orders --query "status:pending" --set "status:on_hold"
rcommerce bulk delete products --filter "status:archived" --confirm

# Process automation file
rcommerce process automation.json --dry-run
```

## Scripting & Automation

### Shell Script Example

```bash
#!/bin/bash
# create-products.sh - Bulk create products from CSV

INPUT_FILE="$1"

# Check if file exists
if [ ! -f "$INPUT_FILE" ]; then
  echo "Usage: $0 products.csv"
  exit 1
fi

# Read CSV and create products
while IFS=',' read -r name price sku category; do
  # Skip header
  if [ "$name" = "name" ]; then continue; fi
  
  echo "Creating product: $name"
  
  rcommerce product create \
    --name "$name" \
    --price "$price" \
    --sku "$sku" \
    --category "$category" \
    --status active
    
  if [ $? -eq 0 ]; then
    echo "✓ Created $name"
  else
    echo "✗ Failed to create $name"
  fi
done < "$INPUT_FILE"

echo "Product creation complete!"
```

### Advanced Script with Error Handling

```bash
#!/bin/bash
# process-orders.sh - Automated order processing

set -euo pipefail

API_KEY="${R_COMMERCE_API_KEY:-}"
ENVIRONMENT="${ENVIRONMENT:-production}"

# Function to process pending orders
process_pending_orders() {
  echo "Fetching pending orders..."
  
  # Get all pending orders
  orders=$(rcommerce --api-key "$API_KEY" order list \
    --status pending \
    --format json | jq -r '.orders[] | .id')
  
  for order_id in $orders; do
    echo "Processing order: $order_id"
    
    # Check inventory
    inventory_result=$(rcommerce --api-key "$API_KEY" order check-inventory "$order_id")
    
    if echo "$inventory_result" | grep -q "sufficient"; then
      echo "Inventory sufficient, confirming order..."
      
      # Update status to confirmed
      rcommerce --api-key "$API_KEY" order update "$order_id" \
        --status confirmed \
        --add-note "Auto-confirmed: inventory available"
      
      # Attempt payment if not paid
      payment_status=$(rcommerce --api-key "$API_KEY" order get "$order_id" | \
        jq -r '.payment_status')
      
      if [ "$payment_status" = "pending" ]; then
        echo "Processing payment..."
        rcommerce --api-key "$API_KEY" payment process "$order_id" \
          --gateway stripe \
          --capture
      fi
      
      echo "✓ Order $order_id processed successfully"
    else
      echo "✗ Order $order_id: insufficient inventory, setting on hold"
      rcommerce --api-key "$API_KEY" order update "$order_id" \
        --status on_hold \
        --add-note "Auto: insufficient inventory"
    fi
  done
}

# Main execution
echo "Starting automated order processing for $ENVIRONMENT"
process_pending_orders
echo "Order processing complete!"
```

### Python Automation Script

```python
#!/usr/bin/env python3
"""
Product price updater based on costs
"""

import sys
import json

def update_product_prices(min_profit_margin=0.3):
    """Update all product prices based on cost + margin"""
    
    # Get all products
    result = subprocess.run(
        ["rcommerce", "product", "list", "--format", "json", "--limit", "1000"],
        capture_output=True,
        text=True
    )
    
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
        sys.exit(1)
    
    products = json.loads(result.stdout)
    
    for product in products["products"]:
        product_id = product["id"]
        cost = product.get("cost", 0)
        
        if cost <= 0:
            print(f"Skipping {product['name']}: no cost set")
            continue
        
        # Calculate selling price
        new_price = cost / (1 - min_profit_margin)
        new_price = round(new_price, 2)
        
        print(f"Updating {product['name']}: ${cost} → ${new_price}")
        
        # Update price
        update_result = subprocess.run([
            "rcommerce", "product", "update", product_id,
            "--price", str(new_price),
            "--add-note", f"Auto-price: {min_profit_margin*100}% margin"
        ])
        
        if update_result.returncode == 0:
            print(f"✓ Updated {product['name']}")
        else:
            print(f"✗ Failed to update {product['name']}")

if __name__ == "__main__":
    update_product_prices(min_profit_margin=0.4)
```

## CLI Configuration File

```toml
# ~/.rcommerce.toml

[cli]
# Default API endpoint
default_api_url = "https://api.yourstore.com"
default_api_key = "sk_live_xxx"

# Output format
default_format = "table"  # json, table, yaml, csv
enable_colors = true
verbose = false

# Command aliases
[cli.aliases]
st = "server status"
p = "product"
o = "order"
c = "customer"

# Custom shortcuts
[cli.shortcuts]
# Add product shortcut
add-shirt = ["product", "create", "--category", "clothing", "--status", "active"]

# Server management
[server]
auto_start = false
log_level = "info"

# Bulk operations
[bulk]
default_chunk_size = 100
max_concurrent = 5
retry_failed = true

# API settings
[api]
timeout_seconds = 30
max_retries = 3
retry_delay_ms = 1000
rate_limit_per_second = 50

# Notification settings (for CLI operations)
[notifications]
webhook_url = "https://your-app.com/webhooks/rcommerce-cli"
events = ["bulk.import.completed", "bulk.export.completed"]
```

## Autocomplete Setup

### Bash

```bash
# Generate autocomplete script
rcommerce completion bash > /usr/local/share/bash-completion/completions/rcommerce

# Or add to .bashrc
echo 'source <(rcommerce completion bash)' >> ~/.bashrc
```

### Zsh

```bash
# Generate autocomplete script
rcommerce completion zsh > /usr/local/share/zsh/site-functions/_rcommerce

# Or add to .zshrc
echo 'source <(rcommerce completion zsh)' >> ~/.zshrc
```

### Fish

```bash
# Generate autocomplete script
rcommerce completion fish > ~/.config/fish/completions/rcommerce.fish
```

## CLI Architecture

The CLI is built with `clap` for argument parsing and structured as a command tree:

```rust
// src/cli/mod.rs

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rcommerce")]
#[command(about = "R commerce headless ecommerce platform CLI")]
#[command(version)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    api_url: Option<String>,

    #[arg(long)]
    api_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start and manage the R commerce server
    Server(ServerCommand),
    
    /// Manage products and variants
    Product(ProductCommand),
    
    /// Manage orders and fulfillments
    Order(OrderCommand),
    
    /// Manage customers and addresses
    Customer(CustomerCommand),
    
    /// Manage discounts and promotions
    Discount(DiscountCommand),
    
    /// Manage shipping and fulfillment
    Shipping(ShippingCommand),
    
    /// View reports and analytics
    Report(ReportCommand),
    
    /// Manage webhooks
    Webhook(WebhookCommand),
    
    /// Manage database operations
    Db(DbCommand),
    
    /// Manage users and permissions
    User(UserCommand),
    
    /// Configuration management
    Config(ConfigCommand),
    
    /// Bulk operations
    Bulk(BulkCommand),
    
    /// Cache management
    Cache(CacheCommand),
}

// Implementation for each command module
mod server;
mod product;
mod order;
mod customer;
mod discount;
mod shipping;
mod report;
mod webhook;
mod db;
mod user;
mod config;
mod bulk;
mod cache;
```

## Error Handling

The CLI provides clear error messages:

```bash
# Example error output
$ rcommerce product create --name "Test" --price invalid
Error: Invalid value for '--price': invalid float "invalid"

$ rcommerce product get invalid-id
Error: Product not found: invalid-id

$ rcommerce order list --status invalid_status
Error: Invalid status 'invalid_status'. Valid options: pending, confirmed, processing, completed, cancelled

$ rcommerce server start --port 80
Error: Cannot start server: Port 80 already in use
```

## Environment Variables

```bash
# API credentials
RCOMMERCE_API_URL=https://api.yourstore.com
RCOMMERCE_API_KEY=sk_live_xxx

# CLI behavior
RCOMMERCE_CONFIG=/etc/rcommerce/cli.toml
RCOMMERCE_VERBOSE=true
RCOMMERCE_FORMAT=json
RCOMMERCE_BATCH_MODE=true

# Server connection (when using server commands locally)
RCOMMERCE_SERVER_HOST=localhost
RCOMMERCE_SERVER_PORT=8080
```

## Exit Codes

The CLI returns standard exit codes:

- `0` - Success
- `1` - General error
- `2` - Invalid arguments
- `3` - Not found
- `4` - Permission denied
- `5` - Rate limited
- `10` - Configuration error
- `11` - Network error
- `12` - Timeout
- `20` - Validation failed
- `21` - Conflict
- `22` - Rate limited

## Best Practices

### 1. Use CLI in Scripts

```bash
# Always check exit code
if ! rcommerce product get "$PRODUCT_ID" > /dev/null 2>&1; then
  echo "Product not found, creating..."
  rcommerce product create ...
fi
```

### 2. Safe Bulk Operations

```bash
# Always use --dry-run first
rcommerce bulk delete products --filter "status:archived" --dry-run

# Then confirm with explicit flag
rcommerce bulk delete products --filter "status:archived" --confirm
```

### 3. Idempotent Operations

```bash
# Safe to run multiple times
rcommerce config set payments.stripe.enabled true

# Create with idempotency
rcommerce product create --sku UNIQUE-001 || echo "Product already exists"
```

### 4. Secure Credentials

```bash
# Never put API keys in command line
# Use environment variables instead
export RCOMMERCE_API_KEY="sk_live_xxx"
rcommerce order list
```

### 5. Version Control Scripts

```bash
# Keep automation scripts in version control
# Use encrypted secrets management
git add scripts/automated-order-processing.sh
# Never commit secrets!
echo "secrets.env" >> .gitignore
```

This comprehensive CLI documentation provides everything needed to manage R commerce via command line, including server management, product/order operations, bulk processing, and automation examples."