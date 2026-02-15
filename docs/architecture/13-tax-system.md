# Tax System Architecture

## Status: ✅ IMPLEMENTED

The tax system has been fully implemented in R Commerce as of February 2026.

## Quick Start

```rust
use rcommerce_core::{
    TaxService, DefaultTaxService, TaxContext, TaxAddress, TaxableItem,
    TransactionType, CustomerTaxInfo, VatId,
};

// Create tax service
let tax_service = DefaultTaxService::new(pool);

// Calculate tax
let context = TaxContext {
    customer: CustomerTaxInfo::default(),
    shipping_address: TaxAddress::new("DE").with_region("BY"),
    billing_address: TaxAddress::new("DE"),
    currency: Currency::EUR,
    transaction_type: TransactionType::B2C,
};

let calculation = tax_service.calculate_tax(&items, &context).await?;
println!("Total tax: {}", calculation.total_tax);
```

## Executive Summary

R Commerce now has **comprehensive tax infrastructure** supporting global e-commerce requirements including EU VAT with OSS, US sales tax with economic nexus tracking, and VAT/GST for major markets.

## Current State

### Existing Tax Fields

The following tax-related fields exist in the database:

| Entity | Field | Type | Description |
|--------|-------|------|-------------|
| `orders` | `tax_total` | DECIMAL(20,2) | Total tax for the order |
| `order_items` | `tax_amount` | DECIMAL(20,2) | Tax for individual line item |
| `customers` | `tax_exempt` | BOOLEAN | Customer-level tax exemption |
| `carts` | `tax_total` | DECIMAL(19,4) | Tax in active cart |

### Current Tax Calculation

Located in `crates/rcommerce-core/src/order/calculation.rs`:

```rust
pub struct OrderCalculator {
    tax_rate: Decimal,        // Single default rate
    shipping_rate: Decimal,
}
```

**Limitations:**
- Only supports a single, flat tax rate
- No geographic tax rules
- No product-specific tax categories
- No VAT/GST support
- No tax exemption handling beyond customer flag
- No tax reporting or OSS integration

## 2026 Tax Requirements

### 1. European Union VAT

#### Key Rules (2026)

| Scenario | VAT Treatment |
|----------|---------------|
| B2C intra-EU (< €10,000) | Charge seller's local VAT rate |
| B2C intra-EU (> €10,000) | Charge customer's country VAT rate |
| B2C extra-EU import (≤ €150) | Use IOSS (Import One Stop Shop) |
| B2C extra-EU import (> €150) | Customer pays VAT at customs |
| B2B intra-EU (valid VAT ID) | 0% VAT (reverse charge) |
| B2B intra-EU (no VAT ID) | Standard B2C rules apply |

#### OSS (One Stop Shop) Schemes

1. **Union OSS**: For EU businesses selling to EU consumers
2. **Non-Union OSS**: For non-EU businesses selling services to EU consumers
3. **IOSS (Import OSS)**: For imports ≤ €150

**Filing Requirements:**
- Quarterly returns for Union/Non-Union OSS
- Monthly returns for IOSS
- 10-year record retention

#### EU VAT Rates (2026)

| Country | Standard Rate | Reduced Rate | Super Reduced |
|---------|--------------|--------------|---------------|
| Germany | 19% | 7% | - |
| France | 20% | 10%, 5.5% | 2.1% |
| Italy | 22% | 10%, 5% | 4% |
| Spain | 21% | 10%, 4% | - |
| Netherlands | 21% | 9% | - |
| Belgium | 21% | 12%, 6% | - |
| Austria | 20% | 13%, 10% | - |
| Poland | 23% | 8%, 5% | - |

### 2. United States Sales Tax

#### Economic Nexus Thresholds (2026)

| State | Threshold | Transaction Rule | Notes |
|-------|-----------|------------------|-------|
| California | $500,000 | None | Gross sales (TPP only) |
| New York | $500,000 | AND 100 transactions | Previous 4 quarters |
| Texas | $500,000 | None | Gross revenue |
| Florida | $100,000 | None | Taxable sales |
| Illinois | $100,000 | None | Transaction rule repealed 2026 |
| Most states | $100,000 | Varies | See full table below |

**Key Changes 2025-2026:**
- Alaska, Utah repealed 200-transaction thresholds
- Illinois removed transaction threshold (Jan 1, 2026)
- Trend toward revenue-only thresholds

#### Sales Tax Complexity

- 45 states + DC have sales tax
- 5 states have no sales tax (DE, MT, NH, OR, AK)
- Thousands of local jurisdictions
- Different taxability rules (food, clothing, services)
- Home rule states allow local rate setting

### 3. Other Global Tax Requirements

| Region | Tax Type | Key Features |
|--------|----------|--------------|
| UK | VAT | 20% standard, post-Brexit import rules |
| Canada | GST/HST | Federal + Provincial (5-15%) |
| Australia | GST | 10%, low-value exemption removed |
| New Zealand | GST | 15%, applies to imports |
| Singapore | GST | 9% (increasing to 10%) |
| Japan | Consumption Tax | 10% (8% for food) |
| India | GST | Multiple slabs (5%, 12%, 18%, 28%) |
| Brazil | ICMS/IPI | Complex state-level system |

### 4. Customs Duties and Tariffs

#### Import Duty Calculation

```
Duty = (Product Value + Shipping + Insurance) × Duty Rate
```

#### DDP vs DDU Incoterms

| Term | Seller Pays | Buyer Pays | Use Case |
|------|-------------|------------|----------|
| DDP (Delivered Duty Paid) | Shipping + Duties + Taxes | Nothing | Premium customer experience |
| DDU/DAP (Delivered at Place) | Shipping only | Duties + Taxes at delivery | Lower upfront cost |

#### HS Codes (Harmonized System)

- 6-digit international standard
- Countries add 2-4 digits for specificity
- Determines duty rates
- Required for customs declarations

## Proposed Tax Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────────┐
│                     Tax Engine                                  │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │ Tax Rate    │  │ Tax Rule    │  │ Tax         │             │
│  │ Provider    │  │ Engine      │  │ Calculator  │             │
│  │ (External)  │  │ (Internal)  │  │ (Internal)  │             │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘             │
│         │                │                │                     │
│         ▼                ▼                ▼                     │
│  ┌─────────────────────────────────────────────────┐           │
│  │              Tax Service                        │           │
│  │  - calculate_tax(Cart/Order)                    │           │
│  │  - validate_vat_id(VatId)                       │           │
│  │  - get_tax_rates(context)                       │           │
│  │  - generate_tax_report(params)                  │           │
│  └─────────────────────────────────────────────────┘           │
└─────────────────────────────────────────────────────────────────┘
```

### Database Schema Additions

#### Tax Zones

```sql
CREATE TABLE tax_zones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    code VARCHAR(20) UNIQUE NOT NULL,
    country_code CHAR(2) NOT NULL,
    region_code VARCHAR(10), -- State/Province code
    postal_code_pattern VARCHAR(50), -- Regex for postal codes
    type VARCHAR(20) NOT NULL, -- 'country', 'state', 'city', 'custom'
    parent_id UUID REFERENCES tax_zones(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Tax Categories

```sql
CREATE TABLE tax_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    -- Product type classifications
    is_digital BOOLEAN NOT NULL DEFAULT false,
    is_food BOOLEAN NOT NULL DEFAULT false,
    is_luxury BOOLEAN NOT NULL DEFAULT false,
    is_medical BOOLEAN NOT NULL DEFAULT false,
    is_educational BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Link products to tax categories
ALTER TABLE products ADD COLUMN tax_category_id UUID REFERENCES tax_categories(id);
```

#### Tax Rates

```sql
CREATE TABLE tax_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    tax_zone_id UUID NOT NULL REFERENCES tax_zones(id),
    tax_category_id UUID REFERENCES tax_categories(id),
    
    -- Rate details
    rate DECIMAL(5,4) NOT NULL, -- e.g., 0.2000 for 20%
    type VARCHAR(20) NOT NULL DEFAULT 'percentage', -- 'percentage', 'fixed'
    
    -- VAT specific
    is_vat BOOLEAN NOT NULL DEFAULT false,
    vat_type VARCHAR(20), -- 'standard', 'reduced', 'super_reduced', 'zero'
    
    -- B2B rules
    b2b_exempt BOOLEAN NOT NULL DEFAULT false,
    reverse_charge BOOLEAN NOT NULL DEFAULT false,
    
    -- Validity period
    valid_from DATE NOT NULL DEFAULT CURRENT_DATE,
    valid_until DATE,
    
    -- Priority for overlapping zones
    priority INTEGER NOT NULL DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tax_zone_id, tax_category_id, valid_from)
);
```

#### VAT ID Validation Cache

```sql
CREATE TABLE vat_id_validations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vat_id VARCHAR(50) NOT NULL UNIQUE,
    country_code CHAR(2) NOT NULL,
    business_name VARCHAR(255),
    business_address TEXT,
    is_valid BOOLEAN NOT NULL,
    validated_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    raw_response JSONB
);
```

#### Tax Exemptions

```sql
CREATE TABLE tax_exemptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID REFERENCES customers(id),
    tax_zone_id UUID REFERENCES tax_zones(id),
    exemption_type VARCHAR(50) NOT NULL, -- 'resale', 'nonprofit', 'government', 'diplomatic'
    exemption_number VARCHAR(100),
    document_url TEXT,
    valid_from DATE NOT NULL,
    valid_until DATE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Tax Transactions (for reporting)

```sql
CREATE TABLE tax_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID REFERENCES orders(id),
    order_item_id UUID REFERENCES order_items(id),
    
    -- Tax details
    tax_rate_id UUID REFERENCES tax_rates(id),
    tax_zone_id UUID REFERENCES tax_zones(id),
    tax_category_id UUID REFERENCES tax_categories(id),
    
    -- Amounts
    taxable_amount DECIMAL(19,4) NOT NULL,
    tax_amount DECIMAL(19,4) NOT NULL,
    tax_rate DECIMAL(5,4) NOT NULL,
    
    -- Jurisdiction
    country_code CHAR(2) NOT NULL,
    region_code VARCHAR(10),
    
    -- For OSS reporting
    oss_scheme VARCHAR(20), -- 'union', 'non_union', 'ioss'
    oss_period VARCHAR(7), -- 'YYYY-MM'
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Rust Implementation

#### Core Types

```rust
// crates/rcommerce-core/src/tax/mod.rs

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tax calculation context
#[derive(Debug, Clone)]
pub struct TaxContext {
    pub customer: CustomerTaxInfo,
    pub shipping_address: Address,
    pub billing_address: Address,
    pub currency: Currency,
    pub transaction_type: TransactionType, // B2B or B2C
}

#[derive(Debug, Clone)]
pub struct CustomerTaxInfo {
    pub customer_id: Option<Uuid>,
    pub is_tax_exempt: bool,
    pub vat_id: Option<VatId>,
    pub tax_exemption_docs: Vec<TaxExemption>,
}

#[derive(Debug, Clone)]
pub struct VatId {
    pub country_code: String,
    pub number: String,
    pub is_validated: bool,
    pub validated_at: Option<DateTime<Utc>>,
}

/// Tax calculation result
#[derive(Debug, Clone)]
pub struct TaxCalculation {
    pub line_items: Vec<LineItemTax>,
    pub shipping_tax: Decimal,
    pub total_tax: Decimal,
    pub tax_breakdown: Vec<TaxBreakdown>,
}

#[derive(Debug, Clone)]
pub struct LineItemTax {
    pub item_id: Uuid,
    pub taxable_amount: Decimal,
    pub tax_amount: Decimal,
    pub tax_rate: Decimal,
    pub tax_rate_id: Uuid,
    pub tax_zone_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct TaxBreakdown {
    pub tax_zone_id: Uuid,
    pub tax_zone_name: String,
    pub tax_rate_id: Uuid,
    pub tax_rate_name: String,
    pub rate: Decimal,
    pub taxable_amount: Decimal,
    pub tax_amount: Decimal,
}
```

#### Tax Service Trait

```rust
// crates/rcommerce-core/src/tax/service.rs

#[async_trait]
pub trait TaxService: Send + Sync {
    /// Calculate tax for a cart or order
    async fn calculate_tax(
        &self,
        items: &[CartItem],
        context: &TaxContext,
    ) -> Result<TaxCalculation>;
    
    /// Validate a VAT ID using VIES
    async fn validate_vat_id(&self, vat_id: &VatId) -> Result<VatValidationResult>;
    
    /// Get applicable tax rates for a location
    async fn get_tax_rates(
        &self,
        country_code: &str,
        region_code: Option<&str>,
        postal_code: Option<&str>,
    ) -> Result<Vec<TaxRate>>;
    
    /// Generate OSS report
    async fn generate_oss_report(
        &self,
        scheme: OssScheme,
        period: &str, // "YYYY-MM"
    ) -> Result<OssReport>;
    
    /// Record tax transaction for reporting
    async fn record_tax_transaction(
        &self,
        order: &Order,
        calculation: &TaxCalculation,
    ) -> Result<()>;
}
```

#### Tax Provider Integration

```rust
// crates/rcommerce-core/src/tax/providers/mod.rs

/// External tax provider (Avalara, TaxJar, Vertex, etc.)
#[async_trait]
pub trait TaxProvider: Send + Sync {
    fn name(&self) -> &str;
    
    async fn calculate_tax(
        &self,
        request: &TaxCalculationRequest,
    ) -> Result<TaxCalculationResponse>;
    
    async fn validate_address(&self, address: &Address) -> Result<ValidatedAddress>;
}

// Built-in provider for simple tax rules
pub struct BuiltInTaxProvider {
    db: PgPool,
}

// Avalara AvaTax integration
pub struct AvalaraProvider {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

// TaxJar integration
pub struct TaxJarProvider {
    client: reqwest::Client,
    api_token: String,
}
```

### API Endpoints

```rust
// crates/rcommerce-api/src/routes/tax.rs

/// Calculate tax for current cart
async fn calculate_cart_tax(
    State(state): State<AppState>,
    Json(request): Json<CalculateTaxRequest>,
) -> Result<Json<TaxCalculation>, ApiError>;

/// Validate VAT ID
async fn validate_vat_id(
    State(state): State<AppState>,
    Json(vat_id): Json<VatIdRequest>,
) -> Result<Json<VatValidationResponse>, ApiError>;

/// Get tax rates for a location
async fn get_tax_rates(
    State(state): State<AppState>,
    Query(params): Query<TaxRateQuery>,
) -> Result<Json<Vec<TaxRate>>, ApiError>;

/// Admin: Create tax rate
async fn create_tax_rate(
    State(state): State<AppState>,
    Auth(auth): Auth,
    Json(rate): Json<CreateTaxRateRequest>,
) -> Result<Json<TaxRate>, ApiError>;

/// Admin: Generate OSS report
async fn generate_oss_report(
    State(state): State<AppState>,
    Auth(auth): Auth,
    Query(params): Query<OssReportQuery>,
) -> Result<Json<OssReport>, ApiError>;
```

### Configuration

```toml
# config.toml

[tax]
# Default tax provider: 'builtin', 'avalara', 'taxjar', 'vertex'
provider = "builtin"

# Enable OSS reporting
enable_oss = true
oss_member_state = "DE" # Your EU member state of identification

# Default tax behavior
default_tax_included = false  # Prices include tax?
default_tax_zone = "US"       # Fallback tax zone

# VAT validation
validate_vat_ids = true
vat_cache_days = 30

[tax.avalara]
api_key = "${AVALARA_API_KEY}"
account_id = "${AVALARA_ACCOUNT_ID}"
environment = "sandbox" # or 'production'

[tax.taxjar]
api_token = "${TAXJAR_API_TOKEN}"
```

## Implementation Phases

### Phase 1: Basic Tax Infrastructure (Weeks 1-2)

1. Create tax tables (zones, categories, rates)
2. Implement built-in tax provider
3. Update order calculation to use tax service
4. Add tax breakdown to order responses

### Phase 2: EU VAT Support (Weeks 3-4)

1. VAT ID validation via VIES API
2. OSS scheme support
3. B2B reverse charge logic
4. EU VAT rate database

### Phase 3: US Sales Tax (Weeks 5-6)

1. State tax rate database
2. Economic nexus tracking
3. Optional: Avalara/TaxJar integration
4. Local jurisdiction support

### Phase 4: Advanced Features (Weeks 7-8)

1. Tax reporting and OSS filing exports
2. Tax exemption management
3. Customs duties calculation
4. Tax audit trail

## Integration with Existing Systems

### Order Lifecycle

```
Cart/Checkout → TaxService::calculate_tax() → Order created with tax breakdown
                    ↓
            Tax transactions recorded
                    ↓
            OSS report generated quarterly
```

### Product Catalog

- Products linked to tax categories
- Tax categories define default tax treatment
- Can override at product level

### Customer Management

- VAT ID stored and validated
- Tax exemptions tracked
- B2B/B2C classification

## Compliance Considerations

### Record Retention

| Jurisdiction | Retention Period |
|--------------|------------------|
| EU VAT | 10 years |
| US Sales Tax | 3-7 years (varies by state) |
| Australia GST | 5 years |

### Security

- VAT IDs are PII - encrypt at rest
- Tax exemption documents - secure storage
- Audit logs for all tax calculations

### Testing Requirements

- Tax calculation accuracy tests
- VAT ID validation tests
- OSS report accuracy tests
- Edge cases (exemptions, zero rates, etc.)

## Recommended External Services

| Service | Use Case | Cost |
|---------|----------|------|
| Avalara AvaTax | US Sales Tax + Global VAT | $$$ |
| TaxJar | US Sales Tax | $$ |
| Vertex | Enterprise tax management | $$$$ |
| VIES | EU VAT ID validation (free) | Free |
| Loqate | Address validation | $$ |

## Migration Path

1. **Backward Compatibility**: Existing `tax_total` fields remain
2. **Data Migration**: Populate new tax transaction table from historical orders
3. **Gradual Rollout**: Feature flags for new tax engine
4. **Testing**: Parallel calculation validation

## Conclusion

The proposed tax architecture provides:

- ✅ Support for EU VAT with OSS
- ✅ US sales tax with economic nexus tracking
- ✅ Global tax compliance
- ✅ Flexible provider integration
- ✅ Comprehensive reporting
- ✅ Audit trail for compliance

**Estimated effort**: 8 weeks for full implementation
**Priority**: High - required for EU and US market compliance
