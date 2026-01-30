# API 参考

欢迎使用 R Commerce API 参考文档。我们的 REST API 提供对所有电商功能的全面访问。

## 基础 URL

```
https://api.rcommerce.app/v1
```

## 认证

所有 API 请求都需要使用 API 密钥进行认证，通过 Authorization 头传递：

```http
Authorization: Bearer YOUR_API_KEY
```

## 内容类型

所有请求都应包含 Content-Type 头：

```http
Content-Type: application/json
```

## 响应格式

所有响应都以 JSON 格式返回，具有统一的结构：

```json
{
  "data": { ... },
  "meta": {
    "request_id": "req_abc123",
    "timestamp": "2024-01-15T10:00:00Z"
  }
}
```

## 分页

列表端点支持基于游标的分页：

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |

## 速率限制

API 请求受到速率限制以确保服务稳定性：

- **公共端点**：100 请求/分钟
- **认证端点**：1000 请求/分钟
- **管理端点**：5000 请求/分钟

所有响应都包含速率限制头：

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705312800
```

## API 部分

- [认证](authentication.md) - API 密钥和 JWT 令牌
- [产品](products.md) - 产品目录管理
- [订单](orders.md) - 订单生命周期管理
- [客户](customers.md) - 客户账户和地址
- [购物车](cart.md) - 购物车操作
- [优惠券](coupons.md) - 折扣码和促销
- [支付](payments.md) - 支付处理
- [Webhooks](webhooks.md) - 事件通知
- [GraphQL](graphql.md) - 替代查询接口

## SDK

官方 SDK 可用于：

- JavaScript/TypeScript: `@rcommerce/sdk`
- Python: `rcommerce-python`
- Rust: `rcommerce-rs`
- PHP: `rcommerce-php`

## 支持

如需 API 支持，请联系：api-support@rcommerce.app
