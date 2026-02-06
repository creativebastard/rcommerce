# 通知 API

通知 API 管理您商店的电子邮件、短信、推送和 Webhook 通知。它提供了发送通知、管理模板和跟踪投递状态的端点。

## 基础 URL

```
/api/v1/notifications
```

## 认证

通知端点需要认证。管理端点需要密钥。

```http
Authorization: Bearer YOUR_API_KEY
```

## 通知对象

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "订单确认：ORD-12345",
  "body": "您的订单已确认...",
  "html_body": "<html>...</html>",
  "priority": "high",
  "status": "delivered",
  "attempt_count": 1,
  "max_attempts": 3,
  "error_message": null,
  "metadata": {
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "type": "order_confirmation"
  },
  "scheduled_at": null,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:01Z"
}
```

### 通知字段

| 字段 | 类型 | 说明 |
|-------|------|-------------|
| `id` | UUID | 唯一标识符 |
| `channel` | string | `email`、`sms`、`push`、`webhook`、`in_app` |
| `recipient` | string | 收件人地址（邮箱、电话或 URL） |
| `subject` | string | 通知主题行 |
| `body` | string | 纯文本内容 |
| `html_body` | string | HTML 格式内容（可选） |
| `priority` | string | `low`、`normal`、`high`、`urgent` |
| `status` | string | `pending`、`sent`、`delivered`、`failed`、`bounced` |
| `attempt_count` | integer | 已尝试的投递次数 |
| `max_attempts` | integer | 最大重试次数 |
| `error_message` | string | 失败时的错误描述 |
| `metadata` | object | 额外的上下文数据 |
| `scheduled_at` | datetime | 计划发送时间（可选） |
| `created_at` | datetime | 创建时间戳 |
| `updated_at` | datetime | 最后更新时间戳 |

## 端点

### 列出通知

```http
GET /api/v1/notifications
```

检索分页的通知列表。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `page` | integer | 页码（默认：1） |
| `per_page` | integer | 每页项目数（默认：20，最大：100） |
| `channel` | string | 按渠道筛选：`email`、`sms`、`push`、`webhook` |
| `status` | string | 按状态筛选：`pending`、`sent`、`delivered`、`failed` |
| `priority` | string | 按优先级筛选：`low`、`normal`、`high`、`urgent` |
| `recipient` | string | 按收件人地址筛选 |
| `created_after` | datetime | 创建日期之后 |
| `created_before` | datetime | 创建日期之前 |
| `sort` | string | `created_at`、`updated_at`、`priority` |
| `order` | string | `asc` 或 `desc`（默认：desc） |

#### 示例请求

```http
GET /api/v1/notifications?channel=email&status=failed&sort=created_at&order=desc
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440500",
      "channel": "email",
      "recipient": "customer@example.com",
      "subject": "订单确认：ORD-12345",
      "priority": "high",
      "status": "failed",
      "attempt_count": 3,
      "error_message": "连接超时",
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

### 发送通知

```http
POST /api/v1/notifications
```

向收件人发送新通知。

#### 请求体

```json
{
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "欢迎加入我们的商店",
  "body": "感谢您的加入！",
  "html_body": "<h1>欢迎！</h1><p>感谢您的加入！</p>",
  "priority": "normal",
  "template_id": "welcome_html",
  "template_variables": {
    "customer_name": "张三",
    "company_name": "R Commerce"
  },
  "metadata": {
    "campaign_id": "welcome_series_1"
  },
  "scheduled_at": null
}
```

#### 请求字段

| 字段 | 类型 | 必需 | 说明 |
|-------|------|----------|-------------|
| `channel` | string | 是 | 通知渠道 |
| `recipient` | string | 是 | 收件人地址 |
| `subject` | string | 条件 | 主题行（邮件必需） |
| `body` | string | 条件 | 纯文本内容（无模板时必需） |
| `html_body` | string | 否 | HTML 内容 |
| `priority` | string | 否 | 优先级（默认：`normal`） |
| `template_id` | string | 否 | 要使用的模板 |
| `template_variables` | object | 否 | 模板替换变量 |
| `metadata` | object | 否 | 额外的上下文数据 |
| `scheduled_at` | datetime | 否 | 计划稍后投递 |

#### 示例响应

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "欢迎加入我们的商店",
  "status": "pending",
  "priority": "normal",
  "attempt_count": 0,
  "max_attempts": 3,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### 获取通知

```http
GET /api/v1/notifications/{id}
```

通过 ID 检索单个通知。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | UUID | 通知 ID |

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `include` | string | 相关数据：`attempts`、`template` |

#### 示例响应

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "订单确认：ORD-12345",
  "body": "您的订单已确认...",
  "html_body": "<html>...</html>",
  "priority": "high",
  "status": "delivered",
  "attempt_count": 1,
  "max_attempts": 3,
  "error_message": null,
  "metadata": {
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "type": "order_confirmation"
  },
  "scheduled_at": null,
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:01Z",
  "attempts": [
    {
      "channel": "email",
      "attempted_at": "2024-01-15T10:00:01Z",
      "status": "delivered",
      "error": null
    }
  ]
}
```

### 重试失败的通知

```http
POST /api/v1/notifications/{id}/retry
```

重试失败的通知投递。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | UUID | 通知 ID |

#### 示例请求

```http
POST /api/v1/notifications/550e8400-e29b-41d4-a716-446655440500/retry
Authorization: Bearer sk_live_xxx
```

#### 示例响应

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "status": "pending",
  "attempt_count": 3,
  "message": "通知已排队等待重试"
}
```

### 取消计划通知

```http
POST /api/v1/notifications/{id}/cancel
```

取消已计划但尚未发送的通知。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | UUID | 通知 ID |

#### 示例响应

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440500",
  "status": "cancelled",
  "message": "计划通知已取消"
}
```

## 模板

### 列出模板

```http
GET /api/v1/notifications/templates
```

检索所有可用的通知模板。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `channel` | string | 按渠道筛选 |
| `category` | string | 按类别筛选 |

#### 示例响应

```json
{
  "data": [
    {
      "id": "order_confirmation_html",
      "name": "订单确认 HTML",
      "channel": "email",
      "subject": "订单已确认：{{ order_number }}",
      "variables": [
        "order_number",
        "order_date",
        "customer_name",
        "order_total"
      ],
      "has_html": true,
      "created_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": "welcome_html",
      "name": "欢迎 HTML",
      "channel": "email",
      "subject": "欢迎来到 {{ company_name }}！",
      "variables": [
        "customer_name",
        "company_name",
        "login_url"
      ],
      "has_html": true,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ]
}
```

### 获取模板

```http
GET /api/v1/notifications/templates/{id}
```

检索特定模板。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | string | 模板 ID |

#### 示例响应

```json
{
  "id": "order_confirmation_html",
  "name": "订单确认 HTML",
  "channel": "email",
  "subject": "订单已确认：{{ order_number }}",
  "body": "您好 {{ customer_name }}，感谢您的订单...",
  "html_body": "<!DOCTYPE html>...</html>",
  "variables": [
    "order_number",
    "order_date",
    "order_total",
    "customer_name",
    "customer_email",
    "shipping_address",
    "billing_address",
    "subtotal",
    "shipping_cost",
    "tax",
    "items",
    "company_name",
    "support_email"
  ],
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### 创建模板

```http
POST /api/v1/notifications/templates
```

创建新的通知模板（仅限管理员）。

#### 请求体

```json
{
  "id": "custom_promotion",
  "name": "自定义促销",
  "channel": "email",
  "subject": "特别优惠：{{ discount_percent }}% 折扣！",
  "body": "您好 {{ customer_name }}，下次订单享受 {{ discount_percent }}% 折扣！",
  "html_body": "<h1>特别优惠！</h1><p>您好 {{ customer_name }}...</p>",
  "variables": [
    "customer_name",
    "discount_percent",
    "promo_code",
    "expiry_date"
  ]
}
```

#### 请求字段

| 字段 | 类型 | 必需 | 说明 |
|-------|------|----------|-------------|
| `id` | string | 是 | 唯一模板标识符 |
| `name` | string | 是 | 人类可读的名称 |
| `channel` | string | 是 | 目标渠道 |
| `subject` | string | 条件 | 主题模板（仅限邮件） |
| `body` | string | 是 | 带占位符的正文模板 |
| `html_body` | string | 否 | HTML 模板 |
| `variables` | array | 是 | 必需变量名称列表 |

#### 示例响应

```json
{
  "id": "custom_promotion",
  "name": "自定义促销",
  "channel": "email",
  "subject": "特别优惠：{{ discount_percent }}% 折扣！",
  "variables": [
    "customer_name",
    "discount_percent",
    "promo_code",
    "expiry_date"
  ],
  "created_at": "2024-01-15T10:00:00Z"
}
```

### 更新模板

```http
PUT /api/v1/notifications/templates/{id}
```

更新现有模板。

#### 请求体

```json
{
  "name": "更新的促销名称",
  "subject": "更新的主题：{{ discount_percent }}% 折扣！",
  "body": "更新的正文内容...",
  "html_body": "<h1>更新的 HTML</h1>...",
  "variables": [
    "customer_name",
    "discount_percent",
    "promo_code"
  ]
}
```

### 删除模板

```http
DELETE /api/v1/notifications/templates/{id}
```

删除自定义模板。系统模板无法删除。

#### 参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `id` | string | 模板 ID |

#### 示例响应

```json
{
  "deleted": true,
  "id": "custom_promotion"
}
```

### 预览模板

```http
POST /api/v1/notifications/templates/{id}/preview
```

使用示例变量预览模板。

#### 请求体

```json
{
  "variables": {
    "customer_name": "张三",
    "order_number": "ORD-12345",
    "order_total": "99.99"
  }
}
```

#### 示例响应

```json
{
  "subject": "订单已确认：ORD-12345",
  "body": "您好 张三，感谢您的订单...",
  "html_body": "<!DOCTYPE html>...",
  "rendered_at": "2024-01-15T10:00:00Z"
}
```

## 内置模板

### 订单模板

| 模板 ID | 渠道 | 说明 |
|-------------|---------|-------------|
| `order_confirmation` | email | 纯文本订单确认 |
| `order_confirmation_html` | email | 带发票的 HTML 订单确认 |
| `order_shipped` | email | 纯文本发货通知 |
| `order_shipped_html` | email | HTML 发货通知 |
| `order_cancelled_html` | email | 订单取消通知 |

### 支付模板

| 模板 ID | 渠道 | 说明 |
|-------------|---------|-------------|
| `payment_successful_html` | email | 支付确认 |
| `payment_failed_html` | email | 支付失败通知 |
| `refund_processed_html` | email | 退款确认 |

### 订阅模板

| 模板 ID | 渠道 | 说明 |
|-------------|---------|-------------|
| `subscription_created_html` | email | 新订阅欢迎 |
| `subscription_renewal_html` | email | 续订确认 |
| `subscription_cancelled_html` | email | 取消通知 |

### 催缴模板

| 模板 ID | 渠道 | 说明 |
|-------------|---------|-------------|
| `dunning_first_html` | email | 首次支付重试通知 |
| `dunning_retry_html` | email | 后续重试通知 |
| `dunning_final_html` | email | 最终支付通知 |

### 客户模板

| 模板 ID | 渠道 | 说明 |
|-------------|---------|-------------|
| `welcome_html` | email | 新客户欢迎 |
| `password_reset_html` | email | 密码重置说明 |
| `abandoned_cart_html` | email | 购物车恢复邮件 |

### 库存模板

| 模板 ID | 渠道 | 说明 |
|-------------|---------|-------------|
| `low_stock_alert` | email | 低库存警告 |

## 批量操作

### 发送批量通知

```http
POST /api/v1/notifications/bulk
```

向多个收件人发送通知。

#### 请求体

```json
{
  "template_id": "promotion_announcement",
  "channel": "email",
  "recipients": [
    {
      "email": "customer1@example.com",
      "variables": {
        "customer_name": "张三",
        "discount_code": "SAVE20"
      }
    },
    {
      "email": "customer2@example.com",
      "variables": {
        "customer_name": "李四",
        "discount_code": "SAVE20"
      }
    }
  ],
  "priority": "normal",
  "metadata": {
    "campaign_id": "summer_sale_2024"
  }
}
```

#### 示例响应

```json
{
  "batch_id": "batch_550e8400e29b41d4a716446655440600",
  "total": 2,
  "queued": 2,
  "failed": 0,
  "notifications": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440501",
      "recipient": "customer1@example.com",
      "status": "pending"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440502",
      "recipient": "customer2@example.com",
      "status": "pending"
    }
  ]
}
```

## 统计

### 获取投递统计

```http
GET /api/v1/notifications/stats
```

检索通知投递统计。

#### 查询参数

| 参数 | 类型 | 说明 |
|-----------|------|-------------|
| `channel` | string | 按渠道筛选 |
| `since` | datetime | 自日期以来的统计 |

#### 示例响应

```json
{
  "period": {
    "start": "2024-01-01T00:00:00Z",
    "end": "2024-01-15T23:59:59Z"
  },
  "overall": {
    "sent": 10000,
    "delivered": 9500,
    "failed": 450,
    "bounced": 50,
    "delivery_rate": 0.95,
    "failure_rate": 0.045
  },
  "by_channel": {
    "email": {
      "sent": 8000,
      "delivered": 7600,
      "failed": 360,
      "bounced": 40
    },
    "sms": {
      "sent": 1500,
      "delivered": 1425,
      "failed": 75,
      "bounced": 0
    },
    "push": {
      "sent": 500,
      "delivered": 475,
      "failed": 15,
      "bounced": 10
    }
  }
}
```

## Webhook 端点

### 传入 Webhooks

通过 Webhooks 接收通知状态更新。

#### 投递状态 Webhook

```http
POST /webhooks/notifications/delivery
```

当通知状态更改时，R Commerce 会发送 Webhook 事件。

#### 负载格式

```json
{
  "id": "evt_550e8400e29b41d4a716446655440700",
  "topic": "notification.delivered",
  "created_at": "2024-01-15T10:00:01Z",
  "data": {
    "notification_id": "550e8400-e29b-41d4-a716-446655440500",
    "channel": "email",
    "recipient": "customer@example.com",
    "status": "delivered",
    "delivered_at": "2024-01-15T10:00:01Z",
    "message_id": "msg_abc123"
  }
}
```

### Webhook 事件

| 事件 | 说明 |
|-------|-------------|
| `notification.created` | 通知已排队等待发送 |
| `notification.sent` | 通知已发送给提供商 |
| `notification.delivered` | 通知成功投递 |
| `notification.failed` | 投递失败 |
| `notification.bounced` | 通知被退回 |
| `notification.opened` | 邮件已打开（仅限邮件） |
| `notification.clicked` | 链接已点击（仅限邮件） |

## 渠道

### 邮件

使用 SMTP 或邮件服务提供商发送邮件通知。

**收件人格式：** 有效的邮箱地址

```json
{
  "channel": "email",
  "recipient": "customer@example.com",
  "subject": "订单确认",
  "body": "...",
  "html_body": "..."
}
```

### 短信

通过 Twilio 或其他提供商发送短信通知。

**收件人格式：** E.164 电话号码 (+1234567890)

```json
{
  "channel": "sms",
  "recipient": "+1234567890",
  "body": "您的订单 ORD-12345 已发货！"
}
```

### 推送

向移动设备发送推送通知。

**收件人格式：** 设备令牌

```json
{
  "channel": "push",
  "recipient": "device_token_abc123",
  "subject": "订单已发货",
  "body": "您的订单已发货！"
}
```

### Webhook

向外部端点发送 HTTP Webhooks。

**收件人格式：** HTTPS URL

```json
{
  "channel": "webhook",
  "recipient": "https://your-app.com/webhooks/notifications",
  "body": "{\"event\": \"order_created\", \"data\": {...}}"
}
```

## 错误代码

| 代码 | HTTP 状态 | 说明 |
|------|-------------|-------------|
| `NOTIFICATION_NOT_FOUND` | 404 | 通知不存在 |
| `TEMPLATE_NOT_FOUND` | 404 | 模板不存在 |
| `INVALID_CHANNEL` | 400 | 不支持的通知渠道 |
| `INVALID_RECIPIENT` | 400 | 渠道的收件人格式无效 |
| `MISSING_SUBJECT` | 400 | 邮件渠道需要主题 |
| `MISSING_TEMPLATE_VARIABLES` | 400 | 未提供必需的模板变量 |
| `TEMPLATE_RENDER_ERROR` | 400 | 模板渲染失败 |
| `NOTIFICATION_ALREADY_SENT` | 409 | 无法重试非失败通知 |
| `NOTIFICATION_CANCELLED` | 409 | 通知已取消 |
| `CANNOT_CANCEL_SENT` | 400 | 无法取消已发送的通知 |
| `BULK_LIMIT_EXCEEDED` | 400 | 批量请求中收件人过多 |
| `RATE_LIMIT_EXCEEDED` | 429 | 通知请求过多 |
| `CHANNEL_DISABLED` | 400 | 通知渠道已禁用 |
| `PROVIDER_ERROR` | 502 | 外部提供商错误 |

## 速率限制

| 端点 | 限制 |
|----------|-------|
| `POST /notifications` | 100/分钟 |
| `POST /notifications/bulk` | 10/分钟，最多 1000 个收件人 |
| `GET /notifications` | 1000/分钟 |
| 其他端点 | 100/分钟 |

## 最佳实践

### 模板变量

使用一致的变量命名：

```json
{
  "customer_name": "张三",
  "order_number": "ORD-12345",
  "order_total": "99.99",
  "company_name": "R Commerce"
}
```

### 重试策略

失败的通知会自动使用指数退避重试：

| 尝试 | 延迟 |
|---------|-------|
| 1 | 立即 |
| 2 | 5 秒 |
| 3 | 25 秒 |

### 计划通知

为最佳投递时间计划通知：

```json
{
  "channel": "email",
  "recipient": "customer@example.com",
  "template_id": "promotion",
  "scheduled_at": "2024-01-20T09:00:00Z"
}
```

### 测试

在发送前使用预览端点测试模板：

```http
POST /api/v1/notifications/templates/welcome_html/preview
{
  "variables": {
    "customer_name": "测试用户",
    "company_name": "测试公司"
  }
}
```
