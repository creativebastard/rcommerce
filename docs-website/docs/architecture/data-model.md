# Data Model

## Overview

R Commerce uses a comprehensive data model designed for ecommerce operations at scale. The schema is optimized for PostgreSQL and supports complex product variants, order management, and customer relationships.

## Entity Relationship Diagram

```
┌─────────────────┐          ┌─────────────────┐
│    Customer     │          │    Address      │
├─────────────────┤          ├─────────────────┤
│ id (UUID)       │◄─────────┤ id (UUID)       │
│ email           │          │ customer_id     │
│ first_name      │          │ street1         │
│ last_name       │          │ city            │
│ phone           │          │ state           │
│ created_at      │          │ postal_code     │
│ updated_at      │          │ country         │
└────────┬────────┘          └─────────────────┘
         │
         │ creates
         ▼
┌─────────────────┐
│     Order       │
├─────────────────┤          ┌─────────────────┐
│ id (UUID)       │          │   OrderNote     │
│ order_number    │◄─────────┤ id (UUID)       │
│ customer_id     │          │ order_id        │
│ total           │          │ content         │
│ status          │          │ created_at      │
│ created_at      │          └─────────────────┘
└────────┬────────┘
         │
         │ contains
         │
         ▼
┌─────────────────┐          ┌─────────────────┐
│ OrderLineItem   │          │    Payment      │
├─────────────────┤          ├─────────────────┤
│ id (UUID)       │          │ id (UUID)       │
│ order_id        │◄─────────┤ order_id        │
│ product_id      │          │ amount          │
│ quantity        │          │ status          │
│ unit_price      │          │ gateway         │
│ total           │          │ created_at      │
└─────────────────┘          └─────────────────┘

┌─────────────────┐          ┌─────────────────┐
│    Product      │          │ ProductVariant  │
├─────────────────┤          ├─────────────────┤
│ id (UUID)       │          │ id (UUID)       │
│ title           │          │ product_id      │
│ slug            │          │ sku             │
│ price           │◄─────────┤ price           │
│ inventory_qty   │          │ inventory_qty   │
│ status          │          │ created_at      │
│ created_at      │          └─────────────────┘
└─────────────────┘
```

## Core Entities

### Customer

The customer entity represents registered users of the store.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| email | String | Unique email address |
| first_name | String | Customer's first name |
| last_name | String | Customer's last name |
| phone | String | Optional phone number |
| password_hash | String | Bcrypt hashed password |
| is_verified | Boolean | Email verification status |
| accepts_marketing | Boolean | Marketing consent |
| currency | Currency | Preferred currency |
| created_at | DateTime | Registration timestamp |
| updated_at | DateTime | Last update timestamp |

### Address

Addresses are associated with customers and used for billing/shipping.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| customer_id | UUID | Reference to customer |
| first_name | String | Recipient first name |
| last_name | String | Recipient last name |
| company | String | Optional company name |
| street1 | String | Street address line 1 |
| street2 | String | Street address line 2 |
| city | String | City name |
| state | String | State/Province |
| postal_code | String | ZIP/Postal code |
| country | String | ISO country code |
| phone | String | Contact phone |
| is_default | Boolean | Default address flag |

### Product

Products are the core sellable items in the catalog.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| title | String | Product name |
| slug | String | URL-friendly identifier |
| description | String | Product description |
| sku | String | Stock keeping unit |
| product_type | Enum | Simple/Variable/Digital/Bundle |
| price | Decimal | Base price |
| compare_at_price | Decimal | Original price for sales |
| cost_price | Decimal | Cost for margin calculation |
| currency | Currency | Price currency |
| inventory_quantity | Integer | Stock level |
| inventory_management | Boolean | Track inventory flag |
| is_active | Boolean | Published status |
| is_featured | Boolean | Featured product flag |
| requires_shipping | Boolean | Physical product flag |
| created_at | DateTime | Creation timestamp |
| updated_at | DateTime | Last update timestamp |

### Product Variant

Variants represent different options of a product (size, color, etc.).

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| product_id | UUID | Parent product reference |
| title | String | Variant name |
| sku | String | Unique SKU |
| price | Decimal | Variant-specific price |
| inventory_quantity | Integer | Variant stock level |
| is_active | Boolean | Available for purchase |

### Order

Orders represent customer purchase transactions.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| order_number | String | Human-readable order ID |
| customer_id | UUID | Reference to customer |
| customer_email | String | Email at time of order |
| subtotal | Decimal | Sum of line items |
| tax_amount | Decimal | Total tax |
| shipping_amount | Decimal | Shipping cost |
| discount_amount | Decimal | Applied discounts |
| total_amount | Decimal | Final total |
| currency | Currency | Order currency |
| status | Enum | pending/confirmed/processing/shipped/completed/cancelled |
| payment_status | Enum | pending/authorized/paid/failed/refunded |
| fulfillment_status | Enum | pending/processing/shipped/delivered |
| created_at | DateTime | Order timestamp |
| updated_at | DateTime | Last update timestamp |

### Order Line Item

Line items represent individual products within an order.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| order_id | UUID | Parent order reference |
| product_id | UUID | Product reference |
| variant_id | UUID | Variant reference (optional) |
| title | String | Product name at time of order |
| sku | String | SKU at time of order |
| quantity | Integer | Quantity ordered |
| unit_price | Decimal | Price per unit |
| total | Decimal | Line total |

### Payment

Payments record transaction attempts and completions.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| order_id | UUID | Associated order |
| gateway | String | Payment provider |
| amount | Decimal | Payment amount |
| currency | Currency | Payment currency |
| status | Enum | pending/authorized/paid/failed/refunded |
| provider_id | String | External transaction ID |
| created_at | DateTime | Transaction timestamp |

### API Key

API keys enable service-to-service authentication.

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Primary key |
| customer_id | UUID | Optional owner reference |
| key_prefix | String | Public identifier |
| key_hash | String | Hashed secret |
| name | String | Descriptive name |
| scopes | Array | Permission scopes |
| is_active | Boolean | Key status |
| expires_at | DateTime | Optional expiration |
| last_used_at | DateTime | Last usage timestamp |
| created_at | DateTime | Creation timestamp |

## Database Schema

### PostgreSQL Types

```sql
-- Order status enum
CREATE TYPE order_status AS ENUM (
    'pending',
    'confirmed',
    'processing',
    'on_hold',
    'shipped',
    'completed',
    'cancelled',
    'refunded'
);

-- Payment status enum
CREATE TYPE payment_status AS ENUM (
    'pending',
    'authorized',
    'paid',
    'partially_refunded',
    'fully_refunded',
    'failed',
    'cancelled'
);

-- Product type enum
CREATE TYPE product_type AS ENUM (
    'Simple',
    'Variable',
    'Subscription',
    'Digital',
    'Bundle'
);

-- Currency enum
CREATE TYPE currency AS ENUM (
    'USD', 'EUR', 'GBP', 'JPY', 
    'AUD', 'CAD', 'CNY', 'HKD', 'SGD'
);
```

### Key Tables

See the [full schema documentation](../../docs/architecture/02-data-modeling.md) for complete SQL definitions.

## Relationships

### Customer Relationships
- Customer has many Addresses
- Customer has many Orders
- Customer has many API Keys

### Product Relationships
- Product has many Variants
- Product has many Images
- Product belongs to Categories
- Product has many Order Line Items

### Order Relationships
- Order belongs to Customer
- Order has many Line Items
- Order has many Payments
- Order has many Fulfillments
- Order has many Notes

## Indexes

Performance-critical indexes:

```sql
-- Customer lookups
CREATE INDEX idx_customers_email ON customers(email);
CREATE INDEX idx_customers_created_at ON customers(created_at DESC);

-- Product searches
CREATE UNIQUE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_products_status ON products(status);
CREATE INDEX idx_products_search ON products USING gin(
    to_tsvector('english', title || ' ' || COALESCE(description, ''))
);

-- Order queries
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);

-- API key lookups
CREATE INDEX idx_api_keys_prefix ON api_keys(key_prefix);
CREATE INDEX idx_api_keys_customer ON api_keys(customer_id);
```

## Data Integrity

### Constraints
- Foreign key constraints with CASCADE for relationships
- Check constraints for positive prices and quantities
- Unique constraints on emails, slugs, SKUs
- Enum validation for status fields

### Triggers
- Automatic `updated_at` timestamp updates
- Inventory adjustment on order placement
- Audit log entries on data changes

## Next Steps

- [Database Abstraction](./database-abstraction.md) - Repository pattern
- [Order Management](./order-management.md) - Order lifecycle
- [Media Storage](./media-storage.md) - File handling
