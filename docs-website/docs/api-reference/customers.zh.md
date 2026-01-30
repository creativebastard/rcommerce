# 客户 API

客户 API 管理客户账户、资料、地址和订单历史。

## 基础 URL

```
/api/v1/customers
```

## 认证

客户端点需要认证。客户只能访问自己的数据，除非使用管理员 API 密钥。

```http
Authorization: Bearer YOUR_API_KEY
```

## 客户对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "email": "customer@example.com",
  "phone": "+1-555-0123",
  "first_name": "John",
  "last_name": "Doe",
  "accepts_marketing": true,
  "marketing_opt_in_at": "2024-01-15T10:00:00Z",
  "tax_exempt": false,
  "tax_exemptions": [],
  "currency": "USD",
  "language": "en",
  "note": "VIP 客户",
  "tags": ["vip", "repeat_customer"],
  "verified_email": true,
  "state": "enabled",
  "last_order_id": "550e8400-e29b-41d4-a716-446655440100",
  "last_order_name": "1001",
  "orders_count": 5,
  "total_spent": "299.95",
  "average_order_value": "59.99",
  "default_address": {
    "id": "550e8400-e29b-41d4-a716-446655440002",
    "first_name": "John",
    "last_name": "Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001",
    "is_default_shipping": true,
    "is_default_billing": true
  },
  "addresses": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "first_name": "John",
      "last_name": "Doe",
      "company": "Acme Inc",
      "phone": "+1-555-0123",
      "address1": "123 Main St",
      "address2": "Apt 4B",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001",
      "is_default_shipping": true,
      "is_default_billing": true,
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:00:00Z"
    }
  ],
  "metafields": {
    "birthday": "1990-05-15",
    "loyalty_tier": "gold"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-20T14:30:00Z"
}
```

### 客户字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 唯一标识符 |
| `email` | string | 主要邮箱地址（唯一） |
| `phone` | string | 电话号码 |
| `first_name` | string | 名字 |
| `last_name` | string | 姓氏 |
| `accepts_marketing` | boolean | 订阅营销邮件 |
| `marketing_opt_in_at` | datetime | 客户选择加入的时间 |
| `tax_exempt` | boolean | 免税 |
| `tax_exemptions` | array | 特定免税 |
| `currency` | string | 首选货币 |
| `language` | string | 首选语言（ISO 639-1） |
| `note` | string | 内部员工备注 |
| `tags` | array | 可搜索标签 |
| `verified_email` | boolean | 邮箱验证状态 |
| `state` | string | `enabled`、`disabled`、`invited`、`declined` |
| `last_order_id` | UUID | 最近订单 |
| `last_order_name` | string | 最近订单的订单号 |
| `orders_count` | integer | 订单总数 |
| `total_spent` | decimal | 终身消费总额 |
| `average_order_value` | decimal | 平均订单价值 |
| `default_address` | object | 主要地址 |
| `addresses` | array | 所有保存的地址 |
| `metafields` | object | 自定义键值数据 |
| `created_at` | datetime | 账户创建时间 |
| `updated_at` | datetime | 最后修改时间 |

## 端点

### 列出客户

```http
GET /api/v1/customers
```

检索分页的客户列表。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `email` | string | 按邮箱地址筛选 |
| `phone` | string | 按电话号码筛选 |
| `state` | string | 按账户状态筛选 |
| `accepts_marketing` | boolean | 按营销同意筛选 |
| `tags` | string | 逗号分隔的标签 |
| `min_orders` | integer | 最低订单数 |
| `min_total_spent` | decimal | 最低终身消费 |
| `created_after` | datetime | 创建日期之后 |
| `created_before` | datetime | 创建日期之前 |
| `updated_after` | datetime | 更新日期之后 |
| `q` | string | 搜索查询（姓名、邮箱、电话） |
| `sort` | string | `created_at`、`updated_at`、`total_spent`、`orders_count` |
| `order` | string | `asc` 或 `desc`（默认：desc） |

#### 示例请求

```http
GET /api/v1/customers?accepts_marketing=true&min_orders=3&sort=total_spent
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "email": "customer@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "orders_count": 5,
      "total_spent": "299.95",
      "accepts_marketing": true,
      "state": "enabled",
      "created_at": "2024-01-15T10:00:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 1250,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 63
    }
  }
}
```

### 获取客户

```http
GET /api/v1/customers/{id}
```

通过 ID 或邮箱检索单个客户。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | string | 客户 UUID 或邮箱地址 |

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `include` | string | 相关数据：`addresses`、`orders`、`metafields` |

#### 示例请求

```http
GET /api/v1/customers/customer@example.com?include=addresses,orders
Authorization: Bearer sk_live_xxx
```

### 创建客户

```http
POST /api/v1/customers
```

创建新客户账户。

#### 请求体

```json
{
  "email": "newcustomer@example.com",
  "phone": "+1-555-0199",
  "first_name": "Jane",
  "last_name": "Smith",
  "accepts_marketing": true,
  "password": "secure_password_123",
  "password_confirmation": "secure_password_123",
  "addresses": [
    {
      "first_name": "Jane",
      "last_name": "Smith",
      "address1": "456 Oak Ave",
      "city": "Los Angeles",
      "state": "CA",
      "country": "US",
      "zip": "90210",
      "phone": "+1-555-0199",
      "is_default_shipping": true,
      "is_default_billing": true
    }
  ],
  "tags": ["newsletter_subscriber"],
  "note": "通过 Google Ads 找到",
  "send_email_invite": true
}
```

#### 必填字段

- `email` - 有效邮箱地址（唯一）

#### 可选字段

- `password` - 如未提供，将邀请客户设置密码
- `send_email_invite` - 发送欢迎邮件（默认：true）

### 更新客户

```http
PUT /api/v1/customers/{id}
```

更新客户信息。

#### 请求体

```json
{
  "first_name": "Jane",
  "last_name": "Smith-Johnson",
  "phone": "+1-555-0200",
  "accepts_marketing": false,
  "note": "已更新联系信息",
  "tags": ["vip", "repeat_customer"]
}
```

### 删除客户

```http
DELETE /api/v1/customers/{id}
```

删除客户账户（GDPR 删除权）。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `anonymize_orders` | boolean | 保留订单但匿名化客户数据 |

### 发送邀请

```http
POST /api/v1/customers/{id}/send_invite
```

向已邀请的客户发送账户激活邮件。

### 账户激活

```http
POST /api/v1/customers/{id}/activate
```

激活客户账户。

#### 请求体

```json
{
  "activation_token": "token_from_email",
  "password": "new_secure_password"
}
```

## 地址

### 列出地址

```http
GET /api/v1/customers/{customer_id}/addresses
```

### 获取地址

```http
GET /api/v1/customers/{customer_id}/addresses/{address_id}
```

### 创建地址

```http
POST /api/v1/customers/{customer_id}/addresses
```

#### 请求体

```json
{
  "first_name": "Jane",
  "last_name": "Smith",
  "company": "Acme Inc",
  "phone": "+1-555-0199",
  "address1": "789 Pine Street",
  "address2": "Suite 100",
  "city": "San Francisco",
  "state": "CA",
  "country": "US",
  "zip": "94102",
  "is_default_shipping": false,
  "is_default_billing": true
}
```

### 更新地址

```http
PUT /api/v1/customers/{customer_id}/addresses/{address_id}
```

### 删除地址

```http
DELETE /api/v1/customers/{customer_id}/addresses/{address_id}
```

### 设置默认地址

```http
POST /api/v1/customers/{customer_id}/addresses/{address_id}/default
```

#### 请求体

```json
{
  "type": "shipping"
}
```

## 客户订单

### 列出客户订单

```http
GET /api/v1/customers/{customer_id}/orders
```

检索特定客户的订单历史。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码 |
| `per_page` | integer | 每页项目数 |
| `status` | string | 按订单状态筛选 |
| `financial_status` | string | 按支付状态筛选 |

## 客户搜索

### 搜索客户

```http
POST /api/v1/customers/search
```

高级客户搜索与筛选。

#### 请求体

```json
{
  "query": "john",
  "filters": {
    "accepts_marketing": true,
    "min_orders": 2,
    "tags": ["vip"]
  },
  "sort": {
    "field": "total_spent",
    "order": "desc"
  }
}
```

## 客户细分

### 列出细分

```http
GET /api/v1/customers/segments
```

### 创建细分

```http
POST /api/v1/customers/segments
```

#### 请求体

```json
{
  "name": "VIP 客户",
  "query": {
    "total_spent": {
      "gte": 500
    },
    "orders_count": {
      "gte": 5
    }
  }
}
```

### 获取细分客户

```http
GET /api/v1/customers/segments/{segment_id}/customers
```

## GDPR 合规

### 导出客户数据

```http
POST /api/v1/customers/{id}/export
```

导出所有客户数据（GDPR 数据可携带权）。

### 匿名化客户

```http
POST /api/v1/customers/{id}/anonymize
```

匿名化客户数据，同时保留订单历史。

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `CUSTOMER_NOT_FOUND` | 404 | 客户不存在 |
| `EMAIL_TAKEN` | 409 | 邮箱地址已被使用 |
| `INVALID_EMAIL` | 400 | 邮箱格式无效 |
| `INVALID_PASSWORD` | 400 | 密码不符合要求 |
| `ADDRESS_NOT_FOUND` | 404 | 地址不存在 |
| `INVALID_ADDRESS` | 400 | 地址验证失败 |
| `CUSTOMER_HAS_ORDERS` | 409 | 无法删除有订单的客户 |
| `INVALID_STATE_TRANSITION` | 400 | 无法更改到请求的状态 |
| `ACTIVATION_TOKEN_INVALID` | 400 | 激活令牌无效或已过期 |

## Webhooks

| 事件 | 说明 |
|-------|-------------|
| `customer.created` | 新客户账户已创建 |
| `customer.updated` | 客户信息已更改 |
| `customer.deleted` | 客户账户已删除 |
| `customer.enabled` | 客户账户已启用 |
| `customer.disabled` | 客户账户已禁用 |
| `customer.address_created` | 新地址已添加 |
| `customer.address_updated` | 地址信息已更改 |
| `customer.address_deleted` | 地址已删除 |
| `customer.password_reset` | 已请求密码重置 |
