use axum::{Json, Router, routing::get, extract::{Path, State}};
use uuid::Uuid;

use crate::state::AppState;

/// List products from database
pub async fn list_products(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    match state.product_service.list_products(None, rcommerce_core::services::PaginationParams::default()).await {
        Ok(product_list) => {
            let products: Vec<serde_json::Value> = product_list.products.into_iter().map(|p| {
                serde_json::json!({
                    "id": p.id,
                    "title": p.title,
                    "slug": p.slug,
                    "price": p.price,
                    "currency": p.currency,
                    "description": p.description,
                    "is_active": p.is_active,
                    "inventory_quantity": p.inventory_quantity,
                    "created_at": p.created_at
                })
            }).collect();
            
            Json(serde_json::json!({
                "products": products,
                "meta": {
                    "total": product_list.pagination.total,
                    "page": product_list.pagination.page,
                    "per_page": product_list.pagination.per_page,
                    "total_pages": product_list.pagination.total_pages
                }
            }))
        }
        Err(e) => {
            tracing::error!("Failed to list products: {}", e);
            // Return empty list on error for now
            Json(serde_json::json!({
                "products": [],
                "meta": {
                    "total": 0,
                    "page": 1,
                    "per_page": 20,
                    "total_pages": 0
                }
            }))
        }
    }
}

/// Get product by ID from database
pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let product_id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Json(serde_json::json!({
                "error": "Invalid product ID format"
            }));
        }
    };
    
    match state.product_service.get_product(product_id).await {
        Ok(Some(product_detail)) => {
            let p = product_detail.product;
            Json(serde_json::json!({
                "product": {
                    "id": p.id,
                    "title": p.title,
                    "slug": p.slug,
                    "description": p.description,
                    "price": p.price,
                    "compare_at_price": p.compare_at_price,
                    "cost_price": p.cost_price,
                    "currency": p.currency,
                    "inventory_quantity": p.inventory_quantity,
                    "inventory_policy": p.inventory_policy,
                    "inventory_management": p.inventory_management,
                    "weight": p.weight,
                    "weight_unit": p.weight_unit,
                    "requires_shipping": p.requires_shipping,
                    "is_active": p.is_active,
                    "is_featured": p.is_featured,
                    "seo_title": p.seo_title,
                    "seo_description": p.seo_description,
                    "created_at": p.created_at,
                    "updated_at": p.updated_at,
                    "published_at": p.published_at,
                    "variants": product_detail.variants.into_iter().map(|v| serde_json::json!({
                        "id": v.id,
                        "title": v.title,
                        "sku": v.sku,
                        "price": v.price,
                        "inventory_quantity": v.inventory_quantity
                    })).collect::<Vec<_>>(),
                    "images": product_detail.images.into_iter().map(|i| serde_json::json!({
                        "id": i.id,
                        "src": i.src,
                        "alt_text": i.alt_text
                    })).collect::<Vec<_>>()
                }
            }))
        }
        Ok(None) => {
            Json(serde_json::json!({
                "error": "Product not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get product: {}", e);
            Json(serde_json::json!({
                "error": "Failed to retrieve product"
            }))
        }
    }
}

/// Router for product routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/products", get(list_products))
        .route("/products/:id", get(get_product))
}
