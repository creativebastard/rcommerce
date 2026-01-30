# Webhooks API

Webhooks API 允许您通过 HTTP 回调接收实时事件通知。

## 概览

Webhooks 使您的应用程序能够在 R Commerce 商店中发生事件时接收推送通知，而不是轮询更改。

## 基础 URL

```
/api/v1/webhooks
```

## 认证

Webhook 管理需要密钥 API。

```http
Authorization: Bearer YOUR_SECRET_KEY
```

## Webhook 对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440300",
  "url": "https://your-app.com/webhooks/rcommerce",
  "topic": "order.created",
  "include_fields": ["id", "order_number", "total_price", "customer"],
  "metafield_namespaces": ["global"],
  "secret": "whsec_...",
  "api_version": "2024-01",
  "is_active": true,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### Webhook 字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 唯一标识符 |
| `url` | string | HTTPS 端点 URL |
| `topic` | string | 订阅的事件主题 |
| `include_fields` | array | 要包含的特定字段（可选） |
| `metafield_namespaces` | array | 要包含的元字段命名空间 |
| `secret` | string | 用于验证的签名密钥 |
| `api_version` | string | 负载格式的 API 版本 |
| `is_active` | boolean | Webhook 是否活跃 |
| `created_at` | datetime | 创建时间戳 |
| `updated_at` | datetime | 最后修改时间 |

## 端点

### 列出 Webhooks

```http
GET /api/v1/webhooks
```

检索所有配置的 webhooks。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `topic` | string | 按事件主题筛选 |
| `is_active` | boolean | 按活跃状态筛选 |

### 获取 Webhook

```http
GET /api/v1/webhooks/{id}
```

检索特定的 webhook 配置。

### 创建 Webhook

```http
POST /api/v1/webhooks
```

注册新的 webhook 端点。

#### 请求体

```json
{
  "url": "https://your-app.com/webhooks/rcommerce",
  "topic": "order.created",
  "include_fields": ["id", "order_number", "total_price"],
  "api_version": "2024-01"
}
```

#### 必填字段

- `url` - 可以接收 POST 请求的 HTTPS URL
- `topic` - 要订阅的事件类型

### 更新 Webhook

```http
PUT /api/v1/webhooks/{id}
```

更新 webhook 配置。

#### 请求体

```json
{
  "url": "https://new-url.com/webhooks",
  "is_active": true,
  "include_fields": ["id", "order_number", "customer"]
}
```

### 删除 Webhook

```http
DELETE /api/v1/webhooks/{id}
```

移除 webhook 订阅。

### 测试 Webhook

```http
POST /api/v1/webhooks/{id}/test
```

向 webhook URL 发送测试事件。

#### 请求体

```json
{
  "event": "order.created"
}
```

## Webhook 主题

### 订单

| 主题 | 说明 |
|-------|-------------|
| `order.created` | 新订单已下单 |
| `order.updated` | 订单信息已更改 |
| `order.cancelled` | 订单已取消 |
| `order.closed` | 订单已关闭 |
| `order.reopened` | 订单已重新打开 |
| `order.payment_received` | 付款已捕获 |
| `order.fulfillment_created` | 履行已创建 |
| `order.fulfillment_updated` | 履行已更新 |
| `order.refund_created` | 退款已处理 |

### 产品

| 主题 | 说明 |
|-------|-------------|
| `product.created` | 新产品已创建 |
| `product.updated` | 产品信息已更改 |
| `product.deleted` | 产品已删除 |
| `product.published` | 产品已发布 |
| `product.unpublished` | 产品已取消发布 |
| `product.inventory_changed` | 库存数量已更新 |
| `product.low_stock` | 库存低于阈值 |
| `product.out_of_stock` | 库存已归零 |

### 客户

| 主题 | 说明 |
|-------|-------------|
| `customer.created` | 新客户账户已创建 |
| `customer.updated` | 客户信息已更改 |
| `customer.deleted` | 客户账户已删除 |
| `customer.address_created` | 新地址已添加 |

### 支付

| 主题 | 说明 |
|-------|-------------|
| `payment.created` | 新支付已发起 |
| `payment.succeeded` | 支付已完成 |
| `payment.failed` | 支付失败 |
| `refund.created` | 退款已发起 |
| `dispute.created` | 争议已开启 |

### 购物车

| 主题 | 说明 |
|-------|-------------|
| `cart.created` | 新购物车已创建 |
| `cart.updated` | 购物车已更新 |
| `cart.converted` | 购物车已转换为订单 |

## Webhook 负载

### 传递头

```http
POST /webhooks/rcommerce HTTP/1.1
Host: your-app.com
Content-Type: application/json
X-RCommerce-Topic: order.created
X-RCommerce-Webhook-Id: 550e8400-e29b-41d4-a716-446655440300
X-RCommerce-Event-Id: evt_550e8400e29b41d4a716446655440301
X-RCommerce-Signature: t=1705312800,v1=abc123...
User-Agent: R-Commerce-Webhook/1.0
```

### 负载格式

```json
{
  "id": "evt_550e8400e29b41d4a716446655440301",
  "topic": "order.created",
  "api_version": "2024-01",
  "created_at": "2024-01-15T10:00:00Z",
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440100",
    "order_number": "1001",
    "total_price": "59.49",
    "currency": "USD",
    "customer": {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "email": "customer@example.com"
    }
  }
}
```

## 签名验证

验证 webhook 签名以确保负载来自 R Commerce。

### 签名格式

```
X-RCommerce-Signature: t=<timestamp>,v1=<signature>
```

### 验证示例 (Node.js)

```javascript
const crypto = require('crypto');

function verifyWebhook(payload, signature, secret) {
  const elements = signature.split(',');
  const signatureHash = elements.find(e => e.startsWith('v1=')).split('v1=')[1];
  const timestamp = elements.find(e => e.startsWith('t=')).split('t=')[1];
  
  // 检查时间戳（防止重放攻击）
  const now = Math.floor(Date.now() / 1000);
  if (now - parseInt(timestamp) > 300) { // 5 分钟容差
    throw new Error('Webhook 时间戳太旧');
  }
  
  // 计算预期签名
  const signedPayload = timestamp + '.' + payload;
  const expectedSignature = crypto
    .createHmac('sha256', secret)
    .update(signedPayload)
    .digest('hex');
  
  // 比较签名
  return crypto.timingSafeEqual(
    Buffer.from(signatureHash),
    Buffer.from(expectedSignature)
  );
}

// 使用
app.post('/webhooks/rcommerce', express.raw({type: 'application/json'}), (req, res) => {
  const signature = req.headers['x-rcommerce-signature'];
  const secret = process.env.WEBHOOK_SECRET;
  
  if (!verifyWebhook(req.body, signature, secret)) {
    return res.status(401).send('签名无效');
  }
  
  const event = JSON.parse(req.body);
  // 处理事件...
  
  res.status(200).send('OK');
});
```

### 验证示例 (Python)

```python
import hmac
import hashlib
import time

def verify_webhook(payload: bytes, signature: str, secret: str) -> bool:
    elements = dict(e.split('=') for e in signature.split(','))
    timestamp = elements['t']
    signature_hash = elements['v1']
    
    # 检查时间戳
    now = int(time.time())
    if now - int(timestamp) > 300:
        raise ValueError('Webhook 时间戳太旧')
    
    # 计算预期签名
    signed_payload = f"{timestamp}.{payload.decode()}"
    expected = hmac.new(
        secret.encode(),
        signed_payload.encode(),
        hashlib.sha256
    ).hexdigest()
    
    return hmac.compare_digest(signature_hash, expected)
```

### 验证示例 (Rust)

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn verify_webhook(payload: &[u8], signature: &str, secret: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let parts: std::collections::HashMap<_, _> = signature
        .split(',')
        .filter_map(|e| {
            let mut kv = e.splitn(2, '=');
            Some((kv.next()?, kv.next()?))
        })
        .collect();
    
    let timestamp = parts.get("t").ok_or("缺少时间戳")?;
    let signature_hash = parts.get("v1").ok_or("缺少签名")?;
    
    // 检查时间戳
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    if now - timestamp.parse::<u64>()? > 300 {
        return Err("Webhook 时间戳太旧".into());
    }
    
    // 计算签名
    let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())?;
    mac.update(signed_payload.as_bytes());
    let expected = hex::encode(mac.finalize().into_bytes());
    
    Ok(expected == *signature_hash)
}
```

## 传递行为

### 重试策略

失败的 webhook 传递使用指数退避重试：

| 尝试 | 延迟 |
|---------|-------|
| 1 | 立即 |
| 2 | 5 秒 |
| 3 | 25 秒 |
| 4 | 2 分钟 |
| 5 | 10 分钟 |

最多 5 次重试尝试，持续 15 分钟。

### 成功标准

如果您的端点返回以下内容，则认为 webhook 传递成功：
- HTTP 200-299 状态码
- 30 秒内响应

### 失败的传递

所有重试用尽后：
- Webhook 自动禁用
- 向商店所有者发送邮件通知
- 失败的传递记录保存 30 天

## 最佳实践

### 端点要求

1. **仅 HTTPS** - 必须使用有效的 SSL 证书
2. **快速响应** - 在处理之前返回 200
3. **幂等** - 优雅地处理重复事件
4. **验证签名** - 始终验证真实性

### 处理事件

```javascript
app.post('/webhooks/rcommerce', async (req, res) => {
  // 1. 验证签名
  if (!verifyWebhook(req.body, req.headers['x-rcommerce-signature'], secret)) {
    return res.status(401).send('签名无效');
  }
  
  // 2. 立即确认
  res.status(200).send('OK');
  
  // 3. 异步处理
  const event = JSON.parse(req.body);
  
  try {
    switch (event.topic) {
      case 'order.created':
        await handleOrderCreated(event.data);
        break;
      case 'order.cancelled':
        await handleOrderCancelled(event.data);
        break;
      // ...
    }
  } catch (error) {
    // 记录错误，提醒团队
    console.error('Webhook 处理失败:', error);
  }
});
```

### 幂等性

使用事件 ID 防止重复处理：

```javascript
const processedEvents = new Set(); // 生产环境使用 Redis

async function handleWebhook(event) {
  if (processedEvents.has(event.id)) {
    return; // 已处理
  }
  
  // 处理事件...
  
  processedEvents.add(event.id);
}
```

## 传递日志

### 列出传递尝试

```http
GET /api/v1/webhooks/{webhook_id}/deliveries
```

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `status` | string | `success`、`failed` |
| `event_id` | string | 按事件筛选 |

### 传递对象

```json
{
  "id": "del_550e8400e29b41d4a716446655440302",
  "event_id": "evt_550e8400e29b41d4a716446655440301",
  "webhook_id": "550e8400-e29b-41d4-a716-446655440300",
  "status": "success",
  "http_status": 200,
  "response_body": "OK",
  "attempts": 1,
  "created_at": "2024-01-15T10:00:00Z",
  "completed_at": "2024-01-15T10:00:01Z"
}
```

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `WEBHOOK_NOT_FOUND` | 404 | Webhook 不存在 |
| `INVALID_URL` | 400 | URL 必须是有效的 HTTPS |
| `INVALID_TOPIC` | 400 | 未知的事件主题 |
| `URL_UNREACHABLE` | 400 | 测试传递失败 |
| `MAX_WEBHOOKS_EXCEEDED` | 429 | 配置的 webhooks 过多 |
| `WEBHOOK_DISABLED` | 400 | Webhook 未激活 |
