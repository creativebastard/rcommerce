use axum::{Json, routing::get};

/// List products - Basic Phase 1 implementation
pub async fn list_products() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "products": [
            {
                "id": "123e4567-e89b-12d3-a456-426614174000",
                "title": "Sample Product 1",
                "slug": "sample-product-1",
                "price": 29.99,
                "currency": "USD",
                "description": "Sample product for Phase 1 MVP",
                "is_active": true,
                "created_at": "2024-01-01T00:00:00Z"
            },
            {
                "id": "123e4567-e89b-12d3-a456-426614174001",
                "title": "Sample Product 2",
                "slug": "sample-product-2", 
                "price": 39.99,
                "currency": "USD",
                "description": "Another sample product",
                "is_active": true,
                "created_at": "2024-01-02T00:00:00Z"
            }
        ],
        "meta": {
            "total": 2,
            "page": 1,
            "per_page": 20,
            "total_pages": 1
        }
    }))
}

/// Get product by ID - Basic Phase 1 implementation  
pub async fn get_product(path: axum::extract::Path<String>) -> Json<serde_json::Value> {
    let _id = path.0; // Extract ID from path but ignore for now
    
    Json(serde_json::json!({
        "product": {
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "title": "Sample Product",
            "slug": "sample-product",
            "price": 29.99,
            "compare_at_price": 39.99,
            "cost_price": 15.00,
            "currency": "USD",
            "description": "This is a sample product response for Phase 1 MVP",
            "inventory_quantity": 100,
            "inventory_policy": "deny",
            "inventory_management": true,
            "weight": 0.5,
            "weight_unit": "kg",
            "requires_shipping": true,
            "is_active": true,
            "is_featured": false,
            "seo_title": "Sample Product - Buy Now",
            "seo_description": "High quality sample product for testing R Commerce API",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "published_at": "2024-01-01T00:00:00Z"
        }
    }))
}

/// Router for product routes
pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/products", get(list_products))
        .route("/products/:id", get(get_product))
}