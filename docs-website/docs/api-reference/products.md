# Products API

The Products API provides complete product catalog management including products, variants, images, collections, and inventory.

## Base URL

```
/api/v1/products
```

## Authentication

All product endpoints require authentication via API key or JWT token.

```http
Authorization: Bearer YOUR_API_KEY
```

## Product Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Premium Cotton T-Shirt",
  "slug": "premium-cotton-t-shirt",
  "description": "High-quality 100% organic cotton t-shirt",
  "short_description": "Organic cotton tee",
  "product_type": "physical",
  "vendor": "Organic Basics",
  "tags": ["clothing", "organic", "t-shirt"],
  "status": "active",
  "is_featured": true,
  "price": "29.99",
  "compare_at_price": "39.99",
  "cost_price": "12.00",
  "currency": "USD",
  "taxable": true,
  "requires_shipping": true,
  "weight": "0.5",
  "weight_unit": "kg",
  "inventory_quantity": 100,
  "inventory_policy": "deny",
  "inventory_management": "rcommerce",
  "low_stock_threshold": 10,
  "has_variants": true,
  "seo_title": "Premium Organic Cotton T-Shirt",
  "seo_description": "Shop our premium organic cotton t-shirts",
  "meta_fields": {
    "material": "100% Organic Cotton",
    "care_instructions": "Machine wash cold"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-20T14:30:00Z",
  "published_at": "2024-01-15T10:00:00Z"
}
```

### Product Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `title` | string | Product name (required) |
| `slug` | string | URL-friendly identifier (unique) |
| `description` | string | Full product description (HTML supported) |
| `short_description` | string | Brief summary for listings |
| `product_type` | string | `physical`, `digital`, `service` |
| `vendor` | string | Brand or manufacturer name |
| `tags` | array | Searchable tags |
| `status` | string | `active`, `draft`, `archived` |
| `is_featured` | boolean | Highlight in featured sections |
| `price` | decimal | Current selling price |
| `compare_at_price` | decimal | Original price (for sales) |
| `cost_price` | decimal | Internal cost (margin calculation) |
| `currency` | string | ISO 4217 currency code |
| `taxable` | boolean | Subject to sales tax |
| `requires_shipping` | boolean | Physical shipment required |
| `weight` | decimal | Product weight |
| `weight_unit` | string | `kg`, `g`, `lb`, `oz` |
| `inventory_quantity` | integer | Current stock level |
| `inventory_policy` | string | `deny` or `continue` (backorder) |
| `inventory_management` | string | `rcommerce` or null |
| `low_stock_threshold` | integer | Alert when stock below this |
| `has_variants` | boolean | Has size/color options |
| `seo_title` | string | Custom title tag |
| `seo_description` | string | Meta description |
| `meta_fields` | object | Custom key-value data |
| `created_at` | datetime | Creation timestamp |
| `updated_at` | datetime | Last modification |
| `published_at` | datetime | Go-live date (null = draft) |

## Endpoints

### List Products

```http
GET /api/v1/products
```

Retrieve a paginated list of products.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `per_page` | integer | Items per page (default: 20, max: 100) |
| `status` | string | Filter by `active`, `draft`, `archived` |
| `product_type` | string | Filter by type |
| `vendor` | string | Filter by vendor name |
| `min_price` | decimal | Minimum price filter |
| `max_price` | decimal | Maximum price filter |
| `tags` | string | Comma-separated tags |
| `ids` | string | Comma-separated product IDs |
| `collection_id` | UUID | Filter by collection |
| `q` | string | Search query (title, description, tags) |
| `sort` | string | `title`, `price`, `created_at`, `updated_at` |
| `order` | string | `asc` or `desc` (default: desc) |
| `created_after` | datetime | Created after date |
| `created_before` | datetime | Created before date |
| `inventory_status` | string | `in_stock`, `low_stock`, `out_of_stock` |
| `is_featured` | boolean | Featured products only |

#### Example Request

```http
GET /api/v1/products?status=active&min_price=10.00&sort=price&order=desc&per_page=50
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "Premium Cotton T-Shirt",
      "slug": "premium-cotton-t-shirt",
      "price": "29.99",
      "compare_at_price": "39.99",
      "currency": "USD",
      "inventory_quantity": 100,
      "status": "active",
      "is_featured": true,
      "created_at": "2024-01-15T10:00:00Z",
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

### Get Product

```http
GET /api/v1/products/{id}
```

Retrieve a single product by ID or slug.

#### Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | string | Product UUID or slug |

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `include` | string | Related data: `variants`, `images`, `collections` |

#### Example Request

```http
GET /api/v1/products/premium-cotton-t-shirt?include=variants,images
Authorization: Bearer sk_live_xxx
```

#### Example Response

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Premium Cotton T-Shirt",
    "slug": "premium-cotton-t-shirt",
    "description": "High-quality 100% organic cotton t-shirt",
    "price": "29.99",
    "compare_at_price": "39.99",
    "currency": "USD",
    "inventory_quantity": 100,
    "status": "active",
    "variants": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440001",
        "title": "Small / Black",
        "sku": "TSH-S-BLK",
        "price": "29.99",
        "inventory_quantity": 25,
        "options": {
          "size": "S",
          "color": "Black"
        }
      }
    ],
    "images": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440002",
        "url": "https://cdn.example.com/products/tshirt-main.jpg",
        "alt_text": "Premium cotton t-shirt front view",
        "position": 1
      }
    ]
  }
}
```

### Create Product

```http
POST /api/v1/products
```

Create a new product.

#### Request Body

```json
{
  "title": "Premium Cotton T-Shirt",
  "slug": "premium-cotton-t-shirt",
  "description": "High-quality 100% organic cotton t-shirt",
  "product_type": "physical",
  "vendor": "Organic Basics",
  "tags": ["clothing", "organic", "t-shirt"],
  "status": "active",
  "price": "29.99",
  "compare_at_price": "39.99",
  "currency": "USD",
  "inventory_quantity": 100,
  "inventory_policy": "deny",
  "requires_shipping": true,
  "weight": "0.5",
  "weight_unit": "kg",
  "seo_title": "Premium Organic Cotton T-Shirt",
  "seo_description": "Shop our premium organic cotton t-shirts"
}
```

#### Required Fields

- `title` - Product name
- `price` - Selling price
- `currency` - ISO 4217 code

#### Example Response

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Premium Cotton T-Shirt",
    "slug": "premium-cotton-t-shirt",
    "price": "29.99",
    "currency": "USD",
    "status": "active",
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-15T10:00:00Z"
  }
}
```

### Update Product

```http
PUT /api/v1/products/{id}
```

Update an existing product (full replacement).

#### Request Body

Same as Create Product, all fields required.

### Patch Product

```http
PATCH /api/v1/products/{id}
```

Partial update of product fields.

#### Request Body

```json
{
  "price": "24.99",
  "compare_at_price": "29.99",
  "inventory_quantity": 150
}
```

### Delete Product

```http
DELETE /api/v1/products/{id}
```

Delete a product. Products with orders cannot be deleted, only archived.

#### Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `force` | boolean | Force delete even with orders (use with caution) |

#### Response

- `204 No Content` - Successfully deleted
- `409 Conflict` - Product has associated orders

## Variants

Product variants represent different options like size and color.

### Variant Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Small / Black",
  "sku": "TSH-S-BLK",
  "barcode": "123456789012",
  "price": "29.99",
  "compare_at_price": "39.99",
  "cost_price": "12.00",
  "inventory_quantity": 25,
  "inventory_policy": "deny",
  "weight": "0.5",
  "weight_unit": "kg",
  "requires_shipping": true,
  "is_default": false,
  "options": {
    "size": "S",
    "color": "Black"
  },
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### List Variants

```http
GET /api/v1/products/{product_id}/variants
```

### Create Variant

```http
POST /api/v1/products/{product_id}/variants
```

#### Request Body

```json
{
  "title": "Medium / White",
  "sku": "TSH-M-WHT",
  "price": "29.99",
  "inventory_quantity": 30,
  "options": {
    "size": "M",
    "color": "White"
  }
}
```

### Update Variant

```http
PUT /api/v1/products/{product_id}/variants/{variant_id}
```

### Delete Variant

```http
DELETE /api/v1/products/{product_id}/variants/{variant_id}
```

## Images

### Image Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440002",
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "url": "https://cdn.example.com/products/tshirt-main.jpg",
  "alt_text": "Premium cotton t-shirt front view",
  "position": 1,
  "width": 1200,
  "height": 1200,
  "file_size": 245760,
  "file_type": "image/jpeg",
  "created_at": "2024-01-15T10:00:00Z"
}
```

### List Images

```http
GET /api/v1/products/{product_id}/images
```

### Upload Image

```http
POST /api/v1/products/{product_id}/images
Content-Type: multipart/form-data
```

#### Request Body

| Field | Type | Description |
|-------|------|-------------|
| `file` | file | Image file (JPG, PNG, WebP, max 10MB) |
| `alt_text` | string | Alt text for accessibility |
| `position` | integer | Display order |

### Update Image

```http
PUT /api/v1/products/{product_id}/images/{image_id}
```

### Delete Image

```http
DELETE /api/v1/products/{product_id}/images/{image_id}
```

## Collections

Collections group products for organization and display.

### Collection Object

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440010",
  "title": "Summer Collection 2024",
  "slug": "summer-collection-2024",
  "description": "Hot styles for summer",
  "collection_type": "manual",
  "rules": [],
  "is_active": true,
  "seo_title": "Summer Collection 2024",
  "seo_description": "Discover our summer styles",
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-01-15T10:00:00Z"
}
```

### List Collections

```http
GET /api/v1/collections
```

### Get Collection

```http
GET /api/v1/collections/{id}
```

### Create Collection

```http
POST /api/v1/collections
```

#### Request Body

```json
{
  "title": "Summer Collection 2024",
  "slug": "summer-collection-2024",
  "description": "Hot styles for summer",
  "collection_type": "manual",
  "is_active": true
}
```

### Add Product to Collection

```http
POST /api/v1/collections/{collection_id}/products
```

#### Request Body

```json
{
  "product_id": "550e8400-e29b-41d4-a716-446655440000",
  "position": 1
}
```

### Remove Product from Collection

```http
DELETE /api/v1/collections/{collection_id}/products/{product_id}
```

## Inventory Management

### Adjust Inventory

```http
POST /api/v1/products/{id}/inventory/adjust
```

#### Request Body

```json
{
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "adjustment": -5,
  "reason": "Customer order #1001"
}
```

### Set Inventory

```http
POST /api/v1/products/{id}/inventory/set
```

#### Request Body

```json
{
  "location_id": "550e8400-e29b-41d4-a716-446655440020",
  "quantity": 100,
  "reason": "Stock count"
}
```

### Transfer Inventory

```http
POST /api/v1/products/{id}/inventory/transfer
```

#### Request Body

```json
{
  "from_location_id": "550e8400-e29b-41d4-a716-446655440020",
  "to_location_id": "550e8400-e29b-41d4-a716-446655440021",
  "quantity": 10,
  "reason": "Restock retail location"
}
```

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `PRODUCT_NOT_FOUND` | 404 | Product does not exist |
| `PRODUCT_SLUG_TAKEN` | 409 | Slug already in use |
| `INVALID_PRODUCT_TYPE` | 400 | Invalid product_type value |
| `INVALID_CURRENCY` | 400 | Invalid currency code |
| `INVALID_PRICE` | 400 | Negative or invalid price |
| `VARIANT_NOT_FOUND` | 404 | Variant does not exist |
| `VARIANT_SKU_TAKEN` | 409 | SKU already in use |
| `COLLECTION_NOT_FOUND` | 404 | Collection does not exist |
| `IMAGE_TOO_LARGE` | 400 | Image exceeds size limit |
| `INVALID_IMAGE_TYPE` | 400 | Unsupported image format |
| `INVENTORY_NEGATIVE` | 400 | Cannot set negative inventory |
| `PRODUCT_HAS_ORDERS` | 409 | Cannot delete product with orders |

## Webhooks

The Products API emits the following webhook events:

| Event | Description |
|-------|-------------|
| `product.created` | New product created |
| `product.updated` | Product information changed |
| `product.deleted` | Product removed |
| `product.published` | Product status changed to active |
| `product.unpublished` | Product status changed to draft |
| `product.inventory_changed` | Stock quantity updated |
| `product.low_stock` | Inventory below threshold |
| `product.out_of_stock` | Inventory reached zero |
| `variant.created` | New variant added |
| `variant.updated` | Variant information changed |
| `variant.deleted` | Variant removed |
| `collection.created` | New collection created |
| `collection.updated` | Collection information changed |
| `collection.product_added` | Product added to collection |
| `collection.product_removed` | Product removed from collection |
