# API Scopes Reference

R Commerce uses a granular scope-based permission system for API keys. Scopes define what resources an API key can access and what actions it can perform.

## Scope Format

Scopes follow the format: `resource:action`

```
products:read
│         │
│         └── Action: read, write, or admin
└─────────── Resource: products, orders, customers, etc.
```

### Global Scopes

Global scopes apply to all resources:

- `read` - Read access to all resources
- `write` - Write access to all resources (includes read)
- `admin` - Full administrative access to all resources (includes read and write)

### Resource-Specific Scopes

Resource-specific scopes apply to a single resource type:

- `products:read` - Read products only
- `products:write` - Write products only (includes read)
- `orders:read` - Read orders only
- `orders:write` - Write orders only (includes read)

## Available Resources

The following resources can be used in scopes:

| Resource | Description | Typical Use Cases |
|----------|-------------|-------------------|
| `products` | Product catalog management | Product sync, inventory updates |
| `orders` | Order management | Order processing, fulfillment |
| `customers` | Customer data | CRM integration, customer lookup |
| `carts` | Shopping carts | Cart persistence, abandoned cart recovery |
| `coupons` | Discount coupons | Promotion management |
| `payments` | Payment records | Payment reconciliation, refunds |
| `inventory` | Inventory tracking | Stock management, warehouse sync |
| `webhooks` | Webhook configuration | Webhook management, event handling |
| `users` | User accounts | User management, access control |
| `settings` | System settings | Configuration management |
| `reports` | Analytics reports | Reporting integrations, data export |
| `imports` | Data imports | Bulk product import, migration |
| `exports` | Data exports | Data backup, reporting |

## Available Actions

Three action levels are available:

| Action | Description | HTTP Methods |
|--------|-------------|--------------|
| `read` | View resources | GET |
| `write` | Create, update, delete resources | POST, PUT, PATCH, DELETE |
| `admin` | Administrative operations | All methods + admin-only endpoints |

### Permission Hierarchy

Actions follow a hierarchical permission model:

```
admin
 └── write
      └── read
```

- **`write`** includes **`read`** permissions
- **`admin`** includes both **`read`** and **`write`** permissions

Example: A key with `products:write` can:
- ✅ Read products (GET /api/v1/products)
- ✅ Create products (POST /api/v1/products)
- ✅ Update products (PUT /api/v1/products/:id)
- ✅ Delete products (DELETE /api/v1/products/:id)

## Scope Combinations

### Single Resource Access

```bash
# Read-only access to products
--scopes "products:read"

# Full access to products
--scopes "products:write"

# Admin access to products
--scopes "products:admin"
```

### Multiple Resources

```bash
# Read products, write orders
--scopes "products:read,orders:write"

# Read products and customers, full access to orders
--scopes "products:read,customers:read,orders:write"
```

### Wildcard Scopes

```bash
# Read access to all resources
--scopes "read"

# Write access to all resources (includes read)
--scopes "write"

# Admin access to all resources
--scopes "admin"
```

### Mixed Scopes

```bash
# Read all resources, but only write orders
--scopes "read,orders:write"

# Read products, full access to everything else
--scopes "products:read,write"
```

## Scope Presets

R Commerce provides predefined scope sets for common use cases:

### Read-Only Access

```bash
--scopes "read"
```

Grants read access to all resources. Ideal for:
- Reporting tools
- Analytics dashboards
- Data synchronization (read-only)
- Audit systems

### Read-Write Access

```bash
--scopes "read,write"
```

Grants read and write access to all resources. Ideal for:
- Full-featured integrations
- Admin dashboards
- Data migration tools

### Admin Access

```bash
--scopes "admin"
```

Grants full administrative access to all resources. Ideal for:
- System administrators
- Full platform access
- Emergency recovery tools

### Products Read-Only

```bash
--scopes "products:read"
```

Grants read-only access to products only. Ideal for:
- Product catalog displays
- Product search integrations
- Price comparison tools

### Products Full Access

```bash
--scopes "products:read,products:write"
```

Grants full access to products. Ideal for:
- Product management tools
- Inventory management systems
- PIM (Product Information Management) integrations

### Orders Read-Only

```bash
--scopes "orders:read"
```

Grants read-only access to orders. Ideal for:
- Order tracking systems
- Reporting tools
- Customer service dashboards

### Orders Full Access

```bash
--scopes "orders:read,orders:write"
```

Grants full access to orders. Ideal for:
- Order management systems
- Fulfillment integrations
- Customer service tools

### Customer Access

```bash
--scopes "products:read,orders:read,orders:write,carts:read,carts:write,customers:read,customers:write,payments:read,payments:write"
```

Grants access needed for customer-facing operations. Ideal for:
- Mobile applications
- Customer portals
- Frontend applications

### Webhook Handler

```bash
--scopes "webhooks:write,orders:read,orders:write,payments:read,payments:write"
```

Grants access for webhook processing. Ideal for:
- Payment gateway webhooks
- Third-party integrations
- Event processing systems

### Inventory Manager

```bash
--scopes "inventory:read,inventory:write,products:read,orders:read"
```

Grants access for inventory management. Ideal for:
- Warehouse management systems
- Inventory synchronization
- Stock level monitoring

## Scope Validation

When creating or updating API keys, scopes are validated to ensure:

1. **Valid resource names** - Must be one of the available resources
2. **Valid action names** - Must be `read`, `write`, or `admin`
3. **Valid format** - Must follow `resource:action` or be a global scope

### Invalid Scope Examples

```bash
# Invalid resource
--scopes "invalid_resource:read"
# Error: Unknown resource: invalid_resource

# Invalid action
--scopes "products:execute"
# Error: Unknown action: execute

# Invalid format
--scopes "products-read"
# Error: Invalid scope format: products-read

# Too many parts
--scopes "products:read:extra"
# Error: Invalid scope format: products:read:extra
```

## Scope Checking

The API automatically checks scopes for each request:

### Example: Checking Read Access

```rust
// Check if API key can read products
if api_key_auth.can_read(Resource::Products) {
    // Allow access
}
```

### Example: Checking Write Access

```rust
// Check if API key can write orders
if api_key_auth.can_write(Resource::Orders) {
    // Allow creating/updating orders
}
```

### Example: Checking Admin Access

```rust
// Check if API key has admin access
if api_key_auth.is_admin() {
    // Allow administrative operations
}
```

## Scope Best Practices

### 1. Principle of Least Privilege

Grant only the minimum permissions necessary:

```bash
# Good: Specific access
--scopes "products:read,orders:read"

# Avoid: Overly broad access when not needed
--scopes "write"
```

### 2. Separate Keys for Different Services

Use different API keys for different services:

```bash
# Key for product sync service
rcommerce api-key create --name "Product Sync" --scopes "products:read,products:write"

# Key for order processing service  
rcommerce api-key create --name "Order Processor" --scopes "orders:read,orders:write"

# Key for reporting service
rcommerce api-key create --name "Reporting" --scopes "read"
```

### 3. Environment-Specific Keys

Create separate keys for different environments:

```bash
# Development
rcommerce api-key create --name "Dev - Product Sync" --scopes "products:write"

# Staging
rcommerce api-key create --name "Staging - Product Sync" --scopes "products:write"

# Production
rcommerce api-key create --name "Prod - Product Sync" --scopes "products:read"
```

### 4. Regular Scope Audits

Review API key scopes regularly:

```bash
# List all keys and their scopes
rcommerce api-key list

# Check specific key details
rcommerce api-key get <prefix>
```

### 5. Document Key Usage

Use descriptive names and document the purpose:

```bash
rcommerce api-key create \
  --name "Mobile App - Production" \
  --scopes "products:read,orders:read,orders:write,carts:read,carts:write"
# Document: Used by iOS and Android apps for customer-facing operations
```

## Common Scope Patterns

### E-Commerce Platform Integration

```bash
--scopes "products:read,products:write,orders:read,orders:write,customers:read,customers:write,inventory:read,inventory:write"
```

### Payment Gateway Integration

```bash
--scopes "orders:read,orders:write,payments:read,payments:write,webhooks:write"
```

### ERP Integration

```bash
--scopes "products:read,products:write,orders:read,orders:write,inventory:read,inventory:write,customers:read,customers:write"
```

### Marketing Automation

```bash
--scopes "customers:read,orders:read,coupons:read,coupons:write,products:read"
```

### Analytics and Reporting

```bash
--scopes "read,reports:read"
```

## Troubleshooting

### 403 Forbidden Errors

If you receive a 403 error, check that your API key has the required scopes:

```bash
# Check your key's scopes
rcommerce api-key get <prefix>
```

Compare with the required scopes for the endpoint you're calling.

### Scope Changes

If you need different scopes, you must create a new API key:

```bash
# Revoke the old key
rcommerce api-key revoke <old_prefix> --reason "Replacing with scoped key"

# Create a new key with correct scopes
rcommerce api-key create --name "Updated Key" --scopes "correct:scopes"
```

> **Note:** Scopes cannot be modified on existing keys for security reasons.

## Next Steps

- [Authentication](authentication.md) - Authentication methods and usage
- [API Keys Guide](../guides/api-keys.md) - Managing API keys
- [CLI Reference](../development/cli-reference.md) - CLI commands reference
