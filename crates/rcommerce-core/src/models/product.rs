use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::{Currency, InventoryPolicy, WeightUnit};

/// Product type - determines how the product is sold
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "product_type", rename_all = "snake_case")]
pub enum ProductType {
    /// Simple product - single SKU, no variants
    Simple,
    /// Variable product - has multiple variants (sizes, colors)
    Variable,
    /// Subscription product - recurring billing
    Subscription,
    /// Digital product - downloadable, no shipping
    Digital,
    /// Bundle product - collection of other products
    Bundle,
}

impl Default for ProductType {
    fn default() -> Self {
        ProductType::Simple
    }
}

/// Subscription interval (billing period)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "subscription_interval", rename_all = "snake_case")]
pub enum SubscriptionInterval {
    Daily,
    Weekly,
    BiWeekly,
    Monthly,
    Quarterly,
    BiAnnually,
    Annually,
}

impl Default for SubscriptionInterval {
    fn default() -> Self {
        SubscriptionInterval::Monthly
    }
}

/// Product entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub sku: Option<String>,
    pub product_type: ProductType,
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
    // Subscription fields (for subscription products)
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>, // e.g., every 3 months
    pub subscription_trial_days: Option<i32>,
    pub subscription_setup_fee: Option<Decimal>,
    pub subscription_min_cycles: Option<i32>,
    pub subscription_max_cycles: Option<i32>,
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

/// Product attribute definition (for variable products)
/// e.g., "Color" with values ["Red", "Blue", "Green"]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductAttribute {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,           // e.g., "Color", "Size"
    pub slug: String,           // e.g., "color", "size"
    pub position: i32,          // Display order
    pub visible: bool,          // Show on product page
    pub variation: bool,        // Used for variations
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Product attribute value
/// e.g., Color="Red"
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductAttributeValue {
    pub id: Uuid,
    pub attribute_id: Uuid,
    pub value: String,          // e.g., "Red", "XL"
    pub color_code: Option<String>, // Hex color code for color attributes
    pub image_url: Option<String>,  // Swatch image
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Links variants to their attribute values
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductVariantAttribute {
    pub id: Uuid,
    pub variant_id: Uuid,
    pub attribute_value_id: Uuid,
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
    
    pub product_type: ProductType,
    
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
    
    // Subscription fields (required when product_type is Subscription)
    pub subscription_interval: Option<SubscriptionInterval>,
    pub subscription_interval_count: Option<i32>,
    pub subscription_trial_days: Option<i32>,
    pub subscription_setup_fee: Option<Decimal>,
    pub subscription_min_cycles: Option<i32>,
    pub subscription_max_cycles: Option<i32>,
    
    // For variable products - attributes like Size, Color
    pub attributes: Option<Vec<CreateAttributeRequest>>,
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
    
    pub product_type: Option<ProductType>,
    
    pub subscription_interval: Option<Option<SubscriptionInterval>>,
    pub subscription_interval_count: Option<Option<i32>>,
    pub subscription_trial_days: Option<Option<i32>>,
    pub subscription_setup_fee: Option<Option<Decimal>>,
    pub subscription_min_cycles: Option<Option<i32>>,
    pub subscription_max_cycles: Option<Option<i32>>,
}

/// Create attribute request (for variable products)
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAttributeRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,           // e.g., "Color", "Size"
    
    #[validate(length(min = 1, max = 100))]
    pub slug: String,           // e.g., "color", "size"
    
    pub position: i32,
    
    pub visible: bool,
    
    pub variation: bool,        // Used for creating variations
    
    pub values: Vec<CreateAttributeValueRequest>,
}

/// Create attribute value request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAttributeValueRequest {
    #[validate(length(min = 1, max = 100))]
    pub value: String,          // e.g., "Red", "XL"
    
    pub color_code: Option<String>, // Hex code for colors
    
    pub image_url: Option<String>,  // Swatch image
    
    pub position: i32,
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