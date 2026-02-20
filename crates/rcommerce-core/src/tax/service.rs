//! Tax Service
//!
//! Main tax service implementation for calculating taxes and managing tax data.
//! Uses TaxRepository for database operations to maintain clean architecture.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::tax::{
    calculator::TaxCalculator, models::*, vat_validation::*, CustomerTaxInfo, OssReport,
    OssScheme, OssTransaction, OssSummary, CountrySummary, TaxAddress, TaxCalculation, TaxCategory, TaxContext, TaxExemption, TaxRate,
    TaxTransaction, TaxZone, TaxableItem, VatId, VatValidationCache,
};
use crate::repository::{
    TaxRepository, TaxZoneQuery,
    CreateTaxZoneRequest as RepoCreateTaxZoneRequest,
    CreateTaxCategoryRequest as RepoCreateTaxCategoryRequest,
    CreateTaxRateRequest as RepoCreateTaxRateRequest,
};
use crate::{Error, Result};

/// Tax service trait
#[async_trait]
pub trait TaxService: Send + Sync {
    /// Calculate tax for items
    async fn calculate_tax(
        &self,
        items: &[TaxableItem],
        context: &TaxContext,
    ) -> Result<TaxCalculation>;

    /// Validate a VAT ID
    async fn validate_vat_id(&self, vat_id: &str) -> Result<VatValidationResult>;

    /// Get applicable tax rates for a location
    async fn get_tax_rates(
        &self,
        country_code: &str,
        region_code: Option<&str>,
        postal_code: Option<&str>,
    ) -> Result<Vec<TaxRateWithDetails>>;

    /// Generate OSS report
    async fn generate_oss_report(
        &self,
        scheme: OssScheme,
        period: &str,
        member_state: &str,
    ) -> Result<OssReport>;

    /// Record tax transaction
    async fn record_tax_transaction(
        &self,
        order_id: Uuid,
        calculation: &TaxCalculation,
    ) -> Result<()>;

    /// Create tax zone
    async fn create_tax_zone(&self, request: CreateTaxZoneRequest) -> Result<TaxZone>;

    /// Create tax category
    async fn create_tax_category(&self, request: CreateTaxCategoryRequest) -> Result<TaxCategory>;

    /// Create tax rate
    async fn create_tax_rate(&self, request: CreateTaxRateRequest) -> Result<TaxRate>;

    /// Get tax zones
    async fn get_tax_zones(&self, query: &TaxZoneQuery) -> Result<Vec<TaxZone>>;

    /// Get tax categories
    async fn get_tax_categories(&self) -> Result<Vec<TaxCategory>>;
}

/// Default tax service implementation
pub struct DefaultTaxService {
    repo: Box<dyn TaxRepository>,
    vat_validator: ViesValidator,
}

impl DefaultTaxService {
    /// Create a new tax service
    pub fn new(repo: Box<dyn TaxRepository>) -> Self {
        Self {
            repo,
            vat_validator: ViesValidator::new(),
        }
    }

    /// Create with custom VAT validator
    pub fn with_validator(repo: Box<dyn TaxRepository>, validator: ViesValidator) -> Self {
        Self {
            repo,
            vat_validator: validator,
        }
    }

    /// Find or create tax zone for address
    async fn find_tax_zone(&self, address: &TaxAddress) -> Result<TaxZone> {
        // Try to find most specific zone
        
        // 1. Try postal code pattern match
        if let Some(postal) = &address.postal_code {
            let zones = self.repo.find_zones(&TaxZoneQuery {
                country_code: Some(address.country_code.clone()),
                zone_type: None,
                search: None,
            }).await?;
            
            // Find zone with matching postal code pattern
            for zone in zones {
                if let Some(ref pattern) = zone.postal_code_pattern {
                    if regex::Regex::new(pattern)
                        .map_or(false, |re| re.is_match(postal)) {
                        return Ok(zone);
                    }
                }
            }
        }

        // 2. Try region match
        if let Some(region) = &address.region_code {
            let zones = self.repo.find_zones(&TaxZoneQuery {
                country_code: Some(address.country_code.clone()),
                zone_type: None,
                search: None,
            }).await?;
            
            for zone in zones {
                if zone.region_code.as_ref() == Some(region) {
                    return Ok(zone);
                }
            }
        }

        // 3. Fall back to country-level zone
        let zones = self.repo.find_zones(&TaxZoneQuery {
            country_code: Some(address.country_code.clone()),
            zone_type: None,
            search: None,
        }).await?;
        
        for zone in zones {
            if zone.region_code.is_none() && zone.postal_code_pattern.is_none() {
                return Ok(zone);
            }
        }

        // 4. Create default zone if none exists
        warn!("Creating default tax zone for {}", address.country_code);
        self.create_default_zone(&address.country_code).await
    }

    /// Create default tax zone for a country
    async fn create_default_zone(&self, country_code: &str) -> Result<TaxZone> {
        let request = RepoCreateTaxZoneRequest {
            name: format!("Default {}", country_code),
            code: country_code.to_uppercase(),
            country_code: country_code.to_uppercase(),
            region_code: None,
            postal_code_pattern: None,
            zone_type: "country".to_string(),
            parent_id: None,
        };
        
        self.repo.create_zone(request).await
    }
}

#[async_trait]
impl TaxService for DefaultTaxService {
    async fn calculate_tax(
        &self,
        items: &[TaxableItem],
        context: &TaxContext,
    ) -> Result<TaxCalculation> {
        info!("Calculating tax for {} items", items.len());

        // Get tax zone for destination
        let tax_zone = self.find_tax_zone(&context.shipping_address).await?;
        debug!("Using tax zone: {} ({})", tax_zone.name, tax_zone.code);

        // Get tax rates for zone
        let rates = self.repo.find_rates_for_zone(tax_zone.id).await?;

        // Get tax categories
        let categories = self.repo.list_categories().await?;

        // Build calculator
        let calculator = TaxCalculator::new(rates, vec![tax_zone], categories);

        // Calculate tax
        let calculation = calculator.calculate(items, context)?;

        info!(
            "Tax calculation complete: total_tax={}",
            calculation.total_tax
        );

        Ok(calculation)
    }

    async fn validate_vat_id(&self, vat_id_str: &str) -> Result<VatValidationResult> {
        info!("Validating VAT ID: {}", vat_id_str);

        // Parse VAT ID
        let vat_id = VatId::parse(vat_id_str)?;

        // Check cache first
        let cached = self.repo.get_vat_validation(vat_id.full_id()).await?;

        if let Some(cache) = cached {
            debug!("Using cached VAT validation for {}", vat_id.full_id());
            return Ok(VatValidationResult {
                is_valid: cache.is_valid,
                country_code: cache.country_code,
                vat_number: vat_id.number.clone(),
                business_name: cache.business_name,
                business_address: cache.business_address,
                validated_at: cache.validated_at,
                error_message: None,
            });
        }

        // Validate via VIES
        let result = self.vat_validator.validate(&vat_id).await?;

        // Cache result
        let cache = VatValidationCache {
            id: Uuid::new_v4(),
            vat_id: vat_id.full_id(),
            country_code: result.country_code.clone(),
            business_name: result.business_name.clone(),
            business_address: result.business_address.clone(),
            is_valid: result.is_valid,
            validated_at: result.validated_at,
            expires_at: result.validated_at + chrono::Duration::days(30),
        };
        
        if let Err(e) = self.repo.cache_vat_validation(&cache).await {
            warn!("Failed to cache VAT validation: {}", e);
        }

        Ok(result)
    }

    async fn get_tax_rates(
        &self,
        country_code: &str,
        region_code: Option<&str>,
        postal_code: Option<&str>,
    ) -> Result<Vec<TaxRateWithDetails>> {
        let address = TaxAddress {
            country_code: country_code.to_string(),
            region_code: region_code.map(|s| s.to_string()),
            postal_code: postal_code.map(|s| s.to_string()),
            city: None,
        };

        let zone = self.find_tax_zone(&address).await?;
        let rates = self.repo.find_rates_for_zone(zone.id).await?;

        // Fetch zone and category info for each rate
        let mut result = Vec::new();
        for rate in rates {
            let category = if let Some(cat_id) = rate.tax_category_id {
                self.repo.find_category_by_id(cat_id).await?
            } else {
                None
            };

            result.push(TaxRateWithDetails {
                id: rate.id,
                name: rate.name.clone(),
                rate: rate.rate,
                rate_type: rate.rate_type.clone(),
                is_vat: rate.is_vat,
                vat_type: rate.vat_type.clone(),
                b2b_exempt: rate.b2b_exempt,
                reverse_charge: rate.reverse_charge,
                tax_zone: TaxZoneInfo {
                    id: zone.id,
                    name: zone.name.clone(),
                    code: zone.code.clone(),
                    country_code: zone.country_code.clone(),
                },
                tax_category: category.map(|c| TaxCategoryInfo {
                    id: c.id,
                    name: c.name,
                    code: c.code,
                }),
                valid_from: rate.valid_from,
                valid_until: rate.valid_until,
            });
        }

        Ok(result)
    }

    async fn generate_oss_report(
        &self,
        scheme: OssScheme,
        period: &str,
        member_state: &str,
    ) -> Result<OssReport> {
        info!(
            "Generating OSS report: scheme={:?}, period={}, member_state={}",
            scheme, period, member_state
        );

        // Fetch transactions for period
        let transactions = self.repo.get_transactions_for_oss(scheme, period).await?;

        // Aggregate by country and rate
        let mut country_map: std::collections::HashMap<(String, Decimal), (Decimal, Decimal, i32)> = 
            std::collections::HashMap::new();
        
        for txn in transactions {
            let key = (txn.country_code.clone(), txn.tax_rate);
            let entry = country_map.entry(key).or_insert((Decimal::ZERO, Decimal::ZERO, 0));
            entry.0 += txn.taxable_amount;
            entry.1 += txn.tax_amount;
            entry.2 += 1;
        }

        let oss_transactions: Vec<OssTransaction> = country_map
            .iter()
            .map(|((country_code, vat_rate), (taxable, tax, count))| OssTransaction {
                country_code: country_code.clone(),
                vat_rate: *vat_rate,
                taxable_amount: *taxable,
                vat_amount: *tax,
                transaction_count: *count,
            })
            .collect();

        let total_taxable: Decimal = oss_transactions.iter().map(|t| t.taxable_amount).sum();
        let total_tax: Decimal = oss_transactions.iter().map(|t| t.vat_amount).sum();
        let total_count: i32 = oss_transactions.iter().map(|t| t.transaction_count).sum();

        let by_country: Vec<CountrySummary> = oss_transactions
            .iter()
            .map(|t| CountrySummary {
                country_code: t.country_code.clone(),
                country_name: country_name(&t.country_code),
                vat_rate: t.vat_rate,
                taxable_amount: t.taxable_amount,
                vat_amount: t.vat_amount,
                transaction_count: t.transaction_count,
            })
            .collect();

        Ok(OssReport {
            scheme,
            period: period.to_string(),
            member_state: member_state.to_string(),
            transactions: oss_transactions,
            summary: OssSummary {
                total_taxable_amount: total_taxable,
                total_vat_amount: total_tax,
                total_transactions: total_count,
                by_country,
            },
        })
    }

    async fn record_tax_transaction(
        &self,
        order_id: Uuid,
        calculation: &TaxCalculation,
    ) -> Result<()> {
        debug!("Recording tax transactions for order {}", order_id);

        for line_item in &calculation.line_items {
            let transaction = TaxTransaction {
                id: Uuid::new_v4(),
                order_id,
                order_item_id: Some(line_item.item_id),
                tax_rate_id: line_item.tax_rate_id,
                tax_zone_id: line_item.tax_zone_id,
                tax_category_id: None, // TODO: Get from item
                taxable_amount: line_item.taxable_amount,
                tax_amount: line_item.tax_amount,
                tax_rate: line_item.tax_rate,
                country_code: String::new(), // TODO: Get from zone
                region_code: None,
                oss_scheme: None, // TODO: Determine from context
                oss_period: None,
                created_at: Utc::now(),
            };
            
            self.repo.record_transaction(&transaction).await?;
        }

        Ok(())
    }

    async fn create_tax_zone(&self, request: CreateTaxZoneRequest) -> Result<TaxZone> {
        info!("Creating tax zone: {} ({})", request.name, request.code);

        let repo_request = RepoCreateTaxZoneRequest {
            name: request.name,
            code: request.code.to_uppercase(),
            country_code: request.country_code.to_uppercase(),
            region_code: request.region_code.map(|s| s.to_uppercase()),
            postal_code_pattern: request.postal_code_pattern,
            zone_type: request.zone_type.to_string(),
            parent_id: request.parent_id,
        };

        self.repo.create_zone(repo_request).await
    }

    async fn create_tax_category(&self, request: CreateTaxCategoryRequest) -> Result<TaxCategory> {
        info!("Creating tax category: {} ({})", request.name, request.code);

        let repo_request = RepoCreateTaxCategoryRequest {
            name: request.name,
            code: request.code.to_uppercase(),
            description: request.description,
            is_digital: request.is_digital,
            is_food: request.is_food,
            is_luxury: request.is_luxury,
            is_medical: request.is_medical,
            is_educational: request.is_educational,
        };

        self.repo.create_category(repo_request).await
    }

    async fn create_tax_rate(&self, request: CreateTaxRateRequest) -> Result<TaxRate> {
        info!("Creating tax rate: {} = {}%", request.name, request.rate * Decimal::from(100));

        let vat_type_str = request.vat_type.map(|v| v.to_string());

        let repo_request = RepoCreateTaxRateRequest {
            name: request.name,
            tax_zone_id: request.tax_zone_id,
            tax_category_id: request.tax_category_id,
            rate: request.rate,
            rate_type: request.rate_type.to_string(),
            is_vat: request.is_vat,
            vat_type: vat_type_str,
            b2b_exempt: request.b2b_exempt,
            reverse_charge: request.reverse_charge,
            valid_from: request.valid_from,
            valid_until: request.valid_until,
            priority: request.priority,
        };

        self.repo.create_rate(repo_request).await
    }

    async fn get_tax_zones(&self, query: &TaxZoneQuery) -> Result<Vec<TaxZone>> {
        self.repo.find_zones(query).await
    }

    async fn get_tax_categories(&self) -> Result<Vec<TaxCategory>> {
        self.repo.list_categories().await
    }
}

/// Get country name from code
fn country_name(code: &str) -> String {
    let countries: std::collections::HashMap<&str, &str> = [
        ("AT", "Austria"),
        ("BE", "Belgium"),
        ("BG", "Bulgaria"),
        ("HR", "Croatia"),
        ("CY", "Cyprus"),
        ("CZ", "Czech Republic"),
        ("DK", "Denmark"),
        ("EE", "Estonia"),
        ("FI", "Finland"),
        ("FR", "France"),
        ("DE", "Germany"),
        ("GR", "Greece"),
        ("HU", "Hungary"),
        ("IE", "Ireland"),
        ("IT", "Italy"),
        ("LV", "Latvia"),
        ("LT", "Lithuania"),
        ("LU", "Luxembourg"),
        ("MT", "Malta"),
        ("NL", "Netherlands"),
        ("PL", "Poland"),
        ("PT", "Portugal"),
        ("RO", "Romania"),
        ("SK", "Slovakia"),
        ("SI", "Slovenia"),
        ("ES", "Spain"),
        ("SE", "Sweden"),
    ]
    .into();

    countries.get(code).map(|&s| s.to_string()).unwrap_or_else(|| code.to_string())
}
