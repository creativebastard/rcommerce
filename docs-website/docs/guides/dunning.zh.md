# 催缴管理指南

本指南涵盖在 R Commerce 中配置和使用催缴系统进行失败支付恢复所需了解的一切内容。

## 催缴管理简介

**催缴管理**是自动重试失败的订阅付款并与客户沟通以恢复收入的过程。当客户付款失败时，催缴系统会接管以：

1. **在战略间隔重试付款**
2. **向客户发送**有用的提醒邮件
3. **在宽限期内保持订阅活跃**
4. **仅在最后手段时取消**当所有恢复尝试失败时

### 催缴管理的重要性

失败的付款是订阅业务收入损失的主要来源：

- 每月有 **15-20%** 的订阅付款失败
- 通过适当的催缴管理可以恢复 **60-80%** 的失败付款
- **非自愿流失**（付款失败）通常超过主动取消
- 每次恢复的付款都保留了**客户生命周期价值**

### R Commerce 催缴管理的工作原理

```
付款失败
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  第 0 天：首次失败                                          │
│  • 订阅状态 → 逾期                                          │
│  • 发送邮件："付款失败 - 请更新"                            │
│  • 下次重试计划：第 +1 天                                   │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  第 1 天：重试 #1（静默）                                   │
│  • 使用存储的付款方式自动重试                               │
│  • 如果成功 → 订阅恢复为活跃状态                            │
│  • 如果失败 → 继续下次重试                                  │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  第 4 天：重试 #2                                           │
│  • 发送邮件："付款再次失败 - 需要操作"                      │
│  • 下次重试计划：第 +7 天                                   │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  第 11 天：重试 #3（最终）                                  │
│  • 发送邮件："最终通知：即将取消"                           │
│  • 这是最后一次尝试                                         │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  第 11 天+：取消                                            │
│  • 订阅被取消                                               │
│  • 发送邮件："订阅已取消"                                   │
│  • 客户可以随时重新激活                                     │
└─────────────────────────────────────────────────────────────┘
```

## 配置催缴设置

### 基本配置

将催缴配置添加到您的 `config.toml`：

```toml
[dunning]
# 取消前重试次数（默认：3）
max_retries = 3

# 重试间隔天数（默认：[1, 3, 7]）
retry_intervals_days = [1, 3, 7]

# 宽限期天数（默认：14）
grace_period_days = 14

# 首次失败时发送邮件（默认：true）
email_on_first_failure = true

# 最终失败时发送邮件（默认：true）
email_on_final_failure = true
```

### 配置选项说明

| 选项 | 描述 | 默认值 | 推荐值 |
|--------|-------------|---------|-------------|
| `max_retries` | 付款重试尝试次数 | 3 | 3-5 |
| `retry_intervals_days` | 每次重试之间的等待天数 | [1, 3, 7] | [1, 3, 7] 或 [3, 7, 14] |
| `grace_period_days` | 催缴期间订阅保持活跃的天数 | 14 | 14-30 |
| `email_on_first_failure` | 首次失败时立即发送邮件 | true | true |
| `email_on_final_failure` | 取消前发送最终警告 | true | true |

### 重试计划示例

#### 标准计划（默认）
```toml
[dunning]
max_retries = 3
retry_intervals_days = [1, 3, 7]
```
- 重试 1：首次失败后 1 天
- 重试 2：重试 1 后 3 天（共 4 天）
- 重试 3：重试 2 后 7 天（共 11 天）

#### 积极恢复
```toml
[dunning]
max_retries = 5
retry_intervals_days = [1, 2, 3, 5, 7]
grace_period_days = 18
```
适合利润紧张或竞争激烈的企业进行更快的重试。

#### 温和恢复
```toml
[dunning]
max_retries = 3
retry_intervals_days = [3, 7, 14]
grace_period_days = 24
```
适合高端服务或 B2B 订阅的较慢计划。

#### 最小恢复
```toml
[dunning]
max_retries = 2
retry_intervals_days = [3, 7]
grace_period_days = 10
```
适合低价值订阅或免费层的较少尝试。

### 滞纳金（可选）

您可以为逾期付款收取滞纳金：

```toml
[dunning]
# 在第 2 次重试后收取滞纳金
late_fee_after_retry = 2

# 滞纳金金额
late_fee_amount = "5.00"
```

**重要考虑事项：**
- 滞纳金可能增加客户流失
- 实施前检查当地法规
- 在服务条款中明确沟通滞纳金政策
- 考虑对首次失败免除滞纳金

## 设置催缴邮件

### 默认邮件模板

R Commerce 包含为每个催缴阶段专业编写的邮件模板：

| 阶段 | 邮件类型 | 目的 |
|-------|------------|---------|
| 第 0 天 | 首次失败 | 友好提醒更新付款方式 |
| 第 4 天 | 重试失败 | 带有尝试次数的紧急通知 |
| 第 11 天 | 最终通知 | 取消前的最后警告 |
| 第 11 天+ | 取消通知 | 带有重新激活链接的专业结束 |
| 任意 | 付款恢复 | 成功确认 |

### 自定义邮件模板

邮件模板存储在 `templates/dunning/` 目录中：

```
templates/
└── dunning/
    ├── first_failure.html
    ├── first_failure.txt
    ├── retry_failure.html
    ├── retry_failure.txt
    ├── final_notice.html
    ├── final_notice.txt
    ├── cancellation_notice.html
    ├── cancellation_notice.txt
    ├── payment_recovered.html
    └── payment_recovered.txt
```

### 可用模板变量

在自定义模板中使用这些变量：

| 变量 | 描述 | 示例 |
|----------|-------------|---------|
| `{{customer_name}}` | 客户名字 | "John" |
| `{{subscription_id}}` | 订阅 ID | "550e8400-e29b-41d4-a716-446655440000" |
| `{{product_name}}` | 订阅产品名称 | "Premium Coffee Subscription" |
| `{{amount}}` | 应付金额 | "$29.99" |
| `{{currency}}` | 货币代码 | "USD" |
| `{{attempt_number}}` | 当前重试次数 | "2" |
| `{{max_attempts}}` | 最大重试次数 | "3" |
| `{{next_retry_date}}` | 下次计划重试 | "January 15, 2026" |
| `{{grace_period_end}}` | 宽限期结束 | "January 25, 2026" |
| `{{update_payment_url}}` | 付款更新链接 | "https://shop.example.com/payment/update" |
| `{{account_url}}` | 客户账户门户 | "https://shop.example.com/account" |
| `{{support_url}}` | 支持联系页面 | "https://shop.example.com/support" |
| `{{company_name}}` | 您的企业名称 | "Acme Inc" |

### 自定义模板示例

```html
<!-- templates/dunning/first_failure.html -->
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .button { background: #007bff; color: white; padding: 12px 24px; 
                  text-decoration: none; border-radius: 4px; display: inline-block; }
        .alert { background: #fff3cd; border: 1px solid #ffeaa7; 
                 padding: 15px; border-radius: 4px; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Hi {{customer_name}},</h1>
        
        <div class="alert">
            <strong>我们无法处理您的付款</strong>
        </div>
        
        <p>您的 <strong>{{product_name}}</strong> 订阅仍然有效，
        但我们需要您更新付款方式。</p>
        
        <p><strong>应付金额：</strong> {{amount}}<br>
        <strong>下次重试：</strong> {{next_retry_date}}</p>
        
        <p style="text-align: center; margin: 30px 0;">
            <a href="{{update_payment_url}}" class="button">更新付款方式</a>
        </p>
        
        <p>付款失败的常见原因：</p>
        <ul>
            <li>信用卡已过期</li>
            <li>资金不足</li>
            <li>银行安全拦截</li>
        </ul>
        
        <p>有问题？<a href="{{support_url}}">联系我们的支持团队</a>。</p>
        
        <p>谢谢，<br>{{company_name}}</p>
    </div>
</body>
</html>
```

### 邮件最佳实践

**应该：**
- ✅ 保持主题行清晰且可操作
- ✅ 包含醒目的行动号召按钮
- ✅ 解释付款失败的原因（当已知时）
- ✅ 保持有帮助、非威胁的语气
- ✅ 提供多种联系选项
- ✅ 在移动设备上测试邮件

**不应该：**
- ❌ 使用攻击性或威胁性语言
- ❌ 隐藏付款更新链接
- ❌ 发送过多邮件
- ❌ 使用全大写或过多标点符号
- ❌ 包含不必要的技术细节

## 监控失败付款

### 仪表板概览

在 `/admin/dunning` 访问催缴仪表板：

```
┌─────────────────────────────────────────────────────────────────────┐
│ 催缴管理仪表板                                                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  恢复指标（最近 30 天）                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   恢复率     │  │   已恢复     │  │    平均      │              │
│  │              │  │   收入       │  │   尝试次数   │              │
│  │    72.5%     │  │  $24,500     │  │    1.8       │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                                                                     │
│  活跃催缴案例（12）                                                  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ 客户            │ 产品                 │ 重试  │ 下次重试   │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │ john@email.com  │ Premium Plan         │ 1/3   │ 今天       │  │
│  │ jane@email.com  │ Basic Subscription   │ 2/3   │ 明天       │  │
│  │ bob@email.com   │ Enterprise Plan      │ 3/3   │ 2 天后     │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
│  按尝试次数的恢复情况                                                │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │ 尝试 1: ████████████████████ 45%                            │  │
│  │ 尝试 2: ██████████████ 28%                                  │  │
│  │ 尝试 3: ████████ 15%                                        │  │
│  │ 已取消: ██████ 12%                                          │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### API 端点

#### 列出失败付款

```bash
GET /api/v1/admin/dunning/failed-payments
```

**响应：**
```json
{
  "data": [
    {
      "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
      "customer": {
        "id": "550e8400-e29b-41d4-a716-446655440001",
        "email": "customer@example.com",
        "name": "John Doe"
      },
      "product_name": "Premium Subscription",
      "amount": "29.99",
      "currency": "USD",
      "failed_attempts": 2,
      "max_attempts": 3,
      "next_retry_at": "2026-01-15T10:00:00Z",
      "status": "past_due",
      "first_failed_at": "2026-01-10T08:30:00Z"
    }
  ],
  "meta": {
    "total": 12,
    "page": 1,
    "per_page": 20
  }
}
```

#### 获取催缴指标

```bash
GET /api/v1/admin/dunning/metrics?period=30d
```

**响应：**
```json
{
  "period": "30d",
  "total_failures": 156,
  "total_recoveries": 113,
  "recovery_rate": 72.44,
  "recovered_revenue": "24500.00",
  "lost_revenue": "3200.00",
  "recovery_by_attempt": [
    { "attempt": 1, "recoveries": 70, "rate": 44.87 },
    { "attempt": 2, "recoveries": 28, "rate": 17.95 },
    { "attempt": 3, "recoveries": 15, "rate": 9.62 }
  ],
  "average_recovery_time_hours": 72.5
}
```

#### 获取订阅催缴历史

```bash
GET /api/v1/subscriptions/{id}/dunning-history
```

**响应：**
```json
{
  "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "active",
  "retry_attempts": [
    {
      "attempt_number": 1,
      "attempted_at": "2026-01-10T08:30:00Z",
      "succeeded": false,
      "error_message": "Card declined",
      "error_code": "insufficient_funds"
    },
    {
      "attempt_number": 2,
      "attempted_at": "2026-01-13T08:30:00Z",
      "succeeded": true,
      "payment_id": "pi_1234567890"
    }
  ],
  "emails_sent": [
    {
      "type": "first_failure",
      "sent_at": "2026-01-10T08:30:00Z",
      "opened_at": "2026-01-10T09:15:00Z",
      "clicked_at": "2026-01-10T09:16:00Z"
    }
  ]
}
```

### Webhook 事件

订阅催缴相关的 webhooks：

```json
{
  "event": "dunning.payment_failed",
  "data": {
    "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "invoice_id": "550e8400-e29b-41d4-a716-446655440002",
    "attempt_number": 1,
    "max_attempts": 3,
    "next_retry_at": "2026-01-15T10:00:00Z",
    "error_message": "Card declined",
    "amount": "29.99",
    "currency": "USD"
  }
}
```

```json
{
  "event": "dunning.payment_recovered",
  "data": {
    "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "invoice_id": "550e8400-e29b-41d4-a716-446655440002",
    "attempt_number": 2,
    "payment_id": "pi_1234567890",
    "amount": "29.99",
    "currency": "USD"
  }
}
```

```json
{
  "event": "dunning.subscription_cancelled",
  "data": {
    "subscription_id": "550e8400-e29b-41d4-a716-446655440000",
    "customer_id": "550e8400-e29b-41d4-a716-446655440001",
    "reason": "payment_failed",
    "total_attempts": 3,
    "cancelled_at": "2026-01-20T10:00:00Z"
  }
}
```

## 付款恢复最佳实践

### 1. 优化重试计划

**测试不同的计划**以找到适合您业务的方案：

| 业务类型 | 推荐计划 |
|--------------|---------------------|
| SaaS / 软件 | [1, 3, 7] - 标准 |
| 实体商品 | [3, 7, 14] - 给客户更多时间 |
| 高价值 B2B | [3, 7, 14, 21] - 更多尝试的延长计划 |
| 低价值 / 免费增值 | [1, 3] - 快速恢复或取消 |

### 2. 简化付款更新

**减少付款更新过程中的摩擦：**

```
✅ 应该：
• 邮件中一键付款更新链接
• 预填充客户信息
• 支持多种付款方式
• 移动优化的付款表单
• 保存付款方式供将来使用

❌ 不应该：
• 要求登录才能更新付款
• 通过多个页面重定向
• 询问您已有的信息
• 使用过期或损坏的链接
```

### 3. 分段方法

**针对不同客户的不同策略：**

```toml
# 高价值客户获得延长宽限期
[dunning.segments.premium]
max_retries = 5
retry_intervals_days = [3, 7, 14, 21, 30]
grace_period_days = 45

# 标准客户使用默认值
[dunning.segments.standard]
max_retries = 3
retry_intervals_days = [1, 3, 7]
grace_period_days = 14

# 低价值或试用客户
[dunning.segments.basic]
max_retries = 2
retry_intervals_days = [1, 3]
grace_period_days = 7
```

### 4. 监控和响应

**需要跟踪的关键指标：**

| 指标 | 目标 | 低于目标时的行动 |
|--------|--------|----------------------|
| 恢复率 | >65% | 审查邮件模板 |
| 首次尝试恢复率 | >40% | 优化重试时间 |
| 邮件打开率 | >50% | 改进主题行 |
| 点击率 | >15% | 更好的行动号召 |

### 5. 主动预防

**在失败发生前减少失败：**

1. **过期提醒**
   ```
   在卡片过期前 30 天邮件客户
   包含更新付款方式的直接链接
   ```

2. **预催缴邮件**
   ```
   账单前 3 天："您的订阅即将续订"
   包含金额和存档的付款方式
   ```

3. **账户更新服务**
   ```
   与 Visa/Mastercard 账户更新集成
   自动更新过期的卡号
   ```

### 6. 客户沟通

**催缴过程中的语气递进：**

| 阶段 | 语气 | 示例开头 |
|-------|------|-----------------|
| 首次失败 | 友好、有帮助 | "别担心，这种情况很常见..." |
| 重试失败 | 紧急、礼貌 | "我们需要您的关注..." |
| 最终通知 | 严肃、明确 | "需要操作以保持您的订阅..." |
| 取消 | 专业、开放 | "我们期待您的回归..." |

### 7. 人工干预

**何时进行人工干预：**

- 高价值客户（企业账户）
- 长期客户（2 年以上）
- 联系支持的客户
- 技术故障（非付款问题）

**催缴的 CLI 命令：**

```bash
# 查看订阅催缴状态
rcommerce subscription dunning-status <subscription-id>

# 手动触发重试
rcommerce subscription retry-payment <subscription-id>

# 延长宽限期
rcommerce subscription extend-grace <subscription-id> --days 7 --reason "Customer contacted support"

# 立即取消订阅
rcommerce subscription cancel <subscription-id> --reason "payment_failed" --immediate
```

## 催缴工作流程示例

### 场景：月度订阅失败

**背景：**
- 客户：Sarah Johnson
- 订阅：Premium Plan（$49/月）
- 付款方式：Visa 尾号 4242
- 账单日期：每月 1 日

**时间线：**

**2026 年 2 月 1 日 - 初始账单失败**
```
08:00: 自动账单尝试失败
08:05: 催缴系统处理失败
08:10: 订阅状态 → 逾期
08:15: 向 Sarah 发送首次失败邮件
08:15: 下次重试计划为 2 月 2 日
```

**2026 年 2 月 1 日 - 收到邮件**
```
主题：付款失败 - 请更新您的付款方式

Hi Sarah,

我们无法处理您的 Premium Plan 订阅付款。

应付金额：$49.00
下次重试：2026 年 2 月 2 日

别担心 - 您的订阅仍然有效！
请更新您的付款方式：

[更新付款方式]
```

**2026 年 2 月 2 日 - 重试 #1（静默）**
```
08:00: 使用同一张卡自动重试
08:01: 卡片再次被拒绝（资金不足）
08:05: 下次重试计划为 2 月 5 日
```

**2026 年 2 月 5 日 - 重试 #2**
```
08:00: 自动重试
08:01: 卡片再次被拒绝
08:05: 发送重试失败邮件
08:05: 下次重试计划为 2 月 12 日
```

**2026 年 2 月 5 日 - 收到邮件**
```
主题：付款再次失败 - 需要操作

Hi Sarah,

您的 Premium Plan 付款再次失败（尝试 2/3）。

应付金额：$49.00
下次重试：2026 年 2 月 12 日

要保持您的订阅活跃，请更新您的付款方式：

[立即更新付款方式]
```

**2026 年 2 月 10 日 - 客户操作**
```
14:30: Sarah 点击邮件链接
14:35: 更新为新的 Mastercard
14:40: 付款成功处理
14:45: 订阅恢复为活跃状态
14:50: 发送付款恢复邮件
```

**2026 年 2 月 10 日 - 成功邮件**
```
主题：付款成功 - 订阅已激活 ✓

Hi Sarah,

好消息！您的付款已成功处理。

已扣款金额：$49.00
下次账单：2026 年 3 月 1 日

您的 Premium Plan 订阅现已激活！
```

**结果：** 在第 2 次重试时恢复付款。总恢复时间：9 天。

## 常见问题

### 一般问题

**Q：什么是催缴管理？**

A：催缴管理是自动重试失败的订阅付款并与客户沟通以恢复收入的过程。它包括计划重试、邮件通知以及当恢复失败时的最终取消。

**Q：催缴管理可以恢复多少收入？**

A：通过适当的配置，催缴管理通常可以恢复 60-80% 的失败订阅付款。恢复率因行业、客户群和催缴策略而异。

**Q：催缴管理是自动的吗？**

A：是的，一旦配置，整个催缴过程都是自动化的。系统无需人工干预即可处理重试、邮件和取消。

### 配置问题

**Q：我可以自定义重试计划吗？**

A：是的，您可以在 `config.toml` 中配置重试次数和间隔：

```toml
[dunning]
max_retries = 3
retry_intervals_days = [1, 3, 7]
```

**Q：推荐的重试计划是什么？**

A：默认计划 `[1, 3, 7]` 适用于大多数企业：
- 重试 1：失败后 1 天（捕获临时问题）
- 重试 2：3 天后（给客户操作时间）
- 重试 3：7 天后（最终尝试）

**Q：我可以禁用催缴管理吗？**

A：虽然不推荐，但您可以通过以下设置有效禁用催缴管理：

```toml
[dunning]
max_retries = 0
```

这将导致在付款失败时立即取消订阅。

### 邮件问题

**Q：我可以自定义催缴邮件吗？**

A：是的，邮件模板存储在 `templates/dunning/` 中，可以使用 HTML 和模板变量进行完全自定义。

**Q：有哪些模板变量可用？**

A：常用变量包括 `{{customer_name}}`、`{{amount}}`、`{{next_retry_date}}`、`{{update_payment_url}}` 等。请参阅[设置催缴邮件](#设置催缴邮件)部分获取完整列表。

**Q：我可以发送短信通知代替邮件吗？**

A：短信通知计划在未来的版本中推出。目前，催缴管理仅支持邮件通知。

### 客户体验问题

**Q：客户在催缴期间会失去访问权限吗？**

A：不会，客户在宽限期内（默认：14 天）保留访问权限。订阅状态变为"逾期"，但在取消前服务继续。

**Q：客户可以在催缴期间更新付款方式吗？**

A：是的，客户可以在催缴过程中的任何时候更新他们的付款方式。下次计划重试将使用新的付款方式。

**Q：如果客户更新他们的卡片会发生什么？**

A：如果客户更新他们的付款方式，下次计划重试将使用新卡。您也可以通过 API 或 CLI 触发立即重试。

### 账单问题

**Q：我可以对逾期付款收取滞纳金吗？**

A：是的，您可以在催缴设置中配置滞纳金：

```toml
[dunning]
late_fee_after_retry = 2
late_fee_amount = "5.00"
```

**Q：滞纳金是添加到原始发票还是新发票？**

A：滞纳金添加到原始发票总额中。当最终处理付款时，它包括订阅金额和任何累积的滞纳金。

**Q：如果催缴需要一段时间，客户会被收取多个月的费用吗？**

A：不会，催缴只尝试收取当前失败的发票。新的账单周期暂停，直到逾期发票解决。

### 恢复问题

**Q：我可以手动重试付款吗？**

A：是的，您可以使用 CLI 手动触发重试：

```bash
rcommerce subscription retry-payment <subscription-id>
```

或通过 API：

```bash
POST /api/v1/subscriptions/{id}/retry-payment
```

**Q：我可以为特定客户延长宽限期吗？**

A：是的，您可以延长宽限期：

```bash
rcommerce subscription extend-grace <subscription-id> --days 7 --reason "Customer contacted support"
```

**Q：所有重试失败后会发生什么？**

A：订阅将以"payment_failed"原因取消。客户会收到取消通知邮件，可以通过下新订单重新激活他们的订阅。

### 分析问题

**Q：如何跟踪催缴绩效？**

A：使用 `/admin/dunning` 的催缴仪表板或指标 API：

```bash
GET /api/v1/admin/dunning/metrics
```

**Q：什么是好的恢复率？**

A：行业基准：
- 优秀：>75%
- 良好：60-75%
- 平均：45-60%
- 需要改进：<45%

**Q：如何提高恢复率？**

A：关键策略：
1. 优化邮件主题行以提高打开率
2. 简化付款更新流程
3. 测试不同的重试计划
4. 按价值/ tenure 细分客户
5. 发送预催缴过期提醒

### 集成问题

**Q：催缴管理与所有支付网关兼容吗？**

A：是的，催缴管理与所有支持的支付网关（Stripe、Airwallex、Alipay、WeChat Pay）兼容。系统使用标准支付网关接口进行重试。

**Q：我可以接收催缴事件的 webhooks 吗？**

A：是的，订阅这些 webhook 事件：
- `dunning.payment_failed`
- `dunning.payment_recovered`
- `dunning.subscription_cancelled`

**Q：催缴管理处理 3D Secure / SCA 吗？**

A：对于需要认证的付款方式，系统会发送带有认证链接的邮件，而不是尝试自动重试。

---

## 相关文档

- [支付](../api-reference/payments.md)
- [支付网关](../payment-gateways/index.md)
- [Webhooks](../api-reference/webhooks.md)
- [CLI 参考](../development/cli-reference.md)
