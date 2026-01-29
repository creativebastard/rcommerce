# API Reference

Welcome to the R Commerce API Reference. Our REST API provides comprehensive access to all e-commerce functionality.

## Base URL

```
https://api.rcommerce.app/v1
```

## Authentication

All API requests require authentication using an API key passed in the Authorization header:

```http
Authorization: Bearer YOUR_API_KEY
```

## Content Type

All requests should include the Content-Type header:

```http
Content-Type: application/json
```

## Response Format

All responses are returned in JSON format with a consistent structure:

```json
{
  "data": { ... },
  "meta": {
    "request_id": "req_abc123",
    "timestamp": "2024-01-15T10:00:00Z"
  }
}
```

## Pagination

List endpoints support pagination using cursor-based pagination:

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |

## Rate Limiting

API requests are rate limited to ensure service stability:

- **Public endpoints**: 100 requests/minute
- **Authenticated endpoints**: 1000 requests/minute
- **Admin endpoints**: 5000 requests/minute

Rate limit headers are included in all responses:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705312800
```

## API Sections

- [Authentication](authentication.md) - API keys and JWT tokens
- [Products](products.md) - Product catalog management
- [Orders](orders.md) - Order lifecycle management
- [Customers](customers.md) - Customer accounts and addresses
- [Cart](cart.md) - Shopping cart operations
- [Coupons](coupons.md) - Discount codes and promotions
- [Payments](payments.md) - Payment processing
- [Webhooks](webhooks.md) - Event notifications
- [GraphQL](graphql.md) - Alternative query interface

## SDKs

Official SDKs are available for:

- JavaScript/TypeScript: `@rcommerce/sdk`
- Python: `rcommerce-python`
- Rust: `rcommerce-rs`
- PHP: `rcommerce-php`

## Support

For API support, contact: api-support@rcommerce.app
