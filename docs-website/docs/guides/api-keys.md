# API Keys Guide

This guide covers everything you need to know about creating, managing, and using API keys in R Commerce.

## Overview

API keys provide secure, long-lived authentication for service-to-service communication. Unlike JWT tokens which are designed for user sessions, API keys are ideal for:

- Backend service integrations
- Webhook handlers
- ETL and data synchronization processes
- Third-party integrations
- Automated scripts and tools

## Creating API Keys

### Basic Creation

Create an API key using the CLI:

```bash
rcommerce api-key create --name "My Integration"
```

**Output:**
```
✅ API Key created successfully!
Key: aB3dEfGh.sEcReTkEy123456789abcdef1234567
Prefix: aB3dEfGh
Scopes: read
Created: 2026-01-15T10:30:00Z
Expires: Never

⚠️  IMPORTANT: Store this key securely. It will not be shown again!
```

### Creating with Specific Scopes

Specify scopes during creation:

```bash
rcommerce api-key create \
  --name "Product Sync Service" \
  --scopes "products:read,products:write"
```

### Creating with Expiration

Set an expiration date for enhanced security:

```bash
rcommerce api-key create \
  --name "Temporary Integration" \
  --scopes "orders:read" \
  --expires "2026-12-31T23:59:59Z"
```

### Creating with Rate Limits

Limit the number of requests per minute:

```bash
rcommerce api-key create \
  --name "Limited Access Key" \
  --scopes "products:read" \
  --rate-limit 100
```

## Managing API Keys

### Listing All Keys

View all API keys in your system:

```bash
rcommerce api-key list
```

**Output:**
```
┌──────────┬──────────────────────┬─────────────────┬─────────┬──────────────────────┐
│ Prefix   │ Name                 │ Scopes          │ Status  │ Last Used            │
├──────────┼──────────────────────┼─────────────────┼─────────┼──────────────────────┤
│ aB3dEfGh │ Product Sync Service │ products:write  │ Active  │ 2026-01-20T14:30:00Z │
│ Xy9ZaBcD │ Mobile App Backend   │ read            │ Active  │ 2026-01-20T15:45:00Z │
│ Mn7OpQrS │ Old Integration      │ write           │ Revoked │ 2026-01-10T09:00:00Z │
└──────────┴──────────────────────┴─────────────────┴─────────┴──────────────────────┘
```

### Viewing Key Details

Get detailed information about a specific key:

```bash
rcommerce api-key get aB3dEfGh
```

**Output:**
```
API Key Details
===============
Prefix:           aB3dEfGh
Name:             Product Sync Service
Scopes:           products:read, products:write
Status:           Active
Created:          2026-01-15T10:30:00Z
Created By:       admin@example.com
Expires:          Never
Rate Limit:       1000 req/min
Last Used:        2026-01-20T14:30:00Z
Last Used IP:     203.0.113.42
Revoked:          -
Revoke Reason:    -
```

### Revoking API Keys

Revoke a key to immediately disable it:

```bash
rcommerce api-key revoke aB3dEfGh --reason "Key compromised"
```

**Output:**
```
✅ API Key revoked successfully
Prefix: aB3dEfGh
Revoked at: 2026-01-20T16:00:00Z
Reason: Key compromised
```

> **Note:** Revoked keys cannot be reactivated. Create a new key if needed.

### Deleting API Keys

Permanently delete a key from the system:

```bash
rcommerce api-key delete aB3dEfGh
```

**Confirmation:**
```
⚠️  WARNING: This action cannot be undone!
Are you sure you want to delete API key 'aB3dEfGh'? [y/N]: y
✅ API Key deleted successfully
```

> **Note:** It's recommended to revoke keys before deletion to maintain an audit trail.

## Using API Keys

### In HTTP Requests

Include the API key in the Authorization header:

```bash
curl -X GET "https://api.rcommerce.app/api/v1/products" \
  -H "Authorization: Bearer aB3dEfGh.sEcReTkEy123456789abcdef1234567"
```

Or without the Bearer prefix:

```bash
curl -X GET "https://api.rcommerce.app/api/v1/products" \
  -H "Authorization: aB3dEfGh.sEcReTkEy123456789abcdef1234567"
```

### In JavaScript/TypeScript

```javascript
const API_KEY = process.env.RCOMMERCE_API_KEY;

async function getProducts() {
  const response = await fetch('https://api.rcommerce.app/api/v1/products', {
    headers: {
      'Authorization': `Bearer ${API_KEY}`,
      'Content-Type': 'application/json'
    }
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}
```

### In Python

```python
import os
import requests

API_KEY = os.environ.get('RCOMMERCE_API_KEY')
BASE_URL = 'https://api.rcommerce.app'

headers = {
    'Authorization': f'Bearer {API_KEY}',
    'Content-Type': 'application/json'
}

# Get products
response = requests.get(f'{BASE_URL}/api/v1/products', headers=headers)
products = response.json()

# Create an order
order_data = {
    'customer_id': '123e4567-e89b-12d3-a456-426614174000',
    'items': [
        {'product_id': '123e4567-e89b-12d3-a456-426614174001', 'quantity': 2}
    ]
}
response = requests.post(f'{BASE_URL}/api/v1/orders', json=order_data, headers=headers)
```

### In Go

```go
package main

import (
    "net/http"
    "os"
)

func main() {
    apiKey := os.Getenv("RCOMMERCE_API_KEY")
    
    req, err := http.NewRequest("GET", "https://api.rcommerce.app/api/v1/products", nil)
    if err != nil {
        panic(err)
    }
    
    req.Header.Set("Authorization", "Bearer "+apiKey)
    
    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        panic(err)
    }
    defer resp.Body.Close()
    
    // Process response...
}
```

### In PHP

```php
<?php
$apiKey = getenv('RCOMMERCE_API_KEY');

$ch = curl_init('https://api.rcommerce.app/api/v1/products');
curl_setopt($ch, CURLOPT_HTTPHEADER, [
    'Authorization: Bearer ' . $apiKey,
    'Content-Type: application/json'
]);
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

$response = curl_exec($ch);
curl_close($ch);

$products = json_decode($response, true);
?>
```

## Security Best Practices

### 1. Store Keys Securely

**Use Environment Variables:**

```bash
# .env file
RCOMMERCE_API_KEY=aB3dEfGh.sEcReTkEy123456789abcdef1234567
```

```python
# Python
import os
api_key = os.environ.get('RCOMMERCE_API_KEY')
```

```javascript
// Node.js
const apiKey = process.env.RCOMMERCE_API_KEY;
```

**Never:**
- ❌ Hardcode keys in source code
- ❌ Commit keys to version control
- ❌ Log keys or include them in error messages
- ❌ Share keys via email or chat

### 2. Use Minimal Scopes

Grant only the permissions necessary:

```bash
# Good: Specific access
rcommerce api-key create --name "Product Viewer" --scopes "products:read"

# Avoid: Broad access when not needed
rcommerce api-key create --name "Product Viewer" --scopes "read"
```

### 3. Rotate Keys Regularly

Establish a key rotation schedule:

```bash
# 1. Create new key
rcommerce api-key create --name "Product Sync - Q1 2026" --scopes "products:write"

# 2. Update your application with the new key

# 3. Test the new key

# 4. Revoke the old key
rcommerce api-key revoke <old_prefix> --reason "Scheduled rotation"
```

**Recommended Rotation Schedule:**
- Production keys: Every 90 days
- Staging keys: Every 180 days
- Development keys: As needed

### 4. Monitor Key Usage

Regularly check key activity:

```bash
# List keys with last used information
rcommerce api-key list

# Get detailed usage for a specific key
rcommerce api-key get <prefix>
```

**Watch for:**
- Keys that haven't been used in a long time (candidates for revocation)
- Unexpected IP addresses in `last_used_ip`
- Sudden spikes in usage

### 5. Use Separate Keys for Different Environments

```bash
# Development
rcommerce api-key create --name "Dev - Product Sync" --scopes "products:write"

# Staging
rcommerce api-key create --name "Staging - Product Sync" --scopes "products:write"

# Production
rcommerce api-key create --name "Prod - Product Sync" --scopes "products:read"
```

### 6. Set Appropriate Rate Limits

Protect your system from accidental abuse:

```bash
# High-volume service
rcommerce api-key create --name "Data Sync" --scopes "write" --rate-limit 10000

# Low-volume webhook
rcommerce api-key create --name "Webhook Handler" --scopes "orders:write" --rate-limit 100

# Occasional script
rcommerce api-key create --name "Daily Report" --scopes "read" --rate-limit 60
```

### 7. Revoke Compromised Keys Immediately

If a key is compromised:

```bash
# 1. Revoke immediately
rcommerce api-key revoke <prefix> --reason "Compromised - accidentally committed to GitHub"

# 2. Create a new key
rcommerce api-key create --name "Replacement Key" --scopes "<same_scopes>"

# 3. Update your application

# 4. Review access logs for unauthorized usage
```

## Use Case Examples

### Example 1: Read-Only API Key for Mobile App

**Scenario:** A mobile app needs to display products but shouldn't modify data.

```bash
# Create the key
rcommerce api-key create \
  --name "Mobile App - Production" \
  --scopes "products:read" \
  --rate-limit 1000
```

**Usage in mobile app:**
```javascript
// React Native example
const API_KEY = Config.RCOMMERCE_API_KEY; // From environment

async function fetchProducts() {
  const response = await fetch(`${API_URL}/api/v1/products`, {
    headers: {
      'Authorization': `Bearer ${API_KEY}`
    }
  });
  return await response.json();
}
```

**Security considerations:**
- Mobile apps can't securely store API keys
- Consider using a backend proxy or JWT authentication for production mobile apps
- This example is suitable for internal/enterprise apps

### Example 2: Webhook Handler API Key

**Scenario:** A payment gateway sends webhooks that need to update orders.

```bash
# Create the key with minimal required scopes
rcommerce api-key create \
  --name "Payment Gateway Webhook" \
  --scopes "orders:read,orders:write,payments:read,payments:write,webhooks:write" \
  --rate-limit 500
```

**Webhook handler implementation:**
```python
from flask import Flask, request
import os

app = Flask(__name__)
API_KEY = os.environ.get('RCOMMERCE_API_KEY')

def verify_webhook_signature(payload, signature):
    # Verify the webhook signature from payment gateway
    pass

@app.route('/webhooks/payment', methods=['POST'])
def handle_payment_webhook():
    # Verify webhook authenticity
    signature = request.headers.get('X-Webhook-Signature')
    if not verify_webhook_signature(request.data, signature):
        return 'Invalid signature', 401
    
    # Process webhook
    event = request.json
    
    if event['type'] == 'payment.success':
        # Update order using R Commerce API
        order_id = event['data']['order_id']
        update_data = {
            'payment_status': 'paid',
            'transaction_id': event['data']['transaction_id']
        }
        
        response = requests.patch(
            f'https://api.rcommerce.app/api/v1/orders/{order_id}',
            json=update_data,
            headers={'Authorization': f'Bearer {API_KEY}'}
        )
        
        if response.ok:
            return 'OK', 200
        return 'Failed to update order', 500
    
    return 'Event ignored', 200
```

### Example 3: Inventory Management Integration

**Scenario:** A warehouse management system needs to sync inventory levels.

```bash
# Create the key
rcommerce api-key create \
  --name "WMS Integration" \
  --scopes "inventory:read,inventory:write,products:read,orders:read" \
  --rate-limit 5000
```

**Sync script:**
```python
import requests
import os
from datetime import datetime

API_KEY = os.environ.get('RCOMMERCE_API_KEY')
BASE_URL = 'https://api.rcommerce.app'
HEADERS = {'Authorization': f'Bearer {API_KEY}'}

def sync_inventory():
    # Get all products from R Commerce
    response = requests.get(f'{BASE_URL}/api/v1/products', headers=HEADERS)
    products = response.json()
    
    for product in products:
        # Get current inventory from WMS
        wms_stock = get_wms_stock(product['sku'])
        
        # Update R Commerce inventory
        if wms_stock != product['inventory_quantity']:
            requests.put(
                f'{BASE_URL}/api/v1/inventory/{product["id"]}',
                json={'quantity': wms_stock},
                headers=HEADERS
            )
            print(f"Updated {product['sku']}: {product['inventory_quantity']} -> {wms_stock}")

def get_wms_stock(sku):
    # Query your WMS for current stock level
    pass

if __name__ == '__main__':
    print(f"Starting inventory sync at {datetime.now()}")
    sync_inventory()
    print("Sync complete")
```

### Example 4: Multi-Service Architecture

**Scenario:** Multiple microservices need different levels of access.

```bash
# Product Service - manages product catalog
rcommerce api-key create \
  --name "Service: Product Manager" \
  --scopes "products:write,inventory:read" \
  --rate-limit 2000

# Order Service - processes orders
rcommerce api-key create \
  --name "Service: Order Processor" \
  --scopes "orders:write,customers:read,products:read,payments:write" \
  --rate-limit 5000

# Notification Service - sends emails
rcommerce api-key create \
  --name "Service: Notifications" \
  --scopes "orders:read,customers:read" \
  --rate-limit 1000

# Analytics Service - generates reports
rcommerce api-key create \
  --name "Service: Analytics" \
  --scopes "read,reports:write" \
  --rate-limit 500
```

## Troubleshooting

### 401 Unauthorized

**Cause:** Invalid or missing API key

**Solutions:**
1. Verify the API key is included in the Authorization header
2. Check that the key hasn't been revoked:
   ```bash
   rcommerce api-key get <prefix>
   ```
3. Ensure the key hasn't expired

### 403 Forbidden

**Cause:** Insufficient permissions for the requested operation

**Solutions:**
1. Check your key's scopes:
   ```bash
   rcommerce api-key get <prefix>
   ```
2. Verify the endpoint requires the scopes you have
3. Create a new key with appropriate scopes if needed

### 429 Too Many Requests

**Cause:** Rate limit exceeded

**Solutions:**
1. Check your key's rate limit:
   ```bash
   rcommerce api-key get <prefix>
   ```
2. Implement exponential backoff in your client
3. Request a higher rate limit if needed:
   ```bash
   # Create a new key with higher limit
   rcommerce api-key create --name "High Volume Key" --scopes "write" --rate-limit 10000
   ```

### Key Not Working After Creation

**Cause:** There might be a delay or the key wasn't copied correctly

**Solutions:**
1. Verify you copied the full key including the prefix and dot
2. Check for extra whitespace
3. Ensure you're using the correct environment (dev/staging/prod)

## Configuration Options

### In config.toml

```toml
[security]
# Length of the key prefix (default: 8)
api_key_prefix_length = 8

# Length of the secret portion (default: 32)
api_key_secret_length = 32

# Default rate limit for new keys (optional)
api_key_default_rate_limit = 1000
```

## Next Steps

- [Scopes Reference](../api-reference/scopes.md) - Complete scope documentation
- [Authentication](../api-reference/authentication.md) - Authentication methods
- [CLI Reference](../development/cli-reference.md) - All CLI commands
- [Error Codes](../api-reference/errors.md) - Error handling reference
