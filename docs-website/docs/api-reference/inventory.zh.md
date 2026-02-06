# 库存 API

库存 API 提供全面的库存管理，包括库存水平、库存预留、库存移动和低库存警报。它支持多地点库存跟踪和订单的自动库存预留。

## 基础 URL

```
/api/v1/inventory
```

## 认证

所有库存端点都需要通过 API 密钥或 JWT 令牌进行认证。

```http
Authorization: Bearer YOUR_API_KEY
```

## 库存对象

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "total_available": 150,
  "total_reserved": 25,
  "total_incoming": 50,
  "low_stock_threshold": 20,
  "locations": [
    {
      "location_id": "550e8400-e29b-41d4-a716-446655440020",
      "location_name": "仓库 A",
      "available": 100,
      "reserved": 15,
      "incoming": 30
    },
    {
      "location_id": "550e8400-e29b-41d4-a716-446655440021",
      "location_name": "仓库 B",
      "available": 50,
      "reserved": 10,
      "incoming": 20
    }
  ]
}
```

### 库存字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `product_id` | UUID | 产品唯一标识符 |
| `total_available` | integer | 所有地点的可用库存总量 |
| `total_reserved` | integer | 待处理订单的预留库存总量 |
| `total_incoming` | integer | 在途库存总量 |
| `low_stock_threshold` | integer | 低库存警报阈值 |
| `locations` | array | 按地点的库存明细 |

### 地点库存字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `location_id` | UUID | 地点唯一标识符 |
| `location_name` | string | 地点名称（人类可读） |
| `available` | integer | 该地点的可用数量 |
| `reserved` | integer | 该地点的预留数量 |
| `incoming` | integer | 该地点的入库数量 |

## 端点

### 列出库存水平

```http
GET /api/v1/inventory
```

检索所有产品的分页库存水平列表。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `product_id` | UUID | 按特定产品筛选 |
| `location_id` | UUID | 按特定地点筛选 |
| `stock_status` | string | 按 `in_stock`、`low_stock`、`out_of_stock` 筛选 |
| `low_stock_only` | boolean | 仅显示低库存项目 |
| `include_incoming` | boolean | 在计算中包含入库库存 |
| `sort` | string | `product_id`、`available`、`updated_at` |
| `order` | string | `asc` 或 `desc`（默认：desc） |

#### 示例请求

```http
GET /api/v1/inventory?stock_status=low_stock&per_page=50
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": [
    {
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "优质棉质 T 恤",
      "total_available": 150,
      "total_reserved": 25,
      "total_incoming": 50,
      "low_stock_threshold": 20,
      "stock_status": "in_stock",
      "locations": [
        {
          "location_id": "550e8400-e29b-41d4-a716-446655440020",
          "location_name": "仓库 A",
          "available": 100,
          "reserved": 15,
          "incoming": 30
        }
      ],
      "updated_at": "2024-01-20T14:30:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 156,
      "per_page": 50,
      "current_page": 1,
      "total_pages": 4,
      "has_next": true,
      "has_prev": false
    },
    "request_id": "req_abc123"
  }
}
```

### 获取产品库存

```http
GET /api/v1/inventory/{product_id}
```

检索特定产品的详细库存信息。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `product_id` | UUID | 产品唯一标识符 |

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `include_history` | boolean | 包含最近的库存移动 |
| `include_reservations` | boolean | 包含活跃的预留 |

#### 示例请求

```http
GET /api/v1/inventory/550e8400-e29b-41d4-a716-446655440000?include_history=true
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": {
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "product_name": "优质棉质 T 恤",
    "total_available": 150,
    "total_reserved": 25,
    "total_incoming": 50,
    "low_stock_threshold": 20,
    "stock_status": "in_stock",
    "locations": [
      {
        "location_id": "550e8400-e29b-41d4-a716-446655440020",
        "location_name": "仓库 A",
        "available": 100,
        "reserved": 15,
        "incoming": 30
      },
      {
        "location_id": "550e8400-e29b-41d4-a716-446655440021",
        "location_name": "仓库 B",
        "available": 50,
        "reserved": 10,
        "incoming": 20
      }
    ],
    "recent_movements": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440030",
        "movement_type": "in",
        "quantity": 50,
        "location_name": "仓库 A",
        "reference": "PO-2024-001",
        "created_at": "2024-01-19T10:00:00Z"
      },
      {
        "id": "550e8400-e29b-41d4-a716-446655440031",
        "movement_type": "out",
        "quantity": -10,
        "location_name": "仓库 A",
        "reference": "订单 #1001",
        "created_at": "2024-01-18T15:30:00Z"
      }
    ],
    "active_reservations": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440040",
        "order_id": "550e8400-e29b-41d4-a716-446655440100",
        "quantity": 5,
        "location_name": "仓库 A",
        "expires_at": "2024-01-20T15:00:00Z"
      }
    ],
    "updated_at": "2024-01-20T14:30:00Z"
  }
}
```

### 调整库存

```http
POST /api/v1/inventory/adjust
```

手动调整库存水平并记录审计跟踪。

#### 请求体

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "adjustment": -5,
  "reason": "damaged",
  "notes": "存储期间水损",
  "reference": "ADJ-2024-001"
}
```

#### 调整原因

| 原因 | 说明 |
|--------|-------------|
| `damaged` | 产品损坏或有缺陷 |
| `lost` | 库存丢失或缺失 |
| `found` | 发现额外库存 |
| `count` | 库存盘点更正 |
| `return` | 客户退货 |
| `other` | 其他原因（在备注中说明） |

#### 必填字段

- `product_id` - 要调整的产品
- `location_id` - 要调整的地点
- `adjustment` - 要添加（正数）或移除（负数）的数量
- `reason` - 调整原因

#### 示例响应

```json
{
  "data": {
    "movement_id": "550e8400-e29b-41d4-a716-446655440050",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "adjustment": -5,
    "quantity_before": 100,
    "quantity_after": 95,
    "reason": "damaged",
    "notes": "存储期间水损",
    "reference": "ADJ-2024-001",
    "adjusted_by": "550e8400-e29b-41d4-a716-446655440200",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### 设置库存

```http
POST /api/v1/inventory/set
```

设置绝对库存数量（适用于库存盘点）。

#### 请求体

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "quantity": 95,
  "reason": "count",
  "notes": "月度库存盘点",
  "reference": "COUNT-2024-01"
}
```

#### 示例响应

```json
{
  "data": {
    "movement_id": "550e8400-e29b-41d4-a716-446655440051",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "quantity_before": 100,
    "quantity_after": 95,
    "adjustment": -5,
    "reason": "count",
    "notes": "月度库存盘点",
    "reference": "COUNT-2024-01",
    "adjusted_by": "550e8400-e29b-41d4-a716-446655440200",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### 接收库存

```http
POST /api/v1/inventory/receive
```

接收入库库存（入库库存）。

#### 请求体

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "quantity": 50,
  "cost_per_unit": "12.50",
  "reference": "PO-2024-001",
  "notes": "从供应商处接收"
}
```

#### 示例响应

```json
{
  "data": {
    "movement_id": "550e8400-e29b-41d4-a716-446655440052",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "movement_type": "in",
    "quantity": 50,
    "quantity_before": 100,
    "quantity_after": 150,
    "cost_per_unit": "12.50",
    "reference": "PO-2024-001",
    "notes": "从供应商处接收",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### 预留库存

```http
POST /api/v1/inventory/reserve
```

为订单预留库存。预留会在配置的超时时间后自动过期（默认：30 分钟）。

#### 请求体

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "quantity": 5
}
```

#### 必填字段

- `product_id` - 要预留的产品
- `location_id` - 要预留的地点
- `order_id` - 要预留的订单
- `quantity` - 要预留的数量

#### 示例响应

```json
{
  "data": {
    "reservation_id": "550e8400-e29b-41d4-a716-446655440060",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "variant_id": "550e8400-e29b-41d4-a716-446655440001",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "quantity": 5,
    "status": "active",
    "expires_at": "2024-01-20T15:00:00Z",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### 释放预留

```http
POST /api/v1/inventory/release
```

释放库存预留（例如，订单取消时）。

#### 请求体

```json
{
  "reservation_id": "550e8400-e29b-41d4-a716-446655440060",
  "reason": "order_cancelled"
}
```

#### 释放原因

| 原因 | 说明 |
|--------|-------------|
| `order_cancelled` | 订单已取消 |
| `order_expired` | 预留已过期 |
| `manual_release` | 管理员手动释放 |
| `order_completed` | 订单已完成（预留已转换） |

#### 示例响应

```json
{
  "data": {
    "reservation_id": "550e8400-e29b-41d4-a716-446655440060",
    "status": "released",
    "released_at": "2024-01-20T14:35:00Z",
    "reason": "order_cancelled"
  }
}
```

### 转移库存

```http
POST /api/v1/inventory/transfer
```

在地点之间转移库存。

#### 请求体

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "from_location_id": "550e8400-e29b-41d4-a716-446655440020",
  "to_location_id": "550e8400-e29b-41d4-a716-446655440021",
  "quantity": 10,
  "reference": "TF-2024-001",
  "notes": "零售店补货"
}
```

#### 示例响应

```json
{
  "data": {
    "transfer_id": "550e8400-e29b-41d4-a716-446655440070",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "from_location_id": "550e8400-e29b-41d4-a716-446655440020",
    "to_location_id": "550e8400-e29b-41d4-a716-446655440021",
    "quantity": 10,
    "status": "completed",
    "reference": "TF-2024-001",
    "notes": "零售店补货",
    "created_at": "2024-01-20T14:30:00Z",
    "completed_at": "2024-01-20T14:30:00Z"
  }
}
```

## 库存移动

### 列出库存移动

```http
GET /api/v1/inventory/movements
```

检索分页的库存移动列表。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `product_id` | UUID | 按产品筛选 |
| `location_id` | UUID | 按地点筛选 |
| `movement_type` | string | 按 `in`、`out`、`return`、`lost`、`found`、`transfer` 筛选 |
| `start_date` | datetime | 日期之后的移动 |
| `end_date` | datetime | 日期之前的移动 |
| `reference` | string | 按参考号筛选 |
| `sort` | string | `created_at`（默认） |
| `order` | string | `asc` 或 `desc`（默认：desc） |

#### 示例请求

```http
GET /api/v1/inventory/movements?product_id=550e8400-e29b-41d4-a716-446655440000&movement_type=in
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440030",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "优质棉质 T 恤",
      "variant_id": null,
      "location_id": "550e8400-e29b-41d4-a716-446655440020",
      "location_name": "仓库 A",
      "quantity": 50,
      "movement_type": "in",
      "cost_per_unit": "12.50",
      "reference": "PO-2024-001",
      "notes": "从供应商处接收",
      "created_at": "2024-01-19T10:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440031",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "优质棉质 T 恤",
      "variant_id": null,
      "location_id": "550e8400-e29b-41d4-a716-446655440020",
      "location_name": "仓库 A",
      "quantity": -10,
      "movement_type": "out",
      "cost_per_unit": null,
      "reference": "订单 #1001",
      "notes": "客户订单履行",
      "created_at": "2024-01-18T15:30:00Z"
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

### 移动类型

| 类型 | 说明 |
|------|-------------|
| `in` | 库存接收 |
| `out` | 库存售出或移除 |
| `return` | 客户退货 |
| `lost` | 库存丢失或损坏 |
| `found` | 发现额外库存 |
| `transfer` | 地点之间的库存转移 |

## 低库存警报

### 列出低库存警报

```http
GET /api/v1/inventory/alerts
```

检索低于阈值产品的低库存警报。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `alert_level` | string | 按 `low` 或 `critical` 筛选 |
| `product_id` | UUID | 按特定产品筛选 |
| `location_id` | UUID | 按特定地点筛选 |
| `acknowledged` | boolean | 按确认状态筛选 |
| `sort` | string | `created_at`、`alert_level` |
| `order` | string | `asc` 或 `desc`（默认：desc） |

#### 示例请求

```http
GET /api/v1/inventory/alerts?alert_level=critical
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": [
    {
      "alert_id": "550e8400-e29b-41d4-a716-446655440080",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "优质棉质 T 恤",
      "current_stock": 5,
      "threshold": 20,
      "alert_level": "critical",
      "recommended_reorder_quantity": 95,
      "locations_affected": [
        {
          "location_id": "550e8400-e29b-41d4-a716-446655440020",
          "location_name": "仓库 A",
          "current_stock": 3,
          "alert_level": "critical"
        },
        {
          "location_id": "550e8400-e29b-41d4-a716-446655440021",
          "location_name": "仓库 B",
          "current_stock": 2,
          "alert_level": "critical"
        }
      ],
      "acknowledged": false,
      "created_at": "2024-01-20T14:30:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 12,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  }
}
```

### 确认警报

```http
POST /api/v1/inventory/alerts/{alert_id}/acknowledge
```

确认低库存警报。

#### 请求体

```json
{
  "notes": "已向供应商下单"
}
```

#### 示例响应

```json
{
  "data": {
    "alert_id": "550e8400-e29b-41d4-a716-446655440080",
    "acknowledged": true,
    "acknowledged_by": "550e8400-e29b-41d4-a716-446655440200",
    "acknowledged_at": "2024-01-20T15:00:00Z",
    "notes": "已向供应商下单"
  }
}
```

### 警报级别

| 级别 | 说明 |
|-------|-------------|
| `low` | 库存低于阈值但高于临界水平（阈值的 50%） |
| `critical` | 库存低于临界水平（阈值的 50%） |

## 库存地点

### 列出地点

```http
GET /api/v1/inventory/locations
```

检索所有库存地点。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `is_active` | boolean | 按活跃状态筛选 |

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440020",
      "name": "仓库 A",
      "code": "WH-A",
      "address": "123 Industrial Blvd, Warehouse District",
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440021",
      "name": "仓库 B",
      "code": "WH-B",
      "address": "456 Commerce St, Business Park",
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 2,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  }
}
```

### 获取地点

```http
GET /api/v1/inventory/locations/{location_id}
```

检索特定地点的详细信息。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `include_inventory` | boolean | 包含库存摘要 |

#### 示例响应

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440020",
    "name": "仓库 A",
    "code": "WH-A",
    "address": "123 Industrial Blvd, Warehouse District",
    "is_active": true,
    "inventory_summary": {
      "total_products": 156,
      "total_value": "45000.00",
      "low_stock_count": 12,
      "out_of_stock_count": 3
    },
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `PRODUCT_NOT_FOUND` | 404 | 产品不存在 |
| `LOCATION_NOT_FOUND` | 404 | 库存地点不存在 |
| `INSUFFICIENT_STOCK` | 400 | 操作可用库存不足 |
| `INVALID_ADJUSTMENT` | 400 | 调整将导致负库存 |
| `RESERVATION_NOT_FOUND` | 404 | 库存预留不存在 |
| `RESERVATION_EXPIRED` | 409 | 预留已过期 |
| `INVALID_MOVEMENT_TYPE` | 400 | 指定的移动类型无效 |
| `INVALID_TRANSFER` | 400 | 无法转移到同一地点 |
| `ALERT_NOT_FOUND` | 404 | 低库存警报不存在 |
| `ALERT_ALREADY_ACKNOWLEDGED` | 409 | 警报已确认 |

## Webhooks

库存 API 触发以下 webhook 事件：

| 事件 | 说明 |
|-------|-------------|
| `inventory.adjusted` | 库存已手动调整 |
| `inventory.received` | 库存已接收 |
| `inventory.transferred` | 库存已在地点之间转移 |
| `inventory.reserved` | 库存已为订单预留 |
| `inventory.reservation_released` | 预留已释放 |
| `inventory.reservation_expired` | 预留已过期 |
| `inventory.low_stock` | 库存低于阈值 |
| `inventory.critical_stock` | 库存低于临界水平 |
| `inventory.out_of_stock` | 库存已归零 |
| `inventory.movement.created` | 库存移动已记录 |

## 配置

### 库存设置

在您的应用程序设置中配置库存行为：

```toml
[inventory]
low_stock_threshold = 20              # 默认低库存阈值 (%)
enable_restock_alerts = true          # 启用自动补货警报
enable_reservations = true            # 启用库存预留
reservation_timeout_minutes = 30      # 预留超时时间
```

### 预留行为

- 预留以 `active` 状态创建
- 预留会在配置的超时时间后自动过期
- 过期的预留可以通过 API 清理或自动清理
- 订单完成时，预留会转换为实际库存减少
- 订单取消时，应释放预留
