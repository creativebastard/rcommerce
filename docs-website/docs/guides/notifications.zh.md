# 通知设置指南

本指南介绍如何在 R Commerce 中配置邮件和短信通知，包括设置模板和测试您的通知系统。

## 概述

R Commerce 提供全面的通知系统，支持：

- **邮件通知**：SMTP、SendGrid、AWS SES、Mailgun
- **短信通知**：Twilio、AWS SNS
- **Webhook 通知**：自定义集成的 HTTP 回调
- **模板系统**：支持变量替换的 HTML 和文本模板
- **多语言支持**：本地化通知

## 步骤 1：配置邮件提供商

### SMTP 配置

对于大多数用例，SMTP 是最简单的选项：

```toml
[notifications.email]
provider = "smtp"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.smtp]
host = "smtp.gmail.com"
port = 587
username = "your-email@gmail.com"
password = "your-app-password"
encryption = "starttls"  # 选项：none, ssl, starttls
```

### SendGrid 配置

对于高容量发送，推荐使用 SendGrid：

```toml
[notifications.email]
provider = "sendgrid"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.sendgrid]
api_key = "SG.your-api-key"
```

### AWS SES 配置

对于 AWS 用户，SES 提供经济高效的邮件投递：

```toml
[notifications.email]
provider = "ses"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.ses]
access_key_id = "AKIA..."
secret_access_key = "..."
region = "us-east-1"
```

### Mailgun 配置

```toml
[notifications.email]
provider = "mailgun"
enabled = true
from_address = "noreply@yourstore.com"
from_name = "Your Store"

[notifications.email.mailgun]
api_key = "key-..."
domain = "mg.yourstore.com"
```

## 步骤 2：配置短信提供商（可选）

### Twilio 配置

```toml
[notifications.sms]
provider = "twilio"
enabled = true
from_number = "+1234567890"

[notifications.sms.twilio]
account_sid = "AC..."
auth_token = "..."
```

### AWS SNS 配置

```toml
[notifications.sms]
provider = "sns"
enabled = true

[notifications.sms.sns]
access_key_id = "AKIA..."
secret_access_key = "..."
region = "us-east-1"
sender_id = "YourStore"
```

## 步骤 3：设置邮件模板

### 模板目录结构

为您的模板创建以下目录结构：

```
templates/
├── emails/
│   ├── order/
│   │   ├── confirmation.html
│   │   ├── confirmation.txt
│   │   ├── shipped.html
│   │   ├── shipped.txt
│   │   ├── delivered.html
│   │   └── delivered.txt
│   ├── customer/
│   │   ├── welcome.html
│   │   ├── welcome.txt
│   │   ├── password_reset.html
│   │   └── password_reset.txt
│   ├── payment/
│   │   ├── receipt.html
│   │   ├── receipt.txt
│   │   ├── failed.html
│   │   └── failed.txt
│   └── subscription/
│       ├── created.html
│       ├── created.txt
│       ├── renewal_reminder.html
│       └── renewal_reminder.txt
└── sms/
    ├── order_shipped.txt
    └── payment_failed.txt
```

### 订单确认邮件模板

```html
<!-- templates/emails/order/confirmation.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>订单确认 - {{order_number}}</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 600px;
            margin: 0 auto;
            padding: 20px;
        }
        .header {
            background: #007bff;
            color: white;
            padding: 20px;
            text-align: center;
        }
        .content {
            background: #f8f9fa;
            padding: 20px;
            margin: 20px 0;
        }
        .order-details {
            background: white;
            padding: 15px;
            margin: 15px 0;
            border: 1px solid #dee2e6;
        }
        .item {
            padding: 10px 0;
            border-bottom: 1px solid #eee;
        }
        .totals {
            text-align: right;
            margin-top: 15px;
        }
        .button {
            display: inline-block;
            background: #007bff;
            color: white;
            padding: 12px 24px;
            text-decoration: none;
            border-radius: 4px;
            margin: 20px 0;
        }
        .footer {
            text-align: center;
            color: #6c757d;
            font-size: 12px;
            margin-top: 30px;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>感谢您的订单！</h1>
    </div>
    
    <div class="content">
        <p>Hi {{customer_name}},</p>
        
        <p>我们已收到您的订单，正在准备发货。</p>
        
        <div class="order-details">
            <h2>订单 #{{order_number}}</h2>
            <p><strong>订单日期：</strong> {{order_date}}<br>
            <strong>状态：</strong> {{order_status}}</p>
            
            <h3>订购商品</h3>
            {{#each items}}
            <div class="item">
                <strong>{{name}}</strong><br>
                数量：{{quantity}} × {{price}} = {{total}}
            </div>
            {{/each}}
            
            <div class="totals">
                <p>小计：{{subtotal}}<br>
                运费：{{shipping_cost}}<br>
                税费：{{tax}}<br>
                <strong>总计：{{total}}</strong></p>
            </div>
        </div>
        
        <div class="order-details">
            <h3>配送地址</h3>
            <p>{{shipping_address.name}}<br>
            {{shipping_address.line1}}<br>
            {{#if shipping_address.line2}}{{shipping_address.line2}}<br>{{/if}}
            {{shipping_address.city}}, {{shipping_address.state}} {{shipping_address.zip}}<br>
            {{shipping_address.country}}</p>
        </div>
        
        <p style="text-align: center;">
            <a href="{{order_tracking_url}}" class="button">追踪您的订单</a>
        </p>
        
        <p>订单发货后，我们会再发送一封邮件通知您。</p>
        
        <p>有问题？联系我们：<a href="mailto:{{support_email}}">{{support_email}}</a></p>
    </div>
    
    <div class="footer">
        <p>{{company_name}}<br>
        {{company_address}}<br>
        <a href="{{unsubscribe_url}}">取消订阅</a></p>
    </div>
</body>
</html>
```

### 纯文本版本

```
<!-- templates/emails/order/confirmation.txt -->
感谢您的订单！

Hi {{customer_name}},

我们已收到您的订单，正在准备发货。

订单 #{{order_number}}
订单日期：{{order_date}}
状态：{{order_status}}

订购商品：
{{#each items}}
- {{name}}
  数量：{{quantity}} × {{price}} = {{total}}
{{/each}}

小计：{{subtotal}}
运费：{{shipping_cost}}
税费：{{tax}}
总计：{{total}}

配送地址：
{{shipping_address.name}}
{{shipping_address.line1}}
{{#if shipping_address.line2}}{{shipping_address.line2}}{{/if}}
{{shipping_address.city}}, {{shipping_address.state}} {{shipping_address.zip}}
{{shipping_address.country}}

追踪您的订单：{{order_tracking_url}}

订单发货后，我们会再发送一封邮件通知您。

有问题？联系我们：{{support_email}}

---
{{company_name}}
{{company_address}}
取消订阅：{{unsubscribe_url}}
```

## 步骤 4：配置通知事件

定义哪些事件触发通知：

```toml
[notifications.events]

# 订单事件
order_created = ["email"]
order_paid = ["email"]
order_shipped = ["email", "sms"]
order_delivered = ["email"]
order_cancelled = ["email"]
order_refunded = ["email"]

# 客户事件
customer_registered = ["email"]
customer_password_reset = ["email"]
customer_welcome = ["email"]

# 支付事件
payment_success = ["email"]
payment_failed = ["email", "sms"]
payment_refunded = ["email"]

# 订阅事件
subscription_created = ["email"]
subscription_renewal_reminder = ["email"]
subscription_payment_failed = ["email", "sms"]
subscription_cancelled = ["email"]

# 运输事件
shipping_label_created = ["email"]
shipping_exception = ["email"]
```

## 步骤 5：测试您的通知

### 使用 CLI

```bash
# 测试邮件配置
rcommerce notifications test-email \
  --to "test@example.com" \
  --template "order/confirmation"

# 测试短信配置
rcommerce notifications test-sms \
  --to "+1234567890" \
  --template "order_shipped"

# 发送测试订单确认
rcommerce notifications send-test \
  --event order_created \
  --to "test@example.com"
```

### 测试检查清单

上线前测试以下场景：

- [ ] 订单确认邮件渲染正确
- [ ] 订单发货邮件包含追踪链接
- [ ] 密码重置邮件端到端正常工作
- [ ] 欢迎邮件在注册时触发
- [ ] 付款收据包含正确的总计
- [ ] 短信通知已收到（如果启用）
- [ ] 取消订阅链接可用
- [ ] 邮件打开和点击被追踪（如果启用）

## 步骤 6：配置高级选项

### 邮件追踪

```toml
[notifications.email.tracking]
enabled = true
track_opens = true
track_clicks = true
webhook_url = "https://yourstore.com/webhooks/email-tracking"
```

### 退信处理

```toml
[notifications.email.bounce_handling]
enabled = true
soft_bounce_threshold = 3
auto_unsubscribe_hard_bounces = true
notification_email = "admin@yourstore.com"
```

### 速率限制

```toml
[notifications.rate_limiting]
enabled = true
max_emails_per_hour = 1000
max_sms_per_hour = 100
burst_allowance = 100
```

### 重试逻辑

```toml
[notifications.retry]
enabled = true
max_retries = 3
retry_delays = [60, 300, 900]  # 秒
```

## 步骤 7：设置 Webhook 通知

对于自定义集成，配置 webhook 通知：

```toml
[notifications.webhooks]
enabled = true

[[notifications.webhooks.endpoints]]
url = "https://your-crm.com/webhooks/rcommerce"
events = ["order.created", "customer.registered"]
secret = "your-webhook-secret"
headers = { "X-Custom-Header" = "value" }

[[notifications.webhooks.endpoints]]
url = "https://your-analytics.com/events"
events = ["*"]  # 所有事件
secret = "another-secret"
```

## 模板变量参考

### 订单变量

| 变量 | 描述 | 示例 |
|----------|-------------|---------|
| `{{order_number}}` | 订单号 | "ORD-2026-001234" |
| `{{order_date}}` | 订单创建日期 | "January 15, 2026" |
| `{{order_status}}` | 当前订单状态 | "Processing" |
| `{{subtotal}}` | 订单小计 | "$99.99" |
| `{{shipping_cost}}` | 运费 | "$10.00" |
| `{{tax}}` | 税费 | "$8.50" |
| `{{total}}` | 订单总计 | "$118.49" |
| `{{items}}` | 订单商品数组 | 见模板 |
| `{{shipping_address}}` | 配送地址对象 | 见模板 |
| `{{billing_address}}` | 账单地址对象 | 见模板 |

### 客户变量

| 变量 | 描述 | 示例 |
|----------|-------------|---------|
| `{{customer_name}}` | 客户全名 | "John Doe" |
| `{{customer_first_name}}` | 客户名字 | "John" |
| `{{customer_email}}` | 客户邮箱 | "john@example.com" |
| `{{customer_id}}` | 客户 ID | "550e8400..." |

### 商店变量

| 变量 | 描述 | 示例 |
|----------|-------------|---------|
| `{{company_name}}` | 商店名称 | "Acme Inc" |
| `{{company_address}}` | 商店地址 | "123 Main St..." |
| `{{support_email}}` | 支持邮箱 | "support@store.com" |
| `{{store_url}}` | 商店 URL | "https://store.com" |
| `{{logo_url}}` | Logo URL | "https://store.com/logo.png" |

### 链接变量

| 变量 | 描述 |
|----------|-------------|
| `{{order_tracking_url}}` | 订单追踪页面 URL |
| `{{account_url}}` | 客户账户页面 URL |
| `{{password_reset_url}}` | 密码重置 URL |
| `{{unsubscribe_url}}` | 邮件取消订阅 URL |

## 最佳实践

### 1. 同时使用 HTML 和文本模板

始终提供 HTML 和纯文本版本：

- HTML 用于丰富的格式和品牌展示
- 文本用于可访问性和不支持 HTML 的邮件客户端

### 2. 保持模板简洁

- 使用内联 CSS（许多邮件客户端阻止外部样式表）
- 在移动设备上测试
- 保持邮件宽度在 600px 以下
- 使用网络安全字体

### 3. 个性化内容

使用客户数据进行个性化：

```html
<p>Hi {{customer_first_name}},</p>
<p>感谢您自 {{customer_since}} 以来一直是我们的客户！</p>
```

### 4. 包含清晰的 CTA

使操作按钮醒目：

```html
<a href="{{order_tracking_url}}" 
   style="background: #007bff; color: white; padding: 12px 24px; 
          text-decoration: none; border-radius: 4px; display: inline-block;">
   追踪您的订单
</a>
```

### 5. 监控可送达性

追踪这些指标：

| 指标 | 良好 | 差 |
|--------|------|------|
| 投递率 | >95% | <90% |
| 打开率 | >20% | <10% |
| 点击率 | >3% | <1% |
| 退信率 | <2% | >5% |
| 垃圾邮件投诉率 | <0.1% | >0.5% |

### 6. 处理取消订阅

始终包含取消订阅链接并及时处理：

```html
<p style="font-size: 12px; color: #6c757d;">
  您收到此邮件是因为您在 {{company_name}} 进行了购买。
  <a href="{{unsubscribe_url}}">取消订阅</a>
</p>
```

## 故障排除

### 邮件未发送

**检查清单：**
- [ ] 邮件提供商凭证正确
- [ ] 配置中 `enabled = true`
- [ ] 模板文件存在且可读
- [ ] 模板中没有语法错误
- [ ] 未超出速率限制

**调试命令：**

```bash
# 检查邮件配置
rcommerce config get notifications.email

# 查看通知日志
rcommerce logs --category notifications

# 使用详细输出测试
rcommerce notifications test-email --to "test@example.com" --verbose
```

### 邮件进入垃圾邮件

**解决方案：**
1. 设置 SPF、DKIM 和 DMARC 记录
2. 使用专用发送域名
3. 逐步预热您的 IP 地址
4. 监控发件人声誉
5. 保持投诉率低

### 短信未送达

**检查清单：**
- [ ] 电话号码格式正确（E.164）
- [ ] 短信提供商账户有余额
- [ ] 消息长度在运营商限制内
- [ ] 不在运营商黑名单中

## 下一步

- [配置催缴邮件](../guides/dunning.md)
- [设置运输通知](../guides/shipping.md)
- [API 参考：Webhooks](../api-reference/webhooks.md)
- [自定义邮件模板](../development/custom-templates.md)
