//! SQLite-compatible models
//!
//! These models use f64 instead of Decimal for SQLite compatibility.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::{Currency, InventoryPolicy, WeightUnit, ProductType, ProductStatus};

/// SQLite-compatible Product
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub sku: Option<String>,
    pub product_type: ProductType,
    pub price: f64,
    pub compare_at_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub currency: Currency,
    pub inventory_quantity: i32,
    pub inventory_policy: InventoryPolicy,
    pub inventory_management: bool,
    pub continues_selling_when_out_of_stock: bool,
    pub weight: Option<f64>,
    pub weight_unit: Option<WeightUnit>,
    pub requires_shipping: bool,
    pub is_active: bool,
    pub is_featured: bool,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

impl From<Product> for crate::models::Product {
    fn from(p: Product) -> Self {
        use rust_decimal::Decimal;
        
        Self {
            id: p.id,
            title: p.title,
            slug: p.slug,
            description: p.description,
            sku: p.sku,
            product_type: p.product_type,
            price: Decimal::from_f64_retain(p.price).unwrap_or_default(),
            compare_at_price: p.compare_at_price.and_then(Decimal::from_f64_retain),
            cost_price: p.cost_price.and_then(Decimal::from_f64_retain),
            currency: p.currency,
            inventory_quantity: p.inventory_quantity,
            inventory_policy: p.inventory_policy,
            inventory_management: p.inventory_management,
            continues_selling_when_out_of_stock: p.continues_selling_when_out_of_stock,
            weight: p.weight.and_then(Decimal::from_f64_retain),
            weight_unit: p.weight_unit,
            requires_shipping: p.requires_shipping,
            is_active: p.is_active,
            is_featured: p.is_featured,
            seo_title: p.seo_title,
            seo_description: p.seo_description,
            created_at: p.created_at,
            updated_at: p.updated_at,
            published_at: p.published_at,
            subscription_interval: None,
            subscription_interval_count: None,
            subscription_trial_days: None,
            subscription_setup_fee: None,
            subscription_min_cycles: None,
            subscription_max_cycles: None,
        }
    }
}

/// SQLite-compatible ProductVariant
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductVariant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub title: String,
    pub sku: Option<String>,
    pub price: f64,
    pub compare_at_price: Option<f64>,
    pub cost_price: Option<f64>,
    pub currency: Currency,
    pub inventory_quantity: i32,
    pub inventory_policy: InventoryPolicy,
    pub weight: Option<f64>,
    pub weight_unit: Option<WeightUnit>,
    pub requires_shipping: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// SQLite-compatible ProductImage
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductImage {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub position: i32,
    pub src: String,
    pub alt_text: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// SQLite-compatible Customer
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub accepts_marketing: bool,
    pub tax_exempt: bool,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub marketing_opt_in: bool,
    pub email_notifications: bool,
    pub sms_notifications: bool,
    pub push_notifications: bool,
}

impl From<Customer> for crate::models::Customer {
    fn from(c: Customer) -> Self {
        crate::models::Customer {
            id: c.id,
            email: c.email,
            first_name: c.first_name,
            last_name: c.last_name,
            phone: c.phone,
            accepts_marketing: c.accepts_marketing,
            tax_exempt: c.tax_exempt,
            currency: c.currency,
            created_at: c.created_at,
            updated_at: c.updated_at,
            confirmed_at: c.confirmed_at,
            timezone: c.timezone,
            marketing_opt_in: c.marketing_opt_in,
            email_notifications: c.email_notifications,
            sms_notifications: c.sms_notifications,
            push_notifications: c.push_notifications,
        }
    }
}
