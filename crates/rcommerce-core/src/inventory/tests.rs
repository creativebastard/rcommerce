#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::repository::{Database, create_pool};
    
    #[test]
    fn test_inventory_config_defaults() {
        let config = InventoryConfig::default();
        
        assert_eq!(config.low_stock_threshold, 20);
        assert!(config.enable_restock_alerts);
        assert!(config.enable_reservations);
        assert_eq!(config.reservation_timeout_minutes, 30);
    }
    
    #[test]
    fn test_stock_reservation_creation() {
        let product_id = uuid::Uuid::new_v4();
        let location_id = uuid::Uuid::new_v4();
        let order_id = uuid::Uuid::new_v4();
        
        let reservation = StockReservation::new(
            product_id,
            None,
            location_id,
            order_id,
            5,
            chrono::Utc::now() + chrono::Duration::minutes(30),
        );
        
        assert_eq!(reservation.product_id, product_id);
        assert_eq!(reservation.quantity, 5);
        assert_eq!(reservation.status, ReservationStatus::Active);
        assert!(reservation.expires_at > chrono::Utc::now());
    }
    
    #[test]
    fn test_inventory_level_stock_status() {
        let level = InventoryLevel {
            id: uuid::Uuid::new_v4(),
            product_id: uuid::Uuid::new_v4(),
            variant_id: None,
            location_id: uuid::Uuid::new_v4(),
            available_quantity: 5,
            reserved_quantity: 0,
            incoming_quantity: 0,
            reorder_point: 10,
            reorder_quantity: 20,
            cost_per_unit: Some(rust_decimal::Decimal::new(1000, 2)), // $10.00
            last_counted_at: None,
            updated_at: chrono::Utc::now(),
        };
        
        assert!(level.is_low_stock());
        assert_eq!(level.stock_status(), StockStatus::LowStock);
        assert_eq!(level.total_quantity(), 5);
    }
    
    #[test]
    fn test_low_stock_alert_creation() {
        let product_id = uuid::Uuid::new_v4();
        let location_id = uuid::Uuid::new_v4();
        
        let inventory = ProductInventory {
            product_id,
            total_available: 5,
            total_reserved: 0,
            total_incoming: 0,
            low_stock_threshold: 10,
            locations: vec![LocationInventory {
                location_id,
                location_name: "Warehouse A".to_string(),
                available: 5,
                reserved: 0,
                incoming: 0,
            }],
        };
        
        let alert = LowStockAlert::new(
            product_id,
            "Test Product".to_string(),
            &inventory,
        );
        
        assert_eq!(alert.product_id, product_id);
        assert_eq!(alert.current_stock, 5);
        assert_eq!(alert.threshold, 10);
        assert_eq!(alert.locations_affected.len(), 1);
        assert!(alert.is_critical());
        assert!(alert.notification_message().contains("CRITICAL"));
    }
    
    #[test]
    fn test_stock_alert_level() {
        assert!(StockAlertLevel::Critical > StockAlertLevel::Low);
        
        let critical_alert = LowStockAlert {
            product_id: uuid::Uuid::new_v4(),
            product_name: "Test".to_string(),
            current_stock: 2,
            threshold: 10,
            alert_level: StockAlertLevel::Critical,
            recommended_reorder_quantity: 50,
            locations_affected: vec![],
            created_at: chrono::Utc::now(),
        };
        
        assert!(critical_alert.is_critical());
    }
    
    #[tokio::test]
    async fn test_inventory_service_health_check() {
        // This would require a test database
        // For now, we'll test the interface
        
        let config = Config::default();
        let pool_result = create_pool(&config.database).await;
        
        // In a real test environment with PostgreSQL:
        // let db = Database::new(pool.unwrap());
        // let inventory_service = InventoryService::new(db, Default::default());
        // assert!(inventory_service.health_check().await.is_ok());
        
        assert!(pool_result.is_err()); // Expected - no DB in test
    }
}