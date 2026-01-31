# Authentication

R Commerce supports two authentication methods:
1. **JWT Tokens** - For user sessions (customers, admins)
2. **API Keys** - For service-to-service authentication

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

API keys are used for service-to-service authentication. They are long-lived and managed via CLI.

### Creating API Keys

Use the CLI to create and manage API keys:

```bash
# Create a new API key
rcommerce api-key create \
  --name "Production Backend" \
  --scopes "read,write"

# Output:
# ✅ API Key created successfully!
# Key: aB3dEfGh.sEcReTkEy123456789
# Prefix: aB3dEfGh
# Scopes: read, write
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

Include the API key in the `Authorization` header:

```http
GET /api/v1/products
Authorization: Bearer aB3dEfGh.sEcReTkEy123456789
```

## Protected Routes

The following routes require authentication (JWT or API key):

| Route | Method | Description |
|-------|--------|-------------|
| `/api/v1/customers` | GET, POST | List/create customers |
| `/api/v1/customers/:id` | GET, PUT, DELETE | Customer operations |
| `/api/v1/orders` | GET, POST | List/create orders |
| `/api/v1/orders/:id` | GET, PUT | Order operations |
| `/api/v1/carts/*` | All | Cart operations |
| `/api/v1/payments/*` | All | Payment operations |
| `/api/v1/coupons` | POST | Create coupons |

### Public Routes

These routes do not require authentication:

| Route | Method | Description |
|-------|--------|-------------|
| `/api/v1/products` | GET | List products |
| `/api/v1/products/:id` | GET | Get product |
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

## Security Best Practices

1. **Never expose API keys** in client-side code
2. **Use environment variables** for storing keys
3. **Rotate keys regularly** (every 90 days recommended)
4. **Use minimal scopes** - only grant necessary permissions
5. **Revoke compromised keys** immediately
6. **Use JWT for user sessions**, API keys for server-to-server
7. **Enable HTTPS** in production

## Configuration

Configure JWT settings in your `config.toml`:

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

- [Customers API](customers.md) - Customer management endpoints
- [Orders API](orders.md) - Order management endpoints
- [Error Codes](errors.md) - Complete error reference
