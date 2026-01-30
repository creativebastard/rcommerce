# API 错误

R Commerce 使用标准 HTTP 状态码并提供详细的错误信息。

## HTTP 状态码

| 代码 | 含义 | 说明 |
|------|---------|-------------|
| 200 | OK | 请求成功 |
| 201 | Created | 资源创建成功 |
| 204 | No Content | 请求成功，无响应体 |
| 400 | Bad Request | 请求参数无效 |
| 401 | Unauthorized | 需要认证 |
| 403 | Forbidden | 权限不足 |
| 404 | Not Found | 资源未找到 |
| 409 | Conflict | 资源冲突 |
| 422 | Unprocessable | 验证失败 |
| 429 | Too Many Requests | 超出速率限制 |
| 500 | Server Error | 内部服务器错误 |
| 503 | Service Unavailable | 服务暂时不可用 |

## 错误响应格式

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "请求验证失败",
    "details": [
      {
        "field": "email",
        "message": "邮箱格式无效"
      }
    ],
    "request_id": "req_550e8400-e29b-41d4-a716-446655440000"
  }
}
```

## 错误代码

### 通用错误

| 代码 | 说明 |
|------|-------------|
| `INTERNAL_ERROR` | 意外的服务器错误 |
| `NOT_FOUND` | 资源未找到 |
| `VALIDATION_ERROR` | 输入验证失败 |
| `UNAUTHORIZED` | 需要认证 |
| `FORBIDDEN` | 权限被拒绝 |
| `RATE_LIMITED` | 请求过多 |
| `CONFLICT` | 资源冲突 |

### 特定错误

请参阅各个 API 部分了解特定领域的错误代码。
