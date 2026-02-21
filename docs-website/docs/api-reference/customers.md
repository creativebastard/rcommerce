# Customers API

The Customers API manages customer accounts, profiles, and addresses. All endpoints are fully implemented and operational.

## Base URL

```
/api/v1/customers
```

## Authentication

All customer endpoints require JWT authentication.

```http
Authorization: Bearer <jwt_token>
```

**Access Control:**
- Customers can only access their own data
- Admin users can access all customer data

## Customer Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "email": "customer@example.com",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+1-555-0123",
  "accepts_marketing": true,
  "tax_exempt": false,
  "currency": "USD",
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-20T14:30:00Z",
  "confirmed_at": "2024-01-15T10:05:00Z"
}
```

### Customer Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `email` | string | Primary email address (unique) |
| `first_name` | string | First name |
| `last_name` | string | Last name |
| `phone` | string | Phone number |
| `accepts_marketing` | boolean | Subscribed to marketing emails |
| `tax_exempt` | boolean | Exempt from taxes |
| `currency` | string | Preferred currency (ISO 4217) |
| `created_at` | datetime | Account creation timestamp |
| `updated_at` | datetime | Last modification timestamp |
| `confirmed_at` | datetime | Email confirmation timestamp |

## Endpoints

### List Customers

Retrieves a paginated list of customers. **Admin access required.**

```http
GET /api/v1/customers
Authorization: Bearer <jwt_token>
```

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `email` | string | Filter by email address |
| `phone` | string | Filter by phone number |
| `accepts_marketing` | boolean | Filter by marketing consent |

#### Example Request

```http
GET /api/v1/customers?accepts_marketing=true&page=1&per_page=20
Authorization: Bearer <jwt_token>
```

#### Example Response

```json
{
  "customers": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "email": "customer@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "phone": "+1-555-0123",
      "accepts_marketing": true,
      "tax_exempt": false,
      "currency": "USD",
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-20T14:30:00Z",
      "confirmed_at": "2024-01-15T10:05:00Z"
    }
  ],
  "meta": {
    "total": 1250,
    "page": 1,
    "per_page": 20,
    "total_pages": 63
  }
}
```

### Get Customer by ID

Retrieves a single customer by ID.

```http
GET /api/v1/customers/{id}
Authorization: Bearer <jwt_token>
```

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Customer unique identifier |

**Access Control:** Users can only access their own profile unless they are admin.

#### Example Response

```json
{
  "customer": {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "email": "customer@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "phone": "+1-555-0123",
    "accepts_marketing": true,
    "tax_exempt": false,
    "currency": "USD",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-20T14:30:00Z",
    "confirmed_at": "2024-01-15T10:05:00Z"
  },
  "addresses": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "first_name": "John",
      "last_name": "Doe",
      "company": "Acme Inc",
      "phone": "+1-555-0123",
      "address1": "123 Main St",
      "address2": "Apt 4B",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001",
      "is_default_shipping": true,
      "is_default_billing": true,
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:00:00Z"
    }
  ]
}
```

### Get Current Customer

Retrieves the profile of the currently authenticated customer.

```http
GET /api/v1/customers/me
Authorization: Bearer <jwt_token>
```

#### Example Response

```json
{
  "customer": {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "email": "customer@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "phone": "+1-555-0123",
    "accepts_marketing": true,
    "tax_exempt": false,
    "currency": "USD",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-20T14:30:00Z",
    "confirmed_at": "2024-01-15T10:05:00Z"
  },
  "addresses": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "first_name": "John",
      "last_name": "Doe",
      "company": "Acme Inc",
      "phone": "+1-555-0123",
      "address1": "123 Main St",
      "address2": "Apt 4B",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001",
      "is_default_shipping": true,
      "is_default_billing": true,
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:00:00Z"
    }
  ]
}
```

## Address Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440002",
  "first_name": "John",
  "last_name": "Doe",
  "company": "Acme Inc",
  "phone": "+1-555-0123",
  "address1": "123 Main St",
  "address2": "Apt 4B",
  "city": "New York",
  "state": "NY",
  "country": "US",
  "zip": "10001",
  "is_default_shipping": true,
  "is_default_billing": true,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### Address Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `first_name` | string | First name |
| `last_name` | string | Last name |
| `company` | string | Company name |
| `phone` | string | Phone number |
| `address1` | string | Street address line 1 |
| `address2` | string | Street address line 2 |
| `city` | string | City name |
| `state` | string | State or province |
| `country` | string | Two-letter country code |
| `zip` | string | Postal/ZIP code |
| `is_default_shipping` | boolean | Default for shipping |
| `is_default_billing` | boolean | Default for billing |
| `created_at` | datetime | Creation timestamp |
| `updated_at` | datetime | Last update timestamp |

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `CUSTOMER_NOT_FOUND` | 404 | Customer does not exist |
| `EMAIL_TAKEN` | 409 | Email address already in use |
| `INVALID_EMAIL` | 400 | Invalid email format |
| `FORBIDDEN` | 403 | Access denied to this customer |
| `ADDRESS_NOT_FOUND` | 404 | Address does not exist |

## Example Usage

### Get Current Customer Profile

```bash
curl -X GET http://localhost:8080/api/v1/customers/me \
  -H "Authorization: Bearer $JWT_TOKEN" | jq
```

### Get Customer by ID

```bash
curl -X GET http://localhost:8080/api/v1/customers/550e8400-e29b-41d4-a716-446655440001 \
  -H "Authorization: Bearer $JWT_TOKEN" | jq
```

### List All Customers (Admin Only)

```bash
curl -X GET "http://localhost:8080/api/v1/customers?page=1&per_page=20" \
  -H "Authorization: Bearer $ADMIN_JWT_TOKEN" | jq
```

## Related Topics

- [Authentication](authentication.md) - JWT authentication
- [Orders API](orders.md) - Customer orders
- [Cart API](cart.md) - Shopping cart
