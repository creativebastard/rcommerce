# Import Guide

This guide covers importing data into R Commerce from various e-commerce platforms and file formats.

## Overview

R Commerce provides multiple import methods:

- **Platform migrations** - Direct import from Shopify, WooCommerce, Magento
- **File imports** - CSV, JSON, XML file uploads
- **API imports** - Programmatic bulk imports
- **CLI tools** - Command-line import utilities

## Importing from Shopify

### Prerequisites

1. Shopify store admin access
2. Private app or custom app with API credentials
3. R Commerce API key with write permissions

### Export from Shopify

#### Products

```bash
# Using Shopify CLI
shopify theme pull

# Or export via Admin API
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/products.json?limit=250" \
  -H "X-Shopify-Access-Token: your_access_token"
```

#### Orders

```bash
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/orders.json?status=any&limit=250" \
  -H "X-Shopify-Access-Token: your_access_token"
```

#### Customers

```bash
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/customers.json?limit=250" \
  -H "X-Shopify-Access-Token: your_access_token"
```

### Import to R Commerce

```bash
# Import products
rcommerce import shopify products \
  --source products.json \
  --config config.toml \
  --batch-size 100

# Import orders with customers
rcommerce import shopify orders \
  --source orders.json \
  --import-customers \
  --config config.toml

# Import everything
rcommerce import shopify all \
  --store your-store.myshopify.com \
  --token your_access_token \
  --config config.toml
```

### Shopify Import Options

| Option | Description | Default |
|--------|-------------|---------|
| `--batch-size` | Records per batch | 100 |
| `--skip-images` | Skip product image download | false |
| `--image-base-url` | Override image URL prefix | - |
| `--currency-map` | Map currencies (USD:USDT) | - |
| `--skip-existing` | Skip existing SKUs | false |
| `--update-existing` | Update existing products | true |

## Importing from WooCommerce

### Export from WooCommerce

#### Using WordPress CLI

```bash
# Export products
wp wc product list --format=json --user=admin > woocommerce_products.json

# Export orders
wp wc shop_order list --format=json --user=admin > woocommerce_orders.json

# Export customers
wp user list --role=customer --format=json > woocommerce_customers.json
```

#### Using REST API

```bash
# Get WooCommerce credentials
CONSUMER_KEY=ck_your_key
CONSUMER_SECRET=cs_your_secret

# Export products
curl -X GET "https://yourstore.com/wp-json/wc/v3/products?per_page=100" \
  -u "$CONSUMER_KEY:$CONSUMER_SECRET" \
  -H "Content-Type: application/json"
```

### Import to R Commerce

> **Note:** For WooCommerce platform imports, use the base store URL (e.g., `https://your-store.com`). 
> The `/wp-json/wc/v3` path is added automatically by the importer.

```bash
# Import products from file
rcommerce import woocommerce products \
  --source woocommerce_products.json \
  --config config.toml

# Import directly from WooCommerce API
rcommerce import platform woocommerce \
  --api-url https://your-store.com \
  --api-key YOUR_CONSUMER_KEY \
  --api-secret YOUR_CONSUMER_SECRET \
  --config config.toml

# Import with overwrite (update existing records)
rcommerce import platform woocommerce \
  --api-url https://your-store.com \
  --api-key YOUR_CONSUMER_KEY \
  --api-secret YOUR_CONSUMER_SECRET \
  --config config.toml \
  --overwrite

# Import with attribute mapping
rcommerce import woocommerce products \
  --source products.json \
  --attribute-map '{"pa_size":"size","pa_color":"color"}' \
  --config config.toml

# Import orders
rcommerce import woocommerce orders \
  --source orders.json \
  --payment-gateway-map '{"bacs":"bank_transfer","cod":"cash_on_delivery"}' \
  --config config.toml
```

#### WooCommerce Platform Import Behavior

When using `rcommerce import platform woocommerce`:

| Flag | Behavior |
|------|----------|
| (no flag) | Skips existing products, customers, and orders |
| `--overwrite` | Updates existing products, customers, and orders |
| `--skip-existing` | Always skips existing records (same as no flag) |

**What gets imported:**
- **Products** - All products with SKUs, images, and inventory
- **Customers** - All customers with billing/shipping addresses
- **Orders** - All orders with line items (linked to imported products and customers)

**Order Item Mapping:**
Order items are automatically linked to products by:
1. Matching SKU (preferred method)
2. Matching product name (fallback)

If a product is not found, the order item is skipped with a warning.

### WooCommerce-Specific Mappings

```toml
[import.woocommerce]
# Attribute mappings
attribute_map = { pa_size = "size", pa_color = "color", pa_material = "material" }

# Category mappings
category_map = { "clothing" = "apparel", "electronics" = "tech" }

# Status mappings
order_status_map = { 
  "processing" = "confirmed",
  "completed" = "completed", 
  "cancelled" = "cancelled",
  "on-hold" = "pending"
}

# Payment gateway mappings
payment_gateway_map = {
  "bacs" = "bank_transfer",
  "cod" = "cash_on_delivery",
  "stripe" = "stripe",
  "paypal" = "paypal"
}
```

## Importing from Magento

### Export from Magento

#### Using Magento CLI

```bash
# Export products
bin/magento export:products --format=json --output=products.json

# Export orders
bin/magento export:orders --format=json --output=orders.json

# Export customers
bin/magento export:customers --format=json --output=customers.json
```

#### Using REST API

```bash
# Get admin token
TOKEN=$(curl -X POST "https://magento.example.com/rest/V1/integration/admin/token" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}' | tr -d '"')

# Export products
curl -X GET "https://magento.example.com/rest/V1/products?searchCriteria[pageSize]=500" \
  -H "Authorization: Bearer $TOKEN"
```

### Import to R Commerce

```bash
# Import products
rcommerce import magento products \
  --source products.json \
  --config config.toml \
  --batch-size 50

# Import with store view mapping
rcommerce import magento products \
  --source products.json \
  --store-view-map '{"default":"en","german":"de"}' \
  --config config.toml

# Import customers with addresses
rcommerce import magento customers \
  --source customers.json \
  --include-addresses \
  --config config.toml
```

## CSV File Format

### Product CSV Format

```csv
sku,name,description,price,currency,category,tags,image_url,inventory_quantity,weight
SKU001,Premium T-Shirt,High quality cotton t-shirt,29.99,USD,apparel,"cotton,summer",https://cdn.example.com/tshirt.jpg,100,0.5
SKU002,Wireless Headphones,Bluetooth headphones with noise canceling,149.99,USD,electronics,"audio,wireless",https://cdn.example.com/headphones.jpg,50,0.3
```

### Required Columns

| Column | Required | Description |
|--------|----------|-------------|
| `sku` | Yes | Unique product identifier |
| `name` | Yes | Product name |
| `price` | Yes | Product price |
| `currency` | Yes | ISO 4217 currency code |

### Optional Columns

| Column | Description |
|--------|-------------|
| `description` | Product description |
| `short_description` | Brief description |
| `category` | Category name or ID |
| `tags` | Comma-separated tags |
| `image_url` | Primary image URL |
| `gallery_urls` | Comma-separated additional images |
| `inventory_quantity` | Stock quantity |
| `weight` | Product weight in kg |
| `dimensions` | LxWxH format (cm) |
| `status` | active/inactive/draft |

### Import CSV

```bash
# Basic import
rcommerce import csv products \
  --file products.csv \
  --config config.toml

# With custom delimiter
rcommerce import csv products \
  --file products.csv \
  --delimiter ";" \
  --encoding utf-8 \
  --config config.toml

# With column mapping
rcommerce import csv products \
  --file products.csv \
  --column-map '{"product_code":"sku","product_name":"name"}' \
  --config config.toml

# Dry run to preview
rcommerce import csv products \
  --file products.csv \
  --dry-run \
  --config config.toml
```

### Customer CSV Format

```csv
email,first_name,last_name,phone,accepts_marketing,address1,city,country,zip
john@example.com,John,Doe,+1234567890,true,123 Main St,New York,US,10001
jane@example.com,Jane,Smith,+0987654321,false,456 Oak Ave,Los Angeles,US,90210
```

### Order CSV Format

```csv
order_number,customer_email,order_date,total,status,line_items
1001,john@example.com,2024-01-15T10:00:00Z,59.99,completed,"[{\"sku\":\"SKU001\",\"qty\":2,\"price\":29.99}]"
1002,jane@example.com,2024-01-16T14:30:00Z,149.99,processing,"[{\"sku\":\"SKU002\",\"qty\":1,\"price\":149.99}]"
```

## JSON/XML Imports

### JSON Import

```bash
# Import products from JSON
rcommerce import json products \
  --file products.json \
  --config config.toml

# Import with schema validation
rcommerce import json products \
  --file products.json \
  --schema product-schema.json \
  --config config.toml
```

#### JSON Product Format

```json
{
  "products": [
    {
      "sku": "SKU001",
      "name": "Premium T-Shirt",
      "description": "High quality cotton t-shirt",
      "price": "29.99",
      "currency": "USD",
      "category": "apparel",
      "tags": ["cotton", "summer"],
      "images": [
        {
          "url": "https://cdn.example.com/tshirt.jpg",
          "alt": "T-Shirt Front",
          "position": 1
        }
      ],
      "inventory": {
        "quantity": 100,
        "track_inventory": true
      },
      "variants": [
        {
          "sku": "SKU001-S",
          "name": "Small",
          "price": "29.99",
          "options": {
            "size": "S"
          },
          "inventory_quantity": 25
        }
      ]
    }
  ]
}
```

### XML Import

```bash
# Import products from XML
rcommerce import xml products \
  --file products.xml \
  --xpath "//product" \
  --config config.toml
```

#### XML Product Format

```xml
<?xml version="1.0" encoding="UTF-8"?>
<products>
  <product>
    <sku>SKU001</sku>
    <name>Premium T-Shirt</name>
    <description>High quality cotton t-shirt</description>
    <price currency="USD">29.99</price>
    <category>apparel</category>
    <inventory>
      <quantity>100</quantity>
    </inventory>
  </product>
</products>
```

## CLI Import Commands

### General Import Command

```bash
rcommerce import [source] [entity] [options]
```

### Available Sources

| Source | Description |
|--------|-------------|
| `shopify` | Shopify store import |
| `woocommerce` | WooCommerce import |
| `magento` | Magento import |
| `csv` | CSV file import |
| `json` | JSON file import |
| `xml` | XML file import |

### Available Entities

| Entity | Description |
|--------|-------------|
| `products` | Product catalog |
| `customers` | Customer accounts |
| `orders` | Order history |
| `coupons` | Discount codes |
| `categories` | Product categories |
| `all` | Everything (platform imports only) |

### Common Options

| Option | Description |
|--------|-------------|
| `--config` | Path to config file |
| `--source` | Source file or URL |
| `--batch-size` | Records per batch |
| `--dry-run` | Preview without importing |
| `--skip-existing` | Skip duplicate records |
| `--update-existing` | Update existing records |
| `--continue-on-error` | Skip failed records |
| `--mapping-file` | Field mapping configuration |

### Import Progress and Monitoring

```bash
# Import with verbose output
rcommerce import csv products --file products.csv --verbose

# Import with progress bar
rcommerce import csv products --file products.csv --progress

# Save import log
rcommerce import csv products --file products.csv --log import.log

# Import with webhook notification
rcommerce import csv products --file products.csv --webhook https://yoursite.com/webhooks/import
```

## Import Validation

### Pre-Import Validation

```bash
# Validate without importing
rcommerce import csv products --file products.csv --validate-only

# Check for common issues
rcommerce import validate --file products.csv --type products
```

### Validation Rules

- **SKU uniqueness** - No duplicate SKUs allowed
- **Price format** - Valid decimal numbers
- **Currency codes** - Valid ISO 4217 codes
- **Email format** - Valid email for customers
- **Required fields** - All mandatory fields present
- **Image URLs** - Valid URL format
- **Category existence** - Categories must exist

### Post-Import Verification

```bash
# Verify import counts
rcommerce import verify --import-id import_abc123

# Compare with source
rcommerce import verify --import-id import_abc123 --source products.csv
```

## Import Best Practices

### Before Importing

1. **Backup your database** - Always backup before bulk imports
2. **Test on staging** - Verify import on non-production first
3. **Validate data** - Clean and validate source data
4. **Prepare mappings** - Define field mappings in advance
5. **Estimate time** - Large imports may take hours

### During Import

1. **Use batches** - Don't import everything at once
2. **Monitor progress** - Watch for errors and warnings
3. **Check resources** - Ensure sufficient CPU/memory
4. **Log everything** - Keep detailed import logs

### After Import

1. **Verify counts** - Compare source and destination counts
2. **Check samples** - Manually verify imported records
3. **Test functionality** - Ensure products/orders work correctly
4. **Update search index** - Rebuild search if needed

### Performance Tips

```toml
[import]
# Increase batch size for faster imports (if memory allows)
batch_size = 500

# Disable search indexing during import
skip_search_index = true

# Disable webhooks during import
skip_webhooks = true

# Parallel processing
workers = 4

# Disable foreign key checks (PostgreSQL)
disable_constraints = true
```

## Troubleshooting

### Common Import Errors

**"Duplicate SKU"**
- Use `--skip-existing` or `--update-existing`
- Check source data for duplicates

**"Invalid currency code"**
- Use ISO 4217 codes (USD, EUR, GBP)
- Check for typos in currency field

**"Category not found"**
- Create categories before importing
- Use `--auto-create-categories`

**"Image download failed"**
- Check image URLs are accessible
- Use `--skip-images` if not needed
- Configure image proxy if blocked

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug rcommerce import csv products --file products.csv

# Save detailed log
rcommerce import csv products --file products.csv --debug --log debug.log
```

### Import Recovery

If import fails mid-way:

```bash
# Resume from last successful batch
rcommerce import csv products --file products.csv --resume --import-id import_abc123

# Import remaining records only
rcommerce import csv products --file products.csv --skip-existing
```

## Related Documentation

- [Shopify Migration](../migration/shopify.md)
- [WooCommerce Migration](../migration/woocommerce.md)
- [Magento Migration](../migration/magento.md)
- [CLI Reference](../development/cli-reference.md)
