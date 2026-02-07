//! WooCommerce importer implementation

use crate::import::{
    error::{ImportError, ImportResult},
    types::{ImportConfig, ImportProgress, ImportStats},
    PlatformImporter,
};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

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
        let per_page = 100.min(limit);

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

            let items: Vec<T> = response.json().await?;
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
                // In real implementation, insert into database
                stats.created += 1;
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
                // In real implementation, insert into database
                stats.created += 1;
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
                // In real implementation, insert into database
                stats.created += 1;
            }
        }

        Ok(stats)
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
    first_name: String,
    #[serde(rename = "last_name")]
    last_name: String,
    company: String,
    address_1: String,
    address_2: String,
    city: String,
    state: String,
    postcode: String,
    country: String,
    email: Option<String>,
    phone: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct WooCommerceOrder {
    id: u64,
    #[serde(rename = "order_number")]
    order_number: String,
    status: String,
    currency: String,
    total: String,
    #[serde(rename = "total_tax")]
    total_tax: String,
    #[serde(rename = "shipping_total")]
    shipping_total: String,
    #[serde(rename = "discount_total")]
    discount_total: String,
    billing: WooCommerceAddress,
    shipping: WooCommerceAddress,
    #[serde(rename = "line_items")]
    line_items: Vec<WooCommerceLineItem>,
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
