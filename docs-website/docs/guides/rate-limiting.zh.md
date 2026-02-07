# 速率限制

速率限制控制客户端在特定时间窗口内可以发出的 API 请求数量，以确保公平使用并保护系统免受滥用。

## 概述

R Commerce 实施速率限制以：

- 防止 API 滥用和拒绝服务攻击
- 确保所有用户之间的公平资源分配
- 维护系统稳定性和性能
- 防止客户端代码中的意外无限循环

## 默认限制

### 每分钟限制

| 客户端类型 | 每分钟请求数 | 突发容量 |
|-------------|---------------------|----------------|
| 匿名（基于 IP） | 60 | 10 |
| 已认证（API 密钥） | 1,000 | 100 |
| 已认证（JWT） | 500 | 50 |

### 每小时限制

| 客户端类型 | 每小时请求数 |
|-------------|-------------------|
| 匿名（基于 IP） | 3,000 |
| 已认证（API 密钥） | 50,000 |
| 已认证（JWT） | 25,000 |

### 特殊端点限制

某些端点有更严格的限制：

| 端点 | 限制 |
|----------|-------|
| 认证（`/api/v1/auth/*`） | 10 请求/分钟 |
| Webhook 传递 | 每个事件 5 次重试 |
| 文件上传 | 10 请求/分钟 |

## 速率限制头

每个 API 响应都包含头中的速率限制信息：

```http
HTTP/1.1 200 OK
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705312800
X-RateLimit-Policy: 1000;w=60
Content-Type: application/json
```

### 头说明

| 头 | 说明 | 示例 |
|--------|-------------|---------|
| `X-RateLimit-Limit` | 当前窗口中允许的最大请求数 | `1000` |
| `X-RateLimit-Remaining` | 当前窗口中剩余的请求数 | `999` |
| `X-RateLimit-Reset` | 限制重置的 Unix 时间戳 | `1705312800` |
| `X-RateLimit-Policy` | 速率限制策略（请求数；窗口秒数） | `1000;w=60` |

## 处理 429 请求过多

当您超过速率限制时，API 返回 `429` 状态码：

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

### 处理速率限制的最佳实践

#### 1. 指数退避

重试时实现指数退避：

```python
import time
import random

def make_request_with_backoff(url, max_retries=5):
    for attempt in range(max_retries):
        response = requests.get(url, headers=headers)
        
        if response.status_code == 429:
            retry_after = int(response.headers.get('Retry-After', 60))
            # 添加抖动以防止惊群效应
            sleep_time = retry_after * (2 ** attempt) + random.uniform(0, 1)
            time.sleep(sleep_time)
            continue
            
        return response
    
    raise Exception("Max retries exceeded")
```

#### 2. 检查剩余请求

主动监控剩余请求：

```javascript
async function makeRequest(url) {
  const response = await fetch(url, { headers });
  
  const remaining = parseInt(response.headers.get('X-RateLimit-Remaining'));
  const resetTime = parseInt(response.headers.get('X-RateLimit-Reset')) * 1000;
  
  if (remaining < 10) {
    const delay = resetTime - Date.now();
    console.warn(`Rate limit low (${remaining} remaining). Reset in ${delay}ms`);
    // 放慢速度或排队请求
  }
  
  return response;
}
```

#### 3. 使用请求队列

实现队列以平滑请求模式：

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

// 使用
const queue = new RateLimitedQueue(900); // 保持在 1000/分钟限制以下
const results = await Promise.all(
  urls.map(url => queue.add(() => fetch(url)))
);
```

## 配置选项

可以在 `config.toml` 中配置速率限制：

```toml
[rate_limiting]
enabled = true

# 默认限制
[rate_limiting.default]
requests_per_minute = 1000
burst_capacity = 100

# 匿名用户
[rate_limiting.anonymous]
requests_per_minute = 60
burst_capacity = 10

# 每个端点的覆盖
[rate_limiting.endpoints]
"/api/v1/auth/login" = { requests_per_minute = 10 }
"/api/v1/auth/register" = { requests_per_minute = 5 }
"/api/v1/uploads" = { requests_per_minute = 10 }

# 每个 API 密钥的覆盖（在数据库中）
# api_keys.rate_limit_multiplier = 2.0  # 双倍默认值
```

### Redis 后端

对于分布式部署，使用 Redis 进行速率限制跟踪：

```toml
[rate_limiting]
backend = "redis"  # 或 "memory"

[redis]
url = "redis://localhost:6379"
```

## 最佳实践

### 对于 API 消费者

1. **缓存响应** - 不要重复相同的请求
2. **使用 webhooks** - 获取通知而不是轮询
3. **批量操作** - 尽可能使用批量端点
4. **尊重 Retry-After** - 在重试前等待指定的时间
5. **实现断路器** - 达到限制时停止请求

### 对于 API 管理员

1. **监控使用模式** - 识别合法与滥用流量
2. **设置适当的限制** - 在保护与可用性之间取得平衡
3. **使用分层限制** - 为付费客户提供更高的限制
4. **白名单内部 IP** - 从限制中排除内部服务
5. **滥用警报** - 获取潜在攻击通知

## 监控速率限制

### 查看当前使用情况（管理员）

```http
GET /api/v1/admin/rate-limits
Authorization: Bearer YOUR_ADMIN_API_KEY
```

响应：

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

### Prometheus 指标

速率限制指标已公开：

```
# HELP rcommerce_rate_limit_hits_total 总速率限制命中数
# TYPE rcommerce_rate_limit_hits_total counter
rcommerce_rate_limit_hits_total{client_type="anonymous"} 123
rcommerce_rate_limit_hits_total{client_type="authenticated"} 45

# HELP rcommerce_rate_limit_current 当前请求速率
# TYPE rcommerce_rate_limit_current gauge
rcommerce_rate_limit_current{client_id="api_key_abc123"} 450
```

## 故障排除

### 意外的 429 错误

**检查您的请求频率：**

```bash
# 监控您的请求速率
curl -s -o /dev/null -w "%{http_code}" \
  -H "Authorization: Bearer $API_KEY" \
  https://api.rcommerce.app/api/v1/products
```

**常见原因：**

1. **缺少认证** - 请求计为匿名（60/分钟）
2. **多个客户端** - 跨多个实例共享 API 密钥
3. **重试循环** - 代码自动重试错误
4. **Webhook 泛滥** - 处理 webhooks 触发 API 调用

### 提高您的速率限制

联系支持以请求更高的限制：

1. 描述您的用例
2. 提供预期的请求量
3. 解释当前架构
4. 考虑升级到更高层级

## GraphQL 速率限制

GraphQL 使用基于复杂度的速率限制：

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 995
X-RateLimit-Cost: 5
```

### 复杂度计算

- 基础成本：1 点
- 每个字段：+1 点
- 嵌套连接：+10 点
- 最大值：每次查询 1000 点

优化您的查询：

```graphql
# 高成本（许多嵌套字段）
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

# 较低成本（较少、特定的字段）
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

## 相关文档

- [错误代码](../api-reference/errors.md)
- [API 认证](../api-reference/authentication.md)
- [最佳实践](./api-keys.md)
