use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Order type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Default)]
#[sqlx(type_name = "order_type", rename_all = "snake_case")]
pub enum OrderType {
    /// One-time purchase
    #[default]
    OneTime,
    /// Subscription initial order
    SubscriptionInitial,
    /// Subscription renewal order
    SubscriptionRenewal,
}

/// Order entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    pub email: String,
    pub currency: super::Currency,
    pub order_type: OrderType,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub shipping_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub status: OrderStatus,
    pub fulfillment_status: FulfillmentStatus,
    pub payment_status: PaymentStatus,
    pub shipping_address_id: Option<Uuid>,
    pub billing_address_id: Option<Uuid>,
    pub payment_method: Option<String>,
    pub shipping_method: Option<String>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub draft: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    // Subscription fields (for subscription orders)
    pub subscription_id: Option<Uuid>,
    pub billing_cycle: Option<i32>, // Which billing cycle this is (1, 2, 3...)
}

/// Order status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    OnHold,
    Completed,
    Cancelled,
    Refunded,
}

/// Fulfillment status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fulfillment_status", rename_all = "snake_case")]
pub enum FulfillmentStatus {
    Pending,
    Processing,
    Partial,
    Shipped,
    Delivered,
    Cancelled,
    Returned,
}

/// Payment status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Paid,
    Failed,
    Cancelled,
    Refunded,
}

/// Order item
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub quantity: i32,
    pub price: Decimal,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub total: Decimal,
    pub sku: Option<String>,
    pub title: String,
    pub variant_title: Option<String>,
    pub requires_shipping: bool,
    pub is_gift_card: bool,
    pub weight: Option<Decimal>,
    pub weight_unit: Option<super::WeightUnit>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Order fulfillment
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Fulfillment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: FulfillmentStatus,
    pub tracking_number: Option<String>,
    pub tracking_url: Option<String>,
    pub tracking_company: Option<String>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payment method types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_method_type", rename_all = "snake_case")]
pub enum PaymentMethodType {
    CreditCard,
    DebitCard,
    BankTransfer,
    CashOnDelivery,
    DigitalWallet,
    Cryptocurrency,
}

/// Order filter for queries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderFilter {
    pub status: Option<OrderStatus>,
    pub fulfillment_status: Option<FulfillmentStatus>,
    pub payment_status: Option<PaymentStatus>,
    pub customer_id: Option<Uuid>,
    pub email: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub updated_after: Option<DateTime<Utc>>,
    pub total_min: Option<Decimal>,
    pub total_max: Option<Decimal>,
    pub tags: Vec<String>,
    pub draft: Option<bool>,
}

/// Create order request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    pub customer_id: Option<Uuid>,
    
    #[validate(email)]
    pub email: String,
    
    pub currency: super::Currency,
    
    pub shipping_address_id: Option<Uuid>,
    
    pub billing_address_id: Option<Uuid>,
    
    pub payment_method: Option<String>,
    
    pub shipping_method: Option<String>,
    
    pub notes: Option<String>,
}

/// Order note
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderNote {
    pub id: Uuid,
    pub order_id: Uuid,
    pub author: Option<String>,
    pub note: String,
    pub is_customer_notified: bool,
    pub created_at: DateTime<Utc>,
}