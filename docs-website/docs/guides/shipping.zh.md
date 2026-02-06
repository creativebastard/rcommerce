# 运输集成

R Commerce 提供全面的运输系统，支持多个承运商、实时运费计算和自动标签生成。

## 概述

运输模块支持：

- **直接承运商集成**：DHL、FedEx、UPS、USPS
- **第三方聚合商**：EasyPost、ShipStation
- **基于重量的计算**：实际重量和体积重量
- **运输区域**：地理费率管理
- **规则引擎**：条件运输逻辑
- **多承运商费率比较**：跨提供商比较费率

## 支持的承运商

### 直接承运商

| 承运商 | 服务 | 国际 |
|---------|----------|---------------|
| DHL Express | EXPRESS_WORLDWIDE、EXPRESS_12:00、ECONOMY_SELECT | 是 |
| FedEx | Ground、2Day、Overnight、International Priority | 是 |
| UPS | Ground、2nd Day Air、Next Day Air、Worldwide Saver | 是 |
| USPS | Ground Advantage、Priority Mail、Priority Express | 有限 |

### 聚合商

| 提供商 | 功能 |
|----------|----------|
| EasyPost | 100+ 承运商、地址验证、保险 |
| ShipStation | 订单管理、批量处理、库存同步 |

## 配置

将运输配置添加到您的 `config.toml`：

```toml
[shipping]
default_provider = "ups"

[shipping.ups]
api_key = "your_api_key"
username = "your_username"
password = "your_password"
account_number = "your_account"
test_mode = false

[shipping.fedex]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = false

[shipping.dhl]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = false

[shipping.easypost]
api_key = "your_api_key"
test_mode = false
```

## 重量计算

### 体积重量

承运商根据实际重量或体积（ dimensional ）重量中较大者收费：

**公式**：`(长 × 宽 × 高) / 体积系数`

| 提供商 | 系数 (cm/kg) | 系数 (in/lb) |
|----------|----------------|----------------|
| DHL | 5000 | - |
| FedEx | 5000 | 139 |
| UPS | 5000 | 139 |
| USPS | - | 166 |

### 计算示例

```rust
use rcommerce_core::shipping::calculation::VolumetricWeightCalculator;

// 计算体积重量
let calc = VolumetricWeightCalculator::standard_international();
let volumetric_weight = calc.calculate(
    Decimal::from(50),  // 长度 cm
    Decimal::from(40),  // 宽度 cm
    Decimal::from(30),  // 高度 cm
);

// volumetric_weight = (50 × 40 × 30) / 5000 = 12 kg
```

### 计费重量

计费重量是实际重量和体积重量中的最大值：

```rust
let calc = VolumetricWeightCalculator::standard_international();
let chargeable = calc.calculate_chargeable_weight(&package);

println!("实际: {} kg", chargeable.actual_weight);
println!("体积: {:?} kg", chargeable.volumetric_weight);
println!("计费: {} kg", chargeable.chargeable_weight);
```

## 获取运输费率

### 单一承运商

```rust
use rcommerce_core::shipping::{UpsProvider, RateOptions, Package};

let ups = UpsProvider::new(api_key, username, password, account);

let package = Package::new(Decimal::from(5), "kg")
    .with_dimensions(Decimal::from(30), Decimal::from(20), Decimal::from(15), "cm");

let options = RateOptions::default();

let rates = ups.get_rates(&from_address, &to_address, &package, &options).await?;

for rate in rates {
    println!("{}: ${} ({} 天)", 
        rate.service_name, 
        rate.total_cost,
        rate.delivery_days.unwrap_or(0)
    );
}
```

### 多承运商费率比较

```rust
use rcommerce_core::shipping::{ShippingProviderFactory, ShippingRateAggregator};

let mut factory = ShippingProviderFactory::new();
factory.register(Box::new(ups));
factory.register(Box::new(fedex));
factory.register(Box::new(dhl));

let aggregator = ShippingRateAggregator::new(factory);

let rates = aggregator.get_all_rates(
    &from_address,
    &to_address,
    &package,
    &RateOptions::default(),
).await?;

// 按总成本排序的费率
```

## 运输区域

定义具有特定费率的地理区域：

```rust
use rcommerce_core::shipping::zones::{ShippingZone, ZoneRate, ZoneCalculator};

// 创建国内区域
let domestic = ShippingZone::new("domestic", "United States")
    .with_country("US")
    .with_rate(
        ZoneRate::new("Standard Ground", Decimal::from(8), Decimal::from(1))
            .with_free_shipping_threshold(Decimal::from(100))
    );

// 创建国际区域
let international = ShippingZone::new("international", "Rest of World")
    .with_rate(
        ZoneRate::new("International", Decimal::from(35), Decimal::from(5))
    );

// 计算运费
let mut calculator = ZoneCalculator::new();
calculator.add_zone(domestic);
calculator.add_zone(international);

if let Some((cost, rate)) = calculator.calculate_shipping(&address, weight, subtotal) {
    println!("运费: ${} 通过 {}", cost, rate.name);
}
```

## 运输规则

创建条件运输逻辑：

```rust
use rcommerce_core::shipping::rules::{ShippingRule, RuleCondition, RuleAction};

// 订单超过 $100 免运费
let free_shipping = ShippingRule::new(
    "Free Shipping",
    RuleCondition::OrderTotal { 
        min: Some(Decimal::from(100)), 
        max: None 
    },
    RuleAction::FreeShipping,
).with_priority(100);

// 快递运输 20% 折扣
let express_discount = ShippingRule::new(
    "Express Discount",
    RuleCondition::ShippingMethod { 
        methods: vec!["express".to_string()] 
    },
    RuleAction::DiscountShipping { percentage: Decimal::from(20) },
);

// 应用规则
let mut engine = ShippingRuleEngine::new();
engine.add_rule(free_shipping);
engine.add_rule(express_discount);

engine.apply_rules(&order, &mut rates);
```

## 创建货件

```rust
let shipment = provider.create_shipment(
    &from_address,
    &to_address,
    &package,
    "PRIORITY_OVERNIGHT",  // 服务代码
    Some(&customs_info),   // 国际运输需要
).await?;

println!("追踪: {}", shipment.tracking_number.unwrap());
println!("标签: {}", shipment.label_url.unwrap());
```

## 追踪货件

### 按号码追踪

```rust
let tracking = provider.track_shipment("1Z999AA10123456784").await?;

println!("状态: {}", tracking.status.description());
for event in &tracking.events {
    println!("{} - {} 在 {:?}", 
        event.timestamp, 
        event.description,
        event.location
    );
}
```

### 从追踪号码检测承运商

```rust
use rcommerce_core::shipping::carriers::detect_carrier_from_tracking;

let carrier = detect_carrier_from_tracking("1Z999AA10123456784");
// 返回: Some("ups")
```

## 国际运输

### 海关信息

```rust
use rcommerce_core::shipping::{CustomsInfo, CustomsItem, ContentsType, NonDeliveryOption};

let customs = CustomsInfo {
    contents_type: ContentsType::Merchandise,
    contents_description: "Electronic components".to_string(),
    non_delivery_option: NonDeliveryOption::Return,
    customs_items: vec![
        CustomsItem {
            description: "Circuit board".to_string(),
            quantity: 2,
            value: Decimal::from(50),
            currency: "USD".to_string(),
            weight: Some(Decimal::from(1)),
            weight_unit: Some("kg".to_string()),
            hs_tariff_number: Some("8517.62.00".to_string()),
            origin_country: "US".to_string(),
        }
    ],
    declaration_value: Decimal::from(100),
    declaration_currency: "USD".to_string(),
};
```

## 包装

### 预定义包装类型

```rust
use rcommerce_core::shipping::packaging::PackageRegistry;

let registry = PackageRegistry::new();

// 获取 USPS 统一费率盒子
let usps_boxes = registry.get_by_carrier("USPS");

// 为商品找到最佳统一费率
let calculator = PackagingCalculator::new();
if let Some(package_type) = calculator.find_best_flat_rate(&items, Some("USPS")) {
    println!("使用 {} 价格 ${}", 
        package_type.name,
        package_type.flat_rate.unwrap()
    );
}
```

## 测试

在开发期间使用测试模式：

```rust
let ups = UpsProvider::new(api_key, username, password, account)
    .with_test_mode(true);

// 所有 API 调用都转到沙盒
// 标签是测试标签（不适用于实际运输）
```

## API 端点

### 获取运输费率

```http
POST /api/v1/shipping/rates
Content-Type: application/json

{
  "from_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "zip": "10001",
    "country": "US"
  },
  "to_address": {
    "first_name": "Jane",
    "last_name": "Smith",
    "address1": "456 Oak Ave",
    "city": "Los Angeles",
    "state": "CA",
    "zip": "90210",
    "country": "US"
  },
  "package": {
    "weight": 5.0,
    "weight_unit": "kg",
    "length": 30,
    "width": 20,
    "height": 15,
    "dimension_unit": "cm"
  },
  "providers": ["ups", "fedex"]
}
```

### 创建货件

```http
POST /api/v1/shipping/shipments
Content-Type: application/json

{
  "order_id": "550e8400-e29b-41d4-a716-446655440000",
  "provider": "ups",
  "service_code": "02",
  "package": {
    "weight": 5.0,
    "weight_unit": "kg"
  }
}
```

### 追踪货件

```http
GET /api/v1/shipping/tracking/1Z999AA10123456784
```

## 最佳实践

1. **缓存费率**：缓存常见路线的运输费率以减少 API 调用
2. **使用测试模式**：在开发/暂存环境中始终使用测试模式
3. **处理错误**：承运商可能出现中断；实施回退逻辑
4. **验证地址**：在创建货件之前使用地址验证
5. **监控成本**：按承运商和区域跟踪运输成本
6. **体积重量**：始终为大/轻包裹计算体积重量

## 故障排除

### 常见问题

**找不到费率**
- 检查地址是否有效且完整
- 验证包裹尺寸是否在承运商限制范围内
- 确保服务对该路线可用

**标签生成失败**
- 验证账户凭据
- 检查与承运商的余额/账户状态
- 确保国际运输提供海关信息

**追踪不更新**
- 某些承运商的追踪更新有延迟
- 直接验证追踪号码格式是否正确
- 直接在承运商的追踪网站上检查
