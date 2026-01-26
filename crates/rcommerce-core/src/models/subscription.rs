//! Subscription model for recurring billing
//!
//! This module provides data structures for managing product subscriptions,
//! including billing cycles, payment scheduling, and subscription lifecycle.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Subscription status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "subscription_status", rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Subscription is active and billing
    Active,
    /// Subscription is paused temporarily
    Paused,
    /// Subscription has been cancelled
    Cancelled,
    /// Subscription has expired (reached max cycles)
    Expired,
    /// Payment failed, subscription past due
    PastDue,
    /// Trial period before billing starts
    Trialing,
    /// Pending activation (e.g., awaiting first payment)
    Pending,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        SubscriptionStatus::Pending
    }
}

/// Cancellation reason
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "cancellation_reason", rename_all = "snake_case")]
pub enum CancellationReason {
    CustomerRequested,
    PaymentFailed,
    Fraudulent,
    TooExpensive,
    NotUseful,
    Other,
}

/// Subscription entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subscription {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub order_id: Uuid,              // Original order that created the subscription
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    
    // Subscription configuration
    pub status: SubscriptionStatus,
    pub interval: super::SubscriptionInterval,
    pub interval_count: i32,         // e.g., every 3 months
    
    // Pricing
    pub currency: super::Currency,
    pub amount: Decimal,             // Amount per billing cycle
    pub setup_fee: Option<Decimal>,
    
    // Trial
    pub trial_days: i32,
    pub trial_ends_at: Option<DateTime<Utc>>,
    
    // Billing cycle tracking
    pub current_cycle: i32,          // Current billing cycle number
    pub min_cycles: Option<i32>,     // Minimum cycles before cancellation
    pub max_cycles: Option<i32>,     // Maximum cycles (None = unlimited)
    
    // Important dates
    pub starts_at: DateTime<Utc>,
    pub next_billing_at: DateTime<Utc>,
    pub last_billing_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,        // When subscription ends
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<CancellationReason>,
    
    // Payment method
    pub payment_method_id: Option<String>,     // Gateway payment method ID
    pub gateway: String,                       // Payment gateway used
    
    // Metadata
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Subscription item (line item within a subscription)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SubscriptionItem {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub quantity: i32,
    pub price: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Subscription billing cycle/invoice
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SubscriptionInvoice {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub order_id: Option<Uuid>,      // Associated order (if generated)
    
    // Billing period
    pub cycle_number: i32,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    
    // Amounts
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
    
    // Status
    pub status: InvoiceStatus,
    pub paid_at: Option<DateTime<Utc>>,
    pub payment_id: Option<String>,  // Gateway payment ID
    
    // Failure tracking
    pub failed_attempts: i32,
    pub last_failed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Invoice status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "invoice_status", rename_all = "snake_case")]
pub enum InvoiceStatus {
    Pending,        // Not yet billed
    Billed,         // Billed, awaiting payment
    Paid,           // Successfully paid
    Failed,         // Payment failed
    PastDue,        // Past due date, payment failed
    Cancelled,      // Cancelled before payment
}

/// Create subscription request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateSubscriptionRequest {
    pub customer_id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    
    pub interval: super::SubscriptionInterval,
    #[validate(range(min = 1, max = 12))]
    pub interval_count: i32,
    
    pub currency: super::Currency,
    pub amount: Decimal,
    pub setup_fee: Option<Decimal>,
    
    pub trial_days: Option<i32>,
    pub min_cycles: Option<i32>,
    pub max_cycles: Option<i32>,
    
    pub payment_method_id: String,
    pub gateway: String,
    
    pub notes: Option<String>,
}

/// Update subscription request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateSubscriptionRequest {
    pub status: Option<SubscriptionStatus>,
    pub interval: Option<super::SubscriptionInterval>,
    pub interval_count: Option<i32>,
    pub amount: Option<Decimal>,
    pub next_billing_at: Option<DateTime<Utc>>,
    pub max_cycles: Option<i32>,
    pub payment_method_id: Option<String>,
    pub notes: Option<String>,
}

/// Cancel subscription request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CancelSubscriptionRequest {
    pub reason: CancellationReason,
    #[validate(length(max = 500))]
    pub reason_details: Option<String>,
    /// When to cancel: immediately or at end of current period
    pub cancel_at_end: bool,
}

/// Subscription filter for queries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionFilter {
    pub customer_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub status: Option<SubscriptionStatus>,
    pub gateway: Option<String>,
    pub billing_before: Option<DateTime<Utc>>,
    pub billing_after: Option<DateTime<Utc>>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
}

/// Subscription summary for dashboards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionSummary {
    pub total_active: i64,
    pub total_cancelled: i64,
    pub total_expired: i64,
    pub total_past_due: i64,
    pub monthly_recurring_revenue: Decimal,
    pub annual_recurring_revenue: Decimal,
}
