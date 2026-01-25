pub mod service;
pub mod reservation;
pub mod tracking;
pub mod notification;

#[cfg(test)]
mod tests;

use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

use crate::Result;

pub use service::{InventoryService, StockAlertLevel};
pub use reservation::{StockReservation, ReservationStatus};
pub use tracking::{InventoryLevel, StockMovement};
pub use notification::LowStockAlert;

/// Inventory configuration
#[derive(Debug, Clone)]
pub struct InventoryConfig {
    /// Low stock threshold (percentage)
    pub low_stock_threshold: u32,
    /// Enable automatic restocking alerts
    pub enable_restock_alerts: bool,
    /// Enable stock reservations
    pub enable_reservations: bool,
    /// Reservation timeout (minutes)
    pub reservation_timeout_minutes: u32,
}

impl Default for InventoryConfig {
    fn default() -> Self {
        Self {
            low_stock_threshold: 20, // 20% remaining
            enable_restock_alerts: true,
            enable_reservations: true,
            reservation_timeout_minutes: 30,
        }
    }
}

/// Inventory location (for multi-warehouse support)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct InventoryLocation {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// Product inventory summary
#[derive(Debug, Clone)]
pub struct ProductInventory {
    pub product_id: Uuid,
    pub total_available: i32,
    pub total_reserved: i32,
    pub total_incoming: i32,
    pub low_stock_threshold: i32,
    pub locations: Vec<LocationInventory>,
}

#[derive(Debug, Clone)]
pub struct LocationInventory {
    pub location_id: Uuid,
    pub location_name: String,
    pub available: i32,
    pub reserved: i32,
    pub incoming: i32,
}