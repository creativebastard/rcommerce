use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::{Currency, InventoryPolicy, WeightUnit};

/// Product entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub sku: Option<String>,
    pub price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub currency: Currency,
    pub inventory_quantity: i32,
    pub inventory_policy: InventoryPolicy,
    pub inventory_management: bool,
    pub continues_selling_when_out_of_stock: bool,
    pub weight: Option<Decimal>,
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

/// Product variant
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductVariant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub title: String,
    pub sku: Option<String>,
    pub price: Decimal,
    pub compare_at_price: Option<Decimal>,
    pub cost_price: Option<Decimal>,
    pub currency: Currency,
    pub inventory_quantity: i32,
    pub inventory_policy: InventoryPolicy,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<WeightUnit>,
    pub requires_shipping: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Product image
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

/// Product option (e.g., Color, Size)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductOption {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Product option value
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductOptionValue {
    pub id: Uuid,
    pub option_id: Uuid,
    pub variant_id: Uuid,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create product request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    
    #[validate(length(min = 1, max = 255))]
    pub slug: String,
    
    pub description: Option<String>,
    
    #[validate(length(max = 100))]
    pub sku: Option<String>,
    
    pub price: Decimal,
    
    pub compare_at_price: Option<Decimal>,
    
    pub cost_price: Option<Decimal>,
    
    pub currency: Currency,
    
    #[validate(range(min = 0))]
    pub inventory_quantity: i32,
    
    pub inventory_policy: InventoryPolicy,
    
    pub inventory_management: bool,
    
    pub continues_selling_when_out_of_stock: bool,
    
    pub weight: Option<Decimal>,
    
    pub weight_unit: Option<WeightUnit>,
    
    pub requires_shipping: bool,
    
    pub is_active: bool,
    
    pub is_featured: bool,
    
    pub seo_title: Option<String>,
    
    pub seo_description: Option<String>,
}

/// Update product request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    
    #[validate(length(min = 1, max = 255))]
    pub slug: Option<String>,
    
    pub description: Option<Option<String>>,
    
    #[validate(length(max = 100))]
    pub sku: Option<Option<String>>,
    
    pub price: Option<Decimal>,
    
    pub compare_at_price: Option<Option<Decimal>>,
    
    pub cost_price: Option<Option<Decimal>>,
    
    pub currency: Option<Currency>,
    
    #[validate(range(min = 0))]
    pub inventory_quantity: Option<i32>,
    
    pub inventory_policy: Option<InventoryPolicy>,
    
    pub inventory_management: Option<bool>,
    
    pub continues_selling_when_out_of_stock: Option<bool>,
    
    pub weight: Option<Option<Decimal>>,
    
    pub weight_unit: Option<Option<WeightUnit>>,
    
    pub requires_shipping: Option<bool>,
    
    pub is_active: Option<bool>,
    
    pub is_featured: Option<bool>,
    
    pub seo_title: Option<Option<String>>,
    
    pub seo_description: Option<Option<String>>,
}

/// Create variant request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateVariantRequest {
    pub product_id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    
    #[validate(length(max = 100))]
    pub sku: Option<String>,
    
    pub price: Decimal,
    
    pub compare_at_price: Option<Decimal>,
    
    pub cost_price: Option<Decimal>,
    
    pub currency: Currency,
    
    #[validate(range(min = 0))]
    pub inventory_quantity: i32,
    
    pub inventory_policy: InventoryPolicy,
    
    pub weight: Option<Decimal>,
    
    pub weight_unit: Option<WeightUnit>,
    
    pub requires_shipping: bool,
    
    pub is_active: bool,
}

/// Product status filter
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProductStatus {
    Active,
    Draft,
    Archived,
}

/// Product filter for queries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductFilter {
    pub status: Option<ProductStatus>,
    pub category_id: Option<Uuid>,
    pub collection_id: Option<Uuid>,
    pub vendor: Option<String>,
    pub product_type: Option<String>,
    pub tags: Vec<String>,
    pub price_min: Option<Decimal>,
    pub price_max: Option<Decimal>,
    pub inventory_status: Option<InventoryStatus>,
    pub created_after: Option<DateTime<Utc>>,
    pub updated_after: Option<DateTime<Utc>>,
}

/// Inventory status filter
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InventoryStatus {
    InStock,
    LowStock,
    OutOfStock,
    OnBackorder,
}

/// Product collection
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Collection {
    pub id: Uuid,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub sort_order: String, // manual, best-selling, created, etc.
    pub published_at: Option<DateTime<Utc>>,
    pub template_suffix: Option<String>,
    pub disjunctive: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_scope: String, // web, global
}