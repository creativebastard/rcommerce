# Authentication

R Commerce supports two authentication methods:
1. **JWT Tokens** - For user sessions (customers, admins)
2. **API Keys** - For service-to-service authentication with granular permissions

## JWT Authentication

JWT (JSON Web Tokens) are used for user authentication. They are short-lived and ideal for frontend applications.

### Login

Authenticate a user and receive access and refresh tokens:

```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:**

```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "customer": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "user@example.com",
    "first_name": "John",
    "last_name": "Doe"
  }
}
```

### Register

Create a new customer account:

```bash
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "securepassword123",
  "first_name": "John",
  "last_name": "Doe"
}
```

### Refresh Token

Get a new access token using a refresh token:

```bash
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
}
```

**Response:**

```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

### Using JWT Tokens

Include the access token in the `Authorization` header:

```http
GET /api/v1/customers
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

### Token Expiration

- **Access Token**: 24 hours (configurable)
- **Refresh Token**: 7 days (configurable)

## API Key Authentication

API keys are used for service-to-service authentication. They are long-lived, support granular permissions through scopes, and are managed via CLI.

### API Key Format

API keys follow the format: `<prefix>.<secret>`

- **Prefix**: An 8-character identifier (e.g., `aB3dEfGh`)
- **Secret**: A 32-character cryptographically secure random string
- **Full Key**: `aB3dEfGh.sEcReTkEy123456789abcdef1234567`

Example API key:
```
Key: aB3dEfGh.sEcReTkEy123456789abcdef1234567
      ↑prefix   ↑secret (32 characters)
```

### Creating API Keys

Use the CLI to create and manage API keys:

```bash
# Create a new API key with specific scopes
rcommerce api-key create \
  --name "Production Backend" \
  --scopes "products:read,orders:write"

# Output:
# ✅ API Key created successfully!
# Key: aB3dEfGh.sEcReTkEy123456789abcdef1234567
# Prefix: aB3dEfGh
# Scopes: products:read, orders:write
# Created: 2026-01-15T10:30:00Z
```

> ⚠️ **Important**: The full key is shown only once. Store it securely!

### Managing API Keys

```bash
# List all API keys
rcommerce api-key list

# Get API key details
rcommerce api-key get <prefix>

# Revoke an API key
rcommerce api-key revoke <prefix> --reason "Key compromised"

# Delete an API key permanently
rcommerce api-key delete <prefix>
```

### Using API Keys

Include the API key in the `Authorization` header using Bearer format:

```http
GET /api/v1/products
Authorization: Bearer aB3dEfGh.sEcReTkEy123456789abcdef1234567
```

Or without the Bearer prefix:

```http
GET /api/v1/products
Authorization: aB3dEfGh.sEcReTkEy123456789abcdef1234567
```

### API Key Scopes

API keys use a granular permission system based on scopes. Scopes define what resources an API key can access and what actions it can perform.

**Scope Format:** `resource:action`

Examples:
- `products:read` - Read access to products
- `products:write` - Create/update/delete products
- `orders:read` - Read access to orders
- `orders:write` - Create/update orders
- `admin` - Full administrative access to all resources
- `read` - Read access to all resources (wildcard)

For complete scope documentation, see [Scopes Reference](scopes.md).

### API Key Permissions

The API key system supports three permission levels:

| Level | Description | Example Scope |
|-------|-------------|---------------|
| **Read** | Can view resources | `products:read` |
| **Write** | Can create, update, delete resources | `products:write` |
| **Admin** | Full control including administrative operations | `products:admin` or `admin` |

**Permission Hierarchy:**
- `write` includes `read` permissions
- `admin` includes both `read` and `write` permissions

### Example Requests with API Keys

#### Read Products

```bash
curl -X GET "https://api.rcommerce.app/api/v1/products" \
  -H "Authorization: Bearer aB3dEfGh.sEcReTkEy123456789abcdef1234567"
```

#### Create an Order

```bash
curl -X POST "https://api.rcommerce.app/api/v1/orders" \
  -H "Authorization: Bearer aB3dEfGh.sEcReTkEy123456789abcdef1234567" \
  -H "Content-Type: application/json" \
  -d '{
    "customer_id": "123e4567-e89b-12d3-a456-426614174000",
    "items": [
      {
        "product_id": "123e4567-e89b-12d3-a456-426614174001",
        "quantity": 2
      }
    ]
  }'
```

#### Update Inventory

```bash
curl -X PUT "https://api.rcommerce.app/api/v1/inventory/123e4567-e89b-12d3-a456-426614174001" \
  -H "Authorization: Bearer aB3dEfGh.sEcReTkEy123456789abcdef1234567" \
  -H "Content-Type: application/json" \
  -d '{
    "quantity": 100
  }'
```

## JWT vs API Keys

Choose the appropriate authentication method based on your use case:

| Feature | JWT Tokens | API Keys |
|---------|------------|----------|
| **Use Case** | User sessions | Service-to-service |
| **Lifetime** | Short (24 hours) | Long (configurable/no expiry) |
| **Permissions** | User-based | Scope-based (granular) |
| **Management** | Automatic (login/refresh) | CLI-managed |
| **Revocation** | Token expiry | Immediate via CLI |
| **Best For** | Frontend apps, mobile apps | Backend services, integrations |

### When to Use JWT

- Frontend web applications
- Mobile applications
- Customer-facing interfaces
- Short-lived sessions

### When to Use API Keys

- Backend service integrations
- Webhook handlers
- ETL/data sync processes
- Third-party integrations
- Server-to-server communication

## Protected Routes

The following routes require authentication (JWT or API key):

| Route | Method | Required Scopes |
|-------|--------|-----------------|
| `/api/v1/products` | GET | `products:read` or `read` |
| `/api/v1/products` | POST | `products:write` or `write` |
| `/api/v1/products/:id` | GET | `products:read` or `read` |
| `/api/v1/products/:id` | PUT/PATCH | `products:write` or `write` |
| `/api/v1/products/:id` | DELETE | `products:write` or `write` |
| `/api/v1/customers` | GET | `customers:read` or `read` |
| `/api/v1/customers` | POST | `customers:write` or `write` |
| `/api/v1/customers/:id` | GET | `customers:read` or `read` |
| `/api/v1/customers/:id` | PUT/DELETE | `customers:write` or `write` |
| `/api/v1/orders` | GET | `orders:read` or `read` |
| `/api/v1/orders` | POST | `orders:write` or `write` |
| `/api/v1/orders/:id` | GET | `orders:read` or `read` |
| `/api/v1/orders/:id` | PUT/PATCH | `orders:write` or `write` |
| `/api/v1/carts/*` | All | `carts:read`, `carts:write` or `read`/`write` |
| `/api/v1/payments/*` | All | `payments:read`, `payments:write` or `read`/`write` |
| `/api/v1/coupons` | GET | `coupons:read` or `read` |
| `/api/v1/coupons` | POST | `coupons:write` or `write` |
| `/api/v1/inventory/*` | All | `inventory:read`, `inventory:write` or `read`/`write` |
| `/api/v1/webhooks/*` | All | `webhooks:write` or `write` |

> **Note:** Product endpoints require authentication to prevent unauthorized data scraping and protect product information.

### Public Routes

These routes do not require authentication:

| Route | Method | Description |
|-------|--------|-------------|
| `/api/v1/auth/register` | POST | Register |
| `/api/v1/auth/login` | POST | Login |
| `/api/v1/auth/refresh` | POST | Refresh token |
| `/health` | GET | Health check |

## Error Responses

### Invalid Token

```json
{
  "error": {
    "message": "Unauthorized: Invalid token",
    "code": 401,
    "category": "auth"
  }
}
```

### Expired Token

```json
{
  "error": {
    "message": "Unauthorized: Token has expired",
    "code": 401,
    "category": "auth"
  }
}
```

### Missing Authorization Header

```json
{
  "error": {
    "message": "Unauthorized",
    "code": 401,
    "category": "auth"
  }
}
```

### Insufficient Permissions

```json
{
  "error": {
    "message": "Forbidden: Insufficient permissions. Required: products:write",
    "code": 403,
    "category": "auth"
  }
}
```

### Invalid API Key

```json
{
  "error": {
    "message": "Unauthorized: Invalid API key",
    "code": 401,
    "category": "auth"
  }
}
```

### Revoked API Key

```json
{
  "error": {
    "message": "Unauthorized: API key has been revoked",
    "code": 401,
    "category": "auth"
  }
}
```

## Security Best Practices

1. **Never expose API keys** in client-side code or public repositories
2. **Use environment variables** for storing keys in applications
3. **Rotate keys regularly** (every 90 days recommended)
4. **Use minimal scopes** - only grant necessary permissions
5. **Revoke compromised keys** immediately using the CLI
6. **Use JWT for user sessions**, API keys for server-to-server
7. **Enable HTTPS** in production
8. **Monitor API key usage** via the `last_used_at` and `last_used_ip` fields
9. **Set appropriate rate limits** for each API key
10. **Use separate API keys** for different services/environments

## Configuration

Configure JWT and API key settings in your `config.toml`:

```toml
[security.jwt]
secret = "your-secure-secret-key-min-32-characters"
expiry_hours = 24
refresh_expiry_hours = 168  # 7 days

[security]
api_key_prefix_length = 8
api_key_secret_length = 32
```

## Next Steps

- [Scopes Reference](scopes.md) - Complete scope documentation
- [API Keys Guide](../guides/api-keys.md) - Managing API keys
- [Customers API](customers.md) - Customer management endpoints
- [Orders API](orders.md) - Order management endpoints
- [Error Codes](errors.md) - Complete error reference
