# GraphQL API

!!! warning "Coming Soon"
    The GraphQL API is planned for a future release (v0.2). The documentation below describes the planned implementation and is subject to change.

The GraphQL API provides a flexible, type-safe query language for accessing your R Commerce data.

## Overview

GraphQL allows clients to request exactly the data they need, reducing over-fetching and enabling powerful data fetching patterns.

## Endpoint

```
POST /graphql
```

## Authentication

Include your API key in the Authorization header:

```http
POST /graphql HTTP/1.1
Host: api.rcommerce.app
Authorization: Bearer YOUR_API_KEY
Content-Type: application/json
```

## Queries

### Products Query

Retrieve products with flexible field selection.

#### Basic Query

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

#### With Variants and Images

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

#### Variables

```json
{
  "id": "gid://rcommerce/Product/550e8400-e29b-41d4-a716-446655440000"
}
```

### Orders Query

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

### Customers Query

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

### Cart Query

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

## Mutations

### Create Cart

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

### Add to Cart

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

#### Variables

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

### Update Cart Line

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

### Remove from Cart

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

### Apply Coupon

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

### Create Checkout

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

#### Variables

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

### Complete Checkout with Payment

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

## Subscriptions

Real-time updates via WebSocket (if enabled).

### Cart Updates

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

### Order Status

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

## Fragments

Reuse common field selections:

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

## Error Handling

GraphQL returns 200 OK even for errors. Check the `errors` array:

```json
{
  "data": {
    "cartLinesAdd": {
      "cart": null,
      "userErrors": [
        {
          "field": ["lines", 0, "merchandiseId"],
          "message": "Product variant not found"
        }
      ]
    }
  }
}
```

### Common Error Types

| Error | Description |
|-------|-------------|
| `NOT_FOUND` | Requested resource doesn't exist |
| `INVALID_INPUT` | Input validation failed |
| `INSUFFICIENT_INVENTORY` | Not enough stock available |
| `PAYMENT_ERROR` | Payment processing failed |
| `AUTHENTICATION_ERROR` | Invalid or missing credentials |
| `RATE_LIMITED` | Too many requests |

## Rate Limiting

GraphQL queries are rated by complexity:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1705312800
X-RateLimit-Cost: 10
```

### Complexity Calculation

- Base cost: 1 point
- Each field: +1 point
- Nested connections: +10 points
- Maximum complexity: 1000 points per query

## Pagination

Use cursor-based pagination for large datasets:

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

### Pagination Variables

```json
{
  "first": 10,
  "after": "eyJpZCI6IjU1MGU4NDAwLWUyOWItNDFkNC1hNzE2LTQ0NjY1NTQ0MDAwMCJ9"
}
```

## Schema Introspection

Explore the schema:

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

Get type details:

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

## SDK Examples

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

## Best Practices

1. **Request only needed fields** - Reduces payload size and improves performance
2. **Use fragments** - Reuse common selections
3. **Handle pagination** - Don't assume all data fits in one request
4. **Check userErrors** - Always handle business logic errors
5. **Cache responses** - Use ETags for cacheable queries
6. **Use variables** - Don't concatenate strings into queries

## Comparison with REST

| Feature | GraphQL | REST |
|---------|---------|------|
| Data fetching | Exact fields needed | Fixed response structure |
| Multiple resources | Single request | Multiple requests |
| Versioning | Schema evolution | URL versioning |
| Introspection | Built-in | Requires documentation |
| File uploads | Multipart mutation | Standard POST |
| Caching | Application-level | HTTP caching |
