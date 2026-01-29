# Customers API

The Customers API manages customer accounts, profiles, addresses, and order history.

## Base URL

```
/api/v1/customers
```

## Authentication

Customer endpoints require authentication. Customers can only access their own data unless using admin API key.

```http
Authorization: Bearer YOUR_API_KEY
```

## Customer Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "email": "customer@example.com",
  "phone": "+1-555-0123",
  "first_name": "John",
  "last_name": "Doe",
  "accepts_marketing": true,
  "marketing_opt_in_at": "2024-01-15T10:00:00Z",
  "tax_exempt": false,
  "tax_exemptions": [],
  "currency": "USD",
  "language": "en",
  "note": "VIP customer",
  "tags": ["vip", "repeat_customer"],
  "verified_email": true,
  "state": "enabled",
  "last_order_id": "550e8400-e29b-41d4-a716-446655440100",
  "last_order_name": "1001",
  "orders_count": 5,
  "total_spent": "299.95",
  "average_order_value": "59.99",
  "default_address": {
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
    "is_default_billing": true
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
  ],
  "metafields": {
    "birthday": "1990-05-15",
    "loyalty_tier": "gold"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-20T14:30:00Z"
}
```

### Customer Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `email` | string | Primary email address (unique) |
| `phone` | string | Phone number |
| `first_name` | string | First name |
| `last_name` | string | Last name |
| `accepts_marketing` | boolean | Subscribed to marketing emails |
| `marketing_opt_in_at` | datetime | When customer opted in |
| `tax_exempt` | boolean | Exempt from taxes |
| `tax_exemptions` | array | Specific tax exemptions |
| `currency` | string | Preferred currency |
| `language` | string | Preferred language (ISO 639-1) |
| `note` | string | Internal staff notes |
| `tags` | array | Searchable tags |
| `verified_email` | boolean | Email verification status |
| `state` | string | `enabled`, `disabled`, `invited`, `declined` |
| `last_order_id` | UUID | Most recent order |
| `last_order_name` | string | Order number of last order |
| `orders_count` | integer | Total number of orders |
| `total_spent` | decimal | Lifetime total spent |
| `average_order_value` | decimal | Average order value |
| `default_address` | object | Primary address |
| `addresses` | array | All saved addresses |
| `metafields` | object | Custom key-value data |
| `created_at` | datetime | Account creation |
| `updated_at` | datetime | Last modification |

## Endpoints

### List Customers

```http
GET /api/v1/customers
```

Retrieve a paginated list of customers.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `email` | string | Filter by email address |
| `phone` | string | Filter by phone number |
| `state` | string | Filter by account state |
| `accepts_marketing` | boolean | Filter by marketing consent |
| `tags` | string | Comma-separated tags |
| `min_orders` | integer | Minimum order count |
| `min_total_spent` | decimal | Minimum lifetime spend |
| `created_after` | datetime | Created after date |
| `created_before` | datetime | Created before date |
| `updated_after` | datetime | Updated after date |
| `q` | string | Search query (name, email, phone) |
| `sort` | string | `created_at`, `updated_at`, `total_spent`, `orders_count` |
| `order` | string | `asc` or `desc` (default: desc) |

#### Example Request

```http
GET /api/v1/customers?accepts_marketing=true&min_orders=3&sort=total_spent
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "email": "customer@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "orders_count": 5,
      "total_spent": "299.95",
      "accepts_marketing": true,
      "state": "enabled",
      "created_at": "2024-01-15T10:00:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 1250,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 63
    }
  }
}
```

### Get Customer

```http
GET /api/v1/customers/{id}
```

Retrieve a single customer by ID or email.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | string | Customer UUID or email address |

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `include` | string | Related data: `addresses`, `orders`, `metafields` |

#### Example Request

```http
GET /api/v1/customers/customer@example.com?include=addresses,orders
Authorization: Bearer sk_live_xxx
```

### Create Customer

```http
POST /api/v1/customers
```

Create a new customer account.

#### Request Body

```json
{
  "email": "newcustomer@example.com",
  "phone": "+1-555-0199",
  "first_name": "Jane",
  "last_name": "Smith",
  "accepts_marketing": true,
  "password": "secure_password_123",
  "password_confirmation": "secure_password_123",
  "addresses": [
    {
      "first_name": "Jane",
      "last_name": "Smith",
      "address1": "456 Oak Ave",
      "city": "Los Angeles",
      "state": "CA",
      "country": "US",
      "zip": "90210",
      "phone": "+1-555-0199",
      "is_default_shipping": true,
      "is_default_billing": true
    }
  ],
  "tags": ["newsletter_subscriber"],
  "note": "Found via Google Ads",
  "send_email_invite": true
}
```

#### Required Fields

- `email` - Valid email address (unique)

#### Optional Fields

- `password` - If not provided, customer will be invited to set password
- `send_email_invite` - Send welcome email (default: true)

### Update Customer

```http
PUT /api/v1/customers/{id}
```

Update customer information.

#### Request Body

```json
{
  "first_name": "Jane",
  "last_name": "Smith-Johnson",
  "phone": "+1-555-0200",
  "accepts_marketing": false,
  "note": "Updated contact info",
  "tags": ["vip", "repeat_customer"]
}
```

### Delete Customer

```http
DELETE /api/v1/customers/{id}
```

Delete a customer account (GDPR right to erasure).

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `anonymize_orders` | boolean | Keep orders but anonymize customer data |

### Send Invite

```http
POST /api/v1/customers/{id}/send_invite
```

Send account activation email to invited customer.

### Account Activation

```http
POST /api/v1/customers/{id}/activate
```

Activate a customer account.

#### Request Body

```json
{
  "activation_token": "token_from_email",
  "password": "new_secure_password"
}
```

## Addresses

### List Addresses

```http
GET /api/v1/customers/{customer_id}/addresses
```

### Get Address

```http
GET /api/v1/customers/{customer_id}/addresses/{address_id}
```

### Create Address

```http
POST /api/v1/customers/{customer_id}/addresses
```

#### Request Body

```json
{
  "first_name": "Jane",
  "last_name": "Smith",
  "company": "Acme Inc",
  "phone": "+1-555-0199",
  "address1": "789 Pine Street",
  "address2": "Suite 100",
  "city": "San Francisco",
  "state": "CA",
  "country": "US",
  "zip": "94102",
  "is_default_shipping": false,
  "is_default_billing": true
}
```

### Update Address

```http
PUT /api/v1/customers/{customer_id}/addresses/{address_id}
```

### Delete Address

```http
DELETE /api/v1/customers/{customer_id}/addresses/{address_id}
```

### Set Default Address

```http
POST /api/v1/customers/{customer_id}/addresses/{address_id}/default
```

#### Request Body

```json
{
  "type": "shipping"
}
```

## Customer Orders

### List Customer Orders

```http
GET /api/v1/customers/{customer_id}/orders
```

Retrieve order history for a specific customer.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number |
| `per_page` | integer | Items per page |
| `status` | string | Filter by order status |
| `financial_status` | string | Filter by payment status |

## Customer Search

### Search Customers

```http
POST /api/v1/customers/search
```

Advanced customer search with filters.

#### Request Body

```json
{
  "query": "john",
  "filters": {
    "accepts_marketing": true,
    "min_orders": 2,
    "tags": ["vip"]
  },
  "sort": {
    "field": "total_spent",
    "order": "desc"
  }
}
```

## Customer Segments

### List Segments

```http
GET /api/v1/customers/segments
```

### Create Segment

```http
POST /api/v1/customers/segments
```

#### Request Body

```json
{
  "name": "VIP Customers",
  "query": {
    "total_spent": {
      "gte": 500
    },
    "orders_count": {
      "gte": 5
    }
  }
}
```

### Get Segment Customers

```http
GET /api/v1/customers/segments/{segment_id}/customers
```

## GDPR Compliance

### Export Customer Data

```http
POST /api/v1/customers/{id}/export
```

Export all customer data (GDPR data portability).

### Anonymize Customer

```http
POST /api/v1/customers/{id}/anonymize
```

Anonymize customer data while preserving order history.

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `CUSTOMER_NOT_FOUND` | 404 | Customer does not exist |
| `EMAIL_TAKEN` | 409 | Email address already in use |
| `INVALID_EMAIL` | 400 | Invalid email format |
| `INVALID_PASSWORD` | 400 | Password does not meet requirements |
| `ADDRESS_NOT_FOUND` | 404 | Address does not exist |
| `INVALID_ADDRESS` | 400 | Address validation failed |
| `CUSTOMER_HAS_ORDERS` | 409 | Cannot delete customer with orders |
| `INVALID_STATE_TRANSITION` | 400 | Cannot change to requested state |
| `ACTIVATION_TOKEN_INVALID` | 400 | Invalid or expired activation token |

## Webhooks

| Event | Description |
|-------|-------------|
| `customer.created` | New customer account created |
| `customer.updated` | Customer information changed |
| `customer.deleted` | Customer account deleted |
| `customer.enabled` | Customer account enabled |
| `customer.disabled` | Customer account disabled |
| `customer.address_created` | New address added |
| `customer.address_updated` | Address information changed |
| `customer.address_deleted` | Address removed |
| `customer.password_reset` | Password reset requested |
