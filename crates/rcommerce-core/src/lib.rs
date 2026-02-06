pub mod config;
pub mod error;
pub mod models;
pub mod traits;
pub mod common;
pub mod repository;
pub mod db;
pub mod services;
pub mod payment;
pub mod inventory;
pub mod order;
pub mod notification;
pub mod middleware;
pub mod websocket;
pub mod cache;
pub mod jobs;
pub mod performance;
pub mod import;
pub mod shipping;

// Re-export commonly used types
pub use error::{Error, Result};
pub use config::{Config, DunningConfig, DunningEmailTemplates, GatewayDunningConfig};
pub use models::{Currency, Pagination, SortDirection, SortParams, ProductType, SubscriptionInterval, OrderType, SubscriptionStatus};
pub use traits::Repository;
pub use repository::{Database, create_pool};
pub use db::migrate::{Migrator, auto_migrate, DbStatus};
pub use services::{ProductService, CustomerService, OrderService, AuthService, ApiKey, JwtClaims, Service, PaginationParams, PaginationInfo, Scope, ScopeChecker, Resource, Action, scope_presets, DunningService, DunningHistory, RetryableInvoice, RetryProcessingResult};
pub use services::dunning_service::{self, EmailService as DunningEmailService};
pub use payment::{PaymentGateway, CreatePaymentRequest, PaymentMethod, CardDetails, PaymentSession, PaymentSessionStatus, Payment, PaymentStatus, Refund, RefundStatus, WebhookEvent, WebhookEventType};
pub use payment::gateways::{stripe::StripeGateway, wechatpay::WeChatPayGateway, alipay::AliPayGateway};
pub use inventory::{InventoryService, StockAlertLevel, StockReservation, ReservationStatus, InventoryLevel, StockMovement, StockStatus, LowStockAlert, InventoryConfig, InventoryLocation, ProductInventory, LocationInventory};
// Order types come from the order module (not models), which includes lifecycle, fulfillment, etc.
pub use order::{Order, OrderItem, OrderStatus, PaymentStatus as OrderPaymentStatus, OrderFilter, CreateOrderRequest, CreateOrderItem, Fulfillment, FulfillmentStatus, TrackingInfo as OrderTrackingInfo, OrderCalculator, OrderTotals, OrderService as OrderManager};

// Shipping module exports
pub use shipping::{
    ShippingProvider, ShippingRate, Shipment, ShipmentStatus, TrackingInfo, TrackingStatus, TrackingEvent,
    Package, RateOptions, AddressValidation, ShippingService, ServiceFeature, CustomsInfo, CustomsItem,
    ContentsType, NonDeliveryOption, ShippingProviderFactory, ShippingRateAggregator,
};

// Shipping calculation exports
pub use shipping::calculation::{
    ShippingCalculator, VolumetricWeightCalculator, WeightConverter, WeightUnit, 
    LengthConverter, LengthUnit, ChargeableWeight, ShippingCalculation, TieredShippingCalculator,
};

// Shipping packaging exports
pub use shipping::packaging::{
    PackageType, PackageRegistry, PackagingCalculator, ItemDimensions, PackageRecommendation,
};

// Shipping zones exports
pub use shipping::zones::{
    ShippingZone, ZoneRate, ZoneCalculator, ZonePresets,
};

// Shipping rules exports
pub use shipping::rules::{
    ShippingRule, ShippingRuleEngine, RuleCondition, RuleAction, RulePresets,
};

// Shipping carriers
pub use shipping::carriers::{DhlProvider, FedExProvider, UpsProvider, UspsProvider};

// Shipping providers (aggregators)
pub use shipping::providers::{EasyPostProvider, ShipStationProvider};

/// Current version of rcommerce
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Error {
    pub fn not_implemented<T: Into<String>>(msg: T) -> Self {
        Error::Other(format!("Not implemented: {}", msg.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_available() {
        assert!(!VERSION.is_empty());
    }
    
    #[test]
    fn test_error_creation() {
        let err = Error::validation("Test validation error");
        assert_eq!(err.status_code(), 400);
        assert_eq!(err.category(), "validation");
    }
}