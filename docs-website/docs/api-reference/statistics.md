# Statistics API (Admin)

The Statistics API provides comprehensive analytics and reporting for your R Commerce store. Use these endpoints to build dashboards, generate reports, and gain insights into your business performance. All endpoints including dashboard stats and API key management are fully implemented and operational.

## Overview

The Statistics API offers:

- **Dashboard Metrics** - Key performance indicators for admin dashboards
- **Sales Analytics** - Revenue trends, order volumes, and growth metrics
- **Order Insights** - Status breakdowns and fulfillment analytics
- **Product Performance** - Top sellers and inventory metrics
- **Customer Analytics** - Acquisition, retention, and lifetime value
- **Subscription Metrics** - MRR, churn, and recurring revenue data

## Authentication

All statistics endpoints require admin-level authentication. Include your API key in the Authorization header:

```http
Authorization: Bearer YOUR_ADMIN_API_KEY
```

!!! note "Admin Access Required"
    Statistics endpoints require either the `admin` scope or `statistics:read` scope. Regular API keys with product or order scopes cannot access these endpoints.

## Base URL

```
https://api.rcommerce.app/api/v1/admin/statistics
```

---

## Dashboard Overview

Get a high-level summary of key metrics for your admin dashboard.

```http
GET /api/v1/admin/statistics/dashboard
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `date_from` | date | `today` | Start date (ISO 8601 or preset) |
| `date_to` | date | `today` | End date (ISO 8601 or preset) |
| `compare_with` | string | `previous_period` | Compare with previous period |

### Date Presets

Instead of specific dates, you can use convenient presets:

| Preset | Description |
|--------|-------------|
| `today`, `yesterday` | Single day |
| `this_week`, `last_week` | Week (Mon-Sun) |
| `this_month`, `last_month` | Calendar month |
| `last_7_days`, `last_30_days`, `last_90_days` | Rolling periods |

### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/dashboard?date_from=last_30_days&compare_with=previous_period" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### Example Response

```json
{
  "data": {
    "period": {
      "from": "2024-01-01",
      "to": "2024-01-30",
      "days": 30
    },
    "sales": {
      "total_revenue": {
        "amount": "45678.90",
        "currency": "USD",
        "change_percentage": 12.5,
        "change_direction": "up"
      },
      "total_orders": {
        "count": 342,
        "change_percentage": 8.2,
        "change_direction": "up"
      },
      "average_order_value": {
        "amount": "133.56",
        "currency": "USD",
        "change_percentage": 3.9,
        "change_direction": "up"
      }
    },
    "orders": {
      "by_status": {
        "pending": 12,
        "processing": 28,
        "completed": 285,
        "cancelled": 17
      },
      "fulfillment_rate": 83.3
    },
    "customers": {
      "new_customers": 89,
      "returning_customers": 253,
      "conversion_rate": 3.2
    },
    "products": {
      "top_selling": [
        {
          "product_id": "550e8400-e29b-41d4-a716-446655440001",
          "title": "Premium Wireless Headphones",
          "quantity_sold": 156,
          "revenue": "23400.00"
        }
      ],
      "low_stock_count": 8
    },
    "subscriptions": {
      "active_subscribers": 1245,
      "mrr": "12450.00",
      "churn_rate": 2.1
    }
  }
}
```

### Response Fields

| Field | Description |
|-------|-------------|
| `sales.total_revenue` | Total revenue with percentage change |
| `sales.total_orders` | Order count with percentage change |
| `sales.average_order_value` | AOV with percentage change |
| `orders.by_status` | Order count by status |
| `orders.fulfillment_rate` | Percentage of orders fulfilled |
| `customers.new_customers` | New customers in period |
| `customers.conversion_rate` | Visitor to customer conversion % |
| `products.top_selling` | Top 5 products by revenue |
| `subscriptions.mrr` | Monthly Recurring Revenue |
| `subscriptions.churn_rate` | Monthly churn percentage |

---

## Sales Statistics

Retrieve detailed sales metrics with time-series data for charts and analysis.

```http
GET /api/v1/admin/statistics/sales
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | Start date |
| `date_to` | date | `today` | End date |
| `period` | string | `day` | Aggregation: `day`, `week`, `month` |
| `group_by` | string | - | Group by: `product`, `category`, `channel` |

### Example Requests

=== "cURL"

    ```bash
    curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/sales?date_from=2024-01-01&date_to=2024-01-31&period=day" \
      -H "Authorization: Bearer sk_live_admin_xxx"
    ```

=== "JavaScript"

    ```javascript
    const response = await fetch(
      'https://api.rcommerce.app/api/v1/admin/statistics/sales?date_from=2024-01-01&period=day',
      {
        headers: {
          'Authorization': 'Bearer sk_live_admin_xxx'
        }
      }
    );
    const data = await response.json();
    ```

=== "Python"

    ```python
    import requests

    response = requests.get(
        'https://api.rcommerce.app/api/v1/admin/statistics/sales',
        headers={'Authorization': 'Bearer sk_live_admin_xxx'},
        params={
            'date_from': '2024-01-01',
            'date_to': '2024-01-31',
            'period': 'day'
        }
    )
    data = response.json()
    ```

### Example Response

```json
{
  "data": {
    "summary": {
      "total_revenue": "156789.50",
      "total_orders": 1247,
      "average_order_value": "125.73",
      "total_discounts": "8765.00",
      "net_revenue": "135246.80"
    },
    "comparison": {
      "revenue_change": 15.3,
      "orders_change": 8.7,
      "aov_change": 6.1
    },
    "time_series": [
      {
        "date": "2024-01-01",
        "revenue": "5234.50",
        "orders": 42,
        "aov": "124.63"
      },
      {
        "date": "2024-01-02",
        "revenue": "4890.25",
        "orders": 38,
        "aov": "128.69"
      }
    ],
    "by_channel": [
      {
        "channel": "web",
        "revenue": "125000.00",
        "orders": 980,
        "percentage": 79.7
      },
      {
        "channel": "mobile_app",
        "revenue": "28789.50",
        "orders": 245,
        "percentage": 19.7
      }
    ],
    "by_payment_method": [
      {
        "method": "credit_card",
        "revenue": "98765.00",
        "orders": 780,
        "percentage": 62.5
      },
      {
        "method": "paypal",
        "revenue": "34567.50",
        "orders": 289,
        "percentage": 23.2
      }
    ]
  }
}
```

---

## Order Statistics

Get detailed order metrics including status breakdowns and fulfillment analytics.

```http
GET /api/v1/admin/statistics/orders
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | Start date |
| `date_to` | date | `today` | End date |
| `period` | string | `day` | Aggregation period |
| `include_geo` | boolean | false | Include geographic data |

### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/orders?date_from=last_90_days&include_geo=true" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### Example Response

```json
{
  "data": {
    "summary": {
      "total_orders": 3456,
      "total_items": 8765,
      "average_items_per_order": 2.54,
      "average_processing_time_hours": 18.5
    },
    "by_status": {
      "pending": 45,
      "confirmed": 123,
      "processing": 234,
      "completed": 2890,
      "cancelled": 128,
      "refunded": 24
    },
    "by_fulfillment_status": {
      "unfulfilled": 402,
      "partial": 156,
      "fulfilled": 2890,
      "returned": 8
    },
    "fulfillment_metrics": {
      "average_fulfillment_time_hours": 24.5,
      "same_day_fulfillment_rate": 35.2,
      "within_48h_rate": 89.3
    },
    "geographic_distribution": [
      {
        "country": "US",
        "orders": 2156,
        "revenue": "98765.00",
        "percentage": 62.4
      },
      {
        "country": "CA",
        "orders": 456,
        "revenue": "23456.00",
        "percentage": 13.2
      }
    ]
  }
}
```

---

## Product Statistics

Analyze product performance with sales data, inventory metrics, and profitability analysis.

```http
GET /api/v1/admin/statistics/products
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | Start date |
| `date_to` | date | `today` | End date |
| `limit` | integer | 20 | Number of products to return |
| `sort_by` | string | `revenue` | Sort: `revenue`, `quantity`, `profit` |
| `category_id` | UUID | - | Filter by category |

### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/products?limit=10&sort_by=revenue" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### Example Response

```json
{
  "data": {
    "summary": {
      "total_products_sold": 8765,
      "unique_products_sold": 234,
      "total_revenue": "156789.50",
      "gross_profit": "77844.50",
      "gross_margin": 49.6
    },
    "top_products": [
      {
        "product_id": "550e8400-e29b-41d4-a716-446655440001",
        "title": "Premium Wireless Headphones",
        "sku": "WH-001",
        "quantity_sold": 456,
        "revenue": "68400.00",
        "profit": "34200.00",
        "margin": 50.0,
        "return_rate": 2.1
      },
      {
        "product_id": "550e8400-e29b-41d4-a716-446655440002",
        "title": "Organic Cotton T-Shirt",
        "sku": "TSH-001",
        "quantity_sold": 389,
        "revenue": "11670.00",
        "profit": "7000.00",
        "margin": 60.0,
        "return_rate": 5.4
      }
    ],
    "by_category": [
      {
        "category_id": "550e8400-e29b-41d4-a716-446655440100",
        "name": "Electronics",
        "revenue": "85000.00",
        "percentage": 54.2
      },
      {
        "category_id": "550e8400-e29b-41d4-a716-446655440101",
        "name": "Clothing",
        "revenue": "45000.00",
        "percentage": 28.7
      }
    ],
    "inventory_metrics": {
      "total_sku_count": 456,
      "low_stock_count": 23,
      "out_of_stock_count": 8,
      "inventory_value": "234567.00"
    }
  }
}
```

---

## Customer Statistics

Analyze customer behavior, acquisition sources, retention rates, and lifetime value.

```http
GET /api/v1/admin/statistics/customers
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | Start date |
| `date_to` | date | `today` | End date |
| `include_segments` | boolean | false | Include customer segments |

### Example Request

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/customers?date_from=last_90_days&include_segments=true" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### Example Response

```json
{
  "data": {
    "summary": {
      "total_customers": 15420,
      "new_customers": 456,
      "returning_customers": 789,
      "churned_customers": 23
    },
    "acquisition": {
      "total_new": 456,
      "by_source": [
        {
          "source": "organic_search",
          "count": 156,
          "percentage": 34.2
        },
        {
          "source": "paid_ads",
          "count": 134,
          "percentage": 29.4
        },
        {
          "source": "social_media",
          "count": 89,
          "percentage": 19.5
        }
      ],
      "conversion_rate": 3.2
    },
    "retention": {
      "repeat_purchase_rate": 42.5,
      "average_time_between_orders_days": 45,
      "customer_lifetime_value": {
        "average": "456.78",
        "median": "234.50"
      }
    },
    "segments": [
      {
        "segment": "vip",
        "count": 234,
        "average_order_value": "345.67",
        "percentage": 1.5
      },
      {
        "segment": "loyal",
        "count": 1234,
        "average_order_value": "156.78",
        "percentage": 8.0
      },
      {
        "segment": "at_risk",
        "count": 456,
        "average_order_value": "89.50",
        "percentage": 3.0
      }
    ]
  }
}
```

### Customer Segments

| Segment | Description |
|---------|-------------|
| `vip` | High-value customers (top 5% by LTV) |
| `loyal` | Regular repeat purchasers |
| `new` | First purchase within last 30 days |
| `at_risk` | No purchase in 60+ days |
| `dormant` | No purchase in 90+ days |

---

## Common Use Cases

### Building a Sales Dashboard

Fetch dashboard data with a 30-day view:

```javascript
async function getDashboardData() {
  const response = await fetch(
    'https://api.rcommerce.app/api/v1/admin/statistics/dashboard?date_from=last_30_days',
    {
      headers: { 'Authorization': 'Bearer YOUR_API_KEY' }
    }
  );
  return await response.json();
}
```

### Generating Monthly Reports

Get sales data aggregated by day for a full month:

```python
import requests
from datetime import datetime, timedelta

# Get last month's data
end_date = datetime.now().replace(day=1) - timedelta(days=1)
start_date = end_date.replace(day=1)

response = requests.get(
    'https://api.rcommerce.app/api/v1/admin/statistics/sales',
    headers={'Authorization': 'Bearer YOUR_API_KEY'},
    params={
        'date_from': start_date.strftime('%Y-%m-%d'),
        'date_to': end_date.strftime('%Y-%m-%d'),
        'period': 'day'
    }
)

report_data = response.json()
```

### Tracking Product Performance

Monitor top-selling products weekly:

```bash
#!/bin/bash

# Weekly product report
API_KEY="sk_live_admin_xxx"
DATE_FROM=$(date -d '7 days ago' +%Y-%m-%d)

curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/products?date_from=${DATE_FROM}&limit=20&sort_by=revenue" \
  -H "Authorization: Bearer ${API_KEY}"
```

### Analyzing Customer Retention

Track customer acquisition and retention:

```javascript
// Get customer stats for cohort analysis
const getCustomerStats = async () => {
  const response = await fetch(
    'https://api.rcommerce.app/api/v1/admin/statistics/customers?date_from=last_90_days&include_segments=true',
    {
      headers: { 'Authorization': 'Bearer YOUR_API_KEY' }
    }
  );
  const data = await response.json();
  
  console.log('Repeat Purchase Rate:', data.data.retention.repeat_purchase_rate);
  console.log('Average LTV:', data.data.retention.customer_lifetime_value.average);
  console.log('At-Risk Customers:', data.data.segments.find(s => s.segment === 'at_risk')?.count);
};
```

---

## Rate Limits

Statistics endpoints have specific rate limits:

| Endpoint | Requests per Minute |
|----------|---------------------|
| Dashboard | 60 |
| Sales | 30 |
| Orders | 30 |
| Products | 30 |
| Customers | 20 |

!!! tip "Caching Recommendation"
    Dashboard data changes slowly. Cache responses for at least 5 minutes to reduce API calls and improve performance.

## Error Handling

### Invalid Date Range

```json
{
  "error": {
    "code": "INVALID_DATE_RANGE",
    "message": "The date_from must be before date_to"
  }
}
```

### Insufficient Permissions

```json
{
  "error": {
    "code": "FORBIDDEN",
    "message": "Admin access required for statistics endpoints"
  }
}
```

### Date Range Too Large

```json
{
  "error": {
    "code": "DATE_RANGE_TOO_LARGE",
    "message": "Date range exceeds maximum of 365 days"
  }
}
```

## Error Codes

| Code | Description |
|------|-------------|
| `INVALID_DATE_RANGE` | date_from is after date_to |
| `INVALID_PERIOD` | Unsupported period value |
| `DATE_RANGE_TOO_LARGE` | Date range exceeds 365 days |
| `FORBIDDEN` | Insufficient permissions |
| `STATS_UNAVAILABLE` | Statistics service temporarily unavailable |

---

## Next Steps

- [Orders API](orders.md) - Manage orders programmatically
- [Products API](products.md) - Product catalog management
- [Customers API](customers.md) - Customer data access
- [Webhooks](webhooks.md) - Real-time event notifications
