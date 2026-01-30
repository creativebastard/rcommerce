# 支付 API

支付 API 处理支付处理、交易、退款和支付方式管理。

## 基础 URL

```
/api/v1/payments
```

## 认证

支付端点需要密钥进行支付处理。只读访问可使用受限密钥。

```http
Authorization: Bearer YOUR_SECRET_KEY
```

## 支付对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440200",
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "amount": "59.49",
  "currency": "USD",
  "status": "succeeded",
  "gateway": "stripe",
  "gateway_payment_id": "pi_3O...",
  "payment_method": {
    "type": "card",
    "card": {
      "brand": "visa",
      "last4": "4242",
      "exp_month": 12,
      "exp_year": 2025,
      "fingerprint": "fp_..."
    }
  },
  "description": "订单 #1001",
  "receipt_email": "customer@example.com",
  "receipt_url": "https://pay.stripe.com/receipts/...",
  "captured": true,
  "capture_method": "automatic",
  "confirmation_method": "automatic",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "refunded_amount": "0.00",
  "refunds": [],
  "dispute": null,
  "metadata": {
    "order_number": "1001",
    "customer_name": "John Doe"
  },
  "created_at": "2024-01-15T10:01:00Z",
  "updated_at": "2024-01-15T10:01:30Z",
  "captured_at": "2024-01-15T10:01:30Z"
}
```

### 支付字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 唯一标识符 |
| `order_id` | UUID | 关联订单 |
| `amount` | decimal | 支付金额 |
| `currency` | string | ISO 4217 货币代码 |
| `status` | string | `pending`、`processing`、`succeeded`、`failed`、`canceled`、`refunded` |
| `gateway` | string | 使用的支付网关 |
| `gateway_payment_id` | string | 网关的交易 ID |
| `payment_method` | object | 支付方式详情 |
| `description` | string | 支付描述 |
| `receipt_email` | string | 收据邮箱 |
| `receipt_url` | string | 查看收据的 URL |
| `captured` | boolean | 资金已捕获 |
| `capture_method` | string | `automatic` 或 `manual` |
| `customer_id` | UUID | 已保存的客户（如适用） |
| `refunded_amount` | decimal | 退款总额 |
| `refunds` | array | 退款列表 |
| `dispute` | object | 争议信息 |
| `metadata` | object | 自定义键值数据 |
| `created_at` | datetime | 创建时间戳 |
| `updated_at` | datetime | 最后修改时间 |
| `captured_at` | datetime | 捕获时间戳 |

## 端点

### 列出支付

```http
GET /api/v1/payments
```

检索分页的支付列表。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `order_id` | UUID | 按订单筛选 |
| `customer_id` | UUID | 按客户筛选 |
| `gateway` | string | 按支付网关筛选 |
| `status` | string | 按状态筛选 |
| `min_amount` | decimal | 最低金额 |
| `max_amount` | decimal | 最高金额 |
| `created_after` | datetime | 创建日期之后 |
| `created_before` | datetime | 创建日期之前 |
| `sort` | string | `created_at`、`amount` |
| `order` | string | `asc` 或 `desc` |

### 获取支付

```http
GET /api/v1/payments/{id}
```

通过 ID 检索单个支付。

### 创建支付

```http
POST /api/v1/payments
```

为订单创建新支付。

#### 请求体

```json
{
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "amount": "59.49",
  "currency": "USD",
  "gateway": "stripe",
  "payment_method": {
    "type": "card",
    "card": {
      "number": "4242424242424242",
      "exp_month": 12,
      "exp_year": 2025,
      "cvc": "123"
    }
  },
  "capture_method": "automatic",
  "receipt_email": "customer@example.com",
  "description": "订单 #1001",
  "metadata": {
    "order_number": "1001"
  }
}
```

### 捕获支付

```http
POST /api/v1/payments/{id}/capture
```

捕获已授权（未捕获）的支付。

#### 请求体

```json
{
  "amount": "59.49"
}
```

### 取消支付

```http
POST /api/v1/payments/{id}/cancel
```

取消未捕获的支付。

## 退款

### 创建退款

```http
POST /api/v1/payments/{id}/refunds
```

对已捕获的支付进行退款。

#### 请求体

```json
{
  "amount": "59.49",
  "reason": "requested_by_customer",
  "metadata": {
    "note": "客户对产品不满意"
  }
}
```

#### 退款原因

- `duplicate` - 重复扣款
- `fraudulent` - 欺诈交易
- `requested_by_customer` - 客户要求

### 获取退款

```http
GET /api/v1/payments/{payment_id}/refunds/{refund_id}
```

### 列出退款

```http
GET /api/v1/payments/{payment_id}/refunds
```

## 支付方式

### 列出客户支付方式

```http
GET /api/v1/customers/{customer_id}/payment_methods
```

### 创建支付方式

```http
POST /api/v1/customers/{customer_id}/payment_methods
```

#### 请求体

```json
{
  "type": "card",
  "card": {
    "number": "4242424242424242",
    "exp_month": 12,
    "exp_year": 2025,
    "cvc": "123"
  },
  "set_as_default": true
}
```

### 删除支付方式

```http
DELETE /api/v1/customers/{customer_id}/payment_methods/{payment_method_id}
```

### 设置默认支付方式

```http
POST /api/v1/customers/{customer_id}/payment_methods/{payment_method_id}/default
```

## 支付意图

支付意图用于具有 3D 安全验证的复杂支付流程。

### 创建支付意图

```http
POST /api/v1/payment_intents
```

#### 请求体

```json
{
  "amount": "59.49",
  "currency": "USD",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "payment_method": "pm_...",
  "confirmation_method": "manual",
  "capture_method": "automatic",
  "setup_future_usage": "off_session",
  "metadata": {
    "order_id": "550e8400-e29b-41d4-a716-446655440100"
  }
}
```

### 确认支付意图

```http
POST /api/v1/payment_intents/{id}/confirm
```

### 捕获支付意图

```http
POST /api/v1/payment_intents/{id}/capture
```

### 取消支付意图

```http
POST /api/v1/payment_intents/{id}/cancel
```

## 争议

### 列出争议

```http
GET /api/v1/disputes
```

### 获取争议

```http
GET /api/v1/disputes/{id}
```

### 提交证据

```http
POST /api/v1/disputes/{id}/evidence
```

#### 请求体

```json
{
  "product_description": "优质棉质 T 恤",
  "customer_email": "customer@example.com",
  "shipping_date": "2024-01-16",
  "shipping_carrier": "UPS",
  "shipping_tracking_number": "1Z999...",
  "access_activity_log": "客户访问数字下载 3 次",
  "uncategorized_text": "其他说明...",
  "uncategorized_file": "file_..."
}
```

## 提现

### 列出提现

```http
GET /api/v1/payouts
```

检索到您银行账户的提现。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `status` | string | `pending`、`in_transit`、`paid`、`failed`、`canceled` |
| `date_after` | date | 日期之后的提现（YYYY-MM-DD） |
| `date_before` | date | 日期之前的提现 |

### 获取提现

```http
GET /api/v1/payouts/{id}
```

## 余额

### 获取余额

```http
GET /api/v1/balance
```

检索当前账户余额。

#### 示例响应

```json
{
  "available": [
    {
      "currency": "USD",
      "amount": "12500.00"
    }
  ],
  "pending": [
    {
      "currency": "USD",
      "amount": "3500.00"
    }
  ],
  "instant_available": [
    {
      "currency": "USD",
      "amount": "5000.00"
    }
  ]
}
```

### 获取余额交易

```http
GET /api/v1/balance/transactions
```

检索详细的余额交易历史。

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `PAYMENT_NOT_FOUND` | 404 | 支付不存在 |
| `INVALID_AMOUNT` | 400 | 支付金额无效 |
| `INVALID_CURRENCY` | 400 | 不支持的货币 |
| `INVALID_PAYMENT_METHOD` | 400 | 卡或银行信息无效 |
| `CARD_DECLINED` | 402 | 卡被拒绝 |
| `INSUFFICIENT_FUNDS` | 402 | 卡余额不足 |
| `EXPIRED_CARD` | 402 | 卡已过期 |
| `INCORRECT_CVC` | 402 | CVC 验证失败 |
| `PROCESSING_ERROR` | 402 | 网关处理错误 |
| `ALREADY_CAPTURED` | 409 | 支付已捕获 |
| `ALREADY_REFUNDED` | 409 | 支付已全额退款 |
| `REFUND_AMOUNT_INVALID` | 400 | 退款超过支付金额 |
| `DISPUTE_NOT_FOUND` | 404 | 争议不存在 |

## Webhooks

| 事件 | 说明 |
|-------|-------------|
| `payment.created` | 新支付已发起 |
| `payment.succeeded` | 支付成功完成 |
| `payment.failed` | 支付失败 |
| `payment.captured` | 已授权支付已捕获 |
| `payment.canceled` | 支付已取消 |
| `refund.created` | 退款已发起 |
| `refund.succeeded` | 退款已完成 |
| `refund.failed` | 退款失败 |
| `dispute.created` | 争议/拒付已开启 |
| `dispute.updated` | 争议状态已更改 |
| `dispute.closed` | 争议已解决 |
| `payout.created` | 提现已发起 |
| `payout.paid` | 提现已存入 |
| `payout.failed` | 提现失败 |
