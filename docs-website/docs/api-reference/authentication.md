# Authentication

R Commerce uses API keys for authentication. This guide covers how to authenticate your API requests.

## API Key Types

### Publishable Keys

- **Prefix**: `pk_`
- **Usage**: Frontend/client-side code
- **Permissions**: Read-only access to public data
- **Example**: `pk_live_1234567890abcdef`

### Secret Keys

- **Prefix**: `sk_`
- **Usage**: Backend/server-side code only
- **Permissions**: Full access to all API endpoints
- **Example**: `sk_live_1234567890abcdef`

### Restricted Keys

- **Prefix**: `rk_`
- **Usage**: Specific integrations or services
- **Permissions**: Customizable, limited scope
- **Example**: `rk_live_1234567890abcdef`

## Authenticating Requests

### HTTP Header

Include your API key in the `Authorization` header:

```http
GET /v1/orders
Authorization: Bearer sk_live_1234567890abcdef
```

### Query Parameter (Not Recommended)

For testing only, you can pass the key as a query parameter:

```http
GET /v1/orders?api_key=sk_live_1234567890abcdef
```

> ⚠️ **Warning**: Never use query parameter authentication in production as it may expose your API key in logs.

## Permission Scopes

Scopes define what actions an API key can perform:

| Scope | Description |
|-------|-------------|
| `products:read` | Read product data |
| `products:write` | Create and update products |
| `orders:read` | Read order data |
| `orders:write` | Create and update orders |
| `customers:read` | Read customer data |
| `customers:write` | Create and update customers |
| `payments:read` | Read payment data |
| `payments:write` | Process payments and refunds |
| `shipping:read` | Read shipping data |
| `shipping:write` | Create fulfillments |
| `analytics:read` | Access analytics and reports |
| `webhooks:read` | Read webhook configurations |
| `webhooks:write` | Create and modify webhooks |
| `*` | Full access (secret keys only) |

## Creating API Keys

### Via CLI

```bash
# Create a new API key
rcommerce api-key create \
  --name "Production Backend" \
  --permissions "orders:write,customers:write" \
  --expires "2025-12-31"

# List all API keys
rcommerce api-key list

# Revoke an API key
rcommerce api-key revoke sk_live_xxx
```

### Via API

```bash
curl -X POST https://api.yourstore.com/v1/api-keys \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Production Backend",
    "permissions": ["orders:write", "customers:write"],
    "expires_at": "2025-12-31T23:59:59Z"
  }'
```

## JWT Authentication

For user sessions (e.g., admin dashboard), use JWT tokens:

### Obtaining a JWT Token

```bash
curl -X POST https://api.yourstore.com/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "your_password"
  }'
```

Response:

```json
{
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_at": "2024-01-24T14:13:35Z",
    "user": {
      "id": "usr_123",
      "email": "admin@example.com",
      "role": "admin"
    }
  }
}
```

### Using JWT Tokens

```http
GET /v1/orders
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## IP Restrictions

Restrict API key usage to specific IP addresses:

```bash
# Create key with IP restrictions
rcommerce api-key create \
  --name "Server Key" \
  --permissions "*" \
  --allowed-ips "203.0.113.0/24,198.51.100.10"
```

## Security Best Practices

1. **Never expose secret keys** in client-side code
2. **Use environment variables** for API keys
3. **Rotate keys regularly** (every 90 days recommended)
4. **Use restricted keys** for specific integrations
5. **Monitor API key usage** for suspicious activity
6. **Revoke compromised keys** immediately

## Testing Authentication

```bash
# Test with curl
curl https://api.yourstore.com/v1/orders \
  -H "Authorization: Bearer sk_live_xxx"

# Expected success response
{"data": [...], "meta": {...}}

# Expected failure response (invalid key)
{"error": {"code": "unauthorized", "message": "Invalid API key"}}
```

## Error Responses

### Invalid API Key

```json
{
  "error": {
    "code": "unauthorized",
    "message": "Invalid API key",
    "details": {
      "request_id": "req_abc123"
    }
  }
}
```

### Insufficient Permissions

```json
{
  "error": {
    "code": "forbidden",
    "message": "API key lacks required permission: orders:write",
    "details": {
      "required": "orders:write",
      "provided": ["orders:read"]
    }
  }
}
```

### Expired API Key

```json
{
  "error": {
    "code": "unauthorized",
    "message": "API key has expired",
    "details": {
      "expired_at": "2024-01-01T00:00:00Z"
    }
  }
}
```

## Next Steps

- [Error Codes](errors.md) - Complete error code reference
- [API Overview](index.md) - Back to API documentation
