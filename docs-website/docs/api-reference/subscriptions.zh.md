# 订阅 API

订阅 API 提供全面的定期计费管理，允许您创建和管理基于订阅的产品，支持自动计费周期、开票和生命周期管理。

## 基础 URL

```
/api/v1/subscriptions
```

## 认证

所有订阅端点都需要通过 API 密钥或 JWT 令牌进行认证。

```http
Authorization: Bearer YOUR_API_KEY
```

## 订阅对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "order_id": "550e8400-e29b-41d4-a716-446655440002",
  "product_id": "550e8400-e29b-41d4-a716-446655440003",
  "variant_id": "550e8400-e29b-41d4-a716-446655440004",
  "status": "active",
  "interval": "monthly",
  "interval_count": 1,
  "currency": "USD",
  "amount": "29.99",
  "setup_fee": "9.99",
  "trial_days": 14,
  "trial_ends_at": "2024-02-15T10:00:00Z",
  "current_cycle": 3,
  "min_cycles": 3,
  "max_cycles": null,
  "starts_at": "2024-01-15T10:00:00Z",
  "next_billing_at": "2024-04-15T10:00:00Z",
  "last_billing_at": "2024-03-15T10:00:00Z",
  "ends_at": null,
  "cancelled_at": null,
  "cancellation_reason": null,
  "payment_method_id": "pm_1234567890",
  "gateway": "stripe",
  "notes": "Premium plan subscription",
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-03-15T10:00:00Z"
}
```

### 订阅字段

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | UUID | 唯一标识符 |
| `customer_id` | UUID | 订阅所有者客户 |
| `order_id` | UUID | 创建订阅的原始订单 |
| `product_id` | UUID | 订阅产品 ID |
| `variant_id` | UUID | 产品变体 ID（如适用） |
| `status` | string | `active`、`paused`、`cancelled`、`expired`、`past_due`、`trialing`、`pending` |
| `interval` | string | `daily`、`weekly`、`bi_weekly`、`monthly`、`quarterly`、`bi_annually`、`annually` |
| `interval_count` | integer | 计费间隔数（例如，每 3 个月为 3） |
| `currency` | string | ISO 4217 货币代码 |
| `amount` | decimal | 每个计费周期收取的金额 |
| `setup_fee` | decimal | 一次性设置费（可选） |
| `trial_days` | integer | 试用天数 |
| `trial_ends_at` | datetime | 试用期结束时间 |
| `current_cycle` | integer | 当前计费周期数 |
| `min_cycles` | integer | 允许取消前的最小周期数 |
| `max_cycles` | integer | 最大周期数（null = 无限） |
| `starts_at` | datetime | 订阅开始日期 |
| `next_billing_at` | datetime | 下次计划计费日期 |
| `last_billing_at` | datetime | 上次成功计费日期 |
| `ends_at` | datetime | 订阅结束时间（如已计划） |
| `cancelled_at` | datetime | 订阅取消时间 |
| `cancellation_reason` | string | 取消原因 |
| `payment_method_id` | string | 支付网关方法 ID |
| `gateway` | string | 使用的支付网关（例如 `stripe`、`airwallex`） |
| `notes` | string | 内部备注 |
| `created_at` | datetime | 创建时间戳 |
| `updated_at` | datetime | 最后修改时间戳 |

## 订阅发票对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
  "order_id": "550e8400-e29b-41d4-a716-446655440020",
  "cycle_number": 3,
  "period_start": "2024-03-15T10:00:00Z",
  "period_end": "2024-04-15T10:00:00Z",
  "subtotal": "29.99",
  "tax_total": "2.99",
  "total": "32.98",
  "status": "paid",
  "paid_at": "2024-03-15T10:05:00Z",
  "payment_id": "pi_1234567890",
  "failed_attempts": 0,
  "last_failed_at": null,
  "failure_reason": null,
  "next_retry_at": null,
  "retry_count": 0,
  "created_at": "2024-03-15T10:00:00Z",
  "updated_at": "2024-03-15T10:05:00Z"
}
```

### 发票字段

| 字段 | 类型 | 描述 |
|------|------|------|
| `id` | UUID | 唯一标识符 |
| `subscription_id` | UUID | 关联订阅 ID |
| `order_id` | UUID | 生成的订单 ID（如适用） |
| `cycle_number` | integer | 计费周期数 |
| `period_start` | datetime | 计费周期开始 |
| `period_end` | datetime | 计费周期结束 |
| `subtotal` | decimal | 税前金额 |
| `tax_total` | decimal | 税额 |
| `total` | decimal | 应付总金额 |
| `status` | string | `pending`、`billed`、`paid`、`failed`、`past_due`、`cancelled` |
| `paid_at` | datetime | 收到付款时间 |
| `payment_id` | string | 网关支付 ID |
| `failed_attempts` | integer | 支付失败尝试次数 |
| `failure_reason` | string | 上次失败原因 |
| `next_retry_at` | datetime | 下次计划重试日期 |
| `retry_count` | integer | 重试尝试次数 |

## 端点

### 列出订阅

```http
GET /api/v1/subscriptions
```

检索认证客户的订阅分页列表。

#### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `status` | string | 按状态筛选：`active`、`paused`、`cancelled`、`expired`、`past_due`、`trialing`、`pending` |
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |

#### 示例请求

```http
GET /api/v1/subscriptions?status=active&page=1&per_page=20
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "subscriptions": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "customer_id": "550e8400-e29b-41d4-a716-446655440001",
      "product_id": "550e8400-e29b-41d4-a716-446655440003",
      "status": "active",
      "interval": "monthly",
      "interval_count": 1,
      "currency": "USD",
      "amount": "29.99",
      "next_billing_at": "2024-04-15T10:00:00Z",
      "created_at": "2024-01-15T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 15,
    "total_pages": 1
  }
}
```

### 创建订阅

```http
POST /api/v1/subscriptions
```

为客户创建新订阅。

#### 请求体

```json
{
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "order_id": "550e8400-e29b-41d4-a716-446655440002",
  "product_id": "550e8400-e29b-41d4-a716-446655440003",
  "variant_id": "550e8400-e29b-41d4-a716-446655440004",
  "interval": "monthly",
  "interval_count": 1,
  "currency": "USD",
  "amount": "29.99",
  "setup_fee": "9.99",
  "trial_days": 14,
  "min_cycles": 3,
  "max_cycles": null,
  "payment_method_id": "pm_1234567890",
  "gateway": "stripe",
  "notes": "Premium plan subscription"
}
```

#### 必填字段

- `customer_id` - 客户 UUID
- `order_id` - 原始订单 UUID
- `product_id` - 产品 UUID
- `interval` - 计费间隔
- `interval_count` - 间隔数（1-12）
- `currency` - ISO 4217 货币代码
- `amount` - 订阅金额
- `payment_method_id` - 网关支付方式 ID
- `gateway` - 支付网关标识符

#### 示例响应

```json
{
  "success": true,
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "product_id": "550e8400-e29b-41d4-a716-446655440003",
    "status": "trialing",
    "interval": "monthly",
    "interval_count": 1,
    "currency": "USD",
    "amount": "29.99",
    "trial_days": 14,
    "trial_ends_at": "2024-01-29T10:00:00Z",
    "next_billing_at": "2024-01-29T10:00:00Z",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-15T10:00:00Z"
  }
}
```

### 获取订阅

```http
GET /api/v1/subscriptions/{id}
```

按 ID 检索单个订阅。

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `id` | UUID | 订阅 ID |

#### 示例请求

```http
GET /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_id": "550e8400-e29b-41d4-a716-446655440003",
    "status": "active",
    "interval": "monthly",
    "interval_count": 1,
    "currency": "USD",
    "amount": "29.99",
    "current_cycle": 3,
    "next_billing_at": "2024-04-15T10:00:00Z",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-03-15T10:00:00Z"
  }
}
```

### 更新订阅

```http
PUT /api/v1/subscriptions/{id}
```

更新现有订阅。

#### 请求体

```json
{
  "interval": "quarterly",
  "interval_count": 1,
  "amount": "79.99",
  "next_billing_at": "2024-04-15T10:00:00Z",
  "max_cycles": 12,
  "payment_method_id": "pm_newpaymentmethod",
  "notes": "Upgraded to quarterly billing"
}
```

#### 示例响应

```json
{
  "success": true,
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "interval": "quarterly",
    "interval_count": 1,
    "amount": "79.99",
    "next_billing_at": "2024-04-15T10:00:00Z",
    "max_cycles": 12,
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### 取消订阅

```http
POST /api/v1/subscriptions/{id}/cancel
```

取消订阅。

#### 请求体

```json
{
  "reason": "too_expensive",
  "reason_details": "Found a better alternative",
  "cancel_at_end": true
}
```

#### 取消原因

| 原因 | 描述 |
|------|------|
| `customer_requested` | 客户主动取消 |
| `payment_failed` | 重复支付失败 |
| `fraudulent` | 检测到欺诈活动 |
| `too_expensive` | 价格问题 |
| `not_useful` | 不再需要产品 |
| `other` | 其他原因 |

#### 参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `cancel_at_end` | boolean | 如果为 true，在当前周期结束时取消；如果为 false，立即取消 |

#### 示例响应

```json
{
  "success": true,
  "message": "Subscription cancelled successfully",
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "cancelled",
    "cancelled_at": "2024-03-20T10:00:00Z",
    "cancellation_reason": "too_expensive",
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### 暂停订阅

```http
POST /api/v1/subscriptions/{id}/pause
```

暂时暂停活动订阅。计费将暂停直到恢复。

#### 示例请求

```http
POST /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/pause
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "success": true,
  "message": "Subscription paused successfully",
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "paused",
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### 恢复订阅

```http
POST /api/v1/subscriptions/{id}/resume
```

恢复暂停的订阅。

#### 示例请求

```http
POST /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/resume
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "success": true,
  "message": "Subscription resumed successfully",
  "subscription": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "status": "active",
    "next_billing_at": "2024-04-15T10:00:00Z",
    "updated_at": "2024-03-20T10:00:00Z"
  }
}
```

### 获取订阅发票

```http
GET /api/v1/subscriptions/{id}/invoices
```

检索订阅的所有发票。

#### 示例请求

```http
GET /api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/invoices
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "invoices": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440010",
      "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
      "cycle_number": 1,
      "period_start": "2024-01-29T10:00:00Z",
      "period_end": "2024-02-29T10:00:00Z",
      "subtotal": "29.99",
      "tax_total": "2.99",
      "total": "32.98",
      "status": "paid",
      "paid_at": "2024-01-29T10:05:00Z",
      "created_at": "2024-01-29T10:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440011",
      "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
      "cycle_number": 2,
      "period_start": "2024-02-29T10:00:00Z",
      "period_end": "2024-03-29T10:00:00Z",
      "subtotal": "29.99",
      "tax_total": "2.99",
      "total": "32.98",
      "status": "paid",
      "paid_at": "2024-02-29T10:05:00Z",
      "created_at": "2024-02-29T10:00:00Z"
    }
  ]
}
```

## 管理员端点

### 列出所有订阅（管理员）

```http
GET /api/v1/admin/subscriptions
```

检索所有客户的所有订阅（仅管理员）。

#### 查询参数

| 参数 | 类型 | 描述 |
|------|------|------|
| `status` | string | 按状态筛选 |
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |

### 获取订阅摘要（管理员）

```http
GET /api/v1/admin/subscriptions/summary
```

获取汇总的订阅统计信息。

#### 示例响应

```json
{
  "summary": {
    "total_active": 1250,
    "total_cancelled": 320,
    "total_expired": 45,
    "total_past_due": 12,
    "monthly_recurring_revenue": "45250.00",
    "annual_recurring_revenue": "543000.00"
  }
}
```

### 处理计费（管理员）

```http
POST /api/v1/admin/subscriptions/process-billing
```

触发所有到期订阅的计费运行。

#### 示例响应

```json
{
  "success": true,
  "message": "Processed 45 subscriptions",
  "invoices_created": 45
}
```

## 代码示例

### cURL

```bash
# 列出订阅
curl -X GET "https://api.rcommerce.app/api/v1/subscriptions?status=active" \
  -H "Authorization: Bearer sk_live_xxx"

# 创建订阅
curl -X POST "https://api.rcommerce.app/api/v1/subscriptions" \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "order_id": "550e8400-e29b-41d4-a716-446655440002",
    "product_id": "550e8400-e29b-41d4-a716-446655440003",
    "interval": "monthly",
    "interval_count": 1,
    "currency": "USD",
    "amount": "29.99",
    "payment_method_id": "pm_1234567890",
    "gateway": "stripe"
  }'

# 取消订阅
curl -X POST "https://api.rcommerce.app/api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/cancel" \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "reason": "too_expensive",
    "cancel_at_end": true
  }'
```

### JavaScript

```javascript
// 列出订阅
const response = await fetch('https://api.rcommerce.app/api/v1/subscriptions?status=active', {
  headers: {
    'Authorization': 'Bearer sk_live_xxx'
  }
});
const data = await response.json();
console.log(data.subscriptions);

// 创建订阅
const createResponse = await fetch('https://api.rcommerce.app/api/v1/subscriptions', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer sk_live_xxx',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    customer_id: '550e8400-e29b-41d4-a716-446655440001',
    order_id: '550e8400-e29b-41d4-a716-446655440002',
    product_id: '550e8400-e29b-41d4-a716-446655440003',
    interval: 'monthly',
    interval_count: 1,
    currency: 'USD',
    amount: '29.99',
    payment_method_id: 'pm_1234567890',
    gateway: 'stripe'
  })
});
const newSubscription = await createResponse.json();

// 暂停订阅
await fetch('https://api.rcommerce.app/api/v1/subscriptions/550e8400-e29b-41d4-a716-446655440000/pause', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer sk_live_xxx'
  }
});
```

### Python

```python
import requests

headers = {
    'Authorization': 'Bearer sk_live_xxx',
    'Content-Type': 'application/json'
}

# 列出订阅
response = requests.get(
    'https://api.rcommerce.app/api/v1/subscriptions',
    headers=headers,
    params={'status': 'active'}
)
subscriptions = response.json()['subscriptions']

# 创建订阅
subscription_data = {
    'customer_id': '550e8400-e29b-41d4-a716-446655440001',
    'order_id': '550e8400-e29b-41d4-a716-446655440002',
    'product_id': '550e8400-e29b-41d4-a716-446655440003',
    'interval': 'monthly',
    'interval_count': 1,
    'currency': 'USD',
    'amount': '29.99',
    'payment_method_id': 'pm_1234567890',
    'gateway': 'stripe'
}
response = requests.post(
    'https://api.rcommerce.app/api/v1/subscriptions',
    headers=headers,
    json=subscription_data
)
new_subscription = response.json()['subscription']

# 获取发票
response = requests.get(
    f'https://api.rcommerce.app/api/v1/subscriptions/{new_subscription["id"]}/invoices',
    headers=headers
)
invoices = response.json()['invoices']
```

## 错误代码

| 代码 | HTTP 状态 | 描述 |
|------|----------|------|
| `SUBSCRIPTION_NOT_FOUND` | 404 | 订阅不存在 |
| `INVALID_INTERVAL` | 400 | 无效的计费间隔 |
| `INVALID_CURRENCY` | 400 | 无效的货币代码 |
| `INVALID_AMOUNT` | 400 | 无效的订阅金额 |
| `PAYMENT_METHOD_REQUIRED` | 400 | 需要支付方式 ID |
| `SUBSCRIPTION_ALREADY_CANCELLED` | 409 | 订阅已取消 |
| `SUBSCRIPTION_NOT_ACTIVE` | 409 | 订阅未处于活动状态 |
| `MIN_CYCLES_NOT_MET` | 409 | 未达到最小计费周期 |
| `CUSTOMER_NOT_FOUND` | 404 | 客户不存在 |
| `PRODUCT_NOT_FOUND` | 404 | 产品不存在 |
| `INVALID_TRIAL_DAYS` | 400 | 指定的试用期无效 |

## Webhooks

订阅 API 触发以下 Webhook 事件：

| 事件 | 描述 |
|------|------|
| `subscription.created` | 新订阅已创建 |
| `subscription.updated` | 订阅详情已更改 |
| `subscription.cancelled` | 订阅已取消 |
| `subscription.paused` | 订阅已暂停 |
| `subscription.resumed` | 订阅已恢复 |
| `subscription.trial_ended` | 试用期已结束 |
| `subscription.payment_succeeded` | 定期支付成功 |
| `subscription.payment_failed` | 定期支付失败 |
| `subscription.past_due` | 订阅进入逾期状态 |
| `subscription.expired` | 订阅达到最大周期 |
| `invoice.created` | 新发票已生成 |
| `invoice.paid` | 发票支付成功 |
| `invoice.payment_failed` | 发票支付失败 |
| `invoice.past_due` | 发票变为逾期 |
