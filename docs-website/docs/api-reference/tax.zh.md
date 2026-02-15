# 税务 API

税务 API 为全球电子商务提供全面的税务计算和管理，包括欧盟增值税（VAT）与 OSS、美国销售税以及增值税/消费税（GST）支持。

## 概述

- **基础 URL**: `/api/v1/tax`
- **认证**: 需要 API 密钥或 JWT
- **所需权限**:
  - `tax:read` - 用于计算和报告
  - `tax:write` - 用于管理税率和区域

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
  "shipping_tax": "0.00",
  "total_tax": "11.40",
  "tax_breakdown": [
    {
      "tax_zone_id": "550e8400-e29b-41d4-a716-446655440011",
      "tax_zone_name": "Germany",
      "tax_rate_id": "550e8400-e29b-41d4-a716-446655440010",
      "tax_rate_name": "German Standard VAT",
      "rate": "0.19",
      "taxable_amount": "59.98",
      "tax_amount": "11.40"
    }
  ],
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

## 配置

在 `config.toml` 中配置税务设置：

```toml
[tax]
provider = "builtin"  # 或 'avalara', 'taxjar'
enable_oss = true
oss_member_state = "DE"
validate_vat_ids = true
vat_cache_days = 30

[tax.avalara]
api_key = "${AVALARA_API_KEY}"
account_id = "${AVALARA_ACCOUNT_ID}"
environment = "sandbox"  # 或 'production'

[tax.taxjar]
api_token = "${TAXJAR_API_TOKEN}"
sandbox = true
```

## 另请参阅

- [税务系统架构](../../architecture/13-tax-system.md)
- [欧盟增值税 OSS 指南](https://vat-one-stop-shop.ec.europa.eu/)
- [Avalara AvaTax 文档](https://developer.avalara.com/)
- [TaxJar API 文档](https://developers.taxjar.com/)
