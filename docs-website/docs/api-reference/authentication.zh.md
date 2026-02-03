# 认证

R Commerce 使用 API 密钥进行认证。本指南介绍如何认证您的 API 请求。

## API 密钥类型

### 可发布密钥

- **前缀**: `pk_`
- **用途**: 前端/客户端代码
- **权限**: 只读访问公共数据
- **示例**: `pk_live_1234567890abcdef`

### 密钥

- **前缀**: `sk_`
- **用途**: 仅限后端/服务器端代码
- **权限**: 完全访问所有 API 端点
- **示例**: `sk_live_1234567890abcdef`

### 受限密钥

- **前缀**: `rk_`
- **用途**: 特定集成或服务
- **权限**: 可自定义，范围有限
- **示例**: `rk_live_1234567890abcdef`

## 认证请求

### HTTP 头

在 `Authorization` 头中包含您的 API 密钥：

```http
GET /v1/orders
Authorization: Bearer sk_live_1234567890abcdef
```

### 查询参数（不推荐）

仅用于测试，您可以将密钥作为查询参数传递：

```http
GET /v1/orders?api_key=sk_live_1234567890abcdef
```

> ⚠️ **警告**: 切勿在生产环境中使用查询参数认证，因为它可能会在日志中暴露您的 API 密钥。

## 权限范围

范围定义 API 密钥可以执行的操作：

| 范围 | 说明 |
|-------|-------------|
| `products:read` | 读取产品数据 |
| `products:write` | 创建和更新产品 |
| `orders:read` | 读取订单数据 |
| `orders:write` | 创建和更新订单 |
| `customers:read` | 读取客户数据 |
| `customers:write` | 创建和更新客户 |
| `payments:read` | 读取支付数据 |
| `payments:write` | 处理支付和退款 |
| `shipping:read` | 读取配送数据 |
| `shipping:write` | 创建履行 |
| `analytics:read` | 访问分析和报告 |
| `webhooks:read` | 读取 Webhook 配置 |
| `webhooks:write` | 创建和修改 Webhooks |
| `*` | 完全访问（仅限密钥） |

## 创建 API 密钥

### 通过 CLI

```bash
# 创建新的 API 密钥
rcommerce api-key create \
  --name "生产后端" \
  --permissions "orders:write,customers:write" \
  --expires "2025-12-31"

# 列出所有 API 密钥
rcommerce api-key list

# 撤销 API 密钥
rcommerce api-key revoke sk_live_xxx
```

### 通过 API

```bash
curl -X POST https://api.yourstore.com/v1/api-keys \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "生产后端",
    "permissions": ["orders:write", "customers:write"],
    "expires_at": "2025-12-31T23:59:59Z"
  }'
```

## JWT 认证

对于用户会话（例如管理仪表板），使用 JWT 令牌：

### 获取 JWT 令牌

```bash
curl -X POST https://api.yourstore.com/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "your_password"
  }'
```

响应：

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

### 使用 JWT 令牌

```http
GET /v1/orders
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## 受保护的路由

以下路由需要认证（JWT 或 API 密钥）：

| 路由 | 方法 | 描述 |
|------|------|------|
| `/api/v1/products` | GET | 列出产品 |
| `/api/v1/products/:id` | GET | 获取产品 |
| `/api/v1/customers` | GET, POST | 列出/创建客户 |
| `/api/v1/orders` | GET, POST | 列出/创建订单 |
| `/api/v1/carts/*` | 全部 | 购物车操作 |
| `/api/v1/payments/*` | 全部 | 支付操作 |

> **注意：** 产品端点需要认证，以防止未经授权的数据抓取和保护产品信息。

### 公开路由

以下路由不需要认证：

| 路由 | 方法 | 描述 |
|------|------|------|
| `/api/v1/auth/register` | POST | 注册 |
| `/api/v1/auth/login` | POST | 登录 |
| `/api/v1/auth/refresh` | POST | 刷新令牌 |
| `/api/v1/webhooks/*` | POST |  webhook 端点（HMAC 验证） |
| `/health` | GET | 健康检查 |

## IP 限制

将 API 密钥使用限制为特定 IP 地址：

```bash
# 创建带有 IP 限制的密钥
rcommerce api-key create \
  --name "服务器密钥" \
  --permissions "*" \
  --allowed-ips "203.0.113.0/24,198.51.100.10"
```

## 安全最佳实践

1. **切勿在客户端代码中暴露密钥**
2. **使用环境变量**存储 API 密钥
3. **定期轮换密钥**（建议每 90 天）
4. **为特定集成使用受限密钥**
5. **监控 API 密钥使用**以发现可疑活动
6. **立即撤销泄露的密钥**

## 测试认证

```bash
# 使用 curl 测试
curl https://api.yourstore.com/v1/orders \
  -H "Authorization: Bearer sk_live_xxx"

# 预期成功响应
{"data": [...], "meta": {...}}

# 预期失败响应（无效密钥）
{"error": {"code": "unauthorized", "message": "Invalid API key"}}
```

## 错误响应

### 无效 API 密钥

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

### 权限不足

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

### API 密钥已过期

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

## 下一步

- [错误代码](errors.md) - 完整的错误代码参考
- [API 概览](index.md) - 返回 API 文档
