# Shipping API

The Shipping API provides comprehensive shipping management including rate calculation, shipment creation, tracking, and carrier management.

## Base URL

```
/api/v1/shipping
```

## Authentication

All shipping endpoints require authentication via API key or JWT token.

```http
Authorization: Bearer YOUR_API_KEY
```

## Shipping Rate Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440300",
  "carrier": "UPS",
  "service_code": "ground",
  "service_name": "UPS Ground",
  "description": "1-5 business days",
  "estimated_days": 3,
  "price": "12.50",
  "currency": "USD",
  "weight_limit": "68.0",
  "weight_unit": "kg",
  "dimensions_limit": {
    "length": 274,
    "width": 274,
    "height": 274,
    "unit": "cm"
  },
  "is_negotiated_rate": false,
  "delivery_date_guaranteed": false,
  "delivery_date": "2024-01-18T17:00:00Z"
}
```

### Shipping Rate Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier for the rate |
| `carrier` | string | Carrier name (UPS, FedEx, USPS, DHL) |
| `service_code` | string | Carrier-specific service code |
| `service_name` | string | Human-readable service name |
| `description` | string | Service description |
| `estimated_days` | integer | Estimated transit days |
| `price` | decimal | Shipping cost |
| `currency` | string | ISO 4217 currency code |
| `weight_limit` | decimal | Maximum weight allowed |
| `weight_unit` | string | Weight unit (kg, lb) |
| `dimensions_limit` | object | Maximum dimensions allowed |
| `is_negotiated_rate` | boolean | Whether this is a negotiated rate |
| `delivery_date_guaranteed` | boolean | If delivery date is guaranteed |
| `delivery_date` | datetime | Estimated delivery date/time |

## Shipment Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440400",
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "carrier": "UPS",
  "service_code": "ground",
  "service_name": "UPS Ground",
  "tracking_number": "1Z999AA10123456784",
  "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
  "label_url": "https://api.rcommerce.app/labels/550e8400-e29b-41d4-a716-446655440400.pdf",
  "status": "in_transit",
  "price": "12.50",
  "currency": "USD",
  "weight": "2.5",
  "weight_unit": "kg",
  "dimensions": {
    "length": 30,
    "width": 20,
    "height": 15,
    "unit": "cm"
  },
  "ship_from": {
    "name": "R Commerce Warehouse",
    "company": "R Commerce Inc",
    "phone": "+1-555-0199",
    "address1": "100 Commerce Street",
    "address2": "Suite 200",
    "city": "Austin",
    "state": "TX",
    "country": "US",
    "zip": "78701"
  },
  "ship_to": {
    "name": "John Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001"
  },
  "packages": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440401",
      "tracking_number": "1Z999AA10123456784",
      "weight": "2.5",
      "weight_unit": "kg",
      "dimensions": {
        "length": 30,
        "width": 20,
        "height": 15,
        "unit": "cm"
      },
      "items": [
        {
          "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
          "quantity": 2
        }
      ]
    }
  ],
  "customs_info": {
    "contents_type": "merchandise",
    "customs_certify": true,
    "customs_signer": "John Smith",
    "eel_pfc": "NOEEI 30.37(a)",
    "non_delivery_option": "return",
    "restriction_type": "none",
    "customs_items": [
      {
        "description": "Premium Cotton T-Shirt",
        "quantity": 2,
        "value": "49.98",
        "currency": "USD",
        "hs_tariff_number": "6109.10.00",
        "origin_country": "US"
      }
    ]
  },
  "insurance": {
    "amount": "100.00",
    "currency": "USD",
    "provider": "carrier"
  },
  "options": {
    "signature_required": true,
    "adult_signature_required": false,
    "saturday_delivery": false,
    "hold_for_pickup": false,
    "dry_ice": false,
    "dangerous_goods": false
  },
  "created_at": "2024-01-15T10:30:00Z",
  "shipped_at": "2024-01-15T14:00:00Z",
  "delivered_at": null,
  "estimated_delivery": "2024-01-18T17:00:00Z"
}
```

### Shipment Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique shipment identifier |
| `order_id` | UUID | Associated order ID |
| `carrier` | string | Shipping carrier name |
| `service_code` | string | Carrier service code |
| `service_name` | string | Human-readable service name |
| `tracking_number` | string | Carrier tracking number |
| `tracking_url` | string | URL to track shipment |
| `label_url` | string | URL to shipping label |
| `status` | string | `pending`, `label_created`, `in_transit`, `out_for_delivery`, `delivered`, `exception`, `cancelled` |
| `price` | decimal | Shipping cost charged |
| `currency` | string | ISO 4217 currency code |
| `weight` | decimal | Total shipment weight |
| `weight_unit` | string | Weight unit |
| `dimensions` | object | Package dimensions |
| `ship_from` | object | Origin address |
| `ship_to` | object | Destination address |
| `packages` | array | Individual packages in shipment |
| `customs_info` | object | Customs information for international shipments |
| `insurance` | object | Insurance details |
| `options` | object | Shipping options |
| `created_at` | datetime | Shipment creation time |
| `shipped_at` | datetime | When carrier received package |
| `delivered_at` | datetime | Delivery timestamp |
| `estimated_delivery` | datetime | Estimated delivery date |

## Endpoints

### Get Shipping Rates

```http
GET /api/v1/shipping/rates
```

Calculate shipping rates for a shipment based on origin, destination, and package details.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `from_country` | string | Yes | Origin country code (ISO 3166-1 alpha-2) |
| `from_zip` | string | Yes | Origin postal code |
| `to_country` | string | Yes | Destination country code |
| `to_zip` | string | Yes | Destination postal code |
| `weight` | decimal | Yes | Package weight |
| `weight_unit` | string | No | Weight unit: `kg`, `lb`, `oz`, `g` (default: kg) |
| `length` | decimal | No | Package length |
| `width` | decimal | No | Package width |
| `height` | decimal | No | Package height |
| `dimension_unit` | string | No | Dimension unit: `cm`, `in` (default: cm) |
| `carriers` | string | No | Comma-separated carrier list (ups,fedex,usps,dhl) |
| `service_code` | string | No | Filter by specific service code |
| `currency` | string | No | Return rates in this currency (default: USD) |

#### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/rates?from_country=US&from_zip=78701&to_country=US&to_zip=10001&weight=2.5&weight_unit=kg&length=30&width=20&height=15&carriers=ups,fedex" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440300",
      "carrier": "UPS",
      "service_code": "ground",
      "service_name": "UPS Ground",
      "description": "1-5 business days",
      "estimated_days": 3,
      "price": "12.50",
      "currency": "USD",
      "delivery_date": "2024-01-18T17:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440301",
      "carrier": "UPS",
      "service_code": "3day_select",
      "service_name": "UPS 3 Day Select",
      "description": "3 business days",
      "estimated_days": 3,
      "price": "28.75",
      "currency": "USD",
      "delivery_date": "2024-01-18T17:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440302",
      "carrier": "FedEx",
      "service_code": "FEDEX_GROUND",
      "service_name": "FedEx Ground",
      "description": "1-5 business days",
      "estimated_days": 3,
      "price": "11.95",
      "currency": "USD",
      "delivery_date": "2024-01-18T20:00:00Z"
    }
  ],
  "meta": {
    "request_id": "req_ship_001",
    "from_address": {
      "country": "US",
      "zip": "78701"
    },
    "to_address": {
      "country": "US",
      "zip": "10001"
    },
    "weight": "2.5",
    "weight_unit": "kg"
  }
}
```

#### JavaScript Example

```javascript
const response = await fetch(
  'https://api.rcommerce.app/api/v1/shipping/rates?' +
  new URLSearchParams({
    from_country: 'US',
    from_zip: '78701',
    to_country: 'US',
    to_zip: '10001',
    weight: '2.5',
    weight_unit: 'kg',
    carriers: 'ups,fedex'
  }),
  {
    headers: {
      'Authorization': 'Bearer sk_live_xxx'
    }
  }
);

const rates = await response.json();
console.log(rates.data);
```

### Create Shipment

```http
POST /api/v1/shipping/shipments
```

Create a new shipment and generate a shipping label.

#### Request Body

```json
{
  "order_id": "550e8400-e29b-41d4-a716-446655440100",
  "carrier": "UPS",
  "service_code": "ground",
  "ship_from": {
    "name": "R Commerce Warehouse",
    "company": "R Commerce Inc",
    "phone": "+1-555-0199",
    "email": "shipping@rcommerce.app",
    "address1": "100 Commerce Street",
    "address2": "Suite 200",
    "city": "Austin",
    "state": "TX",
    "country": "US",
    "zip": "78701",
    "residential": false
  },
  "ship_to": {
    "name": "John Doe",
    "company": "Acme Inc",
    "phone": "+1-555-0123",
    "email": "john@example.com",
    "address1": "123 Main St",
    "address2": "Apt 4B",
    "city": "New York",
    "state": "NY",
    "country": "US",
    "zip": "10001",
    "residential": true
  },
  "packages": [
    {
      "weight": "2.5",
      "weight_unit": "kg",
      "length": 30,
      "width": 20,
      "height": 15,
      "dimension_unit": "cm",
      "items": [
        {
          "line_item_id": "550e8400-e29b-41d4-a716-446655440101",
          "quantity": 2
        }
      ]
    }
  ],
  "customs_info": {
    "contents_type": "merchandise",
    "customs_certify": true,
    "customs_signer": "John Smith",
    "non_delivery_option": "return",
    "customs_items": [
      {
        "description": "Premium Cotton T-Shirt",
        "quantity": 2,
        "value": "49.98",
        "currency": "USD",
        "hs_tariff_number": "6109.10.00",
        "origin_country": "US"
      }
    ]
  },
  "insurance": {
    "amount": "100.00",
    "currency": "USD"
  },
  "options": {
    "signature_required": true,
    "saturday_delivery": false
  },
  "reference": "Order #1001",
  "notify_customer": true
}
```

#### Required Fields

- `carrier` - Shipping carrier name
- `service_code` - Carrier service code
- `ship_from` - Origin address (name, address1, city, country, zip required)
- `ship_to` - Destination address (name, address1, city, country, zip required)
- `packages` - At least one package with weight

#### Example Response

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440400",
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "carrier": "UPS",
    "service_code": "ground",
    "service_name": "UPS Ground",
    "tracking_number": "1Z999AA10123456784",
    "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
    "label_url": "https://api.rcommerce.app/labels/550e8400-e29b-41d4-a716-446655440400.pdf",
    "status": "label_created",
    "price": "12.50",
    "currency": "USD",
    "weight": "2.5",
    "weight_unit": "kg",
    "ship_from": {
      "name": "R Commerce Warehouse",
      "city": "Austin",
      "state": "TX",
      "country": "US",
      "zip": "78701"
    },
    "ship_to": {
      "name": "John Doe",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "packages": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440401",
        "tracking_number": "1Z999AA10123456784",
        "weight": "2.5",
        "weight_unit": "kg"
      }
    ],
    "created_at": "2024-01-15T10:30:00Z",
    "estimated_delivery": "2024-01-18T17:00:00Z"
  }
}
```

#### JavaScript Example

```javascript
const response = await fetch('https://api.rcommerce.app/api/v1/shipping/shipments', {
  method: 'POST',
  headers: {
    'Authorization': 'Bearer sk_live_xxx',
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    order_id: '550e8400-e29b-41d4-a716-446655440100',
    carrier: 'UPS',
    service_code: 'ground',
    ship_from: {
      name: 'R Commerce Warehouse',
      address1: '100 Commerce Street',
      city: 'Austin',
      state: 'TX',
      country: 'US',
      zip: '78701'
    },
    ship_to: {
      name: 'John Doe',
      address1: '123 Main St',
      city: 'New York',
      state: 'NY',
      country: 'US',
      zip: '10001'
    },
    packages: [{
      weight: '2.5',
      weight_unit: 'kg'
    }],
    notify_customer: true
  })
});

const shipment = await response.json();
console.log(shipment.data.tracking_number);
```

### Get Shipment

```http
GET /api/v1/shipping/shipments/{id}
```

Retrieve a shipment by ID.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Shipment ID |

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `include` | string | Related data: `tracking_history`, `order` |

#### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/shipments/550e8400-e29b-41d4-a716-446655440400?include=tracking_history" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### Example Response

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440400",
    "order_id": "550e8400-e29b-41d4-a716-446655440100",
    "carrier": "UPS",
    "service_code": "ground",
    "service_name": "UPS Ground",
    "tracking_number": "1Z999AA10123456784",
    "tracking_url": "https://www.ups.com/track?tracknum=1Z999AA10123456784",
    "label_url": "https://api.rcommerce.app/labels/550e8400-e29b-41d4-a716-446655440400.pdf",
    "status": "in_transit",
    "price": "12.50",
    "currency": "USD",
    "weight": "2.5",
    "ship_from": {
      "name": "R Commerce Warehouse",
      "city": "Austin",
      "state": "TX",
      "country": "US",
      "zip": "78701"
    },
    "ship_to": {
      "name": "John Doe",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001"
    },
    "tracking_history": [
      {
        "status": "picked_up",
        "description": "Package picked up by carrier",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T14:00:00Z"
      },
      {
        "status": "in_transit",
        "description": "Departed facility in Austin, TX",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T18:30:00Z"
      },
      {
        "status": "in_transit",
        "description": "Arrived at facility in Memphis, TN",
        "location": "Memphis, TN",
        "timestamp": "2024-01-16T02:15:00Z"
      }
    ],
    "created_at": "2024-01-15T10:30:00Z",
    "shipped_at": "2024-01-15T14:00:00Z",
    "estimated_delivery": "2024-01-18T17:00:00Z"
  }
}
```

### Cancel Shipment

```http
POST /api/v1/shipping/shipments/{id}/cancel
```

Cancel a shipment. Only shipments with status `pending` or `label_created` can be cancelled.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Shipment ID |

#### Request Body

```json
{
  "reason": "Customer request",
  "void_label": true
}
```

#### Example Request

```bash
curl -X POST "https://api.rcommerce.app/api/v1/shipping/shipments/550e8400-e29b-41d4-a716-446655440400/cancel" \
  -H "Authorization: Bearer sk_live_xxx" \
  -H "Content-Type: application/json" \
  -d '{
    "reason": "Customer request",
    "void_label": true
  }'
```

#### Example Response

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440400",
    "status": "cancelled",
    "cancelled_at": "2024-01-15T11:00:00Z",
    "cancellation_reason": "Customer request",
    "refund_amount": "12.50",
    "refund_currency": "USD"
  }
}
```

### Track Shipment

```http
GET /api/v1/shipping/tracking/{number}
```

Track a shipment using its tracking number. This endpoint can be used without authentication for customer-facing tracking pages.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `number` | string | Tracking number |

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `carrier` | string | Carrier code (optional, auto-detected if not provided) |

#### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/tracking/1Z999AA10123456784"
```

#### Example Response

```json
{
  "data": {
    "tracking_number": "1Z999AA10123456784",
    "carrier": "UPS",
    "status": "in_transit",
    "status_description": "In Transit",
    "estimated_delivery": "2024-01-18T17:00:00Z",
    "delivered_at": null,
    "signed_by": null,
    "tracking_history": [
      {
        "status": "picked_up",
        "status_code": "P",
        "description": "Package picked up by carrier",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T14:00:00Z"
      },
      {
        "status": "in_transit",
        "status_code": "I",
        "description": "Departed facility in Austin, TX",
        "location": "Austin, TX",
        "timestamp": "2024-01-15T18:30:00Z"
      },
      {
        "status": "in_transit",
        "status_code": "I",
        "description": "Arrived at facility in Memphis, TN",
        "location": "Memphis, TN",
        "timestamp": "2024-01-16T02:15:00Z"
      }
    ]
  }
}
```

#### JavaScript Example

```javascript
// Public tracking - no authentication required
const response = await fetch(
  'https://api.rcommerce.app/api/v1/shipping/tracking/1Z999AA10123456784'
);

const tracking = await response.json();
console.log(tracking.data.status);
console.log(tracking.data.estimated_delivery);
```

## Shipping Zones

### List Shipping Zones

```http
GET /api/v1/shipping/zones
```

Retrieve all configured shipping zones.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20) |
| `active_only` | boolean | Only return active zones |

#### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/zones?active_only=true" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440500",
      "name": "Domestic",
      "countries": ["US"],
      "provinces": [],
      "postal_codes": [],
      "weight_based_rates": [
        {
          "name": "Standard Shipping",
          "min_weight": 0,
          "max_weight": 1,
          "price": "5.00"
        },
        {
          "name": "Standard Shipping",
          "min_weight": 1,
          "max_weight": 5,
          "price": "10.00"
        }
      ],
      "price_based_rates": [],
      "carrier_based_rates": [
        {
          "carrier": "UPS",
          "service_code": "ground",
          "markup_percent": 0,
          "markup_amount": "0.00"
        }
      ],
      "is_active": true,
      "display_order": 1,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440501",
      "name": "International",
      "countries": ["CA", "GB", "AU", "DE", "FR"],
      "provinces": [],
      "postal_codes": [],
      "weight_based_rates": [
        {
          "name": "International Shipping",
          "min_weight": 0,
          "max_weight": 2,
          "price": "25.00"
        }
      ],
      "price_based_rates": [],
      "carrier_based_rates": [],
      "is_active": true,
      "display_order": 2,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "meta": {
    "pagination": {
      "total": 2,
      "per_page": 20,
      "current_page": 1,
      "total_pages": 1
    }
  }
}
```

### Get Shipping Zone

```http
GET /api/v1/shipping/zones/{id}
```

Retrieve a specific shipping zone by ID.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | UUID | Shipping zone ID |

## Carriers

### List Carriers

```http
GET /api/v1/shipping/carriers
```

Retrieve all available shipping carriers and their configured services.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `active_only` | boolean | Only return active carriers |
| `country` | string | Filter by supported country |

#### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/shipping/carriers?active_only=true" \
  -H "Authorization: Bearer sk_live_xxx"
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440600",
      "name": "UPS",
      "code": "ups",
      "is_active": true,
      "supports_tracking": true,
      "supports_label_generation": true,
      "supports_rate_calculation": true,
      "supported_countries": ["US", "CA", "MX", "GB", "DE", "FR", "AU", "JP"],
      "services": [
        {
          "code": "ground",
          "name": "UPS Ground",
          "description": "1-5 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "3day_select",
          "name": "UPS 3 Day Select",
          "description": "3 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "2nd_day_air",
          "name": "UPS 2nd Day Air",
          "description": "2 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "next_day_air",
          "name": "UPS Next Day Air",
          "description": "Next business day",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "worldwide_expedited",
          "name": "UPS Worldwide Expedited",
          "description": "2-5 business days",
          "is_active": true,
          "domestic": false,
          "international": true
        }
      ],
      "credentials_configured": true,
      "test_mode": false
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440601",
      "name": "FedEx",
      "code": "fedex",
      "is_active": true,
      "supports_tracking": true,
      "supports_label_generation": true,
      "supports_rate_calculation": true,
      "supported_countries": ["US", "CA", "MX", "GB", "DE", "FR", "AU", "JP"],
      "services": [
        {
          "code": "FEDEX_GROUND",
          "name": "FedEx Ground",
          "description": "1-5 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "FEDEX_2_DAY",
          "name": "FedEx 2Day",
          "description": "2 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "FEDEX_EXPRESS_SAVER",
          "name": "FedEx Express Saver",
          "description": "3 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "FEDEX_PRIORITY_OVERNIGHT",
          "name": "FedEx Priority Overnight",
          "description": "Next business day morning",
          "is_active": true,
          "domestic": true,
          "international": false
        }
      ],
      "credentials_configured": true,
      "test_mode": false
    },
    {
      "id": "550e8400-e29b-41d4-a716-446655440602",
      "name": "USPS",
      "code": "usps",
      "is_active": true,
      "supports_tracking": true,
      "supports_label_generation": true,
      "supports_rate_calculation": true,
      "supported_countries": ["US"],
      "services": [
        {
          "code": "first_class_mail",
          "name": "First-Class Mail",
          "description": "1-3 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "priority_mail",
          "name": "Priority Mail",
          "description": "1-3 business days",
          "is_active": true,
          "domestic": true,
          "international": false
        },
        {
          "code": "priority_mail_express",
          "name": "Priority Mail Express",
          "description": "Overnight to most locations",
          "is_active": true,
          "domestic": true,
          "international": false
        }
      ],
      "credentials_configured": true,
      "test_mode": false
    }
  ]
}
```

### Get Carrier

```http
GET /api/v1/shipping/carriers/{code}
```

Retrieve details for a specific carrier.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `code` | string | Carrier code (ups, fedex, usps, dhl) |

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `SHIPMENT_NOT_FOUND` | 404 | Shipment does not exist |
| `SHIPMENT_ALREADY_CANCELLED` | 409 | Shipment has already been cancelled |
| `SHIPMENT_CANNOT_CANCEL` | 400 | Shipment cannot be cancelled (already shipped) |
| `INVALID_CARRIER` | 400 | Carrier code is not valid |
| `INVALID_SERVICE_CODE` | 400 | Service code is not valid for carrier |
| `INVALID_ADDRESS` | 400 | Shipping address is invalid or incomplete |
| `ADDRESS_NOT_FOUND` | 404 | Address could not be verified |
| `RATE_NOT_AVAILABLE` | 400 | Shipping rate not available for route |
| `WEIGHT_EXCEEDED` | 400 | Package weight exceeds carrier limit |
| `DIMENSIONS_EXCEEDED` | 400 | Package dimensions exceed carrier limit |
| `CARRIER_ERROR` | 502 | Error communicating with carrier API |
| `LABEL_GENERATION_FAILED` | 502 | Failed to generate shipping label |
| `TRACKING_NOT_AVAILABLE` | 404 | Tracking number not found |
| `INVALID_TRACKING_NUMBER` | 400 | Tracking number format is invalid |
| `CUSTOMS_INFO_REQUIRED` | 400 | Customs information required for international shipment |
| `INSURANCE_EXCEEDS_LIMIT` | 400 | Insurance amount exceeds carrier limit |
| `ZONE_NOT_FOUND` | 404 | Shipping zone not found |
| `CARRIER_NOT_CONFIGURED` | 400 | Carrier credentials not configured |

## Webhooks

| Event | Description |
|-------|-------------|
| `shipment.created` | New shipment created |
| `shipment.updated` | Shipment information changed |
| `shipment.cancelled` | Shipment cancelled |
| `shipment.shipped` | Package picked up by carrier |
| `shipment.in_transit` | Package in transit |
| `shipment.out_for_delivery` | Package out for delivery |
| `shipment.delivered` | Package delivered |
| `shipment.exception` | Delivery exception occurred |
| `tracking.updated` | Tracking information updated |
| `rate.calculated` | Shipping rates calculated |
| `label.generated` | Shipping label generated |
| `label.voided` | Shipping label voided |

## Shipment Statuses

| Status | Description |
|--------|-------------|
| `pending` | Shipment created, label not yet generated |
| `label_created` | Shipping label generated, awaiting pickup |
| `in_transit` | Package picked up and in transit |
| `out_for_delivery` | Package out for delivery |
| `delivered` | Package successfully delivered |
| `exception` | Delivery exception (address issue, customs hold, etc.) |
| `cancelled` | Shipment cancelled |
| `returned` | Package returned to sender |

## Address Validation

The API automatically validates addresses when creating shipments. You can also validate addresses separately:

```http
POST /api/v1/shipping/validate-address
```

### Request Body

```json
{
  "name": "John Doe",
  "address1": "123 Main St",
  "address2": "Apt 4B",
  "city": "New York",
  "state": "NY",
  "country": "US",
  "zip": "10001"
}
```

### Response

```json
{
  "data": {
    "valid": true,
    "normalized": {
      "name": "John Doe",
      "address1": "123 Main Street",
      "address2": "Apt 4B",
      "city": "New York",
      "state": "NY",
      "country": "US",
      "zip": "10001-1234",
      "residential": true
    },
    "suggestions": []
  }
}
```
