# Inventory API

!!! warning "Module Only - API Coming Soon"
    The Inventory module is implemented for internal use with product stock tracking. The public REST API endpoints described below are planned for release in v0.2. Basic inventory operations are available through the product API.

The Inventory API provides comprehensive stock management including inventory levels, stock reservations, movements, and low stock alerts. It supports multi-location inventory tracking and automated stock reservation for orders.

## Base URL

```
/api/v1/inventory
```

## Authentication

All inventory endpoints require authentication via API key or JWT token.

```http
Authorization: Bearer YOUR_API_KEY
```

## Inventory Object

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "total_available": 150,
  "total_reserved": 25,
  "total_incoming": 50,
  "low_stock_threshold": 20,
  "locations": [
    {
      "location_id": "550e8400-e29b-41d4-a716-446655440020",
      "location_name": "Warehouse A",
      "available": 100,
      "reserved": 15,
      "incoming": 30
    },
    {
      "location_id": "550e8400-e29b-41d4-a716-446655440021",
      "location_name": "Warehouse B",
      "available": 50,
      "reserved": 10,
      "incoming": 20
    }
  ]
}
```

### Inventory Fields

| Field | Type | Description |
|-------|------|-------------|
| `product_id` | UUID | Product unique identifier |
| `total_available` | integer | Total available quantity across all locations |
| `total_reserved` | integer | Total reserved quantity for pending orders |
| `total_incoming` | integer | Total incoming stock (in transit) |
| `low_stock_threshold` | integer | Alert threshold for low stock |
| `locations` | array | Inventory breakdown by location |

### Location Inventory Fields

| Field | Type | Description |
|-------|------|-------------|
| `location_id` | UUID | Location unique identifier |
| `location_name` | string | Human-readable location name |
| `available` | integer | Available quantity at location |
| `reserved` | integer | Reserved quantity at location |
| `incoming` | integer | Incoming quantity at location |

## Endpoints

### List Inventory Levels

```http
GET /api/v1/inventory
```

Retrieve a paginated list of inventory levels across all products.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `product_id` | UUID | Filter by specific product |
| `location_id` | UUID | Filter by specific location |
| `stock_status` | string | Filter by `in_stock`, `low_stock`, `out_of_stock` |
| `low_stock_only` | boolean | Show only low stock items |
| `include_incoming` | boolean | Include incoming stock in calculations |
| `sort` | string | `product_id`, `available`, `updated_at` |
| `order` | string | `asc` or `desc` (default: desc) |

#### Example Request

```http
GET /api/v1/inventory?stock_status=low_stock&per_page=50
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": [
    {
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "Premium Cotton T-Shirt",
      "total_available": 150,
      "total_reserved": 25,
      "total_incoming": 50,
      "low_stock_threshold": 20,
      "stock_status": "in_stock",
      "locations": [
        {
          "location_id": "550e8400-e29b-41d4-a716-446655440020",
          "location_name": "Warehouse A",
          "available": 100,
          "reserved": 15,
          "incoming": 30
        }
      ],
      "updated_at": "2024-01-20T14:30:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 156,
      "per_page": 50,
      "current_page": 1,
      "total_pages": 4,
      "has_next": true,
      "has_prev": false
    },
    "request_id": "req_abc123"
  }
}
```

### Get Product Inventory

```http
GET /api/v1/inventory/{product_id}
```

Retrieve detailed inventory information for a specific product.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `product_id` | UUID | Product unique identifier |

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `include_history` | boolean | Include recent stock movements |
| `include_reservations` | boolean | Include active reservations |

#### Example Request

```http
GET /api/v1/inventory/550e8400-e29b-41d4-a716-446655440000?include_history=true
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": {
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "product_name": "Premium Cotton T-Shirt",
    "total_available": 150,
    "total_reserved": 25,
    "total_incoming": 50,
    "low_stock_threshold": 20,
    "stock_status": "in_stock",
    "locations": [
      {
        "location_id": "550e8400-e29b-41d4-a716-446655440020",
        "location_name": "Warehouse A",
        "available": 100,
        "reserved": 15,
        "incoming": 30
      },
      {
        "location_id": "550e8400-e29b-41d4-a716-446655440021",
        "location_name": "Warehouse B",
        "available": 50,
        "reserved": 10,
        "incoming": 20
      }
    ],
    "recent_movements": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440030",
        "movement_type": "in",
        "quantity": 50,
        "location_name": "Warehouse A",
        "reference": "PO-2024-001",
        "created_at": "2024-01-19T10:00:00Z"
      },
      {
        "id": "550e8400-e29b-41d4-a716-446655440031",
        "movement_type": "out",
        "quantity": -10,
        "location_name": "Warehouse A",
        "reference": "Order #1001",
        "created_at": "2024-01-18T15:30:00Z"
      }
    ],
    "active_reservations": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440040",
        "order_id": "550e8400-e29b-41d4-a716-446655440100",
        "quantity": 5,
        "location_name": "Warehouse A",
        "expires_at": "2024-01-20T15:00:00Z"
      }
    ],
    "updated_at": "2024-01-20T14:30:00Z"
  }
}
```

### Adjust Inventory

```http
POST /api/v1/inventory/adjust
```

Manually adjust inventory levels with audit trail.

#### Request Body

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "adjustment": -5,
  "reason": "damaged",
  "notes": "Water damage during storage",
  "reference": "ADJ-2024-001"
}
```

#### Adjustment Reasons

| Reason | Description |
|--------|-------------|
| `damaged` | Product damaged or defective |
| `lost` | Inventory lost or missing |
| `found` | Additional inventory found |
| `count` | Stock count correction |
| `return` | Customer return |
| `other` | Other reason (specify in notes) |

#### Required Fields

- `product_id` - Product to adjust
- `location_id` - Location to adjust
- `adjustment` - Quantity to add (positive) or remove (negative)
- `reason` - Reason for adjustment

#### Example Response

```json
{
  "data": {
    "movement_id": "550e8400-e29b-41d4-a716-446655440050",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "adjustment": -5,
    "quantity_before": 100,
    "quantity_after": 95,
    "reason": "damaged",
    "notes": "Water damage during storage",
    "reference": "ADJ-2024-001",
    "adjusted_by": "550e8400-e29b-41d4-a716-446655440200",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### Set Inventory

```http
POST /api/v1/inventory/set
```

Set absolute inventory quantity (useful for stock counts).

#### Request Body

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "quantity": 95,
  "reason": "count",
  "notes": "Monthly stock count",
  "reference": "COUNT-2024-01"
}
```

#### Example Response

```json
{
  "data": {
    "movement_id": "550e8400-e29b-41d4-a716-446655440051",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "quantity_before": 100,
    "quantity_after": 95,
    "adjustment": -5,
    "reason": "count",
    "notes": "Monthly stock count",
    "reference": "COUNT-2024-01",
    "adjusted_by": "550e8400-e29b-41d4-a716-446655440200",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### Receive Stock

```http
POST /api/v1/inventory/receive
```

Receive incoming stock (inbound inventory).

#### Request Body

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "quantity": 50,
  "cost_per_unit": "12.50",
  "reference": "PO-2024-001",
  "notes": "Received from supplier"
}
```

#### Example Response

```json
{
  "data": {
    "movement_id": "550e8400-e29b-41d4-a716-446655440052",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "movement_type": "in",
    "quantity": 50,
    "quantity_before": 100,
    "quantity_after": 150,
    "cost_per_unit": "12.50",
    "reference": "PO-2024-001",
    "notes": "Received from supplier",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### Reserve Stock

```http
POST /api/v1/inventory/reserve
```

Reserve stock for an order. Reservations expire automatically after the configured timeout (default: 30 minutes).

#### Request Body

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "quantity": 5
}
```

#### Required Fields

- `product_id` - Product to reserve
- `location_id` - Location to reserve from
- `order_id` - Order to reserve for
- `quantity` - Quantity to reserve

#### Example Response

```json
{
  "data": {
    "reservation_id": "550e8400-e29b-41d4-a716-446655440060",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "variant_id": "550e8400-e29b-41d4-a716-446655440001",
    "location_id": "550e8400-e29b-41d4-a716-446655440020",
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "quantity": 5,
    "status": "active",
    "expires_at": "2024-01-20T15:00:00Z",
    "created_at": "2024-01-20T14:30:00Z"
  }
}
```

### Release Reservation

```http
POST /api/v1/inventory/release
```

Release a stock reservation (e.g., when order is cancelled).

#### Request Body

```json
{
  "reservation_id": "550e8400-e29b-41d4-a716-446655440060",
  "reason": "order_cancelled"
}
```

#### Release Reasons

| Reason | Description |
|--------|-------------|
| `order_cancelled` | Order was cancelled |
| `order_expired` | Reservation expired |
| `manual_release` | Manually released by admin |
| `order_completed` | Order completed (reservation converted) |

#### Example Response

```json
{
  "data": {
    "reservation_id": "550e8400-e29b-41d4-a716-446655440060",
    "status": "released",
    "released_at": "2024-01-20T14:35:00Z",
    "reason": "order_cancelled"
  }
}
```

### Transfer Inventory

```http
POST /api/v1/inventory/transfer
```

Transfer stock between locations.

#### Request Body

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "variant_id": "550e8400-e29b-41d4-a716-446655440001",
  "from_location_id": "550e8400-e29b-41d4-a716-446655440020",
  "to_location_id": "550e8400-e29b-41d4-a716-446655440021",
  "quantity": 10,
  "reference": "TF-2024-001",
  "notes": "Restock retail location"
}
```

#### Example Response

```json
{
  "data": {
    "transfer_id": "550e8400-e29b-41d4-a716-446655440070",
    "product_id": "550e8400-e29b-41d4-a716-446655440000",
    "from_location_id": "550e8400-e29b-41d4-a716-446655440020",
    "to_location_id": "550e8400-e29b-41d4-a716-446655440021",
    "quantity": 10,
    "status": "completed",
    "reference": "TF-2024-001",
    "notes": "Restock retail location",
    "created_at": "2024-01-20T14:30:00Z",
    "completed_at": "2024-01-20T14:30:00Z"
  }
}
```

## Stock Movements

### List Stock Movements

```http
GET /api/v1/inventory/movements
```

Retrieve a paginated list of stock movements.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `product_id` | UUID | Filter by product |
| `location_id` | UUID | Filter by location |
| `movement_type` | string | Filter by `in`, `out`, `return`, `lost`, `found`, `transfer` |
| `start_date` | datetime | Movements after date |
| `end_date` | datetime | Movements before date |
| `reference` | string | Filter by reference number |
| `sort` | string | `created_at` (default) |
| `order` | string | `asc` or `desc` (default: desc) |

#### Example Request

```http
GET /api/v1/inventory/movements?product_id=550e8400-e29b-41d4-a716-446655440000&movement_type=in
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440030",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "Premium Cotton T-Shirt",
      "variant_id": null,
      "location_id": "550e8400-e29b-41d4-a716-446655440020",
      "location_name": "Warehouse A",
      "quantity": 50,
      "movement_type": "in",
      "cost_per_unit": "12.50",
      "reference": "PO-2024-001",
      "notes": "Received from supplier",
      "created_at": "2024-01-19T10:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440031",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "Premium Cotton T-Shirt",
      "variant_id": null,
      "location_id": "550e8400-e29b-41d4-a716-446655440020",
      "location_name": "Warehouse A",
      "quantity": -10,
      "movement_type": "out",
      "cost_per_unit": null,
      "reference": "Order #1001",
      "notes": "Customer order fulfillment",
      "created_at": "2024-01-18T15:30:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 45,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 3,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

### Movement Types

| Type | Description |
|------|-------------|
| `in` | Stock received |
| `out` | Stock sold or removed |
| `return` | Customer return |
| `lost` | Stock lost or damaged |
| `found` | Additional stock found |
| `transfer` | Stock transferred between locations |

## Low Stock Alerts

### List Low Stock Alerts

```http
GET /api/v1/inventory/alerts
```

Retrieve low stock alerts for products below their threshold.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `alert_level` | string | Filter by `low` or `critical` |
| `product_id` | UUID | Filter by specific product |
| `location_id` | UUID | Filter by specific location |
| `acknowledged` | boolean | Filter by acknowledgment status |
| `sort` | string | `created_at`, `alert_level` |
| `order` | string | `asc` or `desc` (default: desc) |

#### Example Request

```http
GET /api/v1/inventory/alerts?alert_level=critical
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": [
    {
      "alert_id": "550e8400-e29b-41d4-a716-446655440080",
      "product_id": "550e8400-e29b-41d4-a716-446655440000",
      "product_name": "Premium Cotton T-Shirt",
      "current_stock": 5,
      "threshold": 20,
      "alert_level": "critical",
      "recommended_reorder_quantity": 95,
      "locations_affected": [
        {
          "location_id": "550e8400-e29b-41d4-a716-446655440020",
          "location_name": "Warehouse A",
          "current_stock": 3,
          "alert_level": "critical"
        },
        {
          "location_id": "550e8400-e29b-41d4-a716-446655440021",
          "location_name": "Warehouse B",
          "current_stock": 2,
          "alert_level": "critical"
        }
      ],
      "acknowledged": false,
      "created_at": "2024-01-20T14:30:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 12,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  }
}
```

### Acknowledge Alert

```http
POST /api/v1/inventory/alerts/{alert_id}/acknowledge
```

Acknowledge a low stock alert.

#### Request Body

```json
{
  "notes": "Reorder placed with supplier"
}
```

#### Example Response

```json
{
  "data": {
    "alert_id": "550e8400-e29b-41d4-a716-446655440080",
    "acknowledged": true,
    "acknowledged_by": "550e8400-e29b-41d4-a716-446655440200",
    "acknowledged_at": "2024-01-20T15:00:00Z",
    "notes": "Reorder placed with supplier"
  }
}
```

### Alert Levels

| Level | Description |
|-------|-------------|
| `low` | Stock below threshold but above critical level (50% of threshold) |
| `critical` | Stock below critical level (50% of threshold) |

## Inventory Locations

### List Locations

```http
GET /api/v1/inventory/locations
```

Retrieve all inventory locations.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `is_active` | boolean | Filter by active status |

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440020",
      "name": "Warehouse A",
      "code": "WH-A",
      "address": "123 Industrial Blvd, Warehouse District",
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440021",
      "name": "Warehouse B",
      "code": "WH-B",
      "address": "456 Commerce St, Business Park",
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 2,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 1,
      "has_next": false,
      "has_prev": false
    }
  }
}
```

### Get Location

```http
GET /api/v1/inventory/locations/{location_id}
```

Retrieve details for a specific location.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `include_inventory` | boolean | Include inventory summary |

#### Example Response

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440020",
    "name": "Warehouse A",
    "code": "WH-A",
    "address": "123 Industrial Blvd, Warehouse District",
    "is_active": true,
    "inventory_summary": {
      "total_products": 156,
      "total_value": "45000.00",
      "low_stock_count": 12,
      "out_of_stock_count": 3
    },
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `PRODUCT_NOT_FOUND` | 404 | Product does not exist |
| `LOCATION_NOT_FOUND` | 404 | Inventory location does not exist |
| `INSUFFICIENT_STOCK` | 400 | Not enough stock available for operation |
| `INVALID_ADJUSTMENT` | 400 | Adjustment would result in negative inventory |
| `RESERVATION_NOT_FOUND` | 404 | Stock reservation does not exist |
| `RESERVATION_EXPIRED` | 409 | Reservation has already expired |
| `INVALID_MOVEMENT_TYPE` | 400 | Invalid movement type specified |
| `INVALID_TRANSFER` | 400 | Cannot transfer to same location |
| `ALERT_NOT_FOUND` | 404 | Low stock alert does not exist |
| `ALERT_ALREADY_ACKNOWLEDGED` | 409 | Alert already acknowledged |

## Webhooks

The Inventory API emits the following webhook events:

| Event | Description |
|-------|-------------|
| `inventory.adjusted` | Inventory manually adjusted |
| `inventory.received` | Stock received |
| `inventory.transferred` | Stock transferred between locations |
| `inventory.reserved` | Stock reserved for order |
| `inventory.reservation_released` | Reservation released |
| `inventory.reservation_expired` | Reservation expired |
| `inventory.low_stock` | Stock below threshold |
| `inventory.critical_stock` | Stock below critical level |
| `inventory.out_of_stock` | Stock reached zero |
| `inventory.movement.created` | Stock movement recorded |

## Configuration

### Inventory Settings

Configure inventory behavior in your application settings:

```toml
[inventory]
low_stock_threshold = 20              # Default low stock threshold (%)
enable_restock_alerts = true          # Enable automatic restock alerts
enable_reservations = true            # Enable stock reservations
reservation_timeout_minutes = 30      # Reservation timeout
```

### Reservation Behavior

- Reservations are created with `active` status
- Reservations automatically expire after the configured timeout
- Expired reservations can be cleaned up via the API or automatically
- When an order is completed, reservations are converted to actual stock reduction
- When an order is cancelled, reservations should be released
