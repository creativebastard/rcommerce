# 支付 Webhooks

Webhooks 支持在所有支持的网关中发生支付事件时进行实时通知。

## 配置

在您的 R Commerce 设置和每个支付网关的仪表板中配置 webhook 端点。

### R Commerce 端点

| 网关 | 端点 URL |
|---------|-------------|
| Stripe | `https://api.yoursite.com/api/v1/webhooks/payments/stripe` |
| Airwallex | `https://api.yoursite.com/api/v1/webhooks/payments/airwallex` |
| 支付宝 | `https://api.yoursite.com/api/v1/webhooks/payments/alipay` |
| 微信支付 | `https://api.yoursite.com/api/v1/webhooks/payments/wechatpay` |

### Webhook 事件

R Commerce 跨所有网关标准化事件：

| 事件 | 说明 | Stripe | Airwallex | 支付宝 | 微信 |
|-------|-------------|--------|-----------|--------|--------|
| `payment.created` | 支付意图已创建 | ✓ | ✓ | ✓ | ✓ |
| `payment.pending` | 等待客户操作 | ✓ | ✓ | ✓ | ✓ |
| `payment.processing` | 支付正在处理 | ✓ | ✓ | - | - |
| `payment.success` | 支付完成 | ✓ | ✓ | ✓ | ✓ |
| `payment.failed` | 支付失败 | ✓ | ✓ | ✓ | ✓ |
| `payment.cancelled` | 支付已取消 | ✓ | ✓ | ✓ | ✓ |
| `payment.refunded` | 退款已处理 | ✓ | ✓ | ✓ | ✓ |
| `payment.disputed` | 拒付/争议已开启 | ✓ | - | ✓ | ✓ |

## Webhook 负载

标准 webhook 负载格式：

```json
{
  "id": "evt_550e8400-e29b-41d4-a716-446655440000",
  "type": "payment.success",
  "api_version": "v1",
  "created_at": "2026-01-28T10:30:00Z",
  "data": {
    "payment": {
      "id": "pay_550e8400-e29b-41d4-a716-446655440001",
      "gateway": "stripe",
      "gateway_payment_id": "pi_3O...",
      "amount": "99.99",
      "currency": "USD",
      "status": "succeeded",
      "order_id": "ord_550e8400-e29b-41d4-a716-446655440002"
    }
  }
}
```

## 安全

### 签名验证

所有 webhooks 都包含用于验证的签名头：

```http
X-Webhook-Signature: t=1706448000,v1=sha256=...
```

验证签名以确保 webhooks 来自合法来源：

```rust
use rcommerce_core::webhook::verify_signature;

let signature = headers.get("X-Webhook-Signature").unwrap();
let body = request.body();
let secret = std::env::var("WEBHOOK_SECRET").unwrap();

if !verify_signature(signature, body, &secret) {
    return Err(Error::unauthorized("Invalid signature"));
}
```

### 重放保护

- 检查 `created_at` 时间戳（如果 > 5 分钟前则拒绝）
- 跟踪已处理的事件 ID 以防止重复
- 为 webhook 处理程序使用幂等键

## 处理 Webhooks

### 最佳实践

1. **快速确认**: 立即返回 200 OK，异步处理
2. **幂等性**: 优雅地处理重复的 webhooks
3. **重试**: 预期并处理重试（指数退避）
4. **日志**: 记录所有 webhook 事件以供调试
5. **顺序**: 不要假设 webhooks 按顺序到达

### 示例处理程序

```rust
async fn handle_webhook(
    headers: HeaderMap,
    body: Bytes,
    state: AppState,
) -> Result<impl IntoResponse, Error> {
    // 验证签名
    let signature = headers.get("X-Webhook-Signature")
        .ok_or(Error::bad_request("Missing signature"))?;
    
    verify_webhook_signature(&body, signature)?;
    
    // 解析事件
    let event: WebhookEvent = serde_json::from_slice(&body)?;
    
    // 排队进行异步处理
    state.job_queue.enqueue(ProcessWebhookJob {
        event_id: event.id.clone(),
        event_type: event.type_.clone(),
        payload: body.to_vec(),
    }).await?;
    
    // 立即返回
    Ok(StatusCode::OK)
}
```

### 异步处理

```rust
async fn process_webhook(event: WebhookEvent) -> Result<(), Error> {
    // 检查重复
    if is_event_processed(&event.id).await? {
        return Ok(());
    }
    
    match event.type_.as_str() {
        "payment.success" => {
            let payment = event.data.payment;
            update_order_status(&payment.order_id, "paid").await?;
            send_confirmation_email(&payment.order_id).await?;
            update_inventory(&payment.order_id).await?;
        }
        "payment.failed" => {
            let payment = event.data.payment;
            update_order_status(&payment.order_id, "payment_failed").await?;
            send_payment_failed_email(&payment.order_id).await?;
        }
        "payment.refunded" => {
            let refund = event.data.refund;
            process_refund(&refund).await?;
        }
        _ => {
            log::warn!("Unhandled webhook event: {}", event.type_);
        }
    }
    
    // 标记为已处理
    mark_event_processed(&event.id).await?;
    
    Ok(())
}
```

## 重试行为

| 网关 | 重试策略 | 最大重试次数 |
|---------|---------------|-------------|
| Stripe | 指数退避: 1分钟, 5分钟, 25分钟, 125分钟, 625分钟 | 3 天 |
| Airwallex | 指数退避: 5秒, 25秒, 125秒, 625秒, 3125秒 | 24 小时 |
| 支付宝 | 固定: 4分钟, 10分钟, 10分钟, 1小时, 2小时, 6小时, 15小时 | 25 小时 |
| 微信支付 | 指数退避: 8秒, 64秒, 512秒, 4096秒, 32768秒 | 48 小时 |

## 测试 Webhooks

### 本地开发

使用 webhook 转发工具：

```bash
# Stripe CLI
stripe listen --forward-to localhost:8080/webhooks/stripe

# ngrok 用于其他网关
ngrok http 8080
```

### 测试事件

通过 API 触发测试事件：

```http
POST /api/v1/admin/webhooks/test
Authorization: Bearer <admin_token>

{
  "gateway": "stripe",
  "event_type": "payment.success",
  "payload": {
    "payment_id": "pay_test_123"
  }
}
```

## 故障排除

| 问题 | 解决方案 |
|-------|----------|
| 未收到 Webhooks | 检查端点 URL、防火墙规则 |
| 签名验证失败 | 验证密钥与网关匹配 |
| 重复处理 | 实现幂等性检查 |
| 超时 | 异步处理，快速返回 200 |
| 缺失事件 | 检查网关仪表板中的投递状态 |

## 仪表板

在 R Commerce 管理中查看 webhook 投递状态：

```
管理 → 设置 → Webhooks → 日志
```

显示：
- 事件 ID 和类型
- 投递状态
- 响应代码
- 重试次数
- 负载预览
