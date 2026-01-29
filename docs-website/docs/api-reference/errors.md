# API Errors

R Commerce uses standard HTTP status codes and provides detailed error information.

## HTTP Status Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 200 | OK | Request succeeded |
| 201 | Created | Resource created successfully |
| 204 | No Content | Request succeeded, no body |
| 400 | Bad Request | Invalid request parameters |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Resource conflict |
| 422 | Unprocessable | Validation failed |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Server Error | Internal server error |
| 503 | Service Unavailable | Service temporarily unavailable |

## Error Response Format

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "The request validation failed",
    "details": [
      {
        "field": "email",
        "message": "Invalid email format"
      }
    ],
    "request_id": "req_550e8400-e29b-41d4-a716-446655440000"
  }
}
```

## Error Codes

### General Errors

| Code | Description |
|------|-------------|
| `INTERNAL_ERROR` | Unexpected server error |
| `NOT_FOUND` | Resource not found |
| `VALIDATION_ERROR` | Input validation failed |
| `UNAUTHORIZED` | Authentication required |
| `FORBIDDEN` | Permission denied |
| `RATE_LIMITED` | Too many requests |
| `CONFLICT` | Resource conflict |

### Specific Errors

See individual API sections for domain-specific error codes.
