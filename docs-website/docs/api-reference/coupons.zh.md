# 优惠券 API

优惠券 API 提供全面的折扣管理，包括百分比、固定金额、免费配送和买 X 送 Y 促销。

## 概览

- **优惠券类型**: 百分比、固定金额、免费配送、买 X 送 Y
- **使用限制**: 每张优惠券和每位客户的限制
- **限制条件**: 最低订单金额、产品/类别排除
- **基于时间**: 有效期和过期日期

## 基础 URL

```
/api/v1/coupons
```

## 认证

| 端点 | 认证 | 说明 |
|----------|---------------|-------|
| GET /coupons | 可选 | 可用优惠券的公开列表 |
| GET /coupons/{code} | 可选 | 验证优惠券而不应用 |
| POST /coupons | 需要 | 创建新优惠券（管理员） |
| PUT /coupons/{id} | 需要 | 更新优惠券（管理员） |
| DELETE /coupons/{id} | 需要 | 删除优惠券（管理员） |

## 优惠券类型

| 类型 | 说明 | 示例 |
|------|-------------|---------|
| `percentage` | 小计百分比折扣 | 8 折 |
| `fixed_amount` | 小计固定金额折扣 | 减 10 元 |
| `free_shipping` | 免除配送费用 | 免费配送 |
| `buy_x_get_y` | 买 X 数量，送 Y | 买 2 送 1 |

## 端点

### 列出优惠券

返回可用优惠券。可选按活跃状态筛选。

```http
GET /api/v1/coupons?active=true&page=1&per_page=20
```

**响应 (200 OK):**

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "code": "SUMMER20",
      "type": "percentage",
      "value": "20.00",
      "description": "夏季系列 8 折",
      "minimum_order_amount": "50.00",
      "starts_at": "2026-06-01T00:00:00Z",
      "expires_at": "2026-08-31T23:59:59Z",
      "usage_limit": 1000,
      "usage_count": 245,
      "is_active": true
    }
  ],
  "meta": {
    "total": 15,
    "page": 1,
    "per_page": 20,
    "total_pages": 1
  }
}
```

### 获取优惠券

通过代码或 ID 检索特定优惠券的详情。

```http
GET /api/v1/coupons/SUMMER20
```

**响应 (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "code": "SUMMER20",
  "type": "percentage",
  "value": "20.00",
  "description": "夏季系列 8 折",
  "minimum_order_amount": "50.00",
  "maximum_discount_amount": "100.00",
  "starts_at": "2026-06-01T00:00:00Z",
  "expires_at": "2026-08-31T23:59:59Z",
  "usage_limit": 1000,
  "usage_limit_per_customer": 1,
  "usage_count": 245,
  "is_active": true,
  "applies_to": {
    "product_ids": [],
    "collection_ids": ["550e8400-e29b-41d4-a716-446655440001"],
    "exclude_product_ids": ["550e8400-e29b-41d4-a716-446655440002"]
  }
}
```

### 验证优惠券

针对购物车验证优惠券而不应用它。适用于显示折扣预览。

```http
POST /api/v1/coupons/validate
Content-Type: application/json
X-Session-Token: <session_token>

{
  "coupon_code": "SUMMER20",
  "cart_id": "550e8400-e29b-41d4-a716-446655440003"
}
```

**响应 (200 OK) - 有效:**

```json
{
  "valid": true,
  "coupon": {
    "code": "SUMMER20",
    "type": "percentage",
    "value": "20.00",
    "description": "夏季系列 8 折"
  },
  "discount": {
    "subtotal": "150.00",
    "discount_amount": "30.00",
    "new_total": "120.00"
  }
}
```

**响应 (200 OK) - 无效:**

```json
{
  "valid": false,
  "error": {
    "code": "COUPON_MINIMUM_NOT_MET",
    "message": "购物车小计（35.00 元）低于最低订单金额（50.00 元）"
  }
}
```

### 创建优惠券

创建新优惠券。需要管理员认证。

```http
POST /api/v1/coupons
Content-Type: application/json
Authorization: Bearer <admin_jwt_token>

{
  "code": "WELCOME15",
  "type": "percentage",
  "value": "15.00",
  "description": "新客户 85 折",
  "minimum_order_amount": "0.00",
  "maximum_discount_amount": null,
  "starts_at": "2026-01-01T00:00:00Z",
  "expires_at": null,
  "usage_limit": 10000,
  "usage_limit_per_customer": 1,
  "is_active": true,
  "applies_to": {
    "product_ids": [],
    "collection_ids": [],
    "exclude_product_ids": []
  },
  "customer_eligibility": {
    "new_customers_only": true,
    "customer_ids": [],
    "customer_segments": ["new_subscribers"]
  }
}
```

**响应 (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "code": "WELCOME15",
  "type": "percentage",
  "value": "15.00",
  "description": "新客户 85 折",
  "minimum_order_amount": "0.00",
  "starts_at": "2026-01-01T00:00:00Z",
  "expires_at": null,
  "usage_limit": 10000,
  "usage_limit_per_customer": 1,
  "usage_count": 0,
  "is_active": true,
  "created_at": "2026-01-28T10:00:00Z"
}
```

### 创建买 X 送 Y 优惠券

买 X 送 Y 促销的特殊格式。

```http
POST /api/v1/coupons
Content-Type: application/json
Authorization: Bearer <admin_jwt_token>

{
  "code": "BUY2GET1",
  "type": "buy_x_get_y",
  "buy_x_get_y": {
    "buy_quantity": 2,
    "get_quantity": 1,
    "buy_product_ids": ["550e8400-e29b-41d4-a716-446655440001"],
    "get_product_ids": ["550e8400-e29b-41d4-a716-446655440001"],
    "get_discount_type": "percentage",
    "get_discount_value": "100.00"
  },
  "description": "精选产品买 2 送 1",
  "usage_limit": 500,
  "is_active": true
}
```

### 更新优惠券

更新现有优惠券。支持部分更新。

```http
PUT /api/v1/coupons/550e8400-e29b-41d4-a716-446655440000
Content-Type: application/json
Authorization: Bearer <admin_jwt_token>

{
  "usage_limit": 2000,
  "expires_at": "2026-12-31T23:59:59Z"
}
```

**响应 (200 OK):**

返回更新的优惠券。

### 删除优惠券

永久删除优惠券。这不会影响已使用该优惠券的订单。

```http
DELETE /api/v1/coupons/550e8400-e29b-41d4-a716-446655440000
Authorization: Bearer <admin_jwt_token>
```

**响应 (204 No Content)**

### 获取优惠券使用情况

返回优惠券的使用统计。

```http
GET /api/v1/coupons/550e8400-e29b-41d4-a716-446655440000/usage
Authorization: Bearer <admin_jwt_token>
```

**响应 (200 OK):**

```json
{
  "coupon_id": "550e8400-e29b-41d4-a716-446655440000",
  "code": "SUMMER20",
  "usage_limit": 1000,
  "usage_count": 245,
  "remaining": 755,
  "total_discount_amount": "4895.50",
  "orders_count": 245,
  "average_order_value": "142.30",
  "usage_by_day": [
    {
      "date": "2026-01-27",
      "usage_count": 12,
      "discount_amount": "245.00"
    }
  ]
}
```

## 优惠券应用规则

### 折扣计算

**百分比:**
```
discount = subtotal * (value / 100)
if maximum_discount_amount and discount > maximum_discount_amount:
    discount = maximum_discount_amount
```

**固定金额:**
```
discount = value
if discount > subtotal:
    discount = subtotal
```

**免费配送:**
```
discount = shipping_total
```

**买 X 送 Y:**
```
eligible_sets = floor(buy_quantity_in_cart / buy_quantity)
discount_items = eligible_sets * get_quantity
discount = sum(discount_items * applicable_product_prices * discount_percentage)
```

### 应用限制

1. **最低订单金额**: 购物车小计必须达到或超过此值
2. **最大折扣**: 限制百分比优惠券的折扣金额
3. **产品限制**: 仅适用于指定产品/系列
4. **排除**: 从不适用于排除的产品
5. **客户资格**: 可能仅限新客户或特定细分
6. **时间窗口**: 必须在 starts_at 和 expires_at 之间
7. **使用限制**: 不能超过总次数或每客户限制

### 叠加规则

默认情况下，优惠券不叠加。适用以下规则：

- 每个购物车只能使用一张优惠券
- 优惠券不能与自动折扣合并
- 如果配置，免费配送可以与百分比/固定优惠券叠加

## 错误代码

| 代码 | 说明 |
|------|-------------|
| `COUPON_NOT_FOUND` | 优惠券代码不存在 |
| `COUPON_EXPIRED` | 优惠券已过期 |
| `COUPON_NOT_STARTED` | 优惠券有效期尚未开始 |
| `COUPON_INACTIVE` | 优惠券已被停用 |
| `COUPON_USAGE_LIMIT` | 总使用次数已达上限 |
| `COUPON_CUSTOMER_LIMIT` | 客户已使用过此优惠券 |
| `COUPON_MINIMUM_NOT_MET` | 购物车小计低于最低要求 |
| `COUPON_PRODUCT_NOT_ELIGIBLE` | 购物车中没有符合条件的产品 |
| `COUPON_ALREADY_APPLIED` | 优惠券已应用到此购物车 |
| `COUPON_CANNOT_COMBINE` | 无法与现有折扣合并 |
| `COUPON_NEW_CUSTOMERS_ONLY` | 仅对新客户有效 |
| `COUPON_CODE_EXISTS` | 优惠券代码已在使用中 |

## Webhooks

| 事件 | 说明 |
|-------|-------------|
| `coupon.created` | 新优惠券已创建 |
| `coupon.updated` | 优惠券详情已更新 |
| `coupon.deleted` | 优惠券已删除 |
| `coupon.applied` | 优惠券已应用到购物车 |
| `coupon.removed` | 优惠券已从购物车移除 |

## 最佳实践

1. **代码格式**: 使用大写字母数字代码（例如 `SUMMER20`、`WELCOME15`）
2. **过期**: 始终为限时促销设置过期日期
3. **使用限制**: 设置合理的限制以防止滥用
4. **最低订单**: 使用最低订单金额来维持利润率
5. **测试**: 在生产环境之前在测试环境验证优惠券
6. **监控**: 跟踪使用统计以衡量促销效果

## 示例：完整的优惠券活动

```javascript
// 创建夏季促销优惠券
const coupon = await fetch('/api/v1/coupons', {
  method: 'POST',
  headers: { 'Authorization': `Bearer ${adminToken}` },
  body: JSON.stringify({
    code: 'SUMMER2026',
    type: 'percentage',
    value: '25.00',
    description: '2026 夏季促销 - 全场 75 折',
    minimum_order_amount: '0.00',
    maximum_discount_amount: '200.00',
    starts_at: '2026-06-01T00:00:00Z',
    expires_at: '2026-08-31T23:59:59Z',
    usage_limit: 5000,
    usage_limit_per_customer: 2,
    is_active: true
  })
});

// 客户在结账前验证
const validation = await fetch('/api/v1/coupons/validate', {
  method: 'POST',
  headers: { 'X-Session-Token': sessionToken },
  body: JSON.stringify({
    coupon_code: 'SUMMER2026',
    cart_id: cartId
  })
});

// 应用到购物车
await fetch(`/api/v1/carts/${cartId}/coupon`, {
  method: 'POST',
  headers: { 'X-Session-Token': sessionToken },
  body: JSON.stringify({ coupon_code: 'SUMMER2026' })
});
```
