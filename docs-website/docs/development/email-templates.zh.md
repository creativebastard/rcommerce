# 邮件模板

R Commerce 使用可自定义的邮件模板处理所有客户和管理员通知。本指南涵盖可用模板、变量、自定义和测试。

## 概述

邮件模板具有以下特点：

- **HTML 和文本** - 适用于所有客户端的多部分邮件
- **模板引擎** - 用于动态内容的 Handlebars 语法
- **多语言** - 支持本地化
- **可自定义** - 覆盖默认模板

## 可用邮件模板

### 客户通知

| 模板 | 说明 | 触发条件 |
|----------|-------------|---------|
| `order_confirmation` | 订单确认 | 订单创建 |
| `order_shipped` | 发货通知 | 履行创建 |
| `order_delivered` | 送达确认 | 订单送达 |
| `order_cancelled` | 订单取消 | 订单取消 |
| `order_refund` | 退款通知 | 退款处理 |
| `payment_receipt` | 付款确认 | 付款捕获 |
| `payment_failed` | 付款失败提醒 | 付款失败 |
| `account_created` | 欢迎邮件 | 客户注册 |
| `password_reset` | 密码重置链接 | 重置请求 |
| `abandoned_cart` | 购物车恢复 | 购物车放弃（24小时） |

### 管理员通知

| 模板 | 说明 | 触发条件 |
|----------|-------------|---------|
| `admin_order_notification` | 新订单提醒 | 订单创建 |
| `admin_low_stock` | 低库存提醒 | 库存低于阈值 |
| `admin_payment_dispute` | 拒付/争议提醒 | 争议创建 |
| `admin_daily_summary` | 每日销售报告 | 每日定时任务 |

### 订阅通知

| 模板 | 说明 | 触发条件 |
|----------|-------------|---------|
| `subscription_created` | 订阅确认 | 订阅开始 |
| `subscription_renewal` | 即将续费通知 | 续费前 3 天 |
| `subscription_payment_failed` | 续费付款失败 | 续费付款失败 |
| `subscription_cancelled` | 取消确认 | 订阅取消 |
| `subscription_expired` | 订阅结束 | 订阅过期 |

### 催缴管理

| 模板 | 说明 | 触发条件 |
|----------|-------------|---------|
| `dunning_retry_1` | 首次付款重试通知 | 首次重试尝试 |
| `dunning_retry_2` | 第二次付款重试通知 | 第二次重试尝试 |
| `dunning_final` | 取消前最终通知 | 最终重试尝试 |

## 模板变量

### 通用变量

所有模板都可以访问：

```handlebars
{{!-- 商店信息 --}}
{{store.name}}
{{store.url}}
{{store.logo_url}}
{{store.support_email}}
{{store.support_phone}}

{{!-- 当前日期/时间 --}}
{{now}}
{{formatDate now "YYYY-MM-DD"}}

{{!-- 辅助函数 --}}
{{formatCurrency amount currency}}
{{formatDate date format}}
{{uppercase string}}
{{lowercase string}}
```

### 订单模板

```handlebars
{{!-- 订单详情 --}}
{{order.order_number}}
{{order.status}}
{{order.created_at}}
{{order.total}}
{{order.subtotal}}
{{order.tax_total}}
{{order.shipping_total}}
{{order.discount_total}}
{{order.currency}}

{{!-- 客户详情 --}}
{{order.customer.email}}
{{order.customer.first_name}}
{{order.customer.last_name}}
{{order.customer.full_name}}

{{!-- 收货地址 --}}
{{order.shipping_address.name}}
{{order.shipping_address.address1}}
{{order.shipping_address.city}}
{{order.shipping_address.country}}

{{!-- 订单项目 --}}
{{#each order.line_items}}
  {{this.name}}
  {{this.sku}}
  {{this.quantity}}
  {{this.price}}
  {{this.total}}
{{/each}}

{{!-- 履行详情（发货模板） --}}
{{fulfillment.tracking_number}}
{{fulfillment.tracking_url}}
{{fulfillment.carrier}}
{{fulfillment.shipped_at}}
```

### 客户模板

```handlebars
{{!-- 客户详情 --}}
{{customer.email}}
{{customer.first_name}}
{{customer.last_name}}
{{customer.full_name}}
{{customer.phone}}

{{!-- 账户详情 --}}
{{customer.created_at}}
{{customer.orders_count}}
{{customer.total_spent}}

{{!-- 密码重置 --}}
{{reset_url}}
{{reset_token}}
{{reset_expires_at}}

{{!-- 账户创建 --}}
{{login_url}}
```

### 付款模板

```handlebars
{{!-- 付款详情 --}}
{{payment.id}}
{{payment.amount}}
{{payment.currency}}
{{payment.status}}
{{payment.gateway}}
{{payment.created_at}}

{{!-- 卡片详情（如适用） --}}
{{payment.card_brand}}
{{payment.card_last_four}}

{{!-- 收据 URL --}}
{{payment.receipt_url}}
```

### 订阅模板

```handlebars
{{!-- 订阅详情 --}}
{{subscription.id}}
{{subscription.status}}
{{subscription.plan_name}}
{{subscription.plan_amount}}
{{subscription.plan_interval}}
{{subscription.current_period_start}}
{{subscription.current_period_end}}

{{!-- 续费详情 --}}
{{subscription.renewal_date}}
{{subscription.renewal_amount}}

{{!-- 取消 --}}
{{subscription.cancelled_at}}
{{subscription.cancellation_reason}}
```

## 自定义模板

### 模板存储

模板存储在：

```
/etc/rcommerce/templates/          # 系统模板
/opt/rcommerce/templates/          # 自定义模板（FreeBSD）
/var/lib/rcommerce/templates/      # 自定义模板（Linux）
./templates/                       # 开发环境
```

### 模板结构

每个模板有三个文件：

```
templates/
├── order_confirmation/
│   ├── subject.hbs       # 邮件主题
│   ├── html.hbs          # HTML 正文
│   └── text.hbs          # 纯文本正文
```

### 创建自定义模板

1. **创建模板目录：**
   ```bash
   mkdir -p /var/lib/rcommerce/templates/order_confirmation
   ```

2. **创建主题模板：**
   ```handlebars
   {{!-- templates/order_confirmation/subject.hbs --}}
   Order {{order.order_number}} Confirmed - {{store.name}}
   ```

3. **创建 HTML 模板：**
   ```handlebars
   {{!-- templates/order_confirmation/html.hbs --}}
   <!DOCTYPE html>
   <html>
   <head>
     <style>
       body { font-family: Arial, sans-serif; }
       .header { background: #f5f5f5; padding: 20px; }
       .content { padding: 20px; }
     </style>
   </head>
   <body>
     <div class="header">
       <h1>Thank you for your order!</h1>
     </div>
     <div class="content">
       <p>Hi {{order.customer.first_name}},</p>
       <p>Your order #{{order.order_number}} has been confirmed.</p>
       
       <h2>Order Summary</h2>
       <table>
         {{#each order.line_items}}
         <tr>
           <td>{{this.name}} x {{this.quantity}}</td>
           <td>{{formatCurrency this.total ../order.currency}}</td>
         </tr>
         {{/each}}
       </table>
       
       <p><strong>Total: {{formatCurrency order.total order.currency}}</strong></p>
       
       <p><a href="{{store.url}}/orders/{{order.order_number}}">View Order</a></p>
     </div>
   </body>
   </html>
   ```

4. **创建文本模板：**
   ```handlebars
   {{!-- templates/order_confirmation/text.hbs --}}
   Thank you for your order!
   
   Hi {{order.customer.first_name}},
   
   Your order #{{order.order_number}} has been confirmed.
   
   Order Summary:
   {{#each order.line_items}}
   - {{this.name}} x {{this.quantity}}: {{formatCurrency this.total ../order.currency}}
   {{/each}}
   
   Total: {{formatCurrency order.total order.currency}}
   
   View Order: {{store.url}}/orders/{{order.order_number}}
   ```

### 模板配置

在 `config.toml` 中配置模板位置：

```toml
[notifications.email]
template_dir = "/var/lib/rcommerce/templates"
default_from = "noreply@yourstore.com"
reply_to = "support@yourstore.com"

-- 启用模板自动重载（开发环境）
auto_reload = false

-- 模板缓存 TTL
cache_ttl = 3600
```

### 模板继承

为通用元素创建基础模板：

```handlebars
{{!-- templates/base/html.hbs --}}
<!DOCTYPE html>
<html>
<head>
  <style>
    {{> styles}}
  </style>
</head>
<body>
  <div class="email-wrapper">
    {{> header}}
    
    <div class="content">
      {{{body}}}
    </div>
    
    {{> footer}}
  </div>
</body>
</html>
```

使用部分模板：

```handlebars
{{!-- templates/order_confirmation/html.hbs --}}
{{#> base}}
  {{#*inline "body"}}
    <h1>Thank you for your order!</h1>
    <p>Your order #{{order.order_number}} has been confirmed.</p>
  {{/inline}}
{{/base}}
```

## 测试模板

### CLI 测试

使用示例数据测试模板：

```bash
-- 使用默认示例数据测试
rcommerce template test order_confirmation

-- 使用自定义数据文件测试
rcommerce template test order_confirmation --data test_order.json

-- 输出到文件
rcommerce template test order_confirmation --output test_email.html

-- 发送测试邮件
rcommerce template test order_confirmation --send-to admin@example.com
```

### 示例数据文件

```json
{
  "order": {
    "order_number": "1001",
    "status": "confirmed",
    "total": "99.99",
    "currency": "USD",
    "customer": {
      "first_name": "John",
      "last_name": "Doe",
      "email": "john@example.com"
    },
    "line_items": [
      {
        "name": "Premium T-Shirt",
        "quantity": 2,
        "price": "49.99",
        "total": "99.98"
      }
    ]
  },
  "store": {
    "name": "My Store",
    "url": "https://mystore.com"
  }
}
```

### 预览服务器

运行本地预览服务器：

```bash
-- 启动预览服务器
rcommerce template preview

-- 访问 http://localhost:3001/preview/order_confirmation
```

### 邮件测试工具

测试邮件渲染：

```bash
-- 发送到 Litmus/Email on Acid
rcommerce template test order_confirmation --litmus

-- 检查垃圾邮件评分
rcommerce template test order_confirmation --spam-check

-- 验证 HTML
rcommerce template test order_confirmation --validate
```

## 模板辅助函数

### 内置辅助函数

```handlebars
{{!-- 格式化货币 --}}
{{formatCurrency 99.99 "USD"}}  {{!-- $99.99 --}}

{{!-- 格式化日期 --}}
{{formatDate order.created_at "MMM DD, YYYY"}}  {{!-- Jan 15, 2024 --}}

{{!-- 条件 --}}
{{#if order.discount_total}}
  Discount: {{formatCurrency order.discount_total order.currency}}
{{/if}}

{{!-- 带索引的 each --}}
{{#each order.line_items}}
  {{@index}}. {{this.name}}
{{/each}}

{{!-- 比较 --}}
{{#eq order.status "completed"}}
  Your order is complete!
{{/eq}}

{{!-- Unless（反向 if） --}}
{{#unless order.paid}}
  Payment pending
{{/unless}}
```

### 自定义辅助函数

在配置中注册自定义辅助函数：

```toml
[notifications.email.helpers]
-- 定义自定义辅助函数
loyalty_tier = "{{#if (gte customer.total_spent 1000)}}Gold{{else}}Silver{{/if}}"
```

## 本地化

### 多语言模板

创建特定语言的模板：

```
templates/
├── order_confirmation/
│   ├── en/
│   │   ├── subject.hbs
│   │   ├── html.hbs
│   │   └── text.hbs
│   ├── de/
│   │   ├── subject.hbs
│   │   ├── html.hbs
│   │   └── text.hbs
│   └── zh/
│       ├── subject.hbs
│       ├── html.hbs
│       └── text.hbs
```

### 语言选择

语言由以下因素决定：

1. 客户的首选语言
2. 订单的区域设置
3. 商店默认语言

```toml
[notifications.email]
default_language = "en"
supported_languages = ["en", "de", "zh", "fr", "es"]
```

## 最佳实践

### 邮件设计

1. **使用内联样式** - 许多客户端阻止 `<style>` 标签
2. **基于表格的布局** - 更好的邮件客户端支持
3. **最大宽度 600px** - 标准邮件宽度
4. **图片的替代文本** - 可访问性和图片阻止
5. **纯文本版本** - 始终包含文本模板

### 模板维护

1. **版本控制** - 在 Git 中跟踪模板更改
2. **部署前测试** - 始终测试模板更改
3. **监控送达率** - 跟踪退信/垃圾邮件率
4. **A/B 测试** - 测试不同的模板版本

### 安全

1. **转义变量** - 防止邮件中的 XSS
2. **验证 URL** - 确保所有链接有效
3. **不包含敏感数据** - 不要包含密码或令牌

## 故障排除

### 模板未找到

```
ERROR: Template 'order_confirmation' not found
```

**解决方案：**

1. 检查模板目录路径
2. 验证模板文件存在
3. 检查文件权限

### 变量未渲染

```
{{order.unknown_field}} 渲染为空
```

**解决方案：**

1. 检查文档中的可用变量
2. 使用 `{{log order}}` 调试
3. 验证数据是否传递给模板

### 邮件渲染问题

**解决方案：**

1. 使用多个邮件客户端测试
2. 使用邮件测试服务（Litmus、Email on Acid）
3. 检查内联 CSS
4. 验证 HTML

## 相关文档

- [通知指南](../guides/notifications.md)
- [催缴管理](../guides/dunning.md)
- [配置](../getting-started/configuration.md)
