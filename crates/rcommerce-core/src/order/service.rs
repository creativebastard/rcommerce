use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

use crate::{Result, Error};
use crate::order::{Order, OrderItem, CreateOrderRequest, CreateOrderItem, OrderStatus, PaymentStatus, FulfillmentStatus};
use crate::order::lifecycle::OrderEventDispatcher;
use crate::repository::Database;
use crate::payment::PaymentGateway;
use crate::inventory::InventoryService;

pub struct OrderService {
    db: Database,
    payment_gateway: Box<dyn PaymentGateway>,
    inventory_service: InventoryService,
    event_dispatcher: OrderEventDispatcher,
}

impl OrderService {
    pub fn new(
        db: Database,
        payment_gateway: Box<dyn PaymentGateway>,
        inventory_service: InventoryService,
        event_dispatcher: OrderEventDispatcher,
    ) -> Self {
        Self {
            db,
            payment_gateway,
            inventory_service,
            event_dispatcher,
        }
    }
    
    /// Create a new order
    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<Order> {
        // Validate customer if provided
        if let Some(customer_id) = request.customer_id {
            self.validate_customer(customer_id).await?;
        }
        
        // Validate addresses if provided
        if let Some(billing_id) = request.billing_address_id {
            self.validate_address(billing_id).await?;
        }
        
        if let Some(shipping_id) = request.shipping_address_id {
            self.validate_address(shipping_id).await?;
        }
        
        // Validate and reserve inventory for items
        let mut order_items = Vec::new();
        let mut subtotal = Decimal::ZERO;
        let mut total = Decimal::ZERO;
        
        for (index, item) in request.items.iter().enumerate() {
            // Validate product exists and is active
            self.validate_product(item.product_id).await?;
            
            // Reserve inventory
            let reservation = self.inventory_service.reserve_stock(
                crate::inventory::StockReservation::new(
                    item.product_id,
                    item.variant_id,
                    self.get_default_location_id().await?, // TODO: Support multiple locations
                    Uuid::nil(), // Will be updated after order creation
                    item.quantity,
                    chrono::Utc::now() + chrono::Duration::minutes(30),
                )
            ).await?;
            
            // Calculate item totals
            let item_subtotal = item.price * Decimal::from(item.quantity);
            let item_tax = self.calculate_tax(&item).await?;
            let item_total = item_subtotal + item_tax;
            
            subtotal += item_subtotal;
            total += item_total;
            
            // Create order item
            let order_item = OrderItem {
                id: Uuid::new_v4(),
                order_id: Uuid::nil(), // Will be updated
                product_id: item.product_id,
                variant_id: item.variant_id,
                quantity: item.quantity,
                price: item.price,
                subtotal: item_subtotal,
                tax_amount: item_tax,
                total: item_total,
                sku: None, // TODO: Fetch from product
                name: format!("Item {}", index + 1), // TODO: Fetch from product
                variant_name: None,
                weight: None, // TODO: Fetch from product
                metadata: serde_json::json!({
                    "reservation_id": reservation.id,
                }),
                created_at: chrono::Utc::now(),
            };
            
            order_items.push(order_item);
        }
        
        // Generate order number
        let order_number = self.generate_order_number().await?;
        
        // Create order
        let order_id = Uuid::new_v4();
        let order = sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO orders (
                id, order_number, customer_id, customer_email,
                billing_address_id, shipping_address_id,
                status, fulfillment_status, payment_status,
                currency, subtotal, tax_total, shipping_total, discount_total, total,
                notes, tags, metadata
            )
            VALUES (
                $1, $2, $3, $4,
                $5, $6,
                'pending', 'pending', 'pending',
                'USD', $7, $8, 0, 0, $9,
                $10, $11, $12
            )
            RETURNING *
            "#
        )
        .bind(order_id)
        .bind(order_number)
        .bind(request.customer_id)
        .bind(request.customer_email)
        .bind(request.billing_address_id)
        .bind(request.shipping_address_id)
        .bind(subtotal)
        .bind(Decimal::ZERO) // TODO: Calculate actual tax
        .bind(total)
        .bind(request.notes)
        .bind(request.tags)
        .bind(request.metadata)
        .fetch_one(self.db.pool())
        .await?;
        
        // Create order items
        for mut item in order_items {
            item.order_id = order_id;
            
            // Check for reservation before moving metadata
            let reservation_id: Option<Uuid> = if let serde_json::Value::Object(ref map) = &item.metadata {
                map.get("reservation_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
            } else {
                None
            };
            
            sqlx::query(
                r#"
                INSERT INTO order_items (
                    id, order_id, product_id, variant_id,
                    quantity, price, subtotal, tax_amount, total,
                    sku, name, variant_name, weight, metadata
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                "#
            )
            .bind(item.id)
            .bind(item.order_id)
            .bind(item.product_id)
            .bind(item.variant_id)
            .bind(item.quantity)
            .bind(item.price)
            .bind(item.subtotal)
            .bind(item.tax_amount)
            .bind(item.total)
            .bind(item.sku)
            .bind(item.name)
            .bind(item.variant_name)
            .bind(item.weight)
            .bind(&item.metadata)
            .execute(self.db.pool())
            .await?;
            
            // Update reservation reference if found
            if let Some(reservation_id) = reservation_id {
                sqlx::query("UPDATE stock_reservations SET order_id = $1 WHERE id = $2")
                    .bind(order_id)
                    .bind(reservation_id)
                    .execute(self.db.pool())
                    .await?;
            }
        }
        
        // Dispatch order created event
        self.event_dispatcher.order_created(&order).await?;
        
        Ok(order)
    }
    
    /// Get order by ID
    pub async fn get_order(&self, order_id: Uuid) -> Result<Option<OrderDetail>> {
        let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
            .bind(order_id)
            .fetch_optional(self.db.pool())
            .await?;
        
        let order = match order {
            Some(o) => o,
            None => return Ok(None),
        };
        
        let items = self.get_order_items(order_id).await?;
        
        Ok(Some(OrderDetail {
            order: order.clone(), // Clone for the main order
            items,
            // TODO: Add customer, addresses, payments, fulfillments
        }))
    }
    
    /// List orders with filtering
    pub async fn list_orders(&self, filter: super::OrderFilter) -> Result<Vec<Order>> {
        let mut query = String::from("SELECT * FROM orders WHERE 1=1");
        
        if let Some(customer_id) = filter.customer_id {
            query.push_str(&format!(" AND customer_id = '{}'", customer_id));
        }
        
        if let Some(status) = filter.status {
            query.push_str(&format!(" AND status = '{}'", format!("{:?}", status).to_lowercase()));
        }
        
        query.push_str(" ORDER BY created_at DESC");
        
        let orders = sqlx::query_as::<_, Order>(&query)
            .fetch_all(self.db.pool())
            .await?;
        
        Ok(orders)
    }
    
    /// Update order status
    pub async fn update_order_status(&self, order_id: Uuid, new_status: OrderStatus) -> Result<Order> {
        // Get current order
        let order = self.get_order(order_id).await?
            .ok_or_else(|| Error::not_found("Order not found"))?;
        
        // Validate transition
        if !self.is_valid_status_transition(order.order.status, new_status) {
            return Err(Error::validation(format!(
                "Invalid status transition from {:?} to {:?}",
                order.order.status, new_status
            )));
        }
        
        // Update status
        let updated_order = sqlx::query_as::<_, Order>(
            "UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(format!("{:?}", new_status).to_lowercase())
        .bind(order_id)
        .fetch_one(self.db.pool())
        .await?;
        
        // Dispatch status change event
        self.event_dispatcher.order_status_changed(&updated_order, order.order.status).await?;
        
        Ok(updated_order)
    }
    
    /// Process payment for an order
    pub async fn process_payment(&self, order_id: Uuid, payment_method: crate::payment::CreatePaymentRequest) -> Result<crate::payment::Payment> {
        // Get order
        let order = self.get_order(order_id).await?
            .ok_or_else(|| Error::not_found("Order not found"))?;
        
        // Check if already paid
        if order.order.payment_status == PaymentStatus::Paid {
            return Err(Error::validation("Order already paid"));
        }
        
        // Create payment via gateway
        let payment = self.payment_gateway.create_payment(payment_method).await?;
        
        // Confirm payment
        let confirmed_payment = self.payment_gateway.confirm_payment(&payment.id).await?;
        
        // Update order payment status
        let new_payment_status = match confirmed_payment.status {
            crate::payment::PaymentStatus::Succeeded => PaymentStatus::Paid,
            crate::payment::PaymentStatus::Failed => PaymentStatus::Failed,
            _ => PaymentStatus::Pending,
        };
        
        sqlx::query(
            "UPDATE orders SET payment_status = $1, updated_at = NOW() WHERE id = $2"
        )
        .bind(format!("{:?}", new_payment_status).to_lowercase())
        .bind(order_id)
        .execute(self.db.pool())
        .await?;
        
        // If payment succeeded, update order status
        if new_payment_status == PaymentStatus::Paid {
            self.update_order_status(order_id, OrderStatus::Confirmed).await?;
        }
        
        // Commit inventory reservations
        self.commit_inventory_reservations(order_id).await?;
        
        Ok(confirmed_payment)
    }
    
    /// Cancel an order
    pub async fn cancel_order(&self, order_id: Uuid, reason: String) -> Result<Order> {
        // Get order
        let order = self.get_order(order_id).await?
            .ok_or_else(|| Error::not_found("Order not found"))?;
        
        // Check if can be canceled
        if !self.can_cancel_order(&order.order) {
            return Err(Error::validation("Order cannot be canceled in current status"));
        }
        
        // Release inventory reservations
        self.release_inventory_reservations(order_id).await?;
        
        // Update cancellation reason in metadata first (before reason is consumed)
        sqlx::query("UPDATE orders SET metadata = jsonb_set(metadata, '{cancellation_reason}', $1) WHERE id = $2")
            .bind(serde_json::json!(&reason))
            .bind(order_id)
            .execute(self.db.pool())
            .await?;
        
        // Process refund if payment was made
        if order.order.payment_status == PaymentStatus::Paid {
            self.process_refund(order_id, reason).await?;
        }
        
        // Update order status
        let canceled_order = self.update_order_status(order_id, OrderStatus::Canceled).await?;
        
        Ok(canceled_order)
    }
    
    /// Fulfill an order
    pub async fn fulfill_order(&self, order_id: Uuid) -> Result<crate::order::Fulfillment> {
        // Get order
        let order = self.get_order(order_id).await?
            .ok_or_else(|| Error::not_found("Order not found"))?;
        
        // Check if can be fulfilled
        if order.order.status != OrderStatus::Confirmed {
            return Err(Error::validation("Order must be confirmed before fulfillment"));
        }
        
        // Create fulfillment record
        let fulfillment_id = Uuid::new_v4();
        let fulfillment = sqlx::query_as::<_, crate::order::Fulfillment>(
            r#"
            INSERT INTO fulfillments (id, order_id, status, tracking_number, tracking_url, tracking_company)
            VALUES ($1, $2, 'processing', NULL, NULL, NULL)
            RETURNING *
            "#
        )
        .bind(fulfillment_id)
        .bind(order_id)
        .fetch_one(self.db.pool())
        .await?;
        
        // Update order fulfillment status
        sqlx::query("UPDATE orders SET fulfillment_status = 'processing' WHERE id = $1")
            .bind(order_id)
            .execute(self.db.pool())
            .await?;
        
        // Dispatch fulfillment created event
        self.event_dispatcher.fulfillment_created(&fulfillment).await?;
        
        Ok(fulfillment)
    }
    
    // Helper methods
    
    async fn validate_customer(&self, customer_id: Uuid) -> Result<()> {
        sqlx::query("SELECT 1 FROM customers WHERE id = $1")
            .bind(customer_id)
            .fetch_optional(self.db.pool())
            .await?
            .ok_or_else(|| Error::not_found("Customer not found"))?;
        
        Ok(())
    }
    
    async fn validate_address(&self, address_id: Uuid) -> Result<()> {
        sqlx::query("SELECT 1 FROM addresses WHERE id = $1")
            .bind(address_id)
            .fetch_optional(self.db.pool())
            .await?
            .ok_or_else(|| Error::not_found("Address not found"))?;
        
        Ok(())
    }
    
    async fn validate_product(&self, product_id: Uuid) -> Result<()> {
        sqlx::query("SELECT 1 FROM products WHERE id = $1 AND is_active = true")
            .bind(product_id)
            .fetch_optional(self.db.pool())
            .await?
            .ok_or_else(|| Error::not_found("Product not found or inactive"))?;
        
        Ok(())
    }
    
    async fn get_order_items(&self, order_id: Uuid) -> Result<Vec<crate::order::OrderItem>> {
        let items = sqlx::query_as::<_, crate::order::OrderItem>(
            "SELECT * FROM order_items WHERE order_id = $1 ORDER BY created_at"
        )
        .bind(order_id)
        .fetch_all(self.db.pool())
        .await?;
        
        Ok(items)
    }
    
    fn is_valid_status_transition(&self, from: OrderStatus, to: OrderStatus) -> bool {
        use OrderStatus::*;
        
        match (from, to) {
            // Can go from pending to confirmed (payment received)
            (Pending, Confirmed) => true,
            // Can go from confirmed to processing (being prepared)
            (Confirmed, Processing) => true,
            // Can go from processing to shipped (sent to customer)
            (Processing, Shipped) => true,
            // Can go from shipped to delivered (customer received)
            (Shipped, Delivered) => true,
            // Can go from delivered to completed (all done)
            (Delivered, Completed) => true,
            // Can cancel from most states
            (_, Canceled) => true,
            // Can refund from paid states
            (Paid, Refunded) => true,
            (Completed, Refunded) => true,
            // Otherwise invalid
            _ => false,
        }
    }
    
    fn can_cancel_order(&self, order: &Order) -> bool {
        use OrderStatus::*;
        
        match order.status {
            Pending | Confirmed | Processing => true,
            _ => false,
        }
    }
    
    async fn generate_order_number(&self) -> Result<String> {
        // Simple incrementing order number
        // In production, use a more sophisticated system
        let prefix = "ORD";
        let timestamp = chrono::Utc::now().format("%Y%m%d");
        let random: u32 = rand::random();
        
        Ok(format!("{}-{}-{}", prefix, timestamp, random))
    }
    
    async fn calculate_tax(&self, _item: &CreateOrderItem) -> Result<Decimal> {
        // TODO: Implement tax calculation based on customer location
        // For now, return 0 (tax-free)
        Ok(Decimal::ZERO)
    }
    
    async fn get_default_location_id(&self) -> Result<Uuid> {
        // For now, return a static location ID
        // In production, select based on routing rules
        Ok(Uuid::nil())
    }
    
    async fn commit_inventory_reservations(&self, order_id: Uuid) -> Result<()> {
        let reservations = sqlx::query_as::<_, crate::inventory::StockReservation>(
            "SELECT * FROM stock_reservations WHERE order_id = $1 AND status = 'active'"
        )
        .bind(order_id)
        .fetch_all(self.db.pool())
        .await?;
        
        for reservation in reservations {
            self.inventory_service.commit_reservation(reservation.id).await?;
        }
        
        Ok(())
    }
    
    async fn release_inventory_reservations(&self, order_id: Uuid) -> Result<()> {
        let reservations = sqlx::query_as::<_, crate::inventory::StockReservation>(
            "SELECT * FROM stock_reservations WHERE order_id = $1 AND status IN ('active', 'committed')"
        )
        .bind(order_id)
        .fetch_all(self.db.pool())
        .await?;
        
        for reservation in reservations {
            self.inventory_service.release_reservation(reservation.id).await?;
        }
        
        Ok(())
    }
    
    async fn process_refund(&self, _order_id: Uuid, _reason: String) -> Result<()> {
        // TODO: Implement refund processing via payment gateway
        log::info!("Processing refund for order {}", _order_id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::services::Service for OrderService {
    async fn health_check(&self) -> Result<()> {
        // Check database connectivity
        let _ = sqlx::query("SELECT 1").fetch_one(self.db.pool()).await?;
        Ok(())
    }
}

/// Order detail with related data
#[derive(Debug, Clone)]
pub struct OrderDetail {
    pub order: Order,
    pub items: Vec<OrderItem>,
    // TODO: Add customer, addresses, payments, fulfillments
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_valid_status_transition() {
        let service = OrderService {
            db: Database::new(sqlx::PgPool::connect("postgres://localhost:5433/test").await.unwrap()),
            payment_gateway: Box::new(crate::payment::gateways::StripeGateway::new("sk_test".to_string(), "whsec_".to_string())),
            inventory_service: InventoryService::new(db.clone(), Default::default()),
            event_dispatcher: OrderEventDispatcher::new(),
        };
        
        assert!(service.is_valid_status_transition(OrderStatus::Pending, OrderStatus::Confirmed));
        assert!(service.is_valid_status_transition(OrderStatus::Confirmed, OrderStatus::Processing));
        assert!(service.is_valid_status_transition(OrderStatus::Processing, OrderStatus::Shipped));
        assert!(!service.is_valid_status_transition(OrderStatus::Pending, OrderStatus::Shipped));
    }
}