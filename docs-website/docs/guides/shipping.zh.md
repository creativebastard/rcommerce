# 运输设置指南

本指南将引导您完成 R Commerce 商店的运输设置，从配置承运商到创建运输区域和规则。

## 概述

R Commerce 提供全面的运输系统，支持：

- **多个承运商**：DHL、FedEx、UPS、USPS 以及 EasyPost 等聚合商
- **运输区域**：地理费率管理
- **规则引擎**：条件运输逻辑
- **实时费率**：实时承运商费率计算
- **标签生成**：自动化运输标签

## 步骤 1：配置承运商凭证

将运输配置添加到您的 `config.toml`：

```toml
[shipping]
default_provider = "ups"

[shipping.ups]
api_key = "your_api_key"
username = "your_username"
password = "your_password"
account_number = "your_account"
test_mode = true  # 生产环境设置为 false

[shipping.fedex]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = true

[shipping.dhl]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = true

[shipping.easypost]
api_key = "your_api_key"
test_mode = true
```

### 获取承运商凭证

#### UPS

1. 在 [UPS Developer Kit](https://developer.ups.com/) 注册
2. 为您的账户申请 API 访问权限
3. 在开发者门户生成 API 凭证

**所需凭证：**
- API 密钥
- 用户名
- 密码
- 账户号码

**可用的 UPS 服务：**
- UPS Ground
- UPS 3 Day Select
- UPS 2nd Day Air
- UPS Next Day Air
- UPS Worldwide Express

#### FedEx

1. 在 [FedEx Developer Portal](https://developer.fedex.com/) 创建账户
2. 注册您的应用程序
3. 获取 API 密钥和密钥

**所需凭证：**
- API 密钥
- API 密钥
- 账户号码
- 仪表号码（某些服务需要）

**可用的 FedEx 服务：**
- FedEx Ground
- FedEx Express Saver
- FedEx 2Day
- FedEx Priority Overnight
- FedEx International Priority

#### DHL

1. 在 [DHL API Developer Portal](https://developer.dhl.com/) 注册
2. 订阅 Express API
3. 获取您的 API 凭证

**所需凭证：**
- API 密钥
- API 密钥
- 账户号码

**可用的 DHL 服务：**
- DHL Express Worldwide
- DHL Express 9:00
- DHL Express 10:30
- DHL Express 12:00

#### USPS

1. 在 [USPS Web Tools](https://www.usps.com/business/web-tools-apis/) 注册
2. 申请 API 访问权限
3. 通过电子邮件接收凭证

**所需凭证：**
- 用户 ID
- 密码（某些服务需要）

**可用的 USPS 服务：**
- First-Class Mail
- Priority Mail
- Priority Mail Express
- Parcel Select

#### EasyPost（推荐用于多个承运商）

1. 在 [EasyPost](https://www.easypost.com/) 创建账户
2. 从仪表板复制您的 API 密钥
3. 通过 EasyPost 界面添加承运商账户

**所需凭证：**
- API 密钥（测试或生产）

**EasyPost 功能：**
- 100 多个承运商的统一 API
- 自动承运商账户管理
- 地址验证
- 保险选项

### API 凭证设置

使用环境变量安全存储凭证：

```bash
# .env 文件
UPS_API_KEY=your_ups_key
UPS_USERNAME=your_ups_username
UPS_PASSWORD=your_ups_password
UPS_ACCOUNT=your_ups_account

FEDEX_API_KEY=your_fedex_key
FEDEX_SECRET=your_fedex_secret
FEDEX_ACCOUNT=your_fedex_account

DHL_API_KEY=your_dhl_key
DHL_SECRET=your_dhl_secret

EASYPOST_API_KEY=your_easypost_key
```

在配置中引用：

```toml
[shipping.ups]
api_key = "${UPS_API_KEY}"
username = "${UPS_USERNAME}"
password = "${UPS_PASSWORD}"
account_number = "${UPS_ACCOUNT}"
test_mode = false
```

## 步骤 2：设置运输区域

运输区域定义具有特定费率的地理区域。根据您的运输策略创建区域：

### 示例：国内和国际区域

```toml
[shipping.zones.domestic]
name = "United States"
countries = ["US"]

[shipping.zones.domestic.rates.standard]
name = "Standard Ground"
base_rate = 8.00
per_kg_rate = 1.00
free_shipping_threshold = 100.00

[shipping.zones.domestic.rates.express]
name = "Express"
base_rate = 15.00
per_kg_rate = 2.50

[shipping.zones.international]
name = "Rest of World"
countries = ["*"]  # 所有其他国家
exclude = ["US"]

[shipping.zones.international.rates.international]
name = "International Standard"
base_rate = 35.00
per_kg_rate = 5.00
```

### 区域配置选项

| 选项 | 描述 | 示例 |
|--------|-------------|---------|
| `countries` | ISO 国家代码列表 | `["US", "CA", "MX"]` |
| `regions` | 特定地区/州 | `["CA", "NY", "TX"]` |
| `postal_codes` | 特定邮政编码范围 | `["10000-19999"]` |
| `exclude` | 要排除的国家 | `["US"]` |

### 高级区域示例

```toml
# 欧盟区域
[shipping.zones.eu]
name = "European Union"
countries = ["DE", "FR", "IT", "ES", "NL", "BE", "AT"]

[shipping.zones.eu.rates.standard]
name = "EU Standard"
base_rate = 12.00
per_kg_rate = 2.00
delivery_days = [5, 7]

[shipping.zones.eu.rates.express]
name = "EU Express"
base_rate = 25.00
per_kg_rate = 4.00
delivery_days = [1, 3]

# 偏远地区，费率较高
[shipping.zones.remote]
name = "Remote Areas"
countries = ["IS", "GL", "FO"]

[shipping.zones.remote.rates.standard]
name = "Remote Standard"
base_rate = 50.00
per_kg_rate = 10.00
```

## 步骤 3：配置运输规则

运输规则允许您为运输选项创建条件逻辑。

### 常见规则类型

```toml
[shipping.rules.free_shipping]
name = "Free Shipping Over $100"
condition = "order_total >= 100"
action = "set_rate_to_zero"
priority = 100

[shipping.rules.heavy_items]
name = "Heavy Item Surcharge"
condition = "weight > 20"
action = "add_surcharge"
amount = 10.00

[shipping.rules.express_upgrade]
name = "Free Express for VIP Customers"
condition = "customer_tag == 'vip' AND order_total >= 200"
action = "upgrade_to_express"
```

### 可用条件

| 条件 | 描述 | 示例 |
|-----------|-------------|---------|
| `order_total` | 订单小计金额 | `order_total >= 100` |
| `weight` | 订单总重量 | `weight > 10` |
| `item_count` | 商品数量 | `item_count >= 5` |
| `customer_tag` | 客户标签/细分 | `customer_tag == 'vip'` |
| `product_category` | 商品类别 | `product_category == 'fragile'` |
| `destination_country` | 运输目的地 | `destination_country == 'CA'` |

### 可用操作

| 操作 | 描述 | 参数 |
|--------|-------------|------------|
| `set_rate_to_zero` | 免运费 | 无 |
| `add_surcharge` | 添加额外费用 | `amount` |
| `discount_rate` | 应用百分比折扣 | `percentage` |
| `upgrade_to_express` | 升级运输方式 | 无 |
| `hide_method` | 隐藏运输选项 | `method_name` |
| `require_signature` | 需要签名 | 无 |

## 步骤 4：配置包装类型

为您的商品定义标准包装尺寸：

```toml
[shipping.packages]

[shipping.packages.small_box]
name = "Small Box"
length = 20
width = 15
height = 10
unit = "cm"
max_weight = 2.0

[shipping.packages.medium_box]
name = "Medium Box"
length = 30
width = 25
height = 20
unit = "cm"
max_weight = 5.0

[shipping.packages.large_box]
name = "Large Box"
length = 50
width = 40
height = 30
unit = "cm"
max_weight = 20.0

[shipping.packages.flat_rate_envelope]
name = "Flat Rate Envelope"
length = 32
width = 24
height = 2
unit = "cm"
max_weight = 1.0
flat_rate = 8.50
```

## 步骤 5：设置地址验证

启用地址验证以减少运输错误：

```toml
[shipping.address_validation]
enabled = true
provider = "easypost"  # 或 "ups", "fedex"
cache_results = true
cache_duration_hours = 24
```

## 步骤 6：测试您的配置

### 使用 CLI

```bash
# 测试运输费率
rcommerce shipping test-rates \
  --from "123 Main St, New York, NY 10001, US" \
  --to "456 Oak Ave, Los Angeles, CA 90210, US" \
  --weight 5 \
  --providers ups,fedex

# 验证地址
rcommerce shipping validate-address \
  --address "789 Pine Rd, Chicago, IL 60601, US"

# 测试标签生成（测试模式）
rcommerce shipping test-label \
  --provider ups \
  --service "ground" \
  --package medium_box
```

### 测试场景

上线前测试以下常见场景：

1. **国内标准** - 同一国家内的标准陆运
2. **国内快递** - 快递/隔夜运输
3. **国际** - 运输到不同国家
4. **重货** - 超过重量阈值的商品
5. **免费运输** - 符合免费运输条件的订单
6. **偏远地区** - 运输到偏远/延长配送地区

## 步骤 7：上线

### 上线前检查清单

- [ ] 所有承运商凭证都是生产环境（不是测试/沙盒）
- [ ] 所有承运商配置中 `test_mode = false`
- [ ] 运输区域覆盖您发货的所有目的地
- [ ] 所有区域的费率计算正确
- [ ] 免费运输阈值已配置
- [ ] 地址验证已启用
- [ ] 运输规则已测试并正常工作
- [ ] 包装类型已定义
- [ ] 标签打印已测试
- [ ] 追踪号码格式已验证

### 上线后监控

在仪表板中监控这些指标：

| 指标 | 目标 | 不达标时的行动 |
|--------|--------|---------------------|
| 费率计算成功率 | >99% | 检查承运商 API 状态 |
| 地址验证通过率 | >95% | 检查地址输入字段 |
| 标签生成成功率 | >99% | 验证承运商账户余额 |
| 平均运输成本 | 跟踪趋势 | 必要时调整费率 |

## 最佳实践

### 1. 使用聚合商管理多个承运商

如果您使用多个承运商，考虑使用 EasyPost 或 ShipStation 简化集成：

```toml
[shipping]
default_provider = "easypost"

[shipping.easypost]
api_key = "your_api_key"
test_mode = false
carriers = ["ups", "fedex", "usps"]  # 启用的承运商
```

### 2. 实施备用费率

配置备用费率以防承运商 API 不可用：

```toml
[shipping.fallback]
enabled = true
domestic_rate = 10.00
international_rate = 40.00
max_weight_for_fallback = 50
```

### 3. 缓存运输费率

缓存常见路线的费率以提高性能：

```toml
[shipping.cache]
enabled = true
ttl_seconds = 3600  # 1 小时
max_entries = 1000
```

### 4. 处理体积重量

承运商对大件轻货按体积重量收费。确保您的商品有准确的尺寸：

```toml
[shipping.volumetric_weight]
enabled = true
dimensional_factor = 5000  # cm/kg 的标准除数
```

### 5. 设置运输通知

配置运输事件的邮件通知：

```toml
[notifications.shipping]
ship_confirmation = true
delivery_confirmation = true
exception_alerts = true
template_prefix = "shipping_"
```

## 故障排除

### 常见问题

**"没有可用的运输费率"**
- 检查承运商凭证是否有效
- 验证运输区域是否覆盖目的地
- 确保包裹重量在承运商限制范围内
- 检查承运商服务对该路线是否可用

**"地址验证失败"**
- 验证地址格式是否符合国家要求
- 检查是否缺少必填字段（州、邮政编码）
- 尝试标准化地址格式

**"标签生成失败"**
- 验证承运商账户是否活跃且有余额
- 检查包裹尺寸是否在承运商限制范围内
- 确保国际运输提供海关信息
- 验证发货地址是否有效

**费率似乎不正确**
- 检查体积重量计算
- 验证包裹尺寸是否准确
- 检查运输区域配置
- 检查是否有冲突的运输规则

### 调试模式

启用调试日志以排查问题：

```toml
[shipping.debug]
log_requests = true
log_responses = true
log_level = "debug"
```

## 下一步

- [配置税务设置](../guides/tax-setup.md)
- [设置支付网关](../payment-gateways/index.md)
- [配置通知](../guides/notifications.md)
- [API 参考：运输](../api-reference/shipping.md)
