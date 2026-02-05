# 通知

## 概述

R Commerce 提供灵活的通知系统，支持邮件、短信和基于 Webhook 的通知。

## 通知类型

### 邮件通知

内置邮件模板：
- 订单确认
- 发货通知
- 配送确认
- 密码重置
- 欢迎邮件
- 购物车放弃恢复

### 短信通知

可选短信支持：
- 订单确认
- 发货更新
- 配送通知

### Webhooks

实时 HTTP 回调：
- 订单事件
- 支付事件
- 客户事件
- 库存事件

## 邮件配置

```toml
[notifications.email]
enabled = true
provider = "smtp"  # 或 "sendgrid"、"mailgun"
from_address = "store@example.com"
from_name = "My Store"

[notifications.email.smtp]
host = "smtp.example.com"
port = 587
username = "user"
password = "secret"
secure = true
```

## Webhook 配置

```toml
[[webhooks]]
url = "https://example.com/webhooks/orders"
events = ["order.created", "order.paid", "order.shipped"]
secret = "whsec_xxx"
```

## 模板系统

邮件模板使用 Handlebars：

```handlebars
<!-- order_confirmation.hbs -->
<h1>感谢您的订单，{{customer.first_name}}！</h1>

<p>订单号 #{{order.order_number}}</p>

<ul>
{{#each order.items}}
  <li>{{name}} x {{quantity}} - {{unit_price}}</li>
{{/each}}
</ul>

<p>总计：{{order.total}}</p>
```

## 通知队列

通知被排队以确保可靠投递：
- 后台作业处理
- 失败时重试
- 投递跟踪
- 失败通知警报

## 另请参阅

- [配置指南](../getting-started/configuration.md)
- [Webhooks](../api-reference/webhooks.md)
