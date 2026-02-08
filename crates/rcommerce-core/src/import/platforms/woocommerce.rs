//! WooCommerce importer implementation

use crate::import::{
    error::{ImportError, ImportResult},
    types::{ImportConfig, ImportProgress, ImportStats},
    PlatformImporter,
};
use crate::models::{
    CreateProductRequest, UpdateProductRequest, Currency, ProductType,
    InventoryPolicy, WeightUnit, OrderStatus, PaymentStatus, FulfillmentStatus,
};
use crate::repository::{ProductRepository, Database};

use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

/// WooCommerce API client and importer
pub struct WooCommerceImporter {
    client: Client,
}

impl WooCommerceImporter {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Get WooCommerce API URL
    fn api_url(&self, base_url: &str, endpoint: &str) -> String {
        format!("{}/wp-json/wc/v3/{}", base_url.trim_end_matches('/'), endpoint)
    }

    /// Create database pool from config
    async fn create_pool(&self, database_url: &str) -> ImportResult<PgPool> {
        use sqlx::postgres::PgPoolOptions;
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| ImportError::Database(crate::Error::Database(e)))?;
        
        Ok(pool)
    }

    /// Fetch paginated data from WooCommerce
    async fn fetch_paginated<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        consumer_key: &str,
        consumer_secret: &str,
        limit: usize,
    ) -> ImportResult<Vec<T>> {
        let mut results = Vec::new();
        let mut page = 1;
        // WooCommerce requires per_page between 1-100, default to 100 if no limit
        let per_page = if limit == 0 { 100 } else { 100.min(limit) };

        loop {
            let request_url = format!("{}?page={}&per_page={}", url, page, per_page);

            let response = self
                .client
                .get(&request_url)
                .basic_auth(consumer_key, Some(consumer_secret))
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                
                // Provide helpful error messages for common issues
                let help_msg = if status == 404 {
                    format!(
                        "\n\nHelp: The WooCommerce REST API endpoint was not found (404).\n\
                        Common causes:\n\
                        1. Incorrect API URL - verify your store URL (e.g., https://example.com)\n\
                        2. WooCommerce REST API not enabled - enable in WP Admin > WooCommerce > Settings > Advanced > REST API\n\
                        3. Permalinks not configured - set to 'Post name' in WP Admin > Settings > Permalinks\n\
                        4. Pretty permalinks not supported - try adding ?rest_route=/wc/v3/ to your URL\n\
                        Requested URL: {}",
                        request_url
                    )
                } else if status == 401 {
                    "\n\nHelp: Authentication failed. Verify your consumer key and secret are correct.".to_string()
                } else {
                    String::new()
                };
                
                return Err(ImportError::Api(format!(
                    "WooCommerce API error: {} - {}{}",
                    status, body, help_msg
                )));
            }

            // Parse JSON with better error handling
            let body_text = response.text().await?;
            let items: Vec<T> = match serde_json::from_str(&body_text) {
                Ok(items) => items,
                Err(e) => {
                    // Try to parse as error response
                    if let Ok(error_json) = serde_json::from_str::<serde_json::Value>(&body_text) {
                        if let Some(code) = error_json.get("code").and_then(|c| c.as_str()) {
                            return Err(ImportError::Api(format!(
                                "WooCommerce API returned error: {} - {}",
                                code,
                                error_json.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error")
                            )));
                        }
                    }
                    
                    return Err(ImportError::Api(format!(
                        "Failed to parse WooCommerce response: {}. Response preview: {:.200}",
                        e,
                        body_text
                    )));
                }
            };
            let item_count = items.len();

            results.extend(items);

            if limit > 0 && results.len() >= limit {
                results.truncate(limit);
                break;
            }

            if item_count < per_page {
                break;
            }

            page += 1;
        }

        Ok(results)
    }

    /// Convert WooCommerce price string to Decimal
    fn parse_price(&self, price: &str) -> Decimal {
        price.parse::<Decimal>().unwrap_or(Decimal::ZERO)
    }

    /// Map WooCommerce status to OrderStatus
    fn map_order_status(&self, status: &str) -> OrderStatus {
        match status {
            "pending" => OrderStatus::Pending,
            "processing" => OrderStatus::Processing,
            "on-hold" => OrderStatus::OnHold,
            "completed" => OrderStatus::Completed,
            "cancelled" => OrderStatus::Cancelled,
            "refunded" => OrderStatus::Refunded,
            "failed" => OrderStatus::Cancelled,
            _ => OrderStatus::Pending,
        }
    }

    /// Map WooCommerce currency string to Currency
    fn parse_currency(&self, currency: &str) -> Currency {
        match currency {
            "USD" => Currency::USD,
            "EUR" => Currency::EUR,
            "GBP" => Currency::GBP,
            "JPY" => Currency::JPY,
            "AUD" => Currency::AUD,
            "CAD" => Currency::CAD,
            "CNY" => Currency::CNY,
            "HKD" => Currency::HKD,
            "SGD" => Currency::SGD,
            _ => Currency::USD, // Default to USD
        }
    }
}

impl Default for WooCommerceImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PlatformImporter for WooCommerceImporter {
    fn platform(&self) -> &'static str {
        "woocommerce"
    }

    async fn import_products(
        &self,
        config: &ImportConfig,
        progress: &(dyn Fn(ImportProgress) + Send + Sync),
    ) -> ImportResult<ImportStats> {
        let dry_run = config.options.dry_run;
        let (base_url, consumer_key, consumer_secret) = match &config.source {
            crate::import::types::SourceConfig::Platform {
                api_url,
                api_key,
                headers,
                ..
            } => {
                let secret = headers
                    .get("consumer_secret")
                    .cloned()
                    .unwrap_or_default();
                (api_url.clone(), api_key.clone(), secret)
            }
            _ => {
                return Err(ImportError::Configuration(
                    "Invalid source configuration for WooCommerce".to_string(),
                ))
            }
        };

        progress(ImportProgress {
            stage: "products".to_string(),
            current: 0,
            total: 1,
            message: if dry_run {
                "Fetching products from WooCommerce (DRY RUN)...".to_string()
            } else {
                "Fetching products from WooCommerce...".to_string()
            },
        });

        let url = self.api_url(&base_url, "products");
        tracing::info!("WooCommerce API URL: {}", url);
        
        let wc_products: Vec<WooCommerceProduct> = self
            .fetch_paginated(&url, &consumer_key, &consumer_secret, config.options.limit)
            .await?;

        let mut stats = ImportStats {
            total: wc_products.len(),
            ..Default::default()
        };

        // Create database pool and repository if not dry run
        let pool = if !dry_run {
            match self.create_pool(&config.database_url).await {
                Ok(pool) => {
                    tracing::info!("Database pool created successfully");
                    Some(pool)
                }
                Err(e) => {
                    tracing::error!("Failed to create database pool: {}", e);
                    return Err(e);
                }
            }
        } else {
            tracing::info!("Dry run mode - no database connection needed");
            None
        };

        for (i, product) in wc_products.iter().enumerate() {
            progress(ImportProgress {
                stage: "products".to_string(),
                current: i + 1,
                total: wc_products.len(),
                message: if dry_run {
                    format!("Validating: {}", product.name)
                } else {
                    format!("Importing: {}", product.name)
                },
            });

            // Validate product
            if product.name.is_empty() {
                stats.errors += 1;
                stats.error_details.push(format!(
                    "Product {} has no name",
                    product.id
                ));
                continue;
            }

            if dry_run {
                stats.created += 1;
            } else {
                // Create product in database using repository pattern
                if let Some(ref pool) = pool {
                    // Generate slug for lookup
                    let slug = if product.slug.is_empty() {
                        product.name.to_lowercase().replace(" ", "-")
                    } else {
                        product.slug.clone()
                    };

                    // Create repository
                    let db = Database::new(pool.clone());
                    let repo = ProductRepository::new(db);

                    // Check for existing product by slug
                    let existing = repo.find_by_slug(&slug).await
                        .map_err(|e| ImportError::Database(e))?;

                    // Parse price - use sale_price if available, otherwise regular_price or price
                    let price = if !product.sale_price.is_empty() && product.sale_price != "0" {
                        self.parse_price(&product.sale_price)
                    } else if !product.regular_price.is_empty() {
                        self.parse_price(&product.regular_price)
                    } else {
                        self.parse_price(&product.price)
                    };

                    let compare_at_price = if !product.regular_price.is_empty() && product.sale_price != "0" {
                        Some(self.parse_price(&product.regular_price))
                    } else {
                        None
                    };

                    // Determine product type
                    let product_type = match product.product_type.as_str() {
                        "simple" => ProductType::Simple,
                        "variable" => ProductType::Variable,
                        "subscription" => ProductType::Subscription,
                        "bundle" => ProductType::Bundle,
                        _ => ProductType::Simple,
                    };

                    if let Some(existing_product) = existing {
                        // Product exists - check if we should update
                        if config.options.update_existing {
                            // Update existing product using repository
                            let update_request = UpdateProductRequest {
                                title: Some(product.name.clone()),
                                slug: None, // Keep existing slug
                                description: Some(product.description.clone()),
                                sku: Some(product.sku.clone()),
                                price: Some(price),
                                compare_at_price: Some(compare_at_price),
                                cost_price: None,
                                currency: None,
                                inventory_quantity: Some(product.stock_quantity.unwrap_or(0)),
                                inventory_policy: None,
                                inventory_management: Some(product.manage_stock),
                                continues_selling_when_out_of_stock: None,
                                weight: None,
                                weight_unit: None,
                                requires_shipping: None,
                                is_active: Some(product.status == "publish"),
                                is_featured: None,
                                seo_title: Some(Some(product.name.clone())),
                                seo_description: Some(product.short_description.clone()),
                                product_type: Some(product_type),
                                subscription_interval: None,
                                subscription_interval_count: None,
                                subscription_trial_days: None,
                                subscription_setup_fee: None,
                                subscription_min_cycles: None,
                                subscription_max_cycles: None,
                                file_url: None,
                                file_size: None,
                                file_hash: None,
                                download_limit: None,
                                license_key_enabled: None,
                                download_expiry_days: None,
                                bundle_pricing_strategy: None,
                                bundle_discount_percentage: None,
                            };

                            match repo.update_with_request(existing_product.id, update_request).await {
                                Ok(_) => {
                                    stats.updated += 1;
                                    tracing::info!("Updated product: {}", product.name);
                                }
                                Err(e) => {
                                    stats.errors += 1;
                                    let error_msg = format!(
                                        "Failed to update product '{}': {}",
                                        product.name, e
                                    );
                                    tracing::error!("{}", error_msg);
                                    stats.error_details.push(error_msg);
                                    if !config.options.continue_on_error {
                                        break;
                                    }
                                }
                            }
                        } else {
                            // Skip existing product
                            tracing::info!("Product with slug '{}' already exists, skipping", slug);
                            stats.skipped += 1;
                            continue;
                        }
                    } else {
                        // Create new product
                        let create_request = CreateProductRequest {
                            title: product.name.clone(),
                            slug: if product.slug.is_empty() {
                                product.name.to_lowercase().replace(" ", "-")
                            } else {
                                product.slug.clone()
                            },
                            description: product.description.clone(),
                            sku: product.sku.clone(),
                            product_type,
                            price,
                            compare_at_price,
                            cost_price: None,
                            currency: Currency::USD, // Default, could be configurable
                            inventory_quantity: product.stock_quantity.unwrap_or(0),
                            inventory_policy: InventoryPolicy::Deny,
                            inventory_management: product.manage_stock,
                            continues_selling_when_out_of_stock: false,
                            weight: None,
                            weight_unit: Some(WeightUnit::Lb),
                            requires_shipping: product_type != ProductType::Digital,
                            is_active: product.status == "publish",
                            is_featured: false,
                            seo_title: Some(product.name.clone()),
                            seo_description: product.short_description.clone(),
                            subscription_interval: None,
                            subscription_interval_count: None,
                            subscription_trial_days: None,
                            subscription_setup_fee: None,
                            subscription_min_cycles: None,
                            subscription_max_cycles: None,
                            file_url: None,
                            file_size: None,
                            file_hash: None,
                            download_limit: None,
                            license_key_enabled: None,
                            download_expiry_days: None,
                            bundle_pricing_strategy: None,
                            bundle_discount_percentage: None,
                            attributes: None,
                            bundle_components: None,
                        };

                        match repo.create_with_request(create_request).await {
                            Ok(_created_product) => {
                                stats.created += 1;
                                tracing::info!("Created product: {}", product.name);
                                // TODO: Import product images if available
                                // TODO: Import product categories if available
                            }
                            Err(e) => {
                                stats.errors += 1;
                                let error_msg = format!(
                                    "Failed to create product '{}': {}",
                                    product.name, e
                                );
                                tracing::error!("{}", error_msg);
                                stats.error_details.push(error_msg);
                                if !config.options.continue_on_error {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(stats)
    }

    async fn import_customers(
        &self,
        config: &ImportConfig,
        progress: &(dyn Fn(ImportProgress) + Send + Sync),
    ) -> ImportResult<ImportStats> {
        let dry_run = config.options.dry_run;
        let (base_url, consumer_key, consumer_secret) = match &config.source {
            crate::import::types::SourceConfig::Platform {
                api_url,
                api_key,
                headers,
                ..
            } => {
                let secret = headers
                    .get("consumer_secret")
                    .cloned()
                    .unwrap_or_default();
                (api_url.clone(), api_key.clone(), secret)
            }
            _ => {
                return Err(ImportError::Configuration(
                    "Invalid source configuration for WooCommerce".to_string(),
                ))
            }
        };

        progress(ImportProgress {
            stage: "customers".to_string(),
            current: 0,
            total: 1,
            message: if dry_run {
                "Fetching customers from WooCommerce (DRY RUN)...".to_string()
            } else {
                "Fetching customers from WooCommerce...".to_string()
            },
        });

        let url = self.api_url(&base_url, "customers");
        let wc_customers: Vec<WooCommerceCustomer> = self
            .fetch_paginated(&url, &consumer_key, &consumer_secret, config.options.limit)
            .await?;

        let mut stats = ImportStats {
            total: wc_customers.len(),
            ..Default::default()
        };

        // Create database pool and repository if not dry run
        let pool = if !dry_run {
            Some(self.create_pool(&config.database_url).await?)
        } else {
            None
        };

        for (i, customer) in wc_customers.iter().enumerate() {
            progress(ImportProgress {
                stage: "customers".to_string(),
                current: i + 1,
                total: wc_customers.len(),
                message: if dry_run {
                    format!("Validating customer {}/{}", i + 1, wc_customers.len())
                } else {
                    format!("Importing customer {}/{}", i + 1, wc_customers.len())
                },
            });

            // Validate customer
            if customer.email.is_empty() {
                stats.errors += 1;
                stats.error_details.push(format!(
                    "Customer {} has no email",
                    customer.id
                ));
                continue;
            }

            if dry_run {
                stats.created += 1;
            } else {
                // Create customer in database
                if let Some(ref pool) = pool {
                    // Check for existing customer by email (using raw SQL to avoid role column issue)
                    let existing: Option<(Uuid,)> = sqlx::query_as(
                        "SELECT id FROM customers WHERE email = $1"
                    )
                    .bind(&customer.email)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| ImportError::Database(crate::Error::Database(e)))?;
                    
                    // Handle existing customer
                    if let Some((existing_id,)) = existing {
                        if config.options.skip_existing {
                            stats.skipped += 1;
                            continue;
                        }
                        if config.options.update_existing {
                            // Update existing customer
                            let first_name = if customer.first_name.is_empty() {
                                customer.username.clone()
                            } else {
                                customer.first_name.clone()
                            };
                            let last_name = customer.last_name.clone();
                            let phone = customer.billing.as_ref().and_then(|b| b.phone.clone());
                            
                            match sqlx::query(
                                r#"
                                UPDATE customers 
                                SET first_name = $1, last_name = $2, phone = $3, updated_at = NOW()
                                WHERE id = $4
                                "#
                            )
                            .bind(&first_name)
                            .bind(&last_name)
                            .bind(&phone)
                            .bind(existing_id)
                            .execute(pool)
                            .await {
                                Ok(_) => {
                                    stats.updated += 1;
                                    tracing::info!("Updated customer: {}", customer.email);
                                }
                                Err(e) => {
                                    stats.errors += 1;
                                    stats.error_details.push(format!(
                                        "Failed to update customer '{}': {}",
                                        customer.email, e
                                    ));
                                }
                            }
                            continue;
                        }
                        stats.skipped += 1;
                        continue;
                    }

                    // Create customer using raw SQL (without role column)
                    let customer_id = Uuid::new_v4();
                    let first_name = if customer.first_name.is_empty() {
                        customer.username.clone()
                    } else {
                        customer.first_name.clone()
                    };
                    let last_name = customer.last_name.clone();
                    let phone = customer.billing.as_ref().and_then(|b| b.phone.clone());
                    
                    match sqlx::query(
                        r#"
                        INSERT INTO customers (
                            id, email, first_name, last_name, phone, accepts_marketing, 
                            tax_exempt, currency, is_verified, marketing_opt_in,
                            email_notifications, sms_notifications, push_notifications,
                            created_at, updated_at
                        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
                        "#
                    )
                    .bind(customer_id)
                    .bind(&customer.email)
                    .bind(&first_name)
                    .bind(&last_name)
                    .bind(&phone)
                    .bind(false)  // accepts_marketing
                    .bind(false)  // tax_exempt
                    .bind(Currency::USD)  // currency - use proper enum type
                    .bind(false)  // is_verified
                    .bind(false)  // marketing_opt_in
                    .bind(true)   // email_notifications
                    .bind(false)  // sms_notifications
                    .bind(false)  // push_notifications
                    .execute(pool)
                    .await {
                        Ok(_) => {
                            stats.created += 1;
                            tracing::info!("Created customer: {}", customer.email);

                            // Create addresses if available
                            // Billing address
                            if let Some(ref billing) = customer.billing {
                                if let Err(e) = self.create_address(pool, customer_id, billing, true, false).await {
                                    stats.error_details.push(format!(
                                        "Failed to create billing address for customer '{}': {}",
                                        customer.email, e
                                    ));
                                }
                            }

                            // Shipping address
                            if let Some(ref shipping) = customer.shipping {
                                let is_default_shipping = customer.billing.is_none() || 
                                    (shipping.address_1 != customer.billing.as_ref().unwrap().address_1);
                                if let Err(e) = self.create_address(pool, customer_id, shipping, false, is_default_shipping).await {
                                    stats.error_details.push(format!(
                                        "Failed to create shipping address for customer '{}': {}",
                                        customer.email, e
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            stats.errors += 1;
                            let error_msg = format!("Failed to create customer '{}': {}", customer.email, e);
                            tracing::error!("{}", error_msg);
                            stats.error_details.push(error_msg);
                            if !config.options.continue_on_error {
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(stats)
    }

    async fn import_orders(
        &self,
        config: &ImportConfig,
        progress: &(dyn Fn(ImportProgress) + Send + Sync),
    ) -> ImportResult<ImportStats> {
        let dry_run = config.options.dry_run;
        let (base_url, consumer_key, consumer_secret) = match &config.source {
            crate::import::types::SourceConfig::Platform {
                api_url,
                api_key,
                headers,
                ..
            } => {
                let secret = headers
                    .get("consumer_secret")
                    .cloned()
                    .unwrap_or_default();
                (api_url.clone(), api_key.clone(), secret)
            }
            _ => {
                return Err(ImportError::Configuration(
                    "Invalid source configuration for WooCommerce".to_string(),
                ))
            }
        };

        progress(ImportProgress {
            stage: "orders".to_string(),
            current: 0,
            total: 1,
            message: if dry_run {
                "Fetching orders from WooCommerce (DRY RUN)...".to_string()
            } else {
                "Fetching orders from WooCommerce...".to_string()
            },
        });

        let url = self.api_url(&base_url, "orders");
        let wc_orders: Vec<WooCommerceOrder> = self
            .fetch_paginated(&url, &consumer_key, &consumer_secret, config.options.limit)
            .await?;

        let mut stats = ImportStats {
            total: wc_orders.len(),
            ..Default::default()
        };

        // Create database pool if not dry run
        let pool = if !dry_run {
            Some(self.create_pool(&config.database_url).await?)
        } else {
            None
        };

        for (i, order) in wc_orders.iter().enumerate() {
            progress(ImportProgress {
                stage: "orders".to_string(),
                current: i + 1,
                total: wc_orders.len(),
                message: if dry_run {
                    format!("Validating order {}/{}", i + 1, wc_orders.len())
                } else {
                    format!("Importing order {}/{}", i + 1, wc_orders.len())
                },
            });

            // Validate order
            if order.id == 0 {
                stats.errors += 1;
                stats.error_details.push(format!(
                    "Order at index {} has invalid ID",
                    i
                ));
                continue;
            }

            if dry_run {
                stats.created += 1;
            } else {
                // Create order in database
                if let Some(ref pool) = pool {
                    // Get customer email from billing or use placeholder
                    let customer_email = order.billing.as_ref()
                        .and_then(|b| b.email.clone())
                        .unwrap_or_else(|| "unknown@example.com".to_string());

                    // Look up customer by email (needed for both create and update)
                    let customer_id = if !customer_email.is_empty() && customer_email != "unknown@example.com" {
                        sqlx::query_scalar::<_, Uuid>("SELECT id FROM customers WHERE email = $1")
                            .bind(&customer_email)
                            .fetch_optional(pool)
                            .await
                            .map_err(|e| ImportError::Database(crate::Error::Database(e)))?
                    } else {
                        None
                    };

                    // Parse totals
                    let total = self.parse_price(&order.total);
                    let tax_total = order.total_tax.as_ref()
                        .map(|t| self.parse_price(t))
                        .unwrap_or(Decimal::ZERO);
                    let shipping_total = order.shipping_total.as_ref()
                        .map(|s| self.parse_price(s))
                        .unwrap_or(Decimal::ZERO);
                    let discount_total = order.discount_total.as_ref()
                        .map(|d| self.parse_price(d))
                        .unwrap_or(Decimal::ZERO);
                    let subtotal = total - tax_total - shipping_total + discount_total;

                    // Map status
                    let status = self.map_order_status(&order.status);
                    let payment_status = match order.status.as_str() {
                        "completed" | "processing" => PaymentStatus::Paid,
                        "on-hold" => PaymentStatus::Pending,
                        "pending" => PaymentStatus::Pending,
                        "cancelled" => PaymentStatus::Cancelled,
                        "refunded" => PaymentStatus::Refunded,
                        "failed" => PaymentStatus::Failed,
                        _ => PaymentStatus::Pending,
                    };
                    let fulfillment_status = match order.status.as_str() {
                        "completed" => FulfillmentStatus::Delivered,
                        "processing" => FulfillmentStatus::Processing,
                        "on-hold" => FulfillmentStatus::Pending,
                        "pending" => FulfillmentStatus::Pending,
                        "cancelled" => FulfillmentStatus::Cancelled,
                        _ => FulfillmentStatus::Pending,
                    };

                    let currency = self.parse_currency(&order.currency);

                    // Generate order number
                    let order_number = order.order_number.clone()
                        .unwrap_or_else(|| format!("WC-{}", order.id));

                    // Check if order already exists
                    let existing_order: Option<Uuid> = sqlx::query_scalar(
                        "SELECT id FROM orders WHERE order_number = $1"
                    )
                    .bind(&order_number)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| ImportError::Database(crate::Error::Database(e)))?;

                    if let Some(existing_order_id) = existing_order {
                        if config.options.skip_existing {
                            stats.skipped += 1;
                            continue;
                        }
                        if config.options.update_existing {
                            // Delete existing order items first
                            let _ = sqlx::query("DELETE FROM order_items WHERE order_id = $1")
                                .bind(existing_order_id)
                                .execute(pool)
                                .await;
                            
                            // Update existing order
                            match sqlx::query(
                                r#"
                                UPDATE orders 
                                SET customer_id = $1, email = $2, currency = $3, status = $4,
                                    fulfillment_status = $5, payment_status = $6, subtotal = $7,
                                    tax_total = $8, shipping_total = $9, discount_total = $10, total = $11,
                                    updated_at = NOW()
                                WHERE id = $12
                                "#
                            )
                            .bind(customer_id)
                            .bind(&customer_email)
                            .bind(currency)
                            .bind(status)
                            .bind(fulfillment_status)
                            .bind(payment_status)
                            .bind(subtotal)
                            .bind(tax_total)
                            .bind(shipping_total)
                            .bind(discount_total)
                            .bind(total)
                            .bind(existing_order_id)
                            .execute(pool)
                            .await {
                                Ok(_) => {
                                    // Re-create order items
                                    if let Some(ref line_items) = order.line_items {
                                        for item in line_items {
                                            let item_price = self.parse_price(&item.subtotal) / Decimal::from(item.quantity.max(1));
                                            let item_subtotal = self.parse_price(&item.subtotal);
                                            let item_total = self.parse_price(&item.total);
                                            let item_tax = item_total - item_subtotal;

                                            // Look up product by SKU first, then by name
                                            let product_id = if let Some(ref sku) = item.sku {
                                                self.lookup_product_by_sku(pool, sku).await.ok().flatten()
                                            } else {
                                                None
                                            };
                                            
                                            let product_id = match product_id {
                                                Some(id) => Some(id),
                                                None => self.lookup_product_by_name(pool, &item.name).await.ok().flatten(),
                                            };
                                            
                                            let product_id = match product_id {
                                                Some(id) => id,
                                                None => continue,
                                            };

                                            let _ = sqlx::query(
                                                r#"
                                                INSERT INTO order_items (
                                                    id, order_id, product_id, variant_id,
                                                    quantity, price, subtotal, tax_amount, total,
                                                    sku, title, variant_title, requires_shipping,
                                                    created_at, updated_at
                                                )
                                                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
                                                "#
                                            )
                                            .bind(Uuid::new_v4())
                                            .bind(existing_order_id)
                                            .bind(product_id)
                                            .bind(None::<Uuid>)
                                            .bind(item.quantity)
                                            .bind(item_price)
                                            .bind(item_subtotal)
                                            .bind(item_tax)
                                            .bind(item_total)
                                            .bind(item.sku.clone())
                                            .bind(&item.name)
                                            .bind(None::<String>)
                                            .bind(true)
                                            .execute(pool)
                                            .await;
                                        }
                                    }
                                    
                                    stats.updated += 1;
                                    tracing::info!("Updated order: {}", order_number);
                                }
                                Err(e) => {
                                    stats.errors += 1;
                                    stats.error_details.push(format!(
                                        "Failed to update order {}: {}",
                                        order.id, e
                                    ));
                                }
                            }
                            continue;
                        }
                        stats.skipped += 1;
                        continue;
                    }

                    // Create addresses and get their IDs
                    let mut billing_address_id: Option<Uuid> = None;
                    let mut shipping_address_id: Option<Uuid> = None;

                    // Create billing address (with customer_id if found)
                    if let Some(ref billing) = order.billing {
                        match self.create_address_for_order(pool, customer_id, billing).await {
                            Ok(id) => billing_address_id = Some(id),
                            Err(e) => {
                                stats.error_details.push(format!(
                                    "Failed to create billing address for order {}: {}",
                                    order.id, e
                                ));
                            }
                        }
                    }

                    // Create shipping address (with customer_id if found)
                    if let Some(ref shipping) = order.shipping {
                        match self.create_address_for_order(pool, customer_id, shipping).await {
                            Ok(id) => shipping_address_id = Some(id),
                            Err(e) => {
                                stats.error_details.push(format!(
                                    "Failed to create shipping address for order {}: {}",
                                    order.id, e
                                ));
                            }
                        }
                    }

                    // Insert order
                    let order_id = Uuid::new_v4();
                    let result = sqlx::query(
                        r#"
                        INSERT INTO orders (
                            id, order_number, customer_id, email,
                            currency, status, fulfillment_status, payment_status,
                            subtotal, tax_total, shipping_total, discount_total, total,
                            billing_address_id, shipping_address_id,
                            notes, tags, draft, created_at, updated_at
                        )
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, NOW(), NOW())
                        "#
                    )
                    .bind(order_id)
                    .bind(&order_number)
                    .bind(customer_id) // Use looked-up customer_id
                    .bind(&customer_email)
                    .bind(currency)  // Use Currency enum directly
                    .bind(status)  // Use OrderStatus enum directly
                    .bind(fulfillment_status)  // Use FulfillmentStatus enum directly
                    .bind(payment_status)  // Use PaymentStatus enum directly
                    .bind(subtotal)
                    .bind(tax_total)
                    .bind(shipping_total)
                    .bind(discount_total)
                    .bind(total)
                    .bind(billing_address_id)
                    .bind(shipping_address_id)
                    .bind(None::<String>) // notes
                    .bind(&[] as &[String]) // tags
                    .bind(false) // draft
                    .execute(pool)
                    .await;

                    match result {
                        Ok(_) => {
                            // Create order items
                            if let Some(ref line_items) = order.line_items {
                                for item in line_items {
                                    let item_price = self.parse_price(&item.subtotal) / Decimal::from(item.quantity.max(1));
                                    let item_subtotal = self.parse_price(&item.subtotal);
                                    let item_total = self.parse_price(&item.total);
                                    let item_tax = item_total - item_subtotal;

                                    // Look up product by SKU first, then by name
                                    let product_id = if let Some(ref sku) = item.sku {
                                        self.lookup_product_by_sku(pool, sku).await.ok().flatten()
                                    } else {
                                        None
                                    };
                                    
                                    // Fallback to lookup by name if SKU not found
                                    let product_id = match product_id {
                                        Some(id) => Some(id),
                                        None => self.lookup_product_by_name(pool, &item.name).await.ok().flatten(),
                                    };
                                    
                                    // Skip order item if product not found
                                    let product_id = match product_id {
                                        Some(id) => id,
                                        None => {
                                            stats.error_details.push(format!(
                                                "Order item '{}' not found in product catalog for order {}",
                                                item.name, order.id
                                            ));
                                            continue;
                                        }
                                    };

                                    let item_result = sqlx::query(
                                        r#"
                                        INSERT INTO order_items (
                                            id, order_id, product_id, variant_id,
                                            quantity, price, subtotal, tax_amount, total,
                                            sku, title, variant_title, requires_shipping,
                                            created_at, updated_at
                                        )
                                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
                                        "#
                                    )
                                    .bind(Uuid::new_v4())
                                    .bind(order_id)
                                    .bind(product_id)
                                    .bind(None::<Uuid>) // variant_id
                                    .bind(item.quantity)
                                    .bind(item_price)
                                    .bind(item_subtotal)
                                    .bind(item_tax)
                                    .bind(item_total)
                                    .bind(item.sku.clone())
                                    .bind(&item.name)
                                    .bind(None::<String>) // variant_title
                                    .bind(true) // requires_shipping
                                    .execute(pool)
                                    .await;

                                    if let Err(e) = item_result {
                                        stats.error_details.push(format!(
                                            "Failed to create order item for order {}: {}",
                                            order.id, e
                                        ));
                                    }
                                }
                            }

                            stats.created += 1;
                        }
                        Err(e) => {
                            stats.errors += 1;
                            stats.error_details.push(format!(
                                "Failed to create order {}: {}",
                                order.id, e
                            ));
                            if !config.options.continue_on_error {
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(stats)
    }
}

impl WooCommerceImporter {
    /// Helper method to create an address for a customer
    async fn create_address(
        &self,
        pool: &PgPool,
        customer_id: Uuid,
        wc_addr: &WooCommerceAddress,
        is_default_billing: bool,
        is_default_shipping: bool,
    ) -> ImportResult<()> {
        // Only create if we have minimum required fields
        let address1 = match wc_addr.address_1 {
            Some(ref addr) if !addr.is_empty() => addr.clone(),
            _ => return Ok(()), // Skip if no address
        };

        let city = match wc_addr.city {
            Some(ref c) if !c.is_empty() => c.clone(),
            _ => return Ok(()), // Skip if no city
        };

        let country = match wc_addr.country {
            Some(ref c) if !c.is_empty() => c.clone(),
            _ => "US".to_string(), // Default to US
        };

        let zip = match wc_addr.postcode {
            Some(ref z) if !z.is_empty() => z.clone(),
            _ => "00000".to_string(), // Default zip
        };

        let first_name = wc_addr.first_name.clone().unwrap_or_default();
        let last_name = wc_addr.last_name.clone().unwrap_or_default();

        // Use a default name if both are empty
        let (first_name, last_name) = if first_name.is_empty() && last_name.is_empty() {
            ("Unknown".to_string(), "Customer".to_string())
        } else {
            (first_name, last_name)
        };

        let result = sqlx::query(
            r#"
            INSERT INTO addresses (
                id, customer_id, first_name, last_name, company,
                phone, address1, address2, city, province, country, zip,
                is_default_shipping, is_default_billing, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, NOW(), NOW())
            "#
        )
        .bind(Uuid::new_v4())
        .bind(customer_id)
        .bind(first_name)
        .bind(last_name)
        .bind(wc_addr.company.clone())
        .bind(wc_addr.phone.clone())
        .bind(address1)
        .bind(wc_addr.address_2.clone())
        .bind(city)
        .bind(wc_addr.state.clone())
        .bind(country)
        .bind(zip)
        .bind(is_default_shipping)
        .bind(is_default_billing)
        .execute(pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ImportError::Database(crate::Error::Database(e))),
        }
    }

    /// Helper method to create an address for an order (optionally linked to a customer)
    async fn create_address_for_order(
        &self,
        pool: &PgPool,
        customer_id: Option<Uuid>,
        wc_addr: &WooCommerceAddress,
    ) -> ImportResult<Uuid> {
        let address_id = Uuid::new_v4();

        // Only create if we have minimum required fields
        let address1 = match wc_addr.address_1 {
            Some(ref addr) if !addr.is_empty() => addr.clone(),
            _ => "Unknown".to_string(),
        };

        let city = match wc_addr.city {
            Some(ref c) if !c.is_empty() => c.clone(),
            _ => "Unknown".to_string(),
        };

        let country = match wc_addr.country {
            Some(ref c) if !c.is_empty() => c.clone(),
            _ => "US".to_string(),
        };

        let zip = match wc_addr.postcode {
            Some(ref z) if !z.is_empty() => z.clone(),
            _ => "00000".to_string(),
        };

        let first_name = wc_addr.first_name.clone().unwrap_or_default();
        let last_name = wc_addr.last_name.clone().unwrap_or_default();

        // Use a default name if both are empty
        let (first_name, last_name) = if first_name.is_empty() && last_name.is_empty() {
            ("Unknown".to_string(), "Customer".to_string())
        } else {
            (first_name, last_name)
        };

        let result = sqlx::query(
            r#"
            INSERT INTO addresses (
                id, customer_id, first_name, last_name, company,
                phone, address1, address2, city, province, country, zip,
                is_default_shipping, is_default_billing, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, NOW(), NOW())
            "#
        )
        .bind(address_id)
        .bind(customer_id) // Link to customer if found
        .bind(first_name)
        .bind(last_name)
        .bind(wc_addr.company.clone())
        .bind(wc_addr.phone.clone())
        .bind(address1)
        .bind(wc_addr.address_2.clone())
        .bind(city)
        .bind(wc_addr.state.clone())
        .bind(country)
        .bind(zip)
        .bind(false)
        .bind(false)
        .execute(pool)
        .await;

        match result {
            Ok(_) => Ok(address_id),
            Err(e) => Err(ImportError::Database(crate::Error::Database(e))),
        }
    }
    
    /// Look up product ID by SKU
    async fn lookup_product_by_sku(
        &self,
        pool: &PgPool,
        sku: &str,
    ) -> ImportResult<Option<Uuid>> {
        if sku.is_empty() {
            return Ok(None);
        }
        
        let product_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM products WHERE sku = $1 LIMIT 1"
        )
        .bind(sku)
        .fetch_optional(pool)
        .await
        .map_err(|e| ImportError::Database(crate::Error::Database(e)))?;
        
        Ok(product_id)
    }
    
    /// Look up product ID by name (fallback if SKU not found)
    async fn lookup_product_by_name(
        &self,
        pool: &PgPool,
        name: &str,
    ) -> ImportResult<Option<Uuid>> {
        if name.is_empty() {
            return Ok(None);
        }
        
        let product_id = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM products WHERE title = $1 LIMIT 1"
        )
        .bind(name)
        .fetch_optional(pool)
        .await
        .map_err(|e| ImportError::Database(crate::Error::Database(e)))?;
        
        Ok(product_id)
    }
}

// WooCommerce API response types
// These are used for API deserialization - fields are read by serde
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceProduct {
    id: u64,
    name: String,
    slug: String,
    #[serde(rename = "type")]
    product_type: String,
    status: String,
    description: Option<String>,
    #[serde(rename = "short_description")]
    short_description: Option<String>,
    sku: Option<String>,
    price: String,
    #[serde(rename = "regular_price")]
    regular_price: String,
    #[serde(rename = "sale_price")]
    sale_price: String,
    #[serde(rename = "stock_quantity")]
    stock_quantity: Option<i32>,
    #[serde(rename = "manage_stock")]
    manage_stock: bool,
    categories: Vec<WooCommerceCategory>,
    images: Vec<WooCommerceImage>,
    attributes: Vec<WooCommerceAttribute>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceCategory {
    id: u64,
    name: String,
    slug: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceImage {
    id: u64,
    src: String,
    name: Option<String>,
    alt: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceAttribute {
    id: u64,
    name: String,
    options: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceCustomer {
    id: u64,
    email: String,
    #[serde(rename = "first_name")]
    first_name: String,
    #[serde(rename = "last_name")]
    last_name: String,
    username: String,
    billing: Option<WooCommerceAddress>,
    shipping: Option<WooCommerceAddress>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceAddress {
    #[serde(rename = "first_name")]
    first_name: Option<String>,
    #[serde(rename = "last_name")]
    last_name: Option<String>,
    company: Option<String>,
    address_1: Option<String>,
    address_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postcode: Option<String>,
    country: Option<String>,
    email: Option<String>,
    phone: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceOrder {
    id: u64,
    #[serde(rename = "order_number")]
    order_number: Option<String>,
    status: String,
    currency: String,
    total: String,
    #[serde(rename = "total_tax")]
    total_tax: Option<String>,
    #[serde(rename = "shipping_total")]
    shipping_total: Option<String>,
    #[serde(rename = "discount_total")]
    discount_total: Option<String>,
    billing: Option<WooCommerceAddress>,
    shipping: Option<WooCommerceAddress>,
    #[serde(rename = "line_items")]
    line_items: Option<Vec<WooCommerceLineItem>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceLineItem {
    id: u64,
    name: String,
    product_id: u64,
    quantity: i32,
    subtotal: String,
    total: String,
    sku: Option<String>,
}
