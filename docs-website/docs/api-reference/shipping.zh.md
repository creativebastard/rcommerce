# 配送 API

!!! warning "仅模块 - API 即将推出"
    配送模块已实现供内部使用。以下描述的公共 REST API 端点计划在 v0.2 中发布。目前，配送功能可通过核心库和订单管理系统使用。

配送 API 提供全面的配送管理功能，包括运费计算、发货单创建、物流跟踪和承运商管理。

## 基础 URL

```
/api/v1/shipping
```

## 认证

所有配送端点都需要通过 API 密钥或 JWT 令牌进行认证。

```http
Authorization: Bearer YOUR_API_KEY
```

## 运费对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440300",
  "carrier": "UPS",
  "service_code": "ground",
  "service_name": "UPS Ground",
  "description": "1-5 个工作日",
  "estimated_days": 3,
  "price": "12.50",
  "currency": "USD",
  "weight_limit": "68.0",
  "weight_unit": "kg",
  "dimensions_limit": {
    "length": 274,
    "width": 274,
    "height": 274,
    "unit": "cm"
  },
  "is_negotiated_rate": false,
  "delivery_date_guaranteed": false,
  "delivery_date": "2024-01-18T17:00:00Z"
}
```

### 运费字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 运费的唯一标识符 |
| `carrier` | string | 承运商名称（UPS、FedEx、USPS、DHL） |
| `service_code` | string | 承运商特定的服务代码 |
| `service_name` | string | 人类可读的服务名称 |
| `description` | string | 服务说明 |
| `estimated_days` | integer | 预计运输天数 |
| `price` | decimal | 运费 |
| `currency` | string | ISO 4217 货币代码 |
| `weight_limit` | decimal | 允许的最大重量 |
| `weight_unit` | string | 重量单位（kg、lb） |
| `dimensions_limit` | object | 允许的最大尺寸 |
| `is_negotiated_rate` | boolean | 是否为协商价格 |
| `delivery_date_guaranteed` | boolean | 是否保证交货日期 |
| `delivery_date` | datetime | 预计交货日期/时间 |

## 发货单对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440400",
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "carrier": "UPS",
  "service_code": "ground",
  "service_name": "UPS Ground",
  "tracking_number": "1Z999AA10123456784",
  "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
  "label_url": "https://api.rcommerce.app/labels/550e8400-e29b-41d4-a716-446655440400.pdf",
  "status": "in_transit",
  "price": "12.50",
  "currency": "USD",
  "weight": "2.5",
  "weight_unit": "kg",
  "dimensions": {
    "length": 30,
    "width": 20,
    "height": 15,
    "unit": "cm"
  },
  "ship_from": {
    "name": "R Commerce 仓库",
    "company": "R Commerce Inc",
    "phone": "+1-555-0199",
    "address1": "100 Commerce Street",
    "address2": "Suite 200",
    "city": "Austin",
    "state": "TX",
    "country": "US",
    "zip": "78701"
  },
  "ship_to": {
    "name": "John Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "packages": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440401",
      "tracking_number": "1Z999AA10123456784",
      "weight": "2.5",
      "weight_unit": "kg",
      "dimensions": {
        "length": 30,
        "width": 20,
        "height": 15,
        "unit": "cm"
      },
      "items": [
        {
          "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
          "quantity": 2
        }
      ]
    }
  ],
  "customs_info": {
    "contents_type": "merchandise",
    "customs_certify": true,
    "customs_signer": "John Smith",
    "eel_pfc": "NOEEI 30.37(a)",
    "non_delivery_option": "return",
    "restriction_type": "none",
    "customs_items": [
      {
        "description": "优质棉质 T 恤",
        "quantity": 2,
        "value": "49.98",
        "currency": "USD",
        "hs_tariff_number": "6109.10.00",
        "origin_country": "US"
      }
    ]
  },
  "insurance": {
    "amount": "100.00",
    "currency": "USD",
    "provider": "carrier"
  },
  "options": {
    "signature_required": true,
    "adult_signature_required": false,
    "saturday_delivery": false,
    "hold_for_pickup": false,
    "dry_ice": false,
    "dangerous_goods": false
  },
  "created_at": "2024-01-15T10:30:00Z",
  "shipped_at": "2024-01-15T14:00:00Z",
  "delivered_at": null,
  "estimated_delivery": "2024-01-18T17:00:00Z"
}
```

### 发货单字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 发货单唯一标识符 |
| `order_id` | UUID | 关联的订单 ID |
| `carrier` | string | 配送承运商名称 |
| `service_code` | string | 承运商服务代码 |
| `service_name` | string | 人类可读的服务名称 |
| `tracking_number` | string | 承运商跟踪号码 |
| `tracking_url` | string | 跟踪发货单的 URL |
| `label_url` | string | 配送标签的 URL |
| `status` | string | `pending`、`label_created`、`in_transit`、`out_for_delivery`、`delivered`、`exception`、`cancelled` |
| `price` | decimal | 收取的运费 |
| `currency` | string | ISO 4217 货币代码 |
| `weight` | decimal | 发货单总重量 |
| `weight_unit` | string | 重量单位 |
| `dimensions` | object | 包裹尺寸 |
| `ship_from` | object | 发货地址 |
| `ship_to` | object | 收货地址 |
| `packages` | array | 发货单中的单个包裹 |
| `customs_info` | object | 国际发货的海关信息 |
| `insurance` | object | 保险详情 |
| `options` | object | 配送选项 |
| `created_at` | datetime | 发货单创建时间 |
| `shipped_at` | datetime | 承运商收到包裹的时间 |
| `delivered_at` | datetime | 送达时间戳 |
| `estimated_delivery` | datetime | 预计送达日期 |

## 端点

### 获取运费

```http
GET /api/v1/shipping/rates
```

根据发货地、目的地和包裹详情计算运费。

#### 查询参数

| 参数 | 类型 | 必填 | 说明 |
|-----------|------|----------|-------------|
| `from_country` | string | 是 | 发货国家代码（ISO 3166-1 alpha-2） |
| `from_zip` | string | 是 | 发货地邮政编码 |
| `to_country` | string | 是 | 目的地国家代码 |
| `to_zip` | string | 是 | 目的地邮政编码 |
| `weight` | decimal | 是 | 包裹重量 |
| `weight_unit` | string | 否 | 重量单位：`kg`、`lb`、`oz`、`g`（默认：kg） |
| `length` | decimal | 否 | 包裹长度 |
| `width` | decimal | 否 | 包裹宽度 |
| `height` | decimal | 否 | 包裹高度 |
| `dimension_unit` | string | 否 | 尺寸单位：`cm`、`in`（默认：cm） |
| `carriers` | string | 否 | 逗号分隔的承运商列表（ups,fedex,usps,dhl） |
| `service_code` | string | 否 | 按特定服务代码筛选 |
| `currency` | string | 否 | 返回此货币的运费（默认：USD） |

#### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/rates?from_country=US&from_zip=78701&to_country=US&to_zip=10001&weight=2.5&weight_unit=kg&length=30&width=20&height=15&carriers=ups,fedex" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440300",
      "carrier": "UPS",
      "service_code": "ground",
      "service_name": "UPS Ground",
      "description": "1-5 个工作日",
      "estimated_days": 3,
      "price": "12.50",
      "currency": "USD",
      "delivery_date": "2024-01-18T17:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440301",
      "carrier": "UPS",
      "service_code": "3day_select",
      "service_name": "UPS 3 Day Select",
      "description": "3 个工作日",
      "estimated_days": 3,
      "price": "28.75",
      "currency": "USD",
      "delivery_date": "2024-01-18T17:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440302",
      "carrier": "FedEx",
      "service_code": "FEDEX_GROUND",
      "service_name": "FedEx Ground",
      "description": "1-5 个工作日",
      "estimated_days": 3,
      "price": "11.95",
      "currency": "USD",
      "delivery_date": "2024-01-18T20:00:00Z"
    }
  ],
  "meta": {
    "request_id": "req_ship_001",
    "from_address": {
      "country": "US",
      "zip": "78701"
    },
    "to_address": {
      "country": "US",
      "zip": "10001"
    },
    "weight": "2.5",
    "weight_unit": "kg"
  }
}
```

#### JavaScript 示例

```javascript
const response = await fetch(
  'https://api.rcommerce.app/api/v1/shipping/rates?' +
  new URLSearchParams({
    from_country: 'US',
    from_zip: '78701',
    to_country: 'US',
    to_zip: '10001',
    weight: '2.5',
    weight_unit: 'kg',
    carriers: 'ups,fedex'
  }),
  {
    headers: {
      'Authorization': 'Bearer sk_live_xxx'
    }
  }
);

const rates = await response.json();
console.log(rates.data);
```

### 创建发货单

```http
POST /api/v1/shipping/shipments
```

创建新发货单并生成配送标签。

#### 请求体

```json
{
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "carrier": "UPS",
  "service_code": "ground",
  "ship_from": {
    "name": "R Commerce 仓库",
    "company": "R Commerce Inc",
    "phone": "+1-555-0199",
    "email": "shipping@rcommerce.app",
    "address1": "100 Commerce Street",
    "address2": "Suite 200",
    "city": "Austin",
    "state": "TX",
    "country": "US",
    "zip": "78701",
    "residential": false
  },
  "ship_to": {
    "name": "John Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "email": "john@example.com",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001",
    "residential": true
  },
  "packages": [
    {
      "weight": "2.5",
      "weight_unit": "kg",
      "length": 30,
      "width": 20,
      "height": 15,
      "dimension_unit": "cm",
      "items": [
        {
          "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
          "quantity": 2
        }
      ]
    }
  ],
  "customs_info": {
    "contents_type": "merchandise",
    "customs_certify": true,
    "customs_signer": "John Smith",
    "non_delivery_option": "return",
    "customs_items": [
      {
        "description": "优质棉质 T 恤",
        "quantity": 2,
        "value": "49.98",
        "currency": "USD",
        "hs_tariff_number": "6109.10.00",
        "origin_country": "US"
      }
    ]
  },
  "insurance": {
    "amount": "100.00",
    "currency": "USD"
  },
  "options": {
    "signature_required": true,
    "saturday_delivery": false
  },
  "reference": "订单 #1001",
  "notify_customer": true
}
```

#### 必填字段

- `carrier` - 配送承运商名称
- `service_code` - 承运商服务代码
- `ship_from` - 发货地址（需要 name、address1、city、country、zip）
- `ship_to` - 收货地址（需要 name、address1、city、country、zip）
- `packages` - 至少一个包含重量的包裹

#### 示例响应

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440400",
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "carrier": "UPS",
    "service_code": "ground",
    "service_name": "UPS Ground",
    "tracking_number": "1Z999AA10123456784",
    "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
    "label_url": "https://api.rcommerce.app/labels/550e8400-e29b-41d4-a716-446655440400.pdf",
    "status": "label_created",
    "price": "12.50",
    "currency": "USD",
    "weight": "2.5",
    "weight_unit": "kg",
    "ship_from": {
      "name": "R Commerce 仓库",
      "city": "Austin",
      "state": "TX",
      "country": "US",
      "zip": "78701"
    },
    "ship_to": {
      "name": "John Doe",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "packages": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440401",
        "tracking_number": "1Z999AA10123456784",
        "weight": "2.5",
        "weight_unit": "kg"
      }
    ],
    "created_at": "2024-01-15T10:30:00Z",
    "estimated_delivery": "2024-01-18T17:00:00Z"
  }
}
```

#### JavaScript 示例

```javascript
const response = await fetch('https://api.rcommerce.app/api/v1/shipping/shipments', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer sk_live_xxx',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    order_id: '550e8400-e29b-41d4-a716-446655440100',
    carrier: 'UPS',
    service_code: 'ground',
    ship_from: {
      name: 'R Commerce 仓库',
      address1: '100 Commerce Street',
      city: 'Austin',
      state: 'TX',
      country: 'US',
      zip: '78701'
    },
    ship_to: {
      name: 'John Doe',
      address1: '123 Main St',
      city: 'New York',
      state: 'NY',
      country: 'US',
      zip: '10001'
    },
    packages: [{
      weight: '2.5',
      weight_unit: 'kg'
    }],
    notify_customer: true
  })
});

const shipment = await response.json();
console.log(shipment.data.tracking_number);
```

### 获取发货单

```http
GET /api/v1/shipping/shipments/{id}
```

通过 ID 检索发货单。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | UUID | 发货单 ID |

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `include` | string | 相关数据：`tracking_history`、`order` |

#### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/shipments/550e8400-e29b-41d4-a716-446655440400?include=tracking_history" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### 示例响应

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440400",
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "carrier": "UPS",
    "service_code": "ground",
    "service_name": "UPS Ground",
    "tracking_number": "1Z999AA10123456784",
    "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
    "label_url": "https://api.rcommerce.app/labels/550e8400-e29b-41d4-a716-446655440400.pdf",
    "status": "in_transit",
    "price": "12.50",
    "currency": "USD",
    "weight": "2.5",
    "ship_from": {
      "name": "R Commerce 仓库",
      "city": "Austin",
      "state": "TX",
      "country": "US",
      "zip": "78701"
    },
    "ship_to": {
      "name": "John Doe",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "tracking_history": [
      {
        "status": "picked_up",
        "description": "承运商已取件",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T14:00:00Z"
      },
      {
        "status": "in_transit",
        "description": "离开 Austin, TX 设施",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T18:30:00Z"
      },
      {
        "status": "in_transit",
        "description": "到达 Memphis, TN 设施",
        "location": "Memphis, TN",
        "timestamp": "2024-01-16T02:15:00Z"
      }
    ],
    "created_at": "2024-01-15T10:30:00Z",
    "shipped_at": "2024-01-15T14:00:00Z",
    "estimated_delivery": "2024-01-18T17:00:00Z"
  }
}
```

### 取消发货单

```http
POST /api/v1/shipping/shipments/{id}/cancel
```

取消发货单。只有状态为 `pending` 或 `label_created` 的发货单可以取消。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | UUID | 发货单 ID |

#### 请求体

```json
{
  "reason": "客户要求",
  "void_label": true
}
```

#### 示例请求

```bash
curl -X POST "https://api.rcommerce.app/api/v1/shipping/shipments/550e8400-e29b-41d4-a716-446655440400/cancel" \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "reason": "客户要求",
    "void_label": true
  }'
```

#### 示例响应

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440400",
    "status": "cancelled",
    "cancelled_at": "2024-01-15T11:00:00Z",
    "cancellation_reason": "客户要求",
    "refund_amount": "12.50",
    "refund_currency": "USD"
  }
}
```

### 跟踪发货单

```http
GET /api/v1/shipping/tracking/{number}
```

使用跟踪号码跟踪发货单。此端点可以在没有认证的情况下用于面向客户的跟踪页面。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `number` | string | 跟踪号码 |

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `carrier` | string | 承运商代码（可选，如未提供则自动检测） |

#### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/tracking/1Z999AA10123456784"
```

#### 示例响应

```json
{
  "data": {
    "tracking_number": "1Z999AA10123456784",
    "carrier": "UPS",
    "status": "in_transit",
    "status_description": "运输中",
    "estimated_delivery": "2024-01-18T17:00:00Z",
    "delivered_at": null,
    "signed_by": null,
    "tracking_history": [
      {
        "status": "picked_up",
        "status_code": "P",
        "description": "承运商已取件",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T14:00:00Z"
      },
      {
        "status": "in_transit",
        "status_code": "I",
        "description": "离开 Austin, TX 设施",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T18:30:00Z"
      },
      {
        "status": "in_transit",
        "status_code": "I",
        "description": "到达 Memphis, TN 设施",
        "location": "Memphis, TN",
        "timestamp": "2024-01-16T02:15:00Z"
      }
    ]
  }
}
```

#### JavaScript 示例

```javascript
// 公开跟踪 - 无需认证
const response = await fetch(
  'https://api.rcommerce.app/api/v1/shipping/tracking/1Z999AA10123456784'
);

const tracking = await response.json();
console.log(tracking.data.status);
console.log(tracking.data.estimated_delivery);
```

## 配送区域

### 列出配送区域

```http
GET /api/v1/shipping/zones
```

检索所有配置的配送区域。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20） |
| `active_only` | boolean | 仅返回活动区域 |

#### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/zones?active_only=true" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440500",
      "name": "国内",
      "countries": ["US"],
      "provinces": [],
      "postal_codes": [],
      "weight_based_rates": [
        {
          "name": "标准配送",
          "min_weight": 0,
          "max_weight": 1,
          "price": "5.00"
        },
        {
          "name": "标准配送",
          "min_weight": 1,
          "max_weight": 5,
          "price": "10.00"
        }
      ],
      "price_based_rates": [],
      "carrier_based_rates": [
        {
          "carrier": "UPS",
          "service_code": "ground",
          "markup_percent": 0,
          "markup_amount": "0.00"
        }
      ],
      "is_active": true,
      "display_order": 1,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440501",
      "name": "国际",
      "countries": ["CA", "GB", "AU", "DE", "FR"],
      "provinces": [],
      "postal_codes": [],
      "weight_based_rates": [
        {
          "name": "国际配送",
          "min_weight": 0,
          "max_weight": 2,
          "price": "25.00"
        }
      ],
      "price_based_rates": [],
      "carrier_based_rates": [],
      "is_active": true,
      "display_order": 2,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 2,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 1
    }
  }
}
```

### 获取配送区域

```http
GET /api/v1/shipping/zones/{id}
```

通过 ID 检索特定的配送区域。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | UUID | 配送区域 ID |

## 承运商

### 列出承运商

```http
GET /api/v1/shipping/carriers
```

检索所有可用的配送承运商及其配置的服务。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `active_only` | boolean | 仅返回活动的承运商 |
| `country` | string | 按支持的国家筛选 |

#### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/carriers?active_only=true" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440600",
      "name": "UPS",
      "code": "ups",
      "is_active": true,
      "supports_tracking": true,
      "supports_label_generation": true,
      "supports_rate_calculation": true,
      "supported_countries": ["US", "CA", "MX", "GB", "DE", "FR", "AU", "JP"],
      "services": [
        {
          "code": "ground",
          "name": "UPS Ground",
          "description": "1-5 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "3day_select",
          "name": "UPS 3 Day Select",
          "description": "3 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "2nd_day_air",
          "name": "UPS 2nd Day Air",
          "description": "2 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "next_day_air",
          "name": "UPS Next Day Air",
          "description": "下一个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "worldwide_expedited",
          "name": "UPS Worldwide Expedited",
          "description": "2-5 个工作日",
          "is_active": true,
          "domestic": false,
          "international": true
        }
      ],
      "credentials_configured": true,
      "test_mode": false
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440601",
      "name": "FedEx",
      "code": "fedex",
      "is_active": true,
      "supports_tracking": true,
      "supports_label_generation": true,
      "supports_rate_calculation": true,
      "supported_countries": ["US", "CA", "MX", "GB", "DE", "FR", "AU", "JP"],
      "services": [
        {
          "code": "FEDEX_GROUND",
          "name": "FedEx Ground",
          "description": "1-5 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "FEDEX_2_DAY",
          "name": "FedEx 2Day",
          "description": "2 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "FEDEX_EXPRESS_SAVER",
          "name": "FedEx Express Saver",
          "description": "3 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "FEDEX_PRIORITY_OVERNIGHT",
          "name": "FedEx Priority Overnight",
          "description": "下一个工作日上午",
          "is_active": true,
          "domestic": true,
          "international": false
        }
      ],
      "credentials_configured": true,
      "test_mode": false
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440602",
      "name": "USPS",
      "code": "usps",
      "is_active": true,
      "supports_tracking": true,
      "supports_label_generation": true,
      "supports_rate_calculation": true,
      "supported_countries": ["US"],
      "services": [
        {
          "code": "first_class_mail",
          "name": "First-Class Mail",
          "description": "1-3 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "priority_mail",
          "name": "Priority Mail",
          "description": "1-3 个工作日",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "priority_mail_express",
          "name": "Priority Mail Express",
          "description": "隔夜送达大部分地区",
          "is_active": true,
          "domestic": true,
          "international": false
        }
      ],
      "credentials_configured": true,
      "test_mode": false
    }
  ]
}
```

### 获取承运商

```http
GET /api/v1/shipping/carriers/{code}
```

检索特定承运商的详细信息。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `code` | string | 承运商代码（ups、fedex、usps、dhl） |

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `SHIPMENT_NOT_FOUND` | 404 | 发货单不存在 |
| `SHIPMENT_ALREADY_CANCELLED` | 409 | 发货单已取消 |
| `SHIPMENT_CANNOT_CANCEL` | 400 | 无法取消发货单（已发货） |
| `INVALID_CARRIER` | 400 | 承运商代码无效 |
| `INVALID_SERVICE_CODE` | 400 | 承运商服务代码无效 |
| `INVALID_ADDRESS` | 400 | 配送地址无效或不完整 |
| `ADDRESS_NOT_FOUND` | 404 | 无法验证地址 |
| `RATE_NOT_AVAILABLE` | 400 | 该路线无可用运费 |
| `WEIGHT_EXCEEDED` | 400 | 包裹重量超过承运商限制 |
| `DIMENSIONS_EXCEEDED` | 400 | 包裹尺寸超过承运商限制 |
| `CARRIER_ERROR` | 502 | 与承运商 API 通信错误 |
| `LABEL_GENERATION_FAILED` | 502 | 生成配送标签失败 |
| `TRACKING_NOT_AVAILABLE` | 404 | 未找到跟踪号码 |
| `INVALID_TRACKING_NUMBER` | 400 | 跟踪号码格式无效 |
| `CUSTOMS_INFO_REQUIRED` | 400 | 国际发货需要海关信息 |
| `INSURANCE_EXCEEDS_LIMIT` | 400 | 保险金额超过承运商限制 |
| `ZONE_NOT_FOUND` | 404 | 未找到配送区域 |
| `CARRIER_NOT_CONFIGURED` | 400 | 承运商凭据未配置 |

## Webhooks

| 事件 | 说明 |
|-------|-------------|
| `shipment.created` | 新发货单已创建 |
| `shipment.updated` | 发货单信息已更改 |
| `shipment.cancelled` | 发货单已取消 |
| `shipment.shipped` | 包裹已被承运商取件 |
| `shipment.in_transit` | 包裹运输中 |
| `shipment.out_for_delivery` | 包裹正在派送 |
| `shipment.delivered` | 包裹已送达 |
| `shipment.exception` | 发生配送异常 |
| `tracking.updated` | 跟踪信息已更新 |
| `rate.calculated` | 运费已计算 |
| `label.generated` | 配送标签已生成 |
| `label.voided` | 配送标签已作废 |

## 发货单状态

| 状态 | 说明 |
|--------|-------------|
| `pending` | 发货单已创建，标签尚未生成 |
| `label_created` | 配送标签已生成，等待取件 |
| `in_transit` | 包裹已取件并运输中 |
| `out_for_delivery` | 包裹正在派送 |
| `delivered` | 包裹已成功送达 |
| `exception` | 配送异常（地址问题、海关扣留等） |
| `cancelled` | 发货单已取消 |
| `returned` | 包裹已退回发件人 |

## 地址验证

API 在创建发货单时自动验证地址。您也可以单独验证地址：

```http
POST /api/v1/shipping/validate-address
```

### 请求体

```json
{
  "name": "John Doe",
  "address1": "123 Main St",
  "address2": "Apt 4B",
  "city": "New York",
  "state": "NY",
  "country": "US",
  "zip": "10001"
}
```

### 响应

```json
{
  "data": {
    "valid": true,
    "normalized": {
      "name": "John Doe",
      "address1": "123 Main Street",
      "address2": "Apt 4B",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001-1234",
      "residential": true
    },
    "suggestions": []
  }
}
```
