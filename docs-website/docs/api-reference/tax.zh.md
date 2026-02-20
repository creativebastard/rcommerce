# 税务 API

税务 API 为全球电子商务提供全面的税务计算和管理，包括欧盟增值税（VAT）与 OSS、美国销售税以及增值税/消费税（GST）支持。税务系统已与购物车、订单、配送和结账服务完全集成。

## 概述

- **基础 URL**: `/api/v1/tax`
- **认证**: 需要 API 密钥或 JWT
- **所需权限**:
  - `tax:read` - 用于计算和报告
  - `tax:write` - 用于管理税率和区域

## 服务集成

税务 API 已与以下服务集成：

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         结账流程                                         │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                      结账服务 (CheckoutService)                          │
│  协调：购物车 → 税费计算 → 配送 → 订单 → 支付                             │
└─────────────────────────────────────────────────────────────────────────┘
        │              │              │              │
        ▼              ▼              ▼              ▼
┌──────────────┐ ┌──────────┐ ┌────────────┐ ┌──────────────┐
│ 购物车服务   │ │ 税务服务 │ │   配送服务 │ │   订单服务   │
│              │ │          │ │            │ │              │
│ - 商品       │ │- 计算税费│ │ - 运费     │ │ - 创建订单   │
│ - 折扣       │ │- 验证VAT │ │ - 配送方式 │ │ - 记录税费   │
│ - 税费计算   │ │- OSS     │ │ - 追踪     │ │ - 处理支付   │
└──────────────┘ └──────────┘ └────────────┘ └──────────────┘
```

### 购物车服务集成

在获取购物车总额时，系统会根据配送地址自动计算税费：

```http
GET /api/v1/carts/{cart_id}/totals?shipping_address_id={address_id}
```

响应包含：
- `tax_total` - 税费总额
- `tax_breakdown` - 按管辖区的详细税费
- `calculated_total` - 包含税费的最终总额

### 结账集成

在结账过程中，税费会自动计算：

1. **发起结账** - 根据配送地址计算税费
2. **选择配送** - 将配送税费添加到总额
3. **完成结账** - 税费随订单一起记录

## 计算税费

计算购物车或订单的税费。

```http
POST /api/v1/tax/calculate
```

### 请求

```json
{
  "items": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "product_id": "550e8400-e29b-41d4-a716-446655440001",
      "quantity": 2,
      "unit_price": "29.99",
      "tax_category_id": "550e8400-e29b-41d4-a716-446655440002",
      "is_digital": false,
      "title": "Premium T-Shirt",
      "sku": "TSHIRT-001"
    }
  ],
  "shipping_address": {
    "country_code": "DE",
    "region_code": "BY",
    "postal_code": "80331",
    "city": "Munich"
  },
  "billing_address": {
    "country_code": "DE",
    "region_code": "BY",
    "postal_code": "80331",
    "city": "Munich"
  },
  "customer_id": "550e8400-e29b-41d4-a716-446655440003",
  "vat_id": "DE123456789",
  "currency": "EUR"
}
```

### 响应

```json
{
  "line_items": [
    {
      "item_id": "550e8400-e29b-41d4-a716-446655440000",
      "taxable_amount": "59.98",
      "tax_amount": "11.40",
      "tax_rate": "0.19",
      "tax_rate_id": "550e8400-e29b-41d4-a716-446655440010",
      "tax_zone_id": "550e8400-e29b-41d4-a716-446655440011"
    }
  ],
  "shipping_tax": "1.90",
  "total_tax": "13.30",
  "tax_breakdown": [
    {
      "tax_zone_id": "550e8400-e29b-41d4-a716-446655440011",
      "tax_zone_name": "Germany",
      "tax_rate_id": "550e8400-e29b-41d4-a716-446655440010",
      "tax_rate_name": "German Standard VAT",
      "rate": "0.19",
      "taxable_amount": "70.00",
      "tax_amount": "13.30"
    }
  ],
  "currency": "EUR"
}
```

### B2B 交易

对于带有有效 VAT 号码的 B2B 交易，可能适用反向征收：

```json
{
  "items": [...],
  "shipping_address": {
    "country_code": "FR",
    "region_code": "IDF",
    "postal_code": "75001",
    "city": "Paris"
  },
  "customer_id": "550e8400-e29b-41d4-a716-446655440003",
  "vat_id": "FR12345678901",
  "currency": "EUR"
}
```

反向征收响应：

```json
{
  "line_items": [
    {
      "item_id": "550e8400-e29b-41d4-a716-446655440000",
      "taxable_amount": "100.00",
      "tax_amount": "0.00",
      "tax_rate": "0.00",
      "tax_rate_id": "...",
      "tax_zone_id": "...",
      "reverse_charge": true
    }
  ],
  "shipping_tax": "0.00",
  "total_tax": "0.00",
  "tax_breakdown": [],
  "reverse_charge_applied": true,
  "vat_id_valid": true,
  "currency": "EUR"
}
```

## 验证 VAT 号码

使用欧盟 VIES 服务验证增值税号码。

```http
POST /api/v1/tax/validate-vat
```

### 请求

```json
{
  "vat_id": "DE123456789"
}
```

### 响应

```json
{
  "vat_id": "DE123456789",
  "country_code": "DE",
  "is_valid": true,
  "business_name": "Example GmbH",
  "business_address": "Musterstraße 1, 80331 Munich",
  "validated_at": "2026-02-14T10:30:00Z"
}
```

### 错误响应

```json
{
  "error": "Invalid VAT ID format",
  "code": "validation_error"
}
```

## 获取税率

获取特定位置的适用税率。

```http
GET /api/v1/tax/rates?country_code=DE&region_code=BY&postal_code=80331
```

### 查询参数

| 参数 | 类型 | 必需 | 描述 |
|-----------|------|----------|-------------|
| `country_code` | string | 是 | ISO 3166-1 alpha-2 国家代码 |
| `region_code` | string | 否 | 州/省代码 |
| `postal_code` | string | 否 | 邮政编码 |
| `tax_category_id` | UUID | 否 | 按税务类别筛选 |

### 响应

```json
{
  "rates": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440010",
      "name": "German Standard VAT",
      "rate": "0.19",
      "rate_type": "percentage",
      "is_vat": true,
      "vat_type": "standard",
      "b2b_exempt": false,
      "reverse_charge": false,
      "tax_zone": {
        "id": "550e8400-e29b-41d4-a716-446655440011",
        "name": "Germany",
        "code": "DE",
        "country_code": "DE"
      },
      "tax_category": null,
      "valid_from": "2020-01-01",
      "valid_until": null
    }
  ]
}
```

## 计算配送税费

计算配送费用的税费。

```http
POST /api/v1/tax/calculate-shipping
```

### 请求

```json
{
  "shipping_amount": "10.00",
  "shipping_address": {
    "country_code": "DE",
    "region_code": "BY",
    "postal_code": "80331",
    "city": "Munich"
  },
  "currency": "EUR"
}
```

### 响应

```json
{
  "shipping_amount": "10.00",
  "shipping_tax": "1.90",
  "tax_rate": "0.19",
  "total_with_tax": "11.90",
  "currency": "EUR"
}
```

## 生成 OSS 报告

为欧盟销售生成 OSS（一站式申报）增值税报告。

```http
POST /api/v1/tax/oss-report
```

### 请求

```json
{
  "scheme": "union",
  "period": "2026-01",
  "member_state": "DE"
}
```

### 参数

| 参数 | 类型 | 必需 | 描述 |
|-----------|------|----------|-------------|
| `scheme` | string | 是 | `union`、`non_union` 或 `import` |
| `period` | string | 是 | 报告期间，格式 `YYYY-MM` |
| `member_state` | string | 是 | 您的欧盟成员国识别号 |

### 响应

```json
{
  "scheme": "union",
  "period": "2026-01",
  "member_state": "DE",
  "transactions": [
    {
      "country_code": "FR",
      "vat_rate": "0.20",
      "taxable_amount": "1000.00",
      "vat_amount": "200.00",
      "transaction_count": 5
    },
    {
      "country_code": "IT",
      "vat_rate": "0.22",
      "taxable_amount": "500.00",
      "vat_amount": "110.00",
      "transaction_count": 3
    }
  ],
  "summary": {
    "total_taxable_amount": "1500.00",
    "total_vat_amount": "310.00",
    "total_transactions": 8,
    "by_country": [
      {
        "country_code": "FR",
        "country_name": "France",
        "vat_rate": "0.20",
        "taxable_amount": "1000.00",
        "vat_amount": "200.00",
        "transaction_count": 5
      },
      {
        "country_code": "IT",
        "country_name": "Italy",
        "vat_rate": "0.22",
        "taxable_amount": "500.00",
        "vat_amount": "110.00",
        "transaction_count": 3
      }
    ]
  }
}
```

## 管理：创建税务区域

创建新的税务区域（仅限管理员）。

```http
POST /api/v1/admin/tax/zones
```

### 请求

```json
{
  "name": "Bavaria",
  "code": "DE-BY",
  "country_code": "DE",
  "region_code": "BY",
  "zone_type": "state"
}
```

### 响应

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440020",
  "name": "Bavaria",
  "code": "DE-BY",
  "country_code": "DE",
  "region_code": "BY",
  "zone_type": "state",
  "created_at": "2026-02-14T10:30:00Z",
  "updated_at": "2026-02-14T10:30:00Z"
}
```

## 管理：创建税率

创建新的税率（仅限管理员）。

```http
POST /api/v1/admin/tax/rates
```

### 请求

```json
{
  "name": "German Reduced VAT",
  "tax_zone_id": "550e8400-e29b-41d4-a716-446655440011",
  "tax_category_id": "550e8400-e29b-41d4-a716-446655440021",
  "rate": "0.07",
  "rate_type": "percentage",
  "is_vat": true,
  "vat_type": "reduced",
  "b2b_exempt": false,
  "reverse_charge": false,
  "valid_from": "2020-01-01",
  "priority": 10
}
```

### 响应

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440022",
  "name": "German Reduced VAT",
  "tax_zone_id": "550e8400-e29b-41d4-a716-446655440011",
  "tax_category_id": "550e8400-e29b-41d4-a716-446655440021",
  "rate": "0.07",
  "rate_type": "percentage",
  "is_vat": true,
  "vat_type": "reduced",
  "b2b_exempt": false,
  "reverse_charge": false,
  "valid_from": "2020-01-01",
  "valid_until": null,
  "priority": 10,
  "created_at": "2026-02-14T10:30:00Z",
  "updated_at": "2026-02-14T10:30:00Z"
}
```

## 税务类别

### 列出税务类别

```http
GET /api/v1/tax/categories
```

### 响应

```json
{
  "categories": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440021",
      "name": "Food",
      "code": "food",
      "description": "Food and beverages",
      "is_digital": false,
      "is_food": true,
      "is_luxury": false,
      "is_medical": false,
      "is_educational": false
    }
  ]
}
```

## 结账集成示例

### 示例：完整结账流程

```bash
# 步骤 1：发起结账
POST /api/v1/checkout/initiate
{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_address": {
    "country_code": "DE",
    "region_code": "BY",
    "postal_code": "80331",
    "city": "Munich",
    "first_name": "John",
    "last_name": "Doe",
    "address1": "Musterstraße 1"
  },
  "vat_id": "DE123456789"
}

# 响应包含：
# - subtotal（小计）、discount_total（折扣总额）
# - item_tax（商品税费）、shipping_tax（配送税费）、tax_total（税费总额）
# - available_shipping_rates（可用配送费率）
# - tax_breakdown（税费明细）

# 步骤 2：选择配送方式
POST /api/v1/checkout/shipping
{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "shipping_rate_id": "rate_123",
  "shipping_method": "DHL Express"
}

# 步骤 3：完成结账
POST /api/v1/checkout/complete
{
  "cart_id": "550e8400-e29b-41d4-a716-446655440000",
  "payment_method": {
    "type": "card",
    "token": "tok_visa"
  },
  "customer_email": "john@example.com"
}
```

## 欧盟增值税标准税率参考（2026）

| 国家 | 代码 | 标准税率 | 减免税率 |
|---------|------|---------------|--------------|
| 奥地利 | AT | 20% | 13%, 10% |
| 比利时 | BE | 21% | 12%, 6% |
| 保加利亚 | BG | 20% | 9% |
| 克罗地亚 | HR | 25% | 13%, 5% |
| 塞浦路斯 | CY | 19% | 9%, 5% |
| 捷克共和国 | CZ | 21% | 15%, 10% |
| 丹麦 | DK | 25% | - |
| 爱沙尼亚 | EE | 22% | 9%, 5% |
| 芬兰 | FI | 25% | 14%, 10% |
| 法国 | FR | 20% | 10%, 5.5%, 2.1% |
| 德国 | DE | 19% | 7% |
| 希腊 | GR | 24% | 13%, 6% |
| 匈牙利 | HU | 27% | 18%, 5% |
| 爱尔兰 | IE | 23% | 13.5%, 9%, 4.8% |
| 意大利 | IT | 22% | 10%, 5%, 4% |
| 拉脱维亚 | LV | 21% | 12%, 5% |
| 立陶宛 | LT | 21% | 9%, 5% |
| 卢森堡 | LU | 17% | 14%, 8% |
| 马耳他 | MT | 18% | 7%, 5% |
| 荷兰 | NL | 21% | 9% |
| 波兰 | PL | 23% | 8%, 5% |
| 葡萄牙 | PT | 23% | 13%, 6% |
| 罗马尼亚 | RO | 19% | 9%, 5% |
| 斯洛伐克 | SK | 20% | 10% |
| 斯洛文尼亚 | SI | 22% | 9.5% |
| 西班牙 | ES | 21% | 10%, 4% |
| 瑞典 | SE | 25% | 12%, 6% |

## 美国销售税关联阈值（2026）

| 州 | 阈值 | 交易阈值 |
|-------|-----------|----------------------|
| 加利福尼亚 | $500,000 | 无 |
| 纽约 | $500,000 | 100 笔交易 |
| 德克萨斯 | $500,000 | 无 |
| 佛罗里达 | $100,000 | 无 |
| 伊利诺伊 | $100,000 | 无 |
| 宾夕法尼亚 | $100,000 | 无 |
| 俄亥俄 | $100,000 | 200 笔交易 |
| 乔治亚 | $100,000 | 200 笔交易 |
| 北卡罗来纳 | $100,000 | 无 |
| 密歇根 | $100,000 | 200 笔交易 |

## 错误代码

| 代码 | 描述 |
|------|-------------|
| `invalid_vat_id` | VAT 号码格式无效 |
| `vies_unavailable` | VIES 服务不可用 |
| `tax_zone_not_found` | 未找到该位置的税务区域 |
| `invalid_tax_rate` | 税率无效或已过期 |
| `oss_report_failed` | 生成 OSS 报告失败 |
| `tax_calculation_failed` | 税费计算失败 |
| `shipping_tax_failed` | 配送税费计算失败 |

## 配置

在 `config.toml` 中配置税务设置：

```toml
[tax]
provider = "builtin"  # 或 'avalara', 'taxjar'
enable_oss = true
oss_member_state = "DE"
validate_vat_ids = true
vat_cache_days = 30

# 默认税务行为
default_tax_included = false
default_tax_zone = "US"

[tax.avalara]
api_key = "${AVALARA_API_KEY}"
account_id = "${AVALARA_ACCOUNT_ID}"
environment = "sandbox"  # 或 'production'

[tax.taxjar]
api_token = "${TAXJAR_API_TOKEN}"
sandbox = true
```

## SDK 使用示例

### Rust

```rust
use rcommerce_core::{
    TaxService, DefaultTaxService, TaxContext, TaxAddress, TaxableItem,
    TransactionType, CustomerTaxInfo, VatId,
};

// 创建税务服务
let tax_service = DefaultTaxService::new(pool);

// 构建应税商品
let items = vec![TaxableItem {
    id: product_id,
    product_id,
    quantity: 2,
    unit_price: dec!(29.99),
    total_price: dec!(59.98),
    tax_category_id: None,
    is_digital: false,
    title: "Premium T-Shirt".to_string(),
    sku: Some("TSHIRT-001".to_string()),
}];

// 构建税务上下文
let context = TaxContext {
    customer: CustomerTaxInfo {
        customer_id: Some(customer_id),
        is_tax_exempt: false,
        vat_id: Some(VatId::parse("DE123456789")?),
        exemptions: vec![],
    },
    shipping_address: TaxAddress::new("DE")
        .with_region("BY")
        .with_postal_code("80331")
        .with_city("Munich"),
    billing_address: TaxAddress::new("DE"),
    currency: Currency::EUR,
    transaction_type: TransactionType::B2C,
};

// 计算税费
let calculation = tax_service.calculate_tax(&items, &context).await?;
println!("税费总额: {}", calculation.total_tax);

// 验证 VAT 号码
let validation = tax_service.validate_vat_id("DE123456789").await?;
println!("VAT 号码有效: {}", validation.is_valid);
```

### JavaScript/TypeScript

```typescript
import { RCommerceClient } from '@rcommerce/sdk';

const client = new RCommerceClient({
  baseUrl: 'https://api.example.com',
  apiKey: 'your-api-key'
});

// 计算税费
const calculation = await client.tax.calculate({
  items: [{
    id: 'item-123',
    product_id: 'prod-456',
    quantity: 2,
    unit_price: '29.99',
    title: 'Premium T-Shirt'
  }],
  shipping_address: {
    country_code: 'DE',
    region_code: 'BY',
    postal_code: '80331',
    city: 'Munich'
  },
  vat_id: 'DE123456789',
  currency: 'EUR'
});

console.log(`税费总额: ${calculation.total_tax}`);

// 验证 VAT 号码
const validation = await client.tax.validateVatId('DE123456789');
console.log(`VAT 号码有效: ${validation.is_valid}`);
```

## 另请参阅

- [税务系统架构](../../architecture/13-tax-system.md)
- [购物车 API](./cart.md)
- [订单 API](./orders.md)
- [配送 API](./shipping.md)
- [欧盟增值税 OSS 指南](https://vat-one-stop-shop.ec.europa.eu/)
- [Avalara AvaTax 文档](https://developer.avalara.com/)
- [TaxJar API 文档](https://developers.taxjar.com/)
