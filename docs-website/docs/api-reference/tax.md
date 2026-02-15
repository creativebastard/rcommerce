# Tax API

The Tax API provides comprehensive tax calculation and management for global e-commerce, including EU VAT with OSS, US sales tax, and VAT/GST support.

## Overview

- **Base URL**: `/api/v1/tax`
- **Authentication**: API Key or JWT required
- **Scopes Required**: 
  - `tax:read` - for calculation and reporting
  - `tax:write` - for managing tax rates and zones

## Calculate Tax

Calculate tax for a cart or order.

```http
POST /api/v1/tax/calculate
```

### Request

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

### Response

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

## Validate VAT ID

Validate a VAT ID using the EU VIES service.

```http
POST /api/v1/tax/validate-vat
```

### Request

```json
{
  "vat_id": "DE123456789"
}
```

### Response

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

### Error Response

```json
{
  "error": "Invalid VAT ID format",
  "code": "validation_error"
}
```

## Get Tax Rates

Get applicable tax rates for a location.

```http
GET /api/v1/tax/rates?country_code=DE&region_code=BY&postal_code=80331
```

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `country_code` | string | Yes | ISO 3166-1 alpha-2 country code |
| `region_code` | string | No | State/Province code |
| `postal_code` | string | No | Postal/ZIP code |
| `tax_category_id` | UUID | No | Filter by tax category |

### Response

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

## Generate OSS Report

Generate an OSS (One Stop Shop) VAT report for EU sales.

```http
POST /api/v1/tax/oss-report
```

### Request

```json
{
  "scheme": "union",
  "period": "2026-01",
  "member_state": "DE"
}
```

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `scheme` | string | Yes | `union`, `non_union`, or `import` |
| `period` | string | Yes | Report period in format `YYYY-MM` |
| `member_state` | string | Yes | Your EU member state of identification |

### Response

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

## Admin: Create Tax Zone

Create a new tax zone (admin only).

```http
POST /api/v1/admin/tax/zones
```

### Request

```json
{
  "name": "Bavaria",
  "code": "DE-BY",
  "country_code": "DE",
  "region_code": "BY",
  "zone_type": "state"
}
```

### Response

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

## Admin: Create Tax Rate

Create a new tax rate (admin only).

```http
POST /api/v1/admin/tax/rates
```

### Request

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

### Response

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

## Tax Categories

### List Tax Categories

```http
GET /api/v1/tax/categories
```

### Response

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

## EU VAT Rates Reference

### Standard VAT Rates (2026)

| Country | Code | Standard Rate | Reduced Rate |
|---------|------|---------------|--------------|
| Austria | AT | 20% | 13%, 10% |
| Belgium | BE | 21% | 12%, 6% |
| Bulgaria | BG | 20% | 9% |
| Croatia | HR | 25% | 13%, 5% |
| Cyprus | CY | 19% | 9%, 5% |
| Czech Republic | CZ | 21% | 15%, 10% |
| Denmark | DK | 25% | - |
| Estonia | EE | 22% | 9%, 5% |
| Finland | FI | 25% | 14%, 10% |
| France | FR | 20% | 10%, 5.5%, 2.1% |
| Germany | DE | 19% | 7% |
| Greece | GR | 24% | 13%, 6% |
| Hungary | HU | 27% | 18%, 5% |
| Ireland | IE | 23% | 13.5%, 9%, 4.8% |
| Italy | IT | 22% | 10%, 5%, 4% |
| Latvia | LV | 21% | 12%, 5% |
| Lithuania | LT | 21% | 9%, 5% |
| Luxembourg | LU | 17% | 14%, 8% |
| Malta | MT | 18% | 7%, 5% |
| Netherlands | NL | 21% | 9% |
| Poland | PL | 23% | 8%, 5% |
| Portugal | PT | 23% | 13%, 6% |
| Romania | RO | 19% | 9%, 5% |
| Slovakia | SK | 20% | 10% |
| Slovenia | SI | 22% | 9.5% |
| Spain | ES | 21% | 10%, 4% |
| Sweden | SE | 25% | 12%, 6% |

## US Sales Tax Nexus Thresholds (2026)

| State | Threshold | Transaction Threshold |
|-------|-----------|----------------------|
| California | $500,000 | None |
| New York | $500,000 | 100 transactions |
| Texas | $500,000 | None |
| Florida | $100,000 | None |
| Illinois | $100,000 | None |
| Pennsylvania | $100,000 | None |
| Ohio | $100,000 | 200 transactions |
| Georgia | $100,000 | 200 transactions |
| North Carolina | $100,000 | None |
| Michigan | $100,000 | 200 transactions |

## Error Codes

| Code | Description |
|------|-------------|
| `invalid_vat_id` | VAT ID format is invalid |
| `vies_unavailable` | VIES service is unavailable |
| `tax_zone_not_found` | Tax zone not found for location |
| `invalid_tax_rate` | Tax rate is invalid or expired |
| `oss_report_failed` | Failed to generate OSS report |

## Configuration

Configure tax settings in `config.toml`:

```toml
[tax]
provider = "builtin"  # or 'avalara', 'taxjar'
enable_oss = true
oss_member_state = "DE"
validate_vat_ids = true
vat_cache_days = 30

[tax.avalara]
api_key = "${AVALARA_API_KEY}"
account_id = "${AVALARA_ACCOUNT_ID}"
environment = "sandbox"  # or 'production'

[tax.taxjar]
api_token = "${TAXJAR_API_TOKEN}"
sandbox = true
```

## See Also

- [Tax System Architecture](../../architecture/13-tax-system.md)
- [EU VAT OSS Guide](https://vat-one-stop-shop.ec.europa.eu/)
- [Avalara AvaTax Documentation](https://developer.avalara.com/)
- [TaxJar API Documentation](https://developers.taxjar.com/)
