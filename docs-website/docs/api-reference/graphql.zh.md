# GraphQL API

!!! warning "即将推出"
    GraphQL API 计划在未来版本（v0.2）中发布。以下文档描述了计划的实现，可能会有变更。

GraphQL API 提供灵活、类型安全的查询语言，用于访问您的 R Commerce 数据。

## 概览

GraphQL 允许客户端请求所需的确切数据，减少过度获取并实现强大的数据获取模式。

## 端点

```
POST /graphql
```

## 认证

在 Authorization 头中包含您的 API 密钥：

```http
POST /graphql HTTP/1.1
Host: api.rcommerce.app
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json
```

## 查询

### 产品查询

检索具有灵活字段选择的产品。

#### 基本查询

```graphql
query GetProducts {
  products(first: 10) {
    edges {
      node {
        id
        title
        slug
        price
        currency
        inventoryQuantity
      }
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}
```

#### 带变体和图片

```graphql
query GetProductDetails($id: ID!) {
  product(id: $id) {
    id
    title
    description
    price
    compareAtPrice
    currency
    status
    vendor
    tags
    inventoryQuantity
    variants(first: 10) {
      edges {
        node {
          id
          title
          sku
          price
          inventoryQuantity
          options {
            name
            value
          }
        }
      }
    }
    images(first: 5) {
      edges {
        node {
          id
          url
          altText
          position
        }
      }
    }
    collections(first: 5) {
      edges {
        node {
          id
          title
          slug
        }
      }
    }
  }
}
```

#### 变量

```json
{
  "id": "gid://rcommerce/Product/550e8400-e29b-41d4-a716-446655440000"
}
```

### 订单查询

```graphql
query GetOrders($first: Int!, $after: String) {
  orders(first: $first, after: $after, sortKey: CREATED_AT, reverse: true) {
    edges {
      node {
        id
        orderNumber
        email
        totalPrice
        currency
        financialStatus
        fulfillmentStatus
        createdAt
        customer {
          id
          email
          firstName
          lastName
        }
        lineItems(first: 5) {
          edges {
            node {
              title
              quantity
              price
              total
            }
          }
        }
        shippingAddress {
          firstName
          lastName
          address1
          city
          country
        }
      }
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}
```

### 客户查询

```graphql
query GetCustomers($query: String) {
  customers(first: 20, query: $query) {
    edges {
      node {
        id
        email
        firstName
        lastName
        phone
        acceptsMarketing
        ordersCount
        totalSpent
        defaultAddress {
          address1
          city
          country
        }
        addresses {
          edges {
            node {
              id
              address1
              city
              isDefaultShipping
            }
          }
        }
      }
    }
  }
}
```

### 购物车查询

```graphql
query GetCart($token: String!) {
  cart(token: $token) {
    id
    token
    currency
    subtotalPrice
    totalTax
    totalShipping
    totalPrice
    lineItems {
      edges {
        node {
          id
          quantity
          title
          variant {
            id
            title
            price
            product {
              id
              title
              images(first: 1) {
                edges {
                  node {
                    url
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
```

## 变更

### 创建购物车

```graphql
mutation CreateCart {
  cartCreate {
    cart {
      id
      token
      currency
      createdAt
    }
  }
}
```

### 添加到购物车

```graphql
mutation AddToCart($token: String!, $lines: [CartLineInput!]!) {
  cartLinesAdd(cartToken: $token, lines: $lines) {
    cart {
      id
      totalPrice
      lineItems {
        edges {
          node {
            id
            quantity
            title
          }
        }
      }
    }
    userErrors {
      field
      message
    }
  }
}
```

#### 变量

```json
{
  "token": "cart_abc123",
  "lines": [
    {
      "merchandiseId": "gid://rcommerce/ProductVariant/550e8400-e29b-41d4-a716-446655440001",
      "quantity": 2
    }
  ]
}
```

### 更新购物车行

```graphql
mutation UpdateCartLine($token: String!, $lineId: ID!, $quantity: Int!) {
  cartLinesUpdate(cartToken: $token, lines: [{id: $lineId, quantity: $quantity}]) {
    cart {
      id
      totalPrice
    }
    userErrors {
      field
      message
    }
  }
}
```

### 从购物车移除

```graphql
mutation RemoveFromCart($token: String!, $lineIds: [ID!]!) {
  cartLinesRemove(cartToken: $token, lineIds: $lineIds) {
    cart {
      id
      totalPrice
    }
    userErrors {
      field
      message
    }
  }
}
```

### 应用优惠券

```graphql
mutation ApplyCoupon($token: String!, $code: String!) {
  cartCouponApply(cartToken: $token, code: $code) {
    cart {
      id
      discountCodes
      totalPrice
    }
    userErrors {
      field
      message
    }
  }
}
```

### 创建结账

```graphql
mutation CreateCheckout($input: CheckoutCreateInput!) {
  checkoutCreate(input: $input) {
    checkout {
      id
      token
      webUrl
      totalPrice
      lineItems {
        edges {
          node {
            title
            quantity
          }
        }
      }
    }
    userErrors {
      field
      message
    }
  }
}
```

#### 变量

```json
{
  "input": {
    "lineItems": [
      {
        "variantId": "gid://rcommerce/ProductVariant/550e8400-e29b-41d4-a716-446655440001",
        "quantity": 2
      }
    ],
    "email": "customer@example.com",
    "shippingAddress": {
      "firstName": "John",
      "lastName": "Doe",
      "address1": "123 Main St",
      "city": "New York",
      "province": "NY",
      "country": "US",
      "zip": "10001"
    }
  }
}
```

### 完成结账并支付

```graphql
mutation CheckoutComplete($token: String!, $payment: PaymentInput!) {
  checkoutComplete(token: $token, payment: $payment) {
    order {
      id
      orderNumber
      totalPrice
      financialStatus
    }
    payment {
      id
      status
      gateway
    }
    userErrors {
      field
      message
    }
  }
}
```

## 订阅

通过 WebSocket 实时更新（如果启用）。

### 购物车更新

```graphql
subscription OnCartUpdate($token: String!) {
  cartUpdated(token: $token) {
    id
    totalPrice
    lineItems {
      edges {
        node {
          quantity
        }
      }
    }
  }
}
```

### 订单状态

```graphql
subscription OnOrderUpdate($id: ID!) {
  orderUpdated(id: $id) {
    id
    fulfillmentStatus
    financialStatus
    fulfillments {
      edges {
        node {
          status
          trackingNumber
        }
      }
    }
  }
}
```

## 片段

重用常见的字段选择：

```graphql
fragment ProductFields on Product {
  id
  title
  slug
  price
  currency
  inventoryQuantity
  status
}

fragment CustomerFields on Customer {
  id
  email
  firstName
  lastName
  acceptsMarketing
}

query GetProductsWithFragment {
  products(first: 10) {
    edges {
      node {
        ...ProductFields
        vendor
        tags
      }
    }
  }
}
```

## 错误处理

GraphQL 即使出错也返回 200 OK。检查 `errors` 数组：

```json
{
  "data": {
    "cartLinesAdd": {
      "cart": null,
      "userErrors": [
        {
          "field": ["lines", 0, "merchandiseId"],
          "message": "产品变体未找到"
        }
      ]
    }
  }
}
```

### 常见错误类型

| 错误 | 说明 |
|-------|-------------|
| `NOT_FOUND` | 请求的资源不存在 |
| `INVALID_INPUT` | 输入验证失败 |
| `INSUFFICIENT_INVENTORY` | 库存不足 |
| `PAYMENT_ERROR` | 支付处理失败 |
| `AUTHENTICATION_ERROR` | 凭证无效或缺失 |
| `RATE_LIMITED` | 请求过多 |

## 速率限制

GraphQL 查询按复杂度评级：

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705312800
X-RateLimit-Cost: 10
```

### 复杂度计算

- 基础成本：1 点
- 每个字段：+1 点
- 嵌套连接：+10 点
- 最大复杂度：每次查询 1000 点

## 分页

对大数据集使用基于光标的分页：

```graphql
query GetProductsPaginated($first: Int!, $after: String) {
  products(first: $first, after: $after) {
    edges {
      cursor
      node {
        id
        title
      }
    }
    pageInfo {
      hasNextPage
      hasPreviousPage
      startCursor
      endCursor
    }
  }
}
```

### 分页变量

```json
{
  "first": 10,
  "after": "eyJpZCI6IjU1MGU4NDAwLWUyOWItNDFkNC1hNzE2LTQ0NjY1NTQ0MDAwMCJ9"
}
```

## 模式内省

探索模式：

```graphql
{
  __schema {
    types {
      name
      kind
    }
    queryType {
      fields {
        name
        type {
          name
        }
      }
    }
    mutationType {
      fields {
        name
      }
    }
  }
}
```

获取类型详情：

```graphql
{
  __type(name: "Product") {
    name
    fields {
      name
      type {
        name
        kind
      }
    }
  }
}
```

## SDK 示例

### JavaScript/TypeScript

```typescript
import { GraphQLClient, gql } from 'graphql-request'

const client = new GraphQLClient('https://api.rcommerce.app/graphql', {
  headers: {
    authorization: 'Bearer YOUR_API_KEY',
  },
})

const query = gql`
  query GetProduct($id: ID!) {
    product(id: $id) {
      title
      price
    }
  }
`

const data = await client.request(query, { id: 'gid://rcommerce/Product/123' })
```

### Python

```python
from gql import gql, Client
from gql.transport.requests import RequestsHTTPTransport

transport = RequestsHTTPTransport(
    url='https://api.rcommerce.app/graphql',
    headers={'Authorization': 'Bearer YOUR_API_KEY'}
)

client = Client(transport=transport)

query = gql('''
  query GetProduct($id: ID!) {
    product(id: $id) {
      title
      price
    }
  }
''')

result = client.execute(query, variable_values={'id': 'gid://rcommerce/Product/123'})
```

### Rust

```rust
use graphql_client::{GraphQLQuery, Response};
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "src/get_product.graphql"
)]
struct GetProduct;

async fn fetch_product(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let variables = get_product::Variables {
        id: id.to_string(),
    };
    
    let response = client
        .post("https://api.rcommerce.app/graphql")
        .header("Authorization", "Bearer YOUR_API_KEY")
        .json(&GetProduct::build_query(variables))
        .send()
        .await?;
    
    let response_body: Response<get_product::ResponseData> = response.json().await?;
    
    if let Some(data) = response_body.data {
        println!("Product: {:?}", data.product);
    }
    
    Ok(())
}
```

## 最佳实践

1. **只请求需要的字段** - 减少负载大小并提高性能
2. **使用片段** - 重用常见的选择
3. **处理分页** - 不要假设所有数据都在一个请求中
4. **检查 userErrors** - 始终处理业务逻辑错误
5. **缓存响应** - 对可缓存的查询使用 ETags
6. **使用变量** - 不要将字符串连接到查询中

## 与 REST 的比较

| 特性 | GraphQL | REST |
|---------|---------|------|
| 数据获取 | 需要的精确字段 | 固定的响应结构 |
| 多个资源 | 单个请求 | 多个请求 |
| 版本控制 | 模式演进 | URL 版本控制 |
| 内省 | 内置 | 需要文档 |
| 文件上传 | 多部分变更 | 标准 POST |
| 缓存 | 应用层 | HTTP 缓存 |
