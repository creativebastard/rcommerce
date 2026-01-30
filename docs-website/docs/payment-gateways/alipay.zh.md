# 支付宝集成

支付宝是中国领先的数字支付平台，拥有超过 10 亿用户。

## 功能

- **二维码支付**: 店内扫码支付
- **应用内支付**: 在您的移动应用内支付
- **网站支付**: 重定向到支付宝进行支付
- **快捷支付**: 回头客一键支付
- **多币种**: 支持人民币和主要外币

## 先决条件

- 支付宝商户账户
- 完成的商业验证
- 来自支付宝开发者中心的 API 凭证

## 配置

### 环境变量

```bash
ALIPAY_APP_ID=your_app_id
ALIPAY_PRIVATE_KEY=your_private_key
ALIPAY_PUBLIC_KEY=alipay_public_key
ALIPAY_GATEWAY_URL=https://openapi.alipay.com/gateway.do
```

### 配置文件

```toml
[payment.alipay]
enabled = true
app_id = "${ALIPAY_APP_ID}"
private_key = "${ALIPAY_PRIVATE_KEY}"
public_key = "${ALIPAY_PUBLIC_KEY}"
gateway_url = "https://openapi.alipay.com/gateway.do"
sandbox = false
```

## API 使用

### 创建支付（网站）

```http
POST /api/v1/payments
Content-Type: application/json
Authorization: Bearer <token>

{
  "gateway": "alipay",
  "amount": "299.00",
  "currency": "CNY",
  "payment_method": "web",
  "subject": "商品购买",
  "body": "订单 #12345",
  "return_url": "https://yoursite.com/payment/success",
  "notify_url": "https://yoursite.com/webhooks/alipay"
}
```

**响应：**

```json
{
  "id": "2026...",
  "payment_url": "https://mapi.alipay.com/...",
  "status": "pending"
}
```

将客户重定向到 `payment_url` 以完成支付。

### 创建支付（移动应用）

```http
POST /api/v1/payments
{
  "gateway": "alipay",
  "amount": "299.00",
  "currency": "CNY",
  "payment_method": "app",
  "subject": "商品购买"
}
```

**响应：**

```json
{
  "id": "2026...",
  "order_string": "app_id=...&biz_content=...&sign=...",
  "status": "pending"
}
```

将 `order_string` 传递给您的移动应用以启动支付宝 SDK 支付。

### 查询支付状态

```http
GET /api/v1/payments/{payment_id}
Authorization: Bearer <token>
```

## Webhook 通知

支付宝发送异步通知到您的 `notify_url`：

```
POST https://yoursite.com/webhooks/alipay
Content-Type: application/x-www-form-urlencoded

trade_status=TRADE_SUCCESS
out_trade_no=your_order_id
trade_no=alipay_trade_id
total_amount=299.00
...
```

在处理之前验证通知签名。

## 支付状态

| 状态 | 说明 |
|--------|-------------|
| `WAIT_BUYER_PAY` | 等待客户支付 |
| `TRADE_CLOSED` | 支付窗口过期或已退款 |
| `TRADE_SUCCESS` | 支付完成 |
| `TRADE_FINISHED` | 交易完成，不允许退款 |

## 退款

```http
POST /api/v1/payments/{payment_id}/refund
{
  "amount": "299.00",
  "reason": "客户要求"
}
```

## 测试

使用支付宝沙盒：

```toml
[payment.alipay]
sandbox = true
gateway_url = "https://openapi.alipaydev.com/gateway.do"
```

沙盒买家账户：`sandbox_buyer@alipay.com`

## 最佳实践

1. **签名验证**: 始终验证支付宝签名
2. **幂等性**: 优雅地处理重复通知
3. **超时处理**: 支付宝支付在 15 分钟后超时
4. **移动优化**: 确保移动支付流程对移动设备友好
