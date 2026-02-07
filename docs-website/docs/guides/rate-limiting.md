# Rate Limiting

Rate limiting controls the number of API requests a client can make within a specific time window to ensure fair usage and protect the system from abuse.

## Overview

R Commerce implements rate limiting to:

- Prevent API abuse and denial-of-service attacks
- Ensure fair resource allocation among all users
- Maintain system stability and performance
- Protect against accidental infinite loops in client code

## Default Limits

### Per-Minute Limits

| Client Type | Requests per Minute | Burst Capacity |
|-------------|---------------------|----------------|
| Anonymous (IP-based) | 60 | 10 |
| Authenticated (API Key) | 1,000 | 100 |
| Authenticated (JWT) | 500 | 50 |

### Per-Hour Limits

| Client Type | Requests per Hour |
|-------------|-------------------|
| Anonymous (IP-based) | 3,000 |
| Authenticated (API Key) | 50,000 |
| Authenticated (JWT) | 25,000 |

### Special Endpoint Limits

Some endpoints have stricter limits:

| Endpoint | Limit |
|----------|-------|
| Authentication (`/api/v1/auth/*`) | 10 requests/minute |
| Webhook delivery | 5 retries per event |
| File uploads | 10 requests/minute |

## Rate Limit Headers

Every API response includes rate limit information in the headers:

```http
HTTP/1.1 200 OK
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705312800
X-RateLimit-Policy: 1000;w=60
Content-Type: application/json
```

### Header Descriptions

| Header | Description | Example |
|--------|-------------|---------|
| `X-RateLimit-Limit` | Maximum requests allowed in the current window | `1000` |
| `X-RateLimit-Remaining` | Requests remaining in current window | `999` |
| `X-RateLimit-Reset` | Unix timestamp when the limit resets | `1705312800` |
| `X-RateLimit-Policy` | Rate limit policy (requests;window in seconds) | `1000;w=60` |

## Handling 429 Too Many Requests

When you exceed the rate limit, the API returns a `429` status code:

```http
HTTP/1.1 429 Too Many Requests
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1705312800
Retry-After: 45
Content-Type: application/json

{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Please retry after 45 seconds.",
    "retry_after": 45
  }
}
```

### Best Practices for Handling Rate Limits

#### 1. Exponential Backoff

Implement exponential backoff when retrying:

```python
import time
import random

def make_request_with_backoff(url, max_retries=5):
    for attempt in range(max_retries):
        response = requests.get(url, headers=headers)
        
        if response.status_code == 429:
            retry_after = int(response.headers.get('Retry-After', 60))
            # Add jitter to prevent thundering herd
            sleep_time = retry_after * (2 ** attempt) + random.uniform(0, 1)
            time.sleep(sleep_time)
            continue
            
        return response
    
    raise Exception("Max retries exceeded")
```

#### 2. Check Remaining Requests

Monitor remaining requests proactively:

```javascript
async function makeRequest(url) {
  const response = await fetch(url, { headers });
  
  const remaining = parseInt(response.headers.get('X-RateLimit-Remaining'));
  const resetTime = parseInt(response.headers.get('X-RateLimit-Reset')) * 1000;
  
  if (remaining < 10) {
    const delay = resetTime - Date.now();
    console.warn(`Rate limit low (${remaining} remaining). Reset in ${delay}ms`);
    // Slow down or queue requests
  }
  
  return response;
}
```

#### 3. Use Request Queuing

Implement a queue to smooth out request patterns:

```typescript
class RateLimitedQueue {
  private queue: (() => Promise<any>)[] = [];
  private processing = false;
  private minInterval: number;

  constructor(requestsPerMinute: number) {
    this.minInterval = 60000 / requestsPerMinute;
  }

  async add<T>(fn: () => Promise<T>): Promise<T> {
    return new Promise((resolve, reject) => {
      this.queue.push(async () => {
        try {
          const result = await fn();
          resolve(result);
        } catch (error) {
          reject(error);
        }
      });
      
      if (!this.processing) {
        this.process();
      }
    });
  }

  private async process() {
    this.processing = true;
    
    while (this.queue.length > 0) {
      const startTime = Date.now();
      const task = this.queue.shift()!;
      await task();
      
      const elapsed = Date.now() - startTime;
      const delay = Math.max(0, this.minInterval - elapsed);
      await new Promise(resolve => setTimeout(resolve, delay));
    }
    
    this.processing = false;
  }
}

// Usage
const queue = new RateLimitedQueue(900); // Stay under 1000/min limit
const results = await Promise.all(
  urls.map(url => queue.add(() => fetch(url)))
);
```

## Configuration Options

Rate limits can be configured in `config.toml`:

```toml
[rate_limiting]
enabled = true

# Default limits
[rate_limiting.default]
requests_per_minute = 1000
burst_capacity = 100

# Anonymous users
[rate_limiting.anonymous]
requests_per_minute = 60
burst_capacity = 10

# Per-endpoint overrides
[rate_limiting.endpoints]
"/api/v1/auth/login" = { requests_per_minute = 10 }
"/api/v1/auth/register" = { requests_per_minute = 5 }
"/api/v1/uploads" = { requests_per_minute = 10 }

# Per-API-key overrides (in database)
# api_keys.rate_limit_multiplier = 2.0  # Double the default
```

### Redis Backend

For distributed deployments, use Redis for rate limit tracking:

```toml
[rate_limiting]
backend = "redis"  # or "memory"

[redis]
url = "redis://localhost:6379"
```

## Best Practices

### For API Consumers

1. **Cache responses** - Don't repeat identical requests
2. **Use webhooks** - Get notified instead of polling
3. **Batch operations** - Use bulk endpoints when available
4. **Respect Retry-After** - Wait the specified time before retrying
5. **Implement circuit breakers** - Stop requests when limits are hit

### For API Administrators

1. **Monitor usage patterns** - Identify legitimate vs. abusive traffic
2. **Set appropriate limits** - Balance protection with usability
3. **Use tiered limits** - Higher limits for paying customers
4. **Whitelist internal IPs** - Exclude internal services from limits
5. **Alert on abuse** - Get notified of potential attacks

## Monitoring Rate Limits

### View Current Usage (Admin)

```http
GET /api/v1/admin/rate-limits
Authorization: Bearer YOUR_ADMIN_API_KEY
```

Response:

```json
{
  "data": {
    "global": {
      "requests_per_minute": 10000,
      "current_usage": 2340
    },
    "by_client": [
      {
        "client_id": "api_key_abc123",
        "requests_last_minute": 450,
        "limit": 1000,
        "remaining": 550
      }
    ]
  }
}
```

### Prometheus Metrics

Rate limiting metrics are exposed:

```
# HELP rcommerce_rate_limit_hits_total Total rate limit hits
# TYPE rcommerce_rate_limit_hits_total counter
rcommerce_rate_limit_hits_total{client_type="anonymous"} 123
rcommerce_rate_limit_hits_total{client_type="authenticated"} 45

# HELP rcommerce_rate_limit_current Current request rate
# TYPE rcommerce_rate_limit_current gauge
rcommerce_rate_limit_current{client_id="api_key_abc123"} 450
```

## Troubleshooting

### Unexpected 429 Errors

**Check your request frequency:**

```bash
# Monitor your request rate
curl -s -o /dev/null -w "%{http_code}" \
  -H "Authorization: Bearer $API_KEY" \
  https://api.rcommerce.app/api/v1/products
```

**Common causes:**

1. **Missing authentication** - Requests counted as anonymous (60/min)
2. **Multiple clients** - Shared API key across many instances
3. **Retry loops** - Code automatically retrying on errors
4. **Webhook flooding** - Processing webhooks triggering API calls

### Increasing Your Rate Limit

Contact support to request higher limits:

1. Describe your use case
2. Provide expected request volume
3. Explain current architecture
4. Consider upgrading to a higher tier

## GraphQL Rate Limiting

GraphQL uses complexity-based rate limiting:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 995
X-RateLimit-Cost: 5
```

### Complexity Calculation

- Base cost: 1 point
- Each field: +1 point
- Nested connections: +10 points
- Maximum: 1000 points per query

Optimize your queries:

```graphql
# High cost (many nested fields)
query Expensive {
  products(first: 100) {
    edges {
      node {
        variants(first: 100) {
          edges {
            node {
              images(first: 100) { ... }
            }
          }
        }
      }
    }
  }
}

# Lower cost (fewer, specific fields)
query Efficient {
  products(first: 20) {
    edges {
      node {
        id
        title
        price
      }
    }
  }
}
```

## Related Documentation

- [Error Codes](../api-reference/errors.md)
- [API Authentication](../api-reference/authentication.md)
- [Best Practices](./api-keys.md)
