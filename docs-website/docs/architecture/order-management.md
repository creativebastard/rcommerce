# Order Management

## Overview

The order management system in R Commerce handles the complete lifecycle of customer purchases, from cart to fulfillment.

## Order Lifecycle

```
┌─────────┐    ┌──────────┐    ┌───────────┐    ┌──────────┐    ┌───────────┐
│ Pending │───▶│Confirmed │───▶│Processing │───▶│ Shipped  │───▶│Completed  │
└─────────┘    └──────────┘    └───────────┘    └──────────┘    └───────────┘
     │                                                    │
     │              ┌──────────┐                          │
     └─────────────▶│ Cancelled│◀─────────────────────────┘
                    └──────────┘
```

### Status Definitions

| Status | Description |
|--------|-------------|
| **Pending** | Order created, awaiting payment confirmation |
| **Confirmed** | Payment received, ready for processing |
| **Processing** | Order being prepared for shipment |
| **On Hold** | Manual review required |
| **Shipped** | Order dispatched, in transit |
| **Completed** | Order delivered, fulfilled |
| **Cancelled** | Order cancelled (before shipment) |
| **Refunded** | Order refunded (after cancellation or return) |

## Order Structure

### Order Header
Contains summary information:
- Customer details
- Billing/shipping addresses
- Financial totals
- Status tracking

### Line Items
Individual products in the order:
- Product reference
- Variant (size, color, etc.)
- Quantity
- Pricing (unit price, discounts, tax)

### Payments
Payment transactions:
- Gateway used
- Amount and currency
- Status (pending, authorized, paid, failed, refunded)
- Transaction IDs

### Fulfillments
Shipping records:
- Carrier and service
- Tracking numbers
- Items shipped
- Status updates

## Order Processing Flow

### 1. Order Creation
```rust
// Create order from cart
let order = order_service
    .create_from_cart(cart_id, customer_id)
    .await?;
```

### 2. Payment Processing
```rust
// Process payment
let payment = payment_service
    .process(order.id, payment_method)
    .await?;

// Update order status
order_service.update_status(order.id, OrderStatus::Confirmed).await?;
```

### 3. Inventory Reservation
- Reserve inventory when order is confirmed
- Release reservation if order is cancelled
- Adjust inventory on fulfillment

### 4. Fulfillment
```rust
// Create fulfillment
let fulfillment = fulfillment_service
    .create(order.id, items_to_ship)
    .await?;

// Generate shipping label
let label = shipping_service
    .create_label(fulfillment.id, carrier)
    .await?;
```

## Order Notes

Staff can add notes to orders:
- Internal notes (staff only)
- Customer-visible notes
- System-generated notes (status changes, etc.)

## Order Editing

Orders can be edited before shipment:
- Add/remove items
- Change quantities
- Update addresses
- Apply discounts

## Fraud Detection

Basic fraud scoring:
- Order value thresholds
- Velocity checks
- Address verification
- Risk scoring integration

## Webhook Events

Order events trigger webhooks:
- `order.created`
- `order.paid`
- `order.shipped`
- `order.completed`
- `order.cancelled`

## See Also

- [Data Model](./data-model.md) - Order entity relationships
- [Database Abstraction](./database-abstraction.md) - Data access patterns
