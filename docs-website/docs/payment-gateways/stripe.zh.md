# Stripe 集成

Stripe 通过 R Commerce 的服务器端处理 API 提供全球支付处理，支持卡、钱包和订阅。

## 功能

- **卡**：Visa、Mastercard、Amex、Discover、JCB
- **钱包**：Apple Pay、Google Pay、Link
- **银行转账**：ACH、SEPA、BACS
- **先买后付**：Klarna、Afterpay、Affirm
- **订阅**：使用 Stripe Billing 的定期计费
- **3D 安全验证**：自动处理强客户认证 (SCA)

## 配置

### 环境变量

```bash
# 必需 - 仅服务器端
STRIPE_API_KEY=sk_live_...
STRIPE_WEBHOOK_SECRET=whsec_...

# 可选 - 仅在使用 Stripe.js（旧版）时需要
STRIPE_PUBLISHABLE_KEY=pk_live_...
```

**注意：** 使用 R Commerce 的服务器端处理，您只需要**密钥**。新的 v2 API 不需要可发布密钥。

### 配置文件

```toml
[payment.stripe]
enabled = true
api_key = "${STRIPE_API_KEY}"
webhook_secret = "${STRIPE_WEBHOOK_SECRET}"
capture_method = "automatic"  # 或 "manual"
statement_descriptor = "RCOMMERCE"
```

## API 使用 (v2 - 服务器端)

### 获取可用支付方式

```http
POST /api/v2/payments/methods
Content-Type: application/json
Authorization: Bearer <token>

{
  "currency": "USD",
  "amount": "99.99"
}
```

**响应：**

```json
{
  "gateway_id": "stripe",
  "gateway_name": "Stripe",
  "payment_methods": [
    {
      "method_type": "card",
      "display_name": "信用卡/借记卡",
      "requires_redirect": false,
      "supports_3ds": true,
      "required_fields": [
        {
          "name": "number",
          "label": "卡号",
          "field_type": "card_number",
          "required": true,
          "pattern": "^[\\d\\s]{13,19}$"
        },
        {
          "name": "exp_month",
          "label": "到期月份",
          "field_type": "expiry_date",
          "required": true
        },
        {
          "name": "exp_year",
          "label": "到期年份",
          "field_type": "expiry_date",
          "required": true
        },
        {
          "name": "cvc",
          "label": "安全码",
          "field_type": "cvc",
          "required": true
        }
      ]
    }
  ]
}
```

### 发起支付

直接发送卡数据到 R Commerce API（服务器端处理）：

```http
POST /api/v2/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway_id": "stripe",
  "amount": "99.99",
  "currency": "USD",
  "payment_method": {
    "type": "card",
    "card": {
      "number": "4242424242424242",
      "exp_month": 12,
      "exp_year": 2025,
      "cvc": "123",
      "name": "John Doe"
    }
  },
  "order_id": "order_123",
  "customer_email": "customer@example.com",
  "return_url": "https://yoursite.com/checkout/complete"
}
```

**成功响应：**

```json
{
  "type": "success",
  "payment_id": "pay_550e8400-e29b-41d4-a716-446655440000",
  "transaction_id": "pi_3O...",
  "payment_status": "succeeded",
  "payment_method": {
    "method_type": "card",
    "last_four": "4242",
    "card_brand": "visa",
    "exp_month": "12",
    "exp_year": "2025"
  },
  "receipt_url": "https://pay.stripe.com/receipts/..."
}
```

**需要 3D 安全验证响应：**

```json
{
  "type": "requires_action",
  "payment_id": "pay_550e8400-e29b-41d4-a716-446655440000",
  "action_type": "three_d_secure",
  "action_data": {
    "redirect_url": "https://hooks.stripe.com/3d_secure/...",
    "type": "use_stripe_sdk"
  },
  "expires_at": "2026-01-28T11:00:00Z"
}
```

### 完成 3D 安全验证

客户完成 3D 安全验证后：

```http
POST /api/v2/payments/pay_xxx/complete
Content-Type: application/json
Authorization: Bearer <token>

{
  "action_type": "three_d_secure",
  "action_data": {
    "payment_intent": "pi_3O..."
  }
}
```

### 退款

```http
POST /api/v2/payments/pay_xxx/refund
Content-Type: application/json
Authorization: Bearer <token>

{
  "amount": "99.99",
  "reason": "requested_by_customer"
}
```

## 前端集成 (v2)

### JavaScript 示例

```javascript
// checkout.js - 不需要 Stripe.js！

async function processPayment() {
  // 1. 从表单收集卡数据
  const cardData = {
    number: document.getElementById('cardNumber').value,
    exp_month: parseInt(document.getElementById('expMonth').value),
    exp_year: parseInt(document.getElementById('expYear').value),
    cvc: document.getElementById('cvc').value,
    name: document.getElementById('cardName').value
  };
  
  // 2. 发送到 R Commerce API
  const response = await fetch('/api/v2/payments', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${API_KEY}`
    },
    body: JSON.stringify({
      gateway_id: 'stripe',
      amount: '99.99',
      currency: 'USD',
      payment_method: {
        type: 'card',
        card: cardData
      },
      order_id: orderId,
      customer_email: customerEmail,
      return_url: window.location.origin + '/checkout/complete'
    })
  });
  
  const result = await response.json();
  
  // 3. 处理响应
  switch (result.type) {
    case 'success':
      // 支付完成
      window.location.href = '/checkout/success';
      break;
      
    case 'requires_action':
      // 处理 3D 安全验证
      if (result.action_type === 'three_d_secure') {
        // 选项 1：重定向到 Stripe 的 3DS 页面
        window.location.href = result.action_data.redirect_url;
        
        // 选项 2：使用 Stripe.js 进行嵌入式 3DS（可选）
        // const stripe = await loadStripe('pk_...');
        // await stripe.handleCardAction(result.action_data.client_secret);
      }
      break;
      
    case 'failed':
      showError(result.error_message);
      break;
  }
}

// 客户从 3DS 重定向返回时调用
async function handle3DReturn() {
  const urlParams = new URLSearchParams(window.location.search);
  const paymentIntent = urlParams.get('payment_intent');
  const paymentId = sessionStorage.getItem('pending_payment_id');
  
  const response = await fetch(`/api/v2/payments/${paymentId}/complete`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${API_KEY}`
    },
    body: JSON.stringify({
      action_type: 'three_d_secure',
      action_data: { payment_intent: paymentIntent }
    })
  });
  
  const result = await response.json();
  
  if (result.type === 'success') {
    window.location.href = '/checkout/success';
  } else {
    showError(result.error_message);
  }
}
```

## Webhook 事件

配置 webhook 端点：`https://api.yoursite.com/api/v2/webhooks/stripe`

### 必需事件

| 事件 | 说明 |
|-------|-------------|
| `payment_intent.succeeded` | 支付成功完成 |
| `payment_intent.payment_failed` | 支付失败 |
| `payment_intent.canceled` | 支付已取消 |
| `charge.refunded` | 退款已处理 |
| `invoice.paid` | 订阅付款已收到 |
| `invoice.payment_failed` | 订阅付款失败 |

## 测试

### 测试卡

| 卡号 | 场景 |
|-------------|----------|
| `4242424242424242` | 成功 |
| `4000000000000002` | 卡被拒绝 |
| `4000000000009995` | 余额不足 |
| `4000002500003155` | 需要 3D 安全验证 |
| `4000000000003220` | 3D 安全验证 2 无摩擦 |
| `4000008400001629` | 3D 安全验证 2 挑战 |
| `4000000000000127` | CVC 不正确 |
| `4000000000000069` | 卡已过期 |

### 使用 cURL 测试

```bash
# 发起支付
curl -X POST http://localhost:8080/api/v2/payments \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "gateway_id": "stripe",
    "amount": "99.99",
    "currency": "USD",
    "payment_method": {
      "type": "card",
      "card": {
        "number": "4242424242424242",
        "exp_month": 12,
        "exp_year": 2025,
        "cvc": "123",
        "name": "Test User"
      }
    },
    "order_id": "test_order_123"
  }'
```

### 本地 Webhook 测试

```bash
# 安装 Stripe CLI
brew install stripe/stripe-cli/stripe

# 登录
stripe login

# 转发 webhooks 到本地服务器
stripe listen --forward-to localhost:8080/api/v2/webhooks/stripe

# 触发测试事件
stripe trigger payment_intent.succeeded
stripe trigger payment_intent.payment_failed
```

## 旧版集成 (v1)

如果您正在使用带有 Stripe.js 的旧版 v1 API：

```javascript
import { loadStripe } from '@stripe/stripe-js';

const stripe = await loadStripe('pk_live_...');
const { client_secret } = await fetch('/api/v1/payments').then(r => r.json());
const result = await stripe.confirmCardPayment(client_secret, {
  payment_method: { card: cardElement }
});
```

**我们建议迁移到 v2** 以获得更好的安全性和更简单的集成。

## 最佳实践

1. **幂等性**：始终使用幂等键进行重试
   ```json
   {
     "idempotency_key": "每次尝试的唯一键"
   }
   ```

2. **错误处理**：处理所有响应类型（success、requires_action、failed）

3. **3D 安全验证**：始终为可能需要 3DS 的卡提供 `return_url`

4. **Webhooks**：在生产环境中验证 webhook 签名

5. **日志记录**：记录所有支付事件以供审计

6. **测试**：上线前使用测试卡

## 故障排除

### 常见问题

**"Gateway not found" 错误：**
- 检查 Stripe 是否在配置中启用
- 验证 `api_key` 设置是否正确

**"Card declined" 错误：**
- 检查开发环境中是否使用测试密钥
- 验证卡号是否正确

**3D 安全验证不工作：**
- 确保提供了 `return_url`
- 检查 URL 是否可公开访问

**Webhook 错误：**
- 验证 webhook 密钥是否正确
- 检查端点 URL 是否正确 (`/api/v2/webhooks/stripe`)

## 其他资源

- [Stripe 测试文档](https://stripe.com/docs/testing)
- [Stripe 测试卡号](https://stripe.com/docs/testing#cards)
- [3D 安全验证指南](https://stripe.com/docs/payments/3d-secure)
- [Stripe API 参考](https://stripe.com/docs/api)
