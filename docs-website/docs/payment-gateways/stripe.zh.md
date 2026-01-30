# Stripe 集成

Stripe 提供全球支付处理，支持卡、钱包和订阅。

## 功能

- **卡**: Visa、Mastercard、Amex、Discover、JCB
- **钱包**: Apple Pay、Google Pay、Link
- **银行转账**: ACH、SEPA、BACS
- **先买后付**: Klarna、Afterpay、Affirm
- **订阅**: 使用 Stripe Billing 的定期计费

## 配置

### 环境变量

```bash
STRIPE_API_KEY=sk_live_...
STRIPE_WEBHOOK_SECRET=whsec_...
STRIPE_PUBLISHABLE_KEY=pk_live_...
```

### 配置文件

```toml
[payment.stripe]
enabled = true
api_key = "${STRIPE_API_KEY}"
webhook_secret = "${STRIPE_WEBHOOK_SECRET}"
publishable_key = "${STRIPE_PUBLISHABLE_KEY}"
capture_method = "automatic"  # 或 "manual"
```

## API 使用

### 创建支付意图

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "stripe",
  "amount": "99.99",
  "currency": "USD",
  "payment_method_types": ["card"],
  "metadata": {
    "order_id": "order_123"
  }
}
```

**响应：**

```json
{
  "id": "pi_3O...",
  "client_secret": "pi_3O..._secret_...",
  "status": "requires_payment_method",
  "amount": 9999,
  "currency": "USD"
}
```

使用 `client_secret` 与 Stripe.js 在前端完成支付。

### 捕获支付（手动）

如果使用手动捕获：

```http
POST /api/v1/payments/pi_3O.../capture
Authorization: Bearer <token>

{
  "amount": "99.99"
}
```

### 退款

```http
POST /api/v1/payments/pi_3O.../refund
Authorization: Bearer <token>

{
  "amount": "99.99",
  "reason": "requested_by_customer"
}
```

## Webhook 事件

配置 webhook 端点：`https://api.yoursite.com/webhooks/stripe`

| 事件 | 说明 |
|-------|-------------|
| `payment_intent.succeeded` | 支付完成 |
| `payment_intent.payment_failed` | 支付失败 |
| `payment_intent.canceled` | 支付已取消 |
| `charge.refunded` | 退款已处理 |
| `invoice.paid` | 订阅付款已收到 |
| `invoice.payment_failed` | 订阅付款失败 |

## 前端集成

### Stripe.js 示例

```javascript
import { loadStripe } from '@stripe/stripe-js';

const stripe = await loadStripe('pk_live_...');

// 在后端创建支付意图
const { client_secret } = await fetch('/api/v1/payments', {
  method: 'POST',
  body: JSON.stringify({ amount: 99.99, gateway: 'stripe' })
}).then(r => r.json());

// 确认支付
const result = await stripe.confirmCardPayment(client_secret, {
  payment_method: {
    card: cardElement,
    billing_details: { name: 'Customer Name' }
  }
});

if (result.error) {
  // 显示错误
} else {
  // 支付成功
}
```

## 测试

使用 Stripe 测试卡：

| 卡号 | 场景 |
|-------------|----------|
| 4242 4242 4242 4242 | 成功 |
| 4000 0000 0000 0002 | 拒绝 |
| 4000 0000 0000 3220 | 需要 3D 安全验证 |

## 最佳实践

1. **幂等性**: 始终使用幂等键进行重试
2. **Webhooks**: 验证 webhook 签名
3. **日志**: 记录所有支付事件以供审计
4. **错误处理**: 优雅地处理拒绝并提供重试选项
