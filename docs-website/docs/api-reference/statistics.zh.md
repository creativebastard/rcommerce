# 统计 API

统计 API 为您的 R Commerce 商店提供全面的分析和报告功能。使用这些端点构建仪表板、生成报告并获取业务绩效洞察。

## 概述

统计 API 提供：

- **仪表板指标** - 管理后台的关键绩效指标
- **销售分析** - 收入趋势、订单量和增长指标
- **订单洞察** - 状态分类和履行分析
- **产品绩效** - 热销商品和库存指标
- **客户分析** - 获取、留存和生命周期价值
- **订阅指标** - MRR（月度经常性收入）、流失率和经常性收入数据

## 认证

所有统计端点都需要管理员级别的认证。在 Authorization 请求头中包含您的 API 密钥：

```http
Authorization: Bearer YOUR_ADMIN_API_KEY
```

!!! note "需要管理员权限"
    统计端点需要 `admin` 范围或 `statistics:read` 范围。具有产品或订单范围的普通 API 密钥无法访问这些端点。

## 基础 URL

```
https://api.rcommerce.app/api/v1/admin/statistics
```

---

## 仪表板概览

获取管理后台仪表板的关键指标摘要。

```http
GET /api/v1/admin/statistics/dashboard
```

### 查询参数

| 参数 | 类型 | 默认值 | 描述 |
|-----------|------|---------|-------------|
| `date_from` | date | `today` | 开始日期（ISO 8601 或预设值） |
| `date_to` | date | `today` | 结束日期（ISO 8601 或预设值） |
| `compare_with` | string | `previous_period` | 与上一周期比较 |

### 日期预设

您可以使用方便的预设值代替具体日期：

| 预设值 | 描述 |
|--------|-------------|
| `today`, `yesterday` | 单日 |
| `this_week`, `last_week` | 周（周一至周日） |
| `this_month`, `last_month` | 日历月 |
| `last_7_days`, `last_30_days`, `last_90_days` | 滚动周期 |

### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/dashboard?date_from=last_30_days&compare_with=previous_period" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### 示例响应

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

### 响应字段

| 字段 | 描述 |
|-------|-------------|
| `sales.total_revenue` | 总收入及百分比变化 |
| `sales.total_orders` | 订单数量及百分比变化 |
| `sales.average_order_value` | 平均订单价值及百分比变化 |
| `orders.by_status` | 按状态分类的订单数量 |
| `orders.fulfillment_rate` | 已履行订单的百分比 |
| `customers.new_customers` | 周期内新客户数 |
| `customers.conversion_rate` | 访客到客户的转化率 |
| `products.top_selling` | 按收入排名前 5 的产品 |
| `subscriptions.mrr` | 月度经常性收入 |
| `subscriptions.churn_rate` | 月度流失率百分比 |

---

## 销售统计

检索包含时间序列数据的详细销售指标，用于图表和分析。

```http
GET /api/v1/admin/statistics/sales
```

### 查询参数

| 参数 | 类型 | 默认值 | 描述 |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | 开始日期 |
| `date_to` | date | `today` | 结束日期 |
| `period` | string | `day` | 聚合方式：`day`、`week`、`month` |
| `group_by` | string | - | 分组方式：`product`、`category`、`channel` |

### 示例请求

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

### 示例响应

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

## 订单统计

获取详细的订单指标，包括状态分类和履行分析。

```http
GET /api/v1/admin/statistics/orders
```

### 查询参数

| 参数 | 类型 | 默认值 | 描述 |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | 开始日期 |
| `date_to` | date | `today` | 结束日期 |
| `period` | string | `day` | 聚合周期 |
| `include_geo` | boolean | false | 包含地理数据 |

### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/orders?date_from=last_90_days&include_geo=true" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### 示例响应

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

## 产品统计

分析产品绩效，包括销售数据、库存指标和盈利能力分析。

```http
GET /api/v1/admin/statistics/products
```

### 查询参数

| 参数 | 类型 | 默认值 | 描述 |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | 开始日期 |
| `date_to` | date | `today` | 结束日期 |
| `limit` | integer | 20 | 返回的产品数量 |
| `sort_by` | string | `revenue` | 排序方式：`revenue`、`quantity`、`profit` |
| `category_id` | UUID | - | 按分类筛选 |

### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/products?limit=10&sort_by=revenue" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### 示例响应

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

## 客户统计

分析客户行为、获取来源、留存率和生命周期价值。

```http
GET /api/v1/admin/statistics/customers
```

### 查询参数

| 参数 | 类型 | 默认值 | 描述 |
|-----------|------|---------|-------------|
| `date_from` | date | `this_month` | 开始日期 |
| `date_to` | date | `today` | 结束日期 |
| `include_segments` | boolean | false | 包含客户分群 |

### 示例请求

```bash
curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/customers?date_from=last_90_days&include_segments=true" \
  -H "Authorization: Bearer sk_live_admin_xxx"
```

### 示例响应

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

### 客户分群

| 分群 | 描述 |
|---------|-------------|
| `vip` | 高价值客户（按生命周期价值排名前 5%） |
| `loyal` | 常规重复购买者 |
| `new` | 最近 30 天内首次购买 |
| `at_risk` | 60 天以上未购买 |
| `dormant` | 90 天以上未购买 |

---

## 常见用例

### 构建销售仪表板

获取 30 天视图的仪表板数据：

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

### 生成月度报告

获取整月按日聚合的销售数据：

```python
import requests
from datetime import datetime, timedelta

# 获取上月数据
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

### 跟踪产品绩效

每周监控热销产品：

```bash
#!/bin/bash

# 每周产品报告
API_KEY="sk_live_admin_xxx"
DATE_FROM=$(date -d '7 days ago' +%Y-%m-%d)

curl -X GET "https://api.rcommerce.app/api/v1/admin/statistics/products?date_from=${DATE_FROM}&limit=20&sort_by=revenue" \
  -H "Authorization: Bearer ${API_KEY}"
```

### 分析客户留存

跟踪客户获取和留存：

```javascript
// 获取客户统计用于群组分析
const getCustomerStats = async () => {
  const response = await fetch(
    'https://api.rcommerce.app/api/v1/admin/statistics/customers?date_from=last_90_days&include_segments=true',
    {
      headers: { 'Authorization': 'Bearer YOUR_API_KEY' }
    }
  );
  const data = await response.json();
  
  console.log('重复购买率:', data.data.retention.repeat_purchase_rate);
  console.log('平均生命周期价值:', data.data.retention.customer_lifetime_value.average);
  console.log('风险客户数:', data.data.segments.find(s => s.segment === 'at_risk')?.count);
};
```

---

## 速率限制

统计端点有特定的速率限制：

| 端点 | 每分钟请求数 |
|----------|---------------------|
| Dashboard | 60 |
| Sales | 30 |
| Orders | 30 |
| Products | 30 |
| Customers | 20 |

!!! tip "缓存建议"
    仪表板数据变化较慢。建议将响应缓存至少 5 分钟，以减少 API 调用并提高性能。

## 错误处理

### 无效的日期范围

```json
{
  "error": {
    "code": "INVALID_DATE_RANGE",
    "message": "The date_from must be before date_to"
  }
}
```

### 权限不足

```json
{
  "error": {
    "code": "FORBIDDEN",
    "message": "Admin access required for statistics endpoints"
  }
}
```

### 日期范围过大

```json
{
  "error": {
    "code": "DATE_RANGE_TOO_LARGE",
    "message": "Date range exceeds maximum of 365 days"
  }
}
```

## 错误代码

| 代码 | 描述 |
|------|-------------|
| `INVALID_DATE_RANGE` | date_from 在 date_to 之后 |
| `INVALID_PERIOD` | 不支持的周期值 |
| `DATE_RANGE_TOO_LARGE` | 日期范围超过 365 天 |
| `FORBIDDEN` | 权限不足 |
| `STATS_UNAVAILABLE` | 统计服务暂时不可用 |

---

## 下一步

- [订单 API](orders.md) - 以编程方式管理订单
- [产品 API](products.md) - 产品目录管理
- [客户 API](customers.md) - 客户数据访问
- [Webhooks](webhooks.md) - 实时事件通知
