use async_trait::async_trait;
use uuid::Uuid;
use sqlx::Row;

use crate::{
    Result, Pagination, SortParams, SortDirection,
    models::{
        Product, ProductVariant, ProductImage, ProductFilter,
        CreateProductRequest, UpdateProductRequest
    },
    traits::Repository,
};
use super::Database;

pub struct ProductRepository {
    db: Database,
}

impl ProductRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    /// Find products with filtering and pagination
    pub async fn find_with_filter(
        &self,
        filter: &ProductFilter,
        pagination: &Pagination,
        sort: Option<&SortParams>,
    ) -> Result<Vec<Product>> {
        let mut query = String::from(
            "SELECT * FROM products WHERE 1=1"
        );
        
        // Add filters
        if let Some(status) = filter.status {
            match status {
                super::super::models::product::ProductStatus::Active => {
                    query.push_str(" AND is_active = true");
                }
                super::super::models::product::ProductStatus::Draft => {
                    query.push_str(" AND is_active = false AND published_at IS NULL");
                }
                super::super::models::product::ProductStatus::Archived => {
                    // Need to add archived field to products table
                }
            }
        }
        
        if let Some(category_id) = filter.category_id {
            query.push_str(&format!(" AND id IN (SELECT product_id FROM product_category_relations WHERE category_id = '{}')", category_id));
        }
        
        if let Some(price_min) = filter.price_min {
            query.push_str(&format!(" AND price >= {}", price_min));
        }
        
        if let Some(price_max) = filter.price_max {
            query.push_str(&format!(" AND price <= {}", price_max));
        }
        
        // Add sorting
        if let Some(sort) = sort {
            let direction = match sort.direction {
                SortDirection::Asc => "ASC",
                SortDirection::Desc => "DESC",
            };
            query.push_str(&format!(" ORDER BY {} {}", sort.field, direction));
        } else {
            query.push_str(" ORDER BY created_at DESC");
        }
        
        // Add pagination
        query.push_str(&format!(" LIMIT {} OFFSET {}", pagination.per_page, pagination.offset()));
        
        let products = sqlx::query_as::<_, Product>(&query)
            .fetch_all(self.db.pool())
            .await?;
        
        Ok(products)
    }
    
    /// Find variants for a product
    pub async fn find_variants(&self, product_id: Uuid) -> Result<Vec<ProductVariant>> {
        let variants = sqlx::query_as::<_, ProductVariant>(
            "SELECT * FROM product_variants WHERE product_id = $1 ORDER BY created_at"
        )
        .bind(product_id)
        .fetch_all(self.db.pool())
        .await?;
        
        Ok(variants)
    }
    
    /// Find images for a product
    pub async fn find_images(&self, product_id: Uuid) -> Result<Vec<ProductImage>> {
        let images = sqlx::query_as::<_, ProductImage>(
            "SELECT * FROM product_images WHERE product_id = $1 ORDER BY position"
        )
        .bind(product_id)
        .fetch_all(self.db.pool())
        .await?;
        
        Ok(images)
    }
}

#[async_trait]
impl Repository<Product, Uuid> for ProductRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(product)
    }
    
    async fn list(&self) -> Result<Vec<Product>> {
        let products = sqlx::query_as::<_, Product>(
            "SELECT * FROM products ORDER BY created_at DESC"
        )
        .fetch_all(self.db.pool())
        .await?;
        
        Ok(products)
    }
    
    async fn create(&self, _entity: Product) -> Result<Product> {
        // This is a simplified version - in production, use CreateProductRequest
        Err(crate::Error::not_implemented("Use create_with_request instead".to_string()))
    }
    
    async fn update(&self, _entity: Product) -> Result<Product> {
        Err(crate::Error::not_implemented("Use update_with_request instead".to_string()))
    }
    
    async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(id)
            .execute(self.db.pool())
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
}

/// Extension trait for product-specific operations
impl ProductRepository {
    pub async fn create_with_request(&self, request: CreateProductRequest) -> Result<Product> {
        let product = sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO products (
                title, slug, description, sku, price, compare_at_price, cost_price,
                currency, inventory_quantity, inventory_policy, inventory_management,
                continues_selling_when_out_of_stock, weight, weight_unit, requires_shipping,
                is_active, is_featured, seo_title, seo_description
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING *
            "#
        )
        .bind(request.title)
        .bind(request.slug)
        .bind(request.description)
        .bind(request.sku)
        .bind(request.price)
        .bind(request.compare_at_price)
        .bind(request.cost_price)
        .bind(request.currency)
        .bind(request.inventory_quantity)
        .bind(request.inventory_policy)
        .bind(request.inventory_management)
        .bind(request.continues_selling_when_out_of_stock)
        .bind(request.weight)
        .bind(request.weight_unit)
        .bind(request.requires_shipping)
        .bind(request.is_active)
        .bind(request.is_featured)
        .bind(request.seo_title)
        .bind(request.seo_description)
        .fetch_one(self.db.pool())
        .await?;
        
        Ok(product)
    }
    
    pub async fn update_with_request(&self, id: Uuid, request: UpdateProductRequest) -> Result<Product> {
        // Build dynamic update query based on which fields are provided
        let mut query = String::from("UPDATE products SET ");
        let mut sets = Vec::new();
        
        if let Some(title) = request.title {
            sets.push(format!("title = '{}'", title.replace("'", "''")));
        }
        if let Some(slug) = request.slug {
            sets.push(format!("slug = '{}'", slug.replace("'", "''")));
        }
        if let Some(description) = request.description {
            if let Some(desc) = description {
                sets.push(format!("description = '{}'", desc.replace("'", "''")));
            } else {
                sets.push("description = NULL".to_string());
            }
        }
        if let Some(price) = request.price {
            sets.push(format!("price = {}", price));
        }
        if let Some(is_active) = request.is_active {
            sets.push(format!("is_active = {}", is_active));
        }
        
        if sets.is_empty() {
            return Err(crate::Error::Validation("No fields to update".to_string()));
        }
        
        query.push_str(&sets.join(", "));
        query.push_str(&format!(", updated_at = NOW() WHERE id = '{}' RETURNING *", id));
        
        let product = sqlx::query_as::<_, Product>(&query)
            .fetch_one(self.db.pool())
            .await?;
        
        Ok(product)
    }
    
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Product>> {
        let product = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE slug = $1"
        )
        .bind(slug)
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(product)
    }
    
    pub async fn count_by_filter(&self, filter: &ProductFilter) -> Result<i64> {
        let mut query = String::from("SELECT COUNT(*) FROM products WHERE 1=1");
        
        // Add same filters as find_with_filter
        if let Some(_status) = filter.status {
            // TODO: Add status filtering
        }
        
        if filter.category_id.is_some() {
            query.push_str(" AND id IN (SELECT product_id FROM product_category_relations WHERE category_id = $1)");
        }
        
        let row = if filter.category_id.is_some() {
            sqlx::query(&query)
                .bind(filter.category_id.unwrap())
                .fetch_one(self.db.pool())
                .await?
        } else {
            sqlx::query(&query)
                .fetch_one(self.db.pool())
                .await?
        };
        
        let count: i64 = row.get(0);
        Ok(count)
    }
}
