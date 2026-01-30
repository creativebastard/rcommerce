# 订单 API

订单 API 管理从创建到履行的完整订单生命周期。

## 基础 URL

```
/api/v1/orders
```

## 认证

所有订单端点都需要认证。管理端点需要密钥。

```http
Authorization: Bearer YOUR_API_KEY
```

## 订单对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440100",
  "order_number": "1001",
  "customer_id": "550e8400-e29b-41d4-a716-446655440001",
  "email": "customer@example.com",
  "phone": "+1-555-0123",
  "status": "open",
  "financial_status": "paid",
  "fulfillment_status": "unfulfilled",
  "currency": "USD",
  "subtotal_price": "49.99",
  "total_tax": "4.50",
  "total_shipping": "5.00",
  "total_discounts": "0.00",
  "total_price": "59.49",
  "total_weight": "1.0",
  "total_items": 2,
  "line_items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440101",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "variant_id": "550e8400-e29b-41d4-a716-446655440001",
      "title": "优质棉质 T 恤",
      "variant_title": "中号 / 黑色",
      "sku": "TSH-M-BLK",
      "quantity": 2,
      "price": "24.99",
      "total": "49.98",
      "requires_shipping": true,
      "is_gift_card": false
    }
  ],
  "shipping_address": {
    "first_name": "John",
    "last_name": "Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "billing_address": {
    "first_name": "John",
    "last_name": "Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "shipping_lines": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440102",
      "title": "标准配送",
      "price": "5.00",
      "code": "standard",
      "source": "shopify"
    }
  ],
  "tax_lines": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440103",
      "title": "纽约州税",
      "rate": "0.09",
      "price": "4.50"
    }
  ],
  "discount_codes": [],
  "note": "请礼品包装",
  "note_attributes": {
    "gift_message": "生日快乐！"
  },
  "cart_token": "cart_abc123",
  "checkout_token": "checkout_def456",
  "referring_site": "https://google.com",
  "landing_site": "/products/t-shirt",
  "source_name": "web",
  "client_details": {
    "browser_ip": "192.168.1.1",
    "user_agent": "Mozilla/5.0...",
    "session_hash": "sess_xyz789"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:05:00Z",
  "processed_at": "2024-01-15T10:01:00Z",
  "closed_at": null,
  "cancelled_at": null,
  "cancel_reason": null
}
```

### 订单字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 唯一标识符 |
| `order_number` | string | 可读的订单号 |
| `customer_id` | UUID | 关联的客户（游客为 null） |
| `email` | string | 客户邮箱地址 |
| `phone` | string | 客户电话号码 |
| `status` | string | `open`、`closed`、`cancelled` |
| `financial_status` | string | `pending`、`authorized`、`paid`、`partially_paid`、`refunded`、`partially_refunded`、`voided` |
| `fulfillment_status` | string | `unfulfilled`、`partial`、`fulfilled` |
| `currency` | string | ISO 4217 货币代码 |
| `subtotal_price` | decimal | 订单项总和 |
| `total_tax` | decimal | 总税额 |
| `total_shipping` | decimal | 总配送费用 |
| `total_discounts` | decimal | 应用的总折扣 |
| `total_price` | decimal | 最终订单总额 |
| `total_weight` | decimal | 总重量（公斤） |
| `total_items` | integer | 商品总数量 |
| `line_items` | array | 订单中的产品 |
| `shipping_address` | object | 配送地址 |
| `billing_address` | object | 账单地址 |
| `shipping_lines` | array | 选择的配送方式 |
| `tax_lines` | array | 应用的税费 |
| `discount_codes` | array | 应用的折扣码 |
| `note` | string | 客户备注 |
| `note_attributes` | object | 自定义键值数据 |
| `cart_token` | string | 原始购物车引用 |
| `checkout_token` | string | 结账会话引用 |
| `referring_site` | string | 流量来源 URL |
| `landing_site` | string | 访问的首页 |
| `source_name` | string | 订单来源（web、pos、api） |
| `client_details` | object | 浏览器/IP 信息 |
| `created_at` | datetime | 订单创建时间 |
| `updated_at` | datetime | 最后修改时间 |
| `processed_at` | datetime | 订单处理时间 |
| `closed_at` | datetime | 订单关闭时间 |
| `cancelled_at` | datetime | 订单取消时间 |
| `cancel_reason` | string | 取消原因 |

## 端点

### 列出订单

```http
GET /api/v1/orders
```

检索分页的订单列表。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `status` | string | 按订单状态筛选 |
| `financial_status` | string | 按支付状态筛选 |
| `fulfillment_status` | string | 按履行状态筛选 |
| `customer_id` | UUID | 按客户筛选 |
| `email` | string | 按客户邮箱筛选 |
| `order_number` | string | 按订单号搜索 |
| `min_total` | decimal | 最低订单总额 |
| `max_total` | decimal | 最高订单总额 |
| `created_after` | datetime | 创建日期之后 |
| `created_before` | datetime | 创建日期之前 |
| `updated_after` | datetime | 更新日期之后 |
| `sort` | string | `created_at`、`updated_at`、`total_price` |
| `order` | string | `asc` 或 `desc`（默认：desc） |

#### 示例请求

```http
GET /api/v1/orders?status=open&financial_status=paid&sort=created_at&order=desc
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440100",
      "order_number": "1001",
      "email": "customer@example.com",
      "total_price": "59.49",
      "currency": "USD",
      "status": "open",
      "financial_status": "paid",
      "fulfillment_status": "unfulfilled",
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:05:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 45,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 3,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

### 获取订单

```http
GET /api/v1/orders/{id}
```

通过 ID 或订单号检索单个订单。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | string | 订单 UUID 或订单号 |

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `include` | string | 相关数据：`fulfillments`、`refunds`、`transactions`、`notes` |

#### 示例请求

```http
GET /api/v1/orders/1001?include=fulfillments,transactions
Authorization: Bearer sk_live_xxx
```

### 创建订单（管理员）

```http
POST /api/v1/orders
```

手动创建新订单（仅限管理员）。

#### 请求体

```json
{
  "email": "customer@example.com",
  "line_items": [
    {
      "variant_id": "550e8400-e29b-41d4-a716-446655440001",
      "quantity": 2,
      "price": "24.99"
    }
  ],
  "shipping_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "billing_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "shipping_lines": [
    {
      "title": "标准配送",
      "price": "5.00"
    }
  ],
  "note": "请礼品包装",
  "send_receipt": true,
  "inventory_behaviour": "decrement_obeying_policy"
}
```

### 更新订单

```http
PUT /api/v1/orders/{id}
```

更新订单详情（创建后可修改的字段有限）。

#### 请求体

```json
{
  "email": "newemail@example.com",
  "note": "更新备注",
  "tags": ["vip", "wholesale"]
}
```

### 取消订单

```http
POST /api/v1/orders/{id}/cancel
```

取消未完成的订单。

#### 请求体

```json
{
  "reason": "customer_request",
  "restock": true,
  "notify_customer": true
}
```

#### 取消原因

- `customer_request` - 客户要求取消
- `fraudulent` - 订单被标记为欺诈
- `inventory` - 商品缺货
- `other` - 其他原因

### 关闭订单

```http
POST /api/v1/orders/{id}/close
```

关闭已履行的订单。

### 重新打开订单

```http
POST /api/v1/orders/{id}/reopen
```

重新打开已关闭或已取消的订单。

## 订单项

### 添加订单项

```http
POST /api/v1/orders/{order_id}/line_items
```

向现有订单添加产品。

#### 请求体

```json
{
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "quantity": 1,
  "price": "24.99"
}
```

### 更新订单项

```http
PUT /api/v1/orders/{order_id}/line_items/{line_item_id}
```

#### 请求体

```json
{
  "quantity": 3,
  "price": "22.99"
}
```

### 移除订单项

```http
DELETE /api/v1/orders/{order_id}/line_items/{line_item_id}
```

## 履行

### 列出履行

```http
GET /api/v1/orders/{order_id}/fulfillments
```

### 创建履行

```http
POST /api/v1/orders/{order_id}/fulfillments
```

#### 请求体

```json
{
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "line_items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440101",
      "quantity": 2
    }
  ],
  "tracking_number": "1Z999AA10123456784",
  "tracking_company": "UPS",
  "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
  "notify_customer": true
}
```

### 更新履行

```http
PUT /api/v1/orders/{order_id}/fulfillments/{fulfillment_id}
```

### 取消履行

```http
POST /api/v1/orders/{order_id}/fulfillments/{fulfillment_id}/cancel
```

## 订单备注

### 列出备注

```http
GET /api/v1/orders/{order_id}/notes
```

### 创建备注

```http
POST /api/v1/orders/{order_id}/notes
```

#### 请求体

```json
{
  "body": "客户询问配送时间",
  "author": "John Smith",
  "notify_customer": false
}
```

### 删除备注

```http
DELETE /api/v1/orders/{order_id}/notes/{note_id}
```

## 退款

### 计算退款

```http
POST /api/v1/orders/{order_id}/refunds/calculate
```

在处理前计算退款金额。

#### 请求体

```json
{
  "shipping": "full",
  "refund_line_items": [
    {
      "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
      "quantity": 1,
      "restock": true
    }
  ]
}
```

### 创建退款

```http
POST /api/v1/orders/{order_id}/refunds
```

#### 请求体

```json
{
  "note": "客户对产品不满意",
  "notify_customer": true,
  "shipping": {
    "full_refund": true
  },
  "refund_line_items": [
    {
      "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
      "quantity": 1,
      "restock": true
    }
  ],
  "transactions": [
    {
      "parent_id": "550e8400-e29b-41d4-a716-446655440200",
      "amount": "24.99",
      "kind": "refund",
      "gateway": "stripe"
    }
  ]
}
```

## 风险评估

### 获取订单风险

```http
GET /api/v1/orders/{order_id}/risks
```

返回订单的欺诈风险评估。

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `ORDER_NOT_FOUND` | 404 | 订单不存在 |
| `ORDER_ALREADY_CANCELLED` | 409 | 订单已取消 |
| `ORDER_ALREADY_CLOSED` | 409 | 订单已关闭 |
| `INVALID_STATUS_TRANSITION` | 400 | 无法更改到请求的状态 |
| `LINE_ITEM_NOT_FOUND` | 404 | 订单项不存在 |
| `INVALID_QUANTITY` | 400 | 数量必须为正数 |
| `INSUFFICIENT_INVENTORY` | 400 | 库存不足 |
| `FULFILLMENT_NOT_FOUND` | 404 | 履行不存在 |
| `REFUND_EXCEEDS_TOTAL` | 400 | 退款金额过高 |
| `PAYMENT_REQUIRED` | 402 | 订单需要支付 |

## Webhooks

| 事件 | 说明 |
|-------|-------------|
| `order.created` | 新订单已下单 |
| `order.updated` | 订单信息已更改 |
| `order.cancelled` | 订单已取消 |
| `order.closed` | 订单已关闭 |
| `order.reopened` | 订单已重新打开 |
| `order.payment_received` | 付款已捕获 |
| `order.fulfillment_created` | 履行已创建 |
| `order.fulfillment_updated` | 履行已更新 |
| `order.fulfillment_cancelled` | 履行已取消 |
| `order.refund_created` | 退款已处理 |
| `order.note_created` | 备注已添加到订单 |
