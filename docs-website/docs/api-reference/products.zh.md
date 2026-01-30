# 产品 API

产品 API 提供完整的产品目录管理，包括产品、变体、图片、系列和库存。

## 基础 URL

```
/api/v1/products
```

## 认证

所有产品端点都需要通过 API 密钥或 JWT 令牌进行认证。

```http
Authorization: Bearer YOUR_API_KEY
```

## 产品对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "优质棉质 T 恤",
  "slug": "premium-cotton-t-shirt",
  "description": "高品质 100% 有机棉 T 恤",
  "short_description": "有机棉 T 恤",
  "product_type": "physical",
  "vendor": "Organic Basics",
  "tags": ["clothing", "organic", "t-shirt"],
  "status": "active",
  "is_featured": true,
  "price": "29.99",
  "compare_at_price": "39.99",
  "cost_price": "12.00",
  "currency": "USD",
  "taxable": true,
  "requires_shipping": true,
  "weight": "0.5",
  "weight_unit": "kg",
  "inventory_quantity": 100,
  "inventory_policy": "deny",
  "inventory_management": "rcommerce",
  "low_stock_threshold": 10,
  "has_variants": true,
  "seo_title": "优质有机棉 T 恤",
  "seo_description": "选购我们的优质有机棉 T 恤",
  "meta_fields": {
    "material": "100% 有机棉",
    "care_instructions": "冷水机洗"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-20T14:30:00Z",
  "published_at": "2024-01-15T10:00:00Z"
}
```

### 产品字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 唯一标识符 |
| `title` | string | 产品名称（必填） |
| `slug` | string | URL 友好标识符（唯一） |
| `description` | string | 完整产品描述（支持 HTML） |
| `short_description` | string | 列表页的简短摘要 |
| `product_type` | string | `physical`、`digital`、`service` |
| `vendor` | string | 品牌或制造商名称 |
| `tags` | array | 可搜索标签 |
| `status` | string | `active`、`draft`、`archived` |
| `is_featured` | boolean | 在精选区域突出显示 |
| `price` | decimal | 当前售价 |
| `compare_at_price` | decimal | 原价（用于促销） |
| `cost_price` | decimal | 内部成本（计算利润率） |
| `currency` | string | ISO 4217 货币代码 |
| `taxable` | boolean | 需缴纳销售税 |
| `requires_shipping` | boolean | 需要实物配送 |
| `weight` | decimal | 产品重量 |
| `weight_unit` | string | `kg`、`g`、`lb`、`oz` |
| `inventory_quantity` | integer | 当前库存水平 |
| `inventory_policy` | string | `deny` 或 `continue`（缺货可下单） |
| `inventory_management` | string | `rcommerce` 或 null |
| `low_stock_threshold` | integer | 库存低于此值时发出警报 |
| `has_variants` | boolean | 有尺寸/颜色选项 |
| `seo_title` | string | 自定义标题标签 |
| `seo_description` | string | 元描述 |
| `meta_fields` | object | 自定义键值数据 |
| `created_at` | datetime | 创建时间戳 |
| `updated_at` | datetime | 最后修改时间 |
| `published_at` | datetime | 上线日期（null = 草稿） |

## 端点

### 列出产品

```http
GET /api/v1/products
```

检索分页的产品列表。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `status` | string | 按 `active`、`draft`、`archived` 筛选 |
| `product_type` | string | 按类型筛选 |
| `vendor` | string | 按供应商名称筛选 |
| `min_price` | decimal | 最低价格筛选 |
| `max_price` | decimal | 最高价格筛选 |
| `tags` | string | 逗号分隔的标签 |
| `ids` | string | 逗号分隔的产品 ID |
| `collection_id` | UUID | 按系列筛选 |
| `q` | string | 搜索查询（标题、描述、标签） |
| `sort` | string | `title`、`price`、`created_at`、`updated_at` |
| `order` | string | `asc` 或 `desc`（默认：desc） |
| `created_after` | datetime | 创建日期之后 |
| `created_before` | datetime | 创建日期之前 |
| `inventory_status` | string | `in_stock`、`low_stock`、`out_of_stock` |
| `is_featured` | boolean | 仅精选产品 |

#### 示例请求

```http
GET /api/v1/products?status=active&min_price=10.00&sort=price&order=desc&per_page=50
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "优质棉质 T 恤",
      "slug": "premium-cotton-t-shirt",
      "price": "29.99",
      "compare_at_price": "39.99",
      "currency": "USD",
      "inventory_quantity": 100,
      "status": "active",
      "is_featured": true,
      "created_at": "2024-01-15T10:00:00Z",
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

### 获取产品

```http
GET /api/v1/products/{id}
```

通过 ID 或 slug 检索单个产品。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | string | 产品 UUID 或 slug |

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `include` | string | 相关数据：`variants`、`images`、`collections` |

#### 示例请求

```http
GET /api/v1/products/premium-cotton-t-shirt?include=variants,images
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "优质棉质 T 恤",
    "slug": "premium-cotton-t-shirt",
    "description": "高品质 100% 有机棉 T 恤",
    "price": "29.99",
    "compare_at_price": "39.99",
    "currency": "USD",
    "inventory_quantity": 100,
    "status": "active",
    "variants": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440001",
        "title": "小号 / 黑色",
        "sku": "TSH-S-BLK",
        "price": "29.99",
        "inventory_quantity": 25,
        "options": {
          "size": "S",
          "color": "Black"
        }
      }
    ],
    "images": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440002",
        "url": "https://cdn.example.com/products/tshirt-main.jpg",
        "alt_text": "优质棉质 T 恤正面视图",
        "position": 1
      }
    ]
  }
}
```

### 创建产品

```http
POST /api/v1/products
```

创建新产品。

#### 请求体

```json
{
  "title": "优质棉质 T 恤",
  "slug": "premium-cotton-t-shirt",
  "description": "高品质 100% 有机棉 T 恤",
  "product_type": "physical",
  "vendor": "Organic Basics",
  "tags": ["clothing", "organic", "t-shirt"],
  "status": "active",
  "price": "29.99",
  "compare_at_price": "39.99",
  "currency": "USD",
  "inventory_quantity": 100,
  "inventory_policy": "deny",
  "requires_shipping": true,
  "weight": "0.5",
  "weight_unit": "kg",
  "seo_title": "优质有机棉 T 恤",
  "seo_description": "选购我们的优质有机棉 T 恤"
}
```

#### 必填字段

- `title` - 产品名称
- `price` - 售价
- `currency` - ISO 4217 代码

#### 示例响应

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "优质棉质 T 恤",
    "slug": "premium-cotton-t-shirt",
    "price": "29.99",
    "currency": "USD",
    "status": "active",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-15T10:00:00Z"
  }
}
```

### 更新产品

```http
PUT /api/v1/products/{id}
```

更新现有产品（完全替换）。

#### 请求体

与创建产品相同，所有字段必填。

### 部分更新产品

```http
PATCH /api/v1/products/{id}
```

部分更新产品字段。

#### 请求体

```json
{
  "price": "24.99",
  "compare_at_price": "29.99",
  "inventory_quantity": 150
}
```

### 删除产品

```http
DELETE /api/v1/products/{id}
```

删除产品。有订单的产品无法删除，只能归档。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `force` | boolean | 即使有订单也强制删除（谨慎使用） |

#### 响应

- `204 No Content` - 删除成功
- `409 Conflict` - 产品有相关订单

## 变体

产品变体代表不同的选项，如尺寸和颜色。

### 变体对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "小号 / 黑色",
  "sku": "TSH-S-BLK",
  "barcode": "123456789012",
  "price": "29.99",
  "compare_at_price": "39.99",
  "cost_price": "12.00",
  "inventory_quantity": 25,
  "inventory_policy": "deny",
  "weight": "0.5",
  "weight_unit": "kg",
  "requires_shipping": true,
  "is_default": false,
  "options": {
    "size": "S",
    "color": "Black"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### 列出变体

```http
GET /api/v1/products/{product_id}/variants
```

### 创建变体

```http
POST /api/v1/products/{product_id}/variants
```

#### 请求体

```json
{
  "title": "中号 / 白色",
  "sku": "TSH-M-WHT",
  "price": "29.99",
  "inventory_quantity": 30,
  "options": {
    "size": "M",
    "color": "White"
  }
}
```

### 更新变体

```http
PUT /api/v1/products/{product_id}/variants/{variant_id}
```

### 删除变体

```http
DELETE /api/v1/products/{product_id}/variants/{variant_id}
```

## 图片

### 图片对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440002",
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "url": "https://cdn.example.com/products/tshirt-main.jpg",
  "alt_text": "优质棉质 T 恤正面视图",
  "position": 1,
  "width": 1200,
  "height": 1200,
  "file_size": 245760,
  "file_type": "image/jpeg",
  "created_at": "2024-01-15T10:00:00Z"
}
```

### 列出图片

```http
GET /api/v1/products/{product_id}/images
```

### 上传图片

```http
POST /api/v1/products/{product_id}/images
Content-Type: multipart/form-data
```

#### 请求体

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `file` | file | 图片文件（JPG、PNG、WebP，最大 10MB） |
| `alt_text` | string | 无障碍访问的替代文本 |
| `position` | integer | 显示顺序 |

### 更新图片

```http
PUT /api/v1/products/{product_id}/images/{image_id}
```

### 删除图片

```http
DELETE /api/v1/products/{product_id}/images/{image_id}
```

## 系列

系列用于组织和展示产品分组。

### 系列对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "title": "2024 夏季系列",
  "slug": "summer-collection-2024",
  "description": "夏日热销款式",
  "collection_type": "manual",
  "rules": [],
  "is_active": true,
  "seo_title": "2024 夏季系列",
  "seo_description": "探索我们的夏季款式",
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### 列出系列

```http
GET /api/v1/collections
```

### 获取系列

```http
GET /api/v1/collections/{id}
```

### 创建系列

```http
POST /api/v1/collections
```

#### 请求体

```json
{
  "title": "2024 夏季系列",
  "slug": "summer-collection-2024",
  "description": "夏日热销款式",
  "collection_type": "manual",
  "is_active": true
}
```

### 添加产品到系列

```http
POST /api/v1/collections/{collection_id}/products
```

#### 请求体

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "position": 1
}
```

### 从系列中移除产品

```http
DELETE /api/v1/collections/{collection_id}/products/{product_id}
```

## 库存管理

### 调整库存

```http
POST /api/v1/products/{id}/inventory/adjust
```

#### 请求体

```json
{
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "adjustment": -5,
  "reason": "客户订单 #1001"
}
```

### 设置库存

```http
POST /api/v1/products/{id}/inventory/set
```

#### 请求体

```json
{
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "quantity": 100,
  "reason": "库存盘点"
}
```

### 转移库存

```http
POST /api/v1/products/{id}/inventory/transfer
```

#### 请求体

```json
{
  "from_location_id": "550e8400-e29b-41d4-a716-446655440020",
  "to_location_id": "550e8400-e29b-41d4-a716-446655440021",
  "quantity": 10,
  "reason": "零售店补货"
}
```

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `PRODUCT_NOT_FOUND` | 404 | 产品不存在 |
| `PRODUCT_SLUG_TAKEN` | 409 | Slug 已被使用 |
| `INVALID_PRODUCT_TYPE` | 400 | product_type 值无效 |
| `INVALID_CURRENCY` | 400 | 货币代码无效 |
| `INVALID_PRICE` | 400 | 价格无效或为负数 |
| `VARIANT_NOT_FOUND` | 404 | 变体不存在 |
| `VARIANT_SKU_TAKEN` | 409 | SKU 已被使用 |
| `COLLECTION_NOT_FOUND` | 404 | 系列不存在 |
| `IMAGE_TOO_LARGE` | 400 | 图片超过大小限制 |
| `INVALID_IMAGE_TYPE` | 400 | 不支持的图片格式 |
| `INVENTORY_NEGATIVE` | 400 | 无法设置负库存 |
| `PRODUCT_HAS_ORDERS` | 409 | 无法删除有订单的产品 |

## Webhooks

产品 API 触发以下 webhook 事件：

| 事件 | 说明 |
|-------|-------------|
| `product.created` | 新产品已创建 |
| `product.updated` | 产品信息已更改 |
| `product.deleted` | 产品已删除 |
| `product.published` | 产品状态更改为活跃 |
| `product.unpublished` | 产品状态更改为草稿 |
| `product.inventory_changed` | 库存数量已更新 |
| `product.low_stock` | 库存低于阈值 |
| `product.out_of_stock` | 库存已归零 |
| `variant.created` | 新变体已添加 |
| `variant.updated` | 变体信息已更改 |
| `variant.deleted` | 变体已删除 |
| `collection.created` | 新系列已创建 |
| `collection.updated` | 系列信息已更改 |
| `collection.product_added` | 产品已添加到系列 |
| `collection.product_removed` | 产品已从系列中移除 |
