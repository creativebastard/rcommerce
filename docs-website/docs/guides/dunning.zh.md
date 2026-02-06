# 催缴管理设置指南

本指南介绍如何在 R Commerce 中配置和优化催缴系统以恢复失败的付款。

## 什么是催缴管理？

**催缴管理**是自动重试失败的订阅付款并与客户沟通以恢复收入的过程。当客户付款失败时，催缴系统会：

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

## 催缴管理的工作原理

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

## 步骤 1：配置催缴设置

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

### 配置选项

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

## 步骤 2：设置催缴邮件

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

## 步骤 3：配置客户细分

不同客户细分可能需要不同的催缴策略：

```toml
# 高价值客户获得延长宽限期
[dunning.segments.premium]
max_retries = 5
retry_intervals_days = [3, 7, 14, 21, 30]
grace_period_days = 45
email_template_prefix = "premium_"

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

通过标签或元数据为客户分配细分：

```bash
# 通过 CLI 设置客户细分
rcommerce customer set-segment <customer-id> --segment premium
```

## 步骤 4：设置付款更新页面

创建一个客户-facing 的页面用于更新付款方式：

### 配置

```toml
[dunning.payment_update]
page_url = "https://shop.example.com/payment/update"
require_login = false  # 允许从邮件通过令牌访问
session_duration_minutes = 30
success_redirect = "https://shop.example.com/account"
```

### 页面要求

付款更新页面应该：
- 预填充客户信息
- 支持多种付款方式
- 针对移动设备优化
- 显示应付金额和重试计划
- 立即确认成功更新

## 步骤 5：监控和优化

### 需要跟踪的关键指标

| 指标 | 目标 | 低于目标时的行动 |
|--------|--------|----------------------|
| 恢复率 | >65% | 审查邮件模板 |
| 首次尝试恢复率 | >40% | 优化重试时间 |
| 邮件打开率 | >50% | 改进主题行 |
| 点击率 | >15% | 更好的行动号召 |

### 使用仪表板

在 `/admin/dunning` 访问催缴仪表板以查看：

- **恢复指标**：恢复率、已恢复收入、平均尝试次数
- **活跃案例**：当前催缴案例及其重试状态
- **按尝试次数的恢复情况**：哪些重试尝试最成功
- **趋势分析**：随时间变化的恢复率趋势

### 用于监控的 CLI 命令

```bash
# 查看催缴指标
rcommerce dunning metrics --period 30d

# 列出活跃催缴案例
rcommerce dunning list --status active

# 获取订阅催缴历史
rcommerce subscription dunning-status <subscription-id>
```

## 步骤 6：人工干预

有时需要对高价值客户进行人工干预：

### 何时进行人工干预

- 高价值客户（企业账户）
- 长期客户（2 年以上）
- 联系支持的客户
- 技术故障（非付款问题）

### CLI 命令

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

## 最佳实践

### 1. 优化重试计划

测试不同的计划以找到适合您业务的方案：

| 业务类型 | 推荐计划 |
|--------------|---------------------|
| SaaS / 软件 | [1, 3, 7] - 标准 |
| 实体商品 | [3, 7, 14] - 给客户更多时间 |
| 高价值 B2B | [3, 7, 14, 21] - 更多尝试的延长计划 |
| 低价值 / 免费增值 | [1, 3] - 快速恢复或取消 |

### 2. 简化付款更新

减少付款更新过程中的摩擦：

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

### 3. 主动预防

在失败发生前减少失败：

**过期提醒：**
```toml
[notifications.expiration]
enabled = true
days_before_expiration = 30
template = "card_expiring_soon"
```

**预催缴邮件：**
```toml
[notifications.pre_dunning]
enabled = true
days_before_billing = 3
template = "upcoming_billing"
```

**账户更新服务：**
```toml
[payment.account_updater]
enabled = true
providers = ["visa", "mastercard"]
```

### 4. 客户沟通语气

在催缴阶段递进语气：

| 阶段 | 语气 | 示例开头 |
|-------|------|-----------------|
| 首次失败 | 友好、有帮助 | "别担心，这种情况很常见..." |
| 重试失败 | 紧急、礼貌 | "我们需要您的关注..." |
| 最终通知 | 严肃、明确 | "需要操作以保持您的订阅..." |
| 取消 | 专业、开放 | "我们期待您的回归..." |

## 故障排除

### 常见问题

**恢复率低**
- 检查邮件主题行
- 检查付款更新页面的可用性
- 验证重试计划时间
- 测试不同的邮件模板

**取消率高**
- 延长宽限期
- 添加更多重试尝试
- 改进邮件消息
- 提供付款计划选项

**付款更新页面无法使用**
- 检查令牌验证
- 验证 SSL 证书
- 在移动设备上测试
- 检查错误日志

**邮件未发送**
- 验证邮件提供商配置
- 检查模板语法
- 检查垃圾邮件文件夹放置
- 测试邮件可送达性

### 调试模式

为催缴启用调试日志：

```toml
[dunning.debug]
log_emails = true
log_retries = true
log_webhooks = true
```

## 常见问题

**Q：什么是催缴管理？**

A：催缴管理是自动重试失败的订阅付款并与客户沟通以恢复收入的过程。它包括计划重试、邮件通知以及当恢复失败时的最终取消。

**Q：催缴管理可以恢复多少收入？**

A：通过适当的配置，催缴管理通常可以恢复 60-80% 的失败订阅付款。恢复率因行业、客户群和催缴策略而异。

**Q：催缴管理是自动的吗？**

A：是的，一旦配置，整个催缴过程都是自动化的。系统无需人工干预即可处理重试、邮件和取消。

**Q：客户在催缴期间会失去访问权限吗？**

A：不会，客户在宽限期内（默认：14 天）保留访问权限。订阅状态变为"逾期"，但在取消前服务继续。

**Q：客户可以在催缴期间更新付款方式吗？**

A：是的，客户可以在催缴过程中的任何时候更新他们的付款方式。下次计划重试将使用新的付款方式。

**Q：什么是好的恢复率？**

A：行业基准：
- 优秀：>75%
- 良好：60-75%
- 平均：45-60%
- 需要改进：<45%

## 下一步

- [配置通知](../guides/notifications.md)
- [设置支付网关](../payment-gateways/index.md)
- [API 参考：订阅](../api-reference/subscriptions.md)
- [Webhooks 参考](../api-reference/webhooks.md)
