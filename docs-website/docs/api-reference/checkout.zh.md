# 结账 API

结账 API 协调完整的结账流程，包括税费计算、运费计算和支付处理。

## 概述

结账流程遵循三个步骤：

1. **发起结账** - 计算税费并获取运费
2. **选择配送方式** - 选择配送方法
3. **完成结账** - 处理支付并创建订单

## 基础 URL

```
/api/v1/checkout
```

## 认证

所有结账端点都需要通过 JWT 令牌进行认证。

```http
Authorization: Bearer <jwt_token>
```

## 端点

### 发起结账

计算客户购物车的总计、税费和可用运费。

```http
POST /api/v1/checkout/initiate
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_address": {
    "first_name": "张",
    "last_name": "三",
    "address1": "朝阳区建国路88号",
    "city": "北京市",
    "state": "北京市",
    "country": "CN",
    "zip": "100022",
    "phone": "+86-138-0013-8000"
  },
  "billing_address": {
    "first_name": "张",
    "last_name": "三",
    "address1": "朝阳区建国路88号",
    "city": "北京市",
    "state": "北京市",
    "country": "CN",
    "zip": "100022",
    "phone": "+86-138-0013-8000"
  },
  "vat_id": null,
  "currency": "CNY"
}
```

**参数：**

| 字段 | 类型 | 必填 | 说明 |
|-------|------|----------|-------------|
| `cart_id` | UUID | 是 | 要结账的购物车 ID |
| `shipping_address` | 地址 | 是 | 配送地址对象 |
| `billing_address` | 地址 | 否 | 账单地址（默认使用配送地址） |
| `vat_id` | string | 否 | 增值税 ID，用于免税 |
| `currency` | string | 否 | 货币代码（例如："CNY"、"USD"） |

**地址对象：**

| 字段 | 类型 | 必填 | 说明 |
|-------|------|----------|-------------|
| `first_name` | string | 是 | 名字 |
| `last_name` | string | 是 | 姓氏 |
| `company` | string | 否 | 公司名称 |
| `address1` | string | 是 | 街道地址第一行 |
| `address2` | string | 否 | 街道地址第二行 |
| `city` | string | 是 | 城市名称 |
| `state` | string | 是 | 州或省 |
| `country` | string | 是 | 两位国家代码（ISO 3166-1 alpha-2） |
| `zip` | string | 是 | 邮政编码 |
| `phone` | string | 否 | 电话号码 |

**响应（200 OK）：**

```json
{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "product_id": "550e8400-e29b-41d4-a716-446655440002",
      "variant_id": null,
      "title": "优质 T 恤",
      "sku": "PROD-001",
      "quantity": 2,
      "unit_price": "75.00",
      "total": "150.00"
    }
  ],
  "subtotal": "150.00",
  "discount_total": "15.00",
  "shipping_total": "10.00",
  "shipping_tax": "0.90",
  "item_tax": "13.50",
  "tax_total": "14.40",
  "total": "159.40",
  "currency": "CNY",
  "available_shipping_rates": [
    {
      "provider_id": "sf_express",
      "carrier": "顺丰",
      "service_code": "standard",
      "service_name": "顺丰标准快递",
      "rate": "10.00",
      "currency": "CNY",
      "delivery_days": 3,
      "total_cost": "10.00"
    },
    {
      "provider_id": "sf_express",
      "carrier": "顺丰",
      "service_code": "express",
      "service_name": "顺丰特快",
      "rate": "25.00",
      "currency": "CNY",
      "delivery_days": 1,
      "total_cost": "25.00"
    }
  ],
  "selected_shipping_rate": null,
  "tax_breakdown": [
    {
      "tax_zone_name": "中国",
      "tax_rate_name": "增值税",
      "rate": "0.13",
      "taxable_amount": "150.00",
      "tax_amount": "19.50"
    }
  ],
  "vat_id_valid": null
}
```

### 选择配送方式

使用选定的配送方法更新结账信息并重新计算总计。

```http
POST /api/v1/checkout/shipping
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_rate": {
    "provider_id": "sf_express",
    "carrier": "顺丰",
    "service_code": "standard",
    "service_name": "顺丰标准快递",
    "rate": "10.00",
    "currency": "CNY",
    "delivery_days": 3,
    "total_cost": "10.00"
  }
}
```

**参数：**

| 字段 | 类型 | 必填 | 说明 |
|-------|------|----------|-------------|
| `cart_id` | UUID | 是 | 购物车 ID |
| `shipping_rate` | 配送费率 | 是 | 选定的配送费率 |

**配送费率对象：**

| 字段 | 类型 | 必填 | 说明 |
|-------|------|----------|-------------|
| `provider_id` | string | 是 | 配送提供商 ID |
| `carrier` | string | 是 | 承运商名称（例如："顺丰"、"中通"） |
| `service_code` | string | 是 | 服务代码标识符 |
| `service_name` | string | 是 | 人类可读的服务名称 |
| `rate` | decimal | 是 | 基础运费 |
| `currency` | string | 是 | 货币代码 |
| `delivery_days` | integer | 否 | 预计配送天数 |
| `total_cost` | decimal | 是 | 包含费用的总成本 |

**响应（200 OK）：**

返回应用了选定配送费率的更新结账摘要。

### 完成结账

完成结账、创建订单并处理支付。

```http
POST /api/v1/checkout/complete
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_address": {
    "first_name": "张",
    "last_name": "三",
    "address1": "朝阳区建国路88号",
    "city": "北京市",
    "state": "北京市",
    "country": "CN",
    "zip": "100022",
    "phone": "+86-138-0013-8000"
  },
  "billing_address": {
    "first_name": "张",
    "last_name": "三",
    "address1": "朝阳区建国路88号",
    "city": "北京市",
    "state": "北京市",
    "country": "CN",
    "zip": "100022",
    "phone": "+86-138-0013-8000"
  },
  "payment_method": {
    "type": "card",
    "token": "tok_visa"
  },
  "customer_email": "zhangsan@example.com",
  "vat_id": null,
  "notes": "请放在门口",
  "selected_shipping_rate": {
    "provider_id": "sf_express",
    "carrier": "顺丰",
    "service_code": "standard",
    "service_name": "顺丰标准快递",
    "rate": "10.00",
    "currency": "CNY",
    "delivery_days": 3,
    "total_cost": "10.00"
  }
}
```

**支付方式类型：**

| 类型 | 字段 | 说明 |
|------|--------|-------------|
| `card` | `token` | 信用卡/借记卡令牌 |
| `bank_transfer` | `account_number`, `routing_number` | 银行转账 |
| `digital_wallet` | `provider`, `token` | 数字钱包（支付宝、微信支付） |
| `buy_now_pay_later` | `provider` | 先买后付提供商（花呗、白条） |
| `cash_on_delivery` | 无 | 货到付款 |

**响应（201 Created）：**

```json
{
  "order": {
    "id": "550e8400-e29b-41d4-a716-446655440010",
    "order_number": "1001",
    "customer_id": "550e8400-e29b-41d4-a716-446655440005",
    "customer_email": "zhangsan@example.com",
    "status": "pending",
    "payment_status": "paid",
    "fulfillment_status": "unfulfilled",
    "currency": "CNY",
    "subtotal": "150.00",
    "tax_total": "14.40",
    "shipping_total": "10.00",
    "discount_total": "15.00",
    "total": "159.40",
    "items": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440011",
        "product_id": "550e8400-e29b-41d4-a716-446655440002",
        "variant_id": null,
        "title": "优质 T 恤",
        "sku": "PROD-001",
        "quantity": 2,
        "price": "75.00",
        "total": "150.00",
        "tax_amount": "12.00"
      }
    ],
    "created_at": "2026-02-21T06:30:00Z",
    "metadata": {}
  },
  "payment_id": "pay_1234567890",
  "total_charged": "159.40",
  "currency": "CNY"
}
```

## 完整结账流程示例

### 第 1 步：发起结账

```bash
curl -X POST http://localhost:8080/api/v1/checkout/initiate \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "550e8400-e29b-41d4-a716-446655440000",
    "shipping_address": {
      "first_name": "张",
      "last_name": "三",
      "address1": "朝阳区建国路88号",
      "city": "北京市",
      "state": "北京市",
      "country": "CN",
      "zip": "100022"
    },
    "customer_email": "zhangsan@example.com"
  }' | jq
```

### 第 2 步：选择配送方式

```bash
curl -X POST http://localhost:8080/api/v1/checkout/shipping \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "550e8400-e29b-41d4-a716-446655440000",
    "shipping_rate": {
      "provider_id": "sf_express",
      "carrier": "顺丰",
      "service_code": "standard",
      "service_name": "顺丰标准快递",
      "rate": "10.00",
      "currency": "CNY",
      "delivery_days": 3,
      "total_cost": "10.00"
    }
  }' | jq
```

### 第 3 步：完成结账

```bash
curl -X POST http://localhost:8080/api/v1/checkout/complete \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "cart_id": "550e8400-e29b-41d4-a716-446655440000",
    "shipping_address": {
      "first_name": "张",
      "last_name": "三",
      "address1": "朝阳区建国路88号",
      "city": "北京市",
      "state": "北京市",
      "country": "CN",
      "zip": "100022"
    },
    "payment_method": {
      "type": "card",
      "token": "tok_visa"
    },
    "customer_email": "zhangsan@example.com",
    "selected_shipping_rate": {
      "provider_id": "sf_express",
      "carrier": "顺丰",
      "service_code": "standard",
      "service_name": "顺丰标准快递",
      "rate": "10.00",
      "currency": "CNY",
      "delivery_days": 3,
      "total_cost": "10.00"
    }
  }' | jq
```

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `CART_NOT_FOUND` | 404 | 购物车 ID 不存在 |
| `CART_EMPTY` | 400 | 无法使用空购物车结账 |
| `INVALID_ADDRESS` | 400 | 地址验证失败 |
| `INVALID_PAYMENT_METHOD` | 400 | 不支持的支付方式 |
| `PAYMENT_FAILED` | 400 | 支付处理失败 |
| `SHIPPING_UNAVAILABLE` | 400 | 该地址无法配送 |
| `TAX_CALCULATION_ERROR` | 500 | 税费计算失败 |

## Webhooks

结账系统触发以下 webhook 事件：

| 事件 | 说明 |
|-------|-------------|
| `checkout.initiated` | 结账流程已开始 |
| `checkout.shipping_selected` | 已选择配送方式 |
| `checkout.completed` | 结账成功完成 |
| `checkout.failed` | 结账失败 |
| `order.created` | 从结账创建了新订单 |
| `payment.processed` | 支付成功处理 |
| `payment.failed` | 支付处理失败 |

## 相关主题

- [购物车 API](cart.zh.md) - 管理购物车
- [订单 API](orders.zh.md) - 订单管理
- [支付 API](payments.zh.md) - 支付处理
- [配送 API](shipping.zh.md) - 配送配置
