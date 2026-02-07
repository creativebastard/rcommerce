# 购物车 API

购物车 API 提供完整的购物车系统，支持游客用户（通过会话令牌）和已认证客户。所有端点均已完全实现并正常运行。

## 概览

- **游客购物车**: 通过会话令牌识别，无需认证
- **客户购物车**: 绑定到已认证的客户账户
- **购物车合并**: 游客登录时自动合并
- **过期**: 购物车在不活动 30 天后过期

## 基础 URL

```
/api/v1/carts
```

## 认证

| 端点 | 认证 |
|----------|---------------|
| GET /carts/guest | 无（使用会话令牌） |
| GET /carts/me | 需要（JWT 或 API 密钥） |
| 所有其他端点 | 可选（会话令牌或 JWT） |

## 会话管理

对于游客用户，在头中包含会话令牌：

```http
X-Session-Token: <session_token>
```

会话令牌在创建游客购物车时返回，应存储在客户端（localStorage、cookie 等）。

## 端点

### 创建游客购物车

为游客用户创建新购物车。

```http
POST /api/v1/carts/guest
Content-Type: application/json

{
  "currency": "USD"
}
```

**响应 (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "session_token": "sess_abc123xyz",
  "currency": "USD",
  "subtotal": "0.00",
  "discount_total": "0.00",
  "tax_total": "0.00",
  "shipping_total": "0.00",
  "total": "0.00",
  "item_count": 0,
  "items": [],
  "expires_at": "2026-02-27T10:00:00Z"
}
```

**重要:** 将 `session_token` 存储在客户端以供后续请求使用。

### 获取或创建客户购物车

返回当前客户的活跃购物车或创建新购物车。

```http
GET /api/v1/carts/me
Authorization: Bearer <jwt_token>
```

**响应 (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "customer_id": "550e8400-e29b-41d4-a716-446655440002",
  "currency": "USD",
  "subtotal": "150.00",
  "discount_total": "15.00",
  "tax_total": "13.50",
  "shipping_total": "10.00",
  "total": "158.50",
  "coupon_code": "SUMMER10",
  "item_count": 3,
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440003",
      "product_id": "550e8400-e29b-41d4-a716-446655440004",
      "variant_id": "550e8400-e29b-41d4-a716-446655440005",
      "quantity": 2,
      "unit_price": "50.00",
      "original_price": "50.00",
      "subtotal": "100.00",
      "discount_amount": "10.00",
      "total": "90.00",
      "sku": "PROD-001-L",
      "title": "优质 T 恤",
      "variant_title": "大号 / 蓝色",
      "image_url": "https://cdn.example.com/products/001.jpg",
      "requires_shipping": true,
      "is_gift_card": false
    }
  ],
  "expires_at": "2026-02-27T10:00:00Z"
}
```

### 通过 ID 获取购物车

检索特定购物车及其所有商品。

```http
GET /api/v1/carts/{cart_id}
X-Session-Token: <session_token>
```

### 添加商品到购物车

将产品添加到购物车。如果商品已存在，数量将合并。

```http
POST /api/v1/carts/{cart_id}/items
Content-Type: application/json
X-Session-Token: <session_token>

{
  "product_id": "550e8400-e29b-41d4-a716-446655440004",
  "variant_id": "550e8400-e29b-41d4-a716-446655440005",
  "quantity": 2,
  "custom_attributes": {
    "engraving": "生日快乐"
  }
}
```

**响应 (201 Created):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440003",
  "product_id": "550e8400-e29b-41d4-a716-446655440004",
  "variant_id": "550e8400-e29b-41d4-a716-446655440005",
  "quantity": 2,
  "unit_price": "50.00",
  "original_price": "50.00",
  "subtotal": "100.00",
  "discount_amount": "10.00",
  "total": "90.00",
  "sku": "PROD-001-L",
  "title": "优质 T 恤",
  "variant_title": "大号 / 蓝色",
  "image_url": "https://cdn.example.com/products/001.jpg",
  "requires_shipping": true,
  "is_gift_card": false,
  "custom_attributes": {
    "engraving": "生日快乐"
  }
}
```

### 更新购物车商品

更新购物车商品的数量或自定义属性。

```http
PUT /api/v1/carts/{cart_id}/items/{item_id}
Content-Type: application/json
X-Session-Token: <session_token>

{
  "quantity": 3,
  "custom_attributes": {
    "engraving": "周年快乐"
  }
}
```

**响应 (200 OK):**

返回更新的购物车商品。

### 从购物车移除商品

从购物车中移除商品。

```http
DELETE /api/v1/carts/{cart_id}/items/{item_id}
X-Session-Token: <session_token>
```

**响应 (204 No Content)**

### 清空购物车

从购物车中移除所有商品。

```http
DELETE /api/v1/carts/{cart_id}/items
X-Session-Token: <session_token>
```

**响应 (204 No Content)**

### 应用优惠券

将折扣优惠券应用到购物车。

```http
POST /api/v1/carts/{cart_id}/coupon
Content-Type: application/json
X-Session-Token: <session_token>

{
  "coupon_code": "SUMMER20"
}
```

**响应 (200 OK):**

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "coupon_code": "SUMMER20",
  "subtotal": "150.00",
  "discount_total": "30.00",
  "tax_total": "12.00",
  "shipping_total": "10.00",
  "total": "142.00"
}
```

**错误响应:**

- `400 Bad Request` - 优惠券代码无效
- `409 Conflict` - 优惠券无法与现有折扣合并

### 移除优惠券

从购物车中移除已应用的优惠券。

```http
DELETE /api/v1/carts/{cart_id}/coupon
X-Session-Token: <session_token>
```

**响应 (200 OK):**

返回不含优惠券的更新购物车。

### 更新购物车详情

更新购物车级别的信息，如邮箱、配送方式或备注。

```http
PUT /api/v1/carts/{cart_id}
Content-Type: application/json
X-Session-Token: <session_token>

{
  "email": "customer@example.com",
  "shipping_method": "express",
  "notes": "请礼品包装"
}
```

**响应 (200 OK):**

返回更新的购物车。

### 合并游客购物车

将游客购物车合并到已认证客户的购物车中。用户登录后调用此接口。

```http
POST /api/v1/carts/merge
Content-Type: application/json
Authorization: Bearer <jwt_token>

{
  "session_token": "sess_abc123xyz"
}
```

**响应 (200 OK):**

返回合并后的购物车，包含两个购物车的所有商品。

**行为:**

- 如果两个购物车有相同商品，数量将相加
- 如果两个购物车都有优惠券，保留客户的优惠券
- 游客购物车被标记为已转换

### 删除购物车

永久删除购物车及其所有商品。

```http
DELETE /api/v1/carts/{cart_id}
X-Session-Token: <session_token>
```

**响应 (204 No Content)**

## 购物车计算

购物车自动计算：

```
subtotal = sum(item.quantity * item.unit_price)
discount_total = sum(item.discount_amount) [来自优惠券]
tax_total = 基于配送地址计算
shipping_total = 基于配送方式计算
total = subtotal - discount_total + tax_total + shipping_total
```

## 错误代码

| 代码 | 说明 |
|------|-------------|
| `CART_NOT_FOUND` | 购物车 ID 不存在 |
| `CART_EXPIRED` | 购物车已过期 |
| `CART_CONVERTED` | 购物车已转换为订单 |
| `ITEM_NOT_FOUND` | 购物车商品 ID 不存在 |
| `INVALID_QUANTITY` | 数量必须在 1 到 9999 之间 |
| `PRODUCT_NOT_AVAILABLE` | 产品未激活或缺货 |
| `COUPON_INVALID` | 优惠券代码无效或已过期 |
| `COUPON_MINIMUM_NOT_MET` | 购物车小计低于优惠券最低要求 |
| `COUPON_USAGE_LIMIT` | 优惠券使用次数已达上限 |

## Webhooks

购物车系统触发以下 webhook 事件：

| 事件 | 说明 |
|-------|-------------|
| `cart.created` | 新购物车已创建 |
| `cart.updated` | 购物车详情已更新 |
| `cart.item_added` | 商品已添加到购物车 |
| `cart.item_updated` | 商品数量/属性已更新 |
| `cart.item_removed` | 商品已从购物车移除 |
| `cart.coupon_applied` | 优惠券已应用到购物车 |
| `cart.coupon_removed` | 优惠券已从购物车移除 |
| `cart.merged` | 游客购物车已合并到客户购物车 |
| `cart.converted` | 购物车已转换为订单 |

## 最佳实践

1. **会话令牌存储**: 使用适当的安全设置将会话令牌存储在 localStorage 或 cookie 中
2. **购物车持久化**: 在创建新购物车之前始终检查现有购物车
3. **数量验证**: 在添加到购物车之前验证产品可用性
4. **错误处理**: 通过创建新购物车来优雅地处理购物车过期
5. **登录时合并**: 游客用户登录时始终调用合并端点

## 示例流程：游客到客户

```javascript
// 1. 游客添加商品到购物车
const guestCart = await fetch('/api/v1/carts/guest', {
  method: 'POST',
  body: JSON.stringify({ currency: 'USD' })
});
localStorage.setItem('cart_session', guestCart.session_token);

// 2. 游客添加商品
await fetch(`/api/v1/carts/${guestCart.id}/items`, {
  method: 'POST',
  headers: { 'X-Session-Token': guestCart.session_token },
  body: JSON.stringify({ product_id: '...', quantity: 2 })
});

// 3. 游客登录
const login = await fetch('/api/v1/auth/login', { ... });

// 4. 将游客购物车合并到客户购物车
await fetch('/api/v1/carts/merge', {
  method: 'POST',
  headers: { 'Authorization': `Bearer ${login.token}` },
  body: JSON.stringify({ session_token: guestCart.session_token })
});

// 5. 清除游客会话
localStorage.removeItem('cart_session');
```
