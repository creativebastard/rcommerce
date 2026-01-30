# Airwallex 集成

Airwallex 提供多币种支付处理，具有竞争力的外汇汇率和全球覆盖。

## 功能

- **多币种**: 接受 60+ 币种，以您的本币结算
- **外汇优化**: 实时汇率，低利润
- **支付方式**: 卡、银行转账、本地支付方式
- **全球覆盖**: 在亚太地区强大，全球扩展
- **付款**: 向全球供应商和合作伙伴发送付款

## 配置

### 环境变量

```bash
AIRWALLEX_API_KEY=your_api_key
AIRWALLEX_CLIENT_ID=your_client_id
AIRWALLEX_WEBHOOK_SECRET=your_webhook_secret
```

### 配置文件

```toml
[payment.airwallex]
enabled = true
api_key = "${AIRWALLEX_API_KEY}"
client_id = "${AIRWALLEX_CLIENT_ID}"
webhook_secret = "${AIRWALLEX_WEBHOOK_SECRET}"
sandbox = false
```

## API 使用

### 创建支付意图

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "airwallex",
  "amount": "150.00",
  "currency": "AUD",
  "customer_email": "customer@example.com",
  "payment_method_types": ["card", "alipaycn"]
}
```

**响应：**

```json
{
  "id": "int_...",
  "client_secret": "int_..._secret_...",
  "status": "requires_payment_method",
  "amount": 150.00,
  "currency": "AUD",
  "available_payment_methods": ["card", "alipaycn"]
}
```

### 多币种示例

```http
POST /api/v1/payments
Content-Type: application/json

{
  "gateway": "airwallex",
  "amount": "10000",
  "currency": "JPY",
  "settlement_currency": "USD"
}
```

## Webhook 事件

配置 webhook 端点：`https://api.yoursite.com/webhooks/airwallex`

| 事件 | 说明 |
|-------|-------------|
| `payment_intent.created` | 支付意图已创建 |
| `payment_intent.requires_payment_method` | 等待支付方式 |
| `payment_intent.requires_capture` | 已授权，等待捕获 |
| `payment_intent.succeeded` | 支付完成 |
| `payment_intent.cancelled` | 支付已取消 |
| `refund.succeeded` | 退款已处理 |

## 外汇和多币种

### 支持的币种

主要币种：USD、EUR、GBP、AUD、CAD、JPY、CNY、HKD、SGD、NZD

### 外汇汇率锁定

请求汇率锁定以确保价格确定：

```http
POST /api/v1/payments
{
  "gateway": "airwallex",
  "amount": "1000.00",
  "currency": "EUR",
  "lock_fx_rate": true,
  "fx_rate_valid_until": "2026-01-29T10:00:00Z"
}
```

## 测试

使用 Airwallex 沙盒环境：

```toml
[payment.airwallex]
sandbox = true
```

测试卡：

| 卡号 | 结果 |
|-------------|--------|
| 4111 1111 1111 1111 | 成功 |
| 4000 0000 0000 0002 | 拒绝 |

## 最佳实践

1. **币种显示**: 以客户本币显示价格
2. **外汇透明度**: 预先显示汇率和费用
3. **结算**: 根据您的成本选择结算币种
4. **Webhook 处理**: 幂等地处理 webhooks
