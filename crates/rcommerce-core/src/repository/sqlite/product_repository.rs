use async_trait::async_trait;
use uuid::Uuid;
use sqlx::Row;

use crate::{
    Result, Pagination, SortParams, SortDirection,
    models::{
        ProductFilter, CreateProductRequest, UpdateProductRequest
    },
    models::sqlite::{Product, ProductVariant, ProductImage},
};
use crate::repository::traits::ProductRepositoryTrait;
use super::SqliteDb;

#[derive(Clone)]
pub struct SqliteProductRepository {
    db: SqliteDb,
}

impl SqliteProductRepository {
    pub fn new(db: SqliteDb) -> Self {
        Self { db }
    }
    
    // Helper to convert SQLite product to standard product
    fn convert_product(&self, p: Product) -> crate::models::Product {
        p.into()
    }
}

#[async_trait]
impl ProductRepositoryTrait for SqliteProductRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::models::Product>> {
        let product: Option<Product> = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE id = ?1"
        )
        .bind(id.to_string())
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(product.map(|p| self.convert_product(p)))
    }
    
    async fn list(&self) -> Result<Vec<crate::models::Product>> {
        let products: Vec<Product> = sqlx::query_as::<_, Product>(
            "SELECT * FROM products ORDER BY created_at DESC"
        )
        .fetch_all(self.db.pool())
        .await?;
        
        Ok(products.into_iter().map(|p| self.convert_product(p)).collect())
    }
    
    async fn find_with_filter(
        &self,
        filter: &ProductFilter,
        pagination: &Pagination,
        sort: Option<&SortParams>,
    ) -> Result<Vec<crate::models::Product>> {
        let mut query = String::from("SELECT * FROM products WHERE 1=1");
        
        if let Some(status) = filter.status {
            match status {
                crate::models::ProductStatus::Active => {
                    query.push_str(" AND is_active = 1");
                }
                crate::models::ProductStatus::Draft => {
                    query.push_str(" AND is_active = 0 AND published_at IS NULL");
                }
                crate::models::ProductStatus::Archived => {}
            }
        }
        
        if let Some(sort) = sort {
            let direction = match sort.direction {
                SortDirection::Asc => "ASC",
                SortDirection::Desc => "DESC",
            };
            query.push_str(&format!(" ORDER BY {} {}", sort.field, direction));
        } else {
            query.push_str(" ORDER BY created_at DESC");
        }
        
        query.push_str(&format!(" LIMIT {} OFFSET {}", pagination.per_page, pagination.offset()));
        
        let products: Vec<Product> = sqlx::query_as::<_, Product>(&query)
            .fetch_all(self.db.pool())
            .await?;
        
        Ok(products.into_iter().map(|p| self.convert_product(p)).collect())
    }
    
    async fn find_by_slug(&self, slug: &str) -> Result<Option<crate::models::Product>> {
        let product: Option<Product> = sqlx::query_as::<_, Product>(
            "SELECT * FROM products WHERE slug = ?1"
        )
        .bind(slug)
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(product.map(|p| self.convert_product(p)))
    }
    
    async fn count_by_filter(&self, filter: &ProductFilter) -> Result<i64> {
        let mut query = String::from("SELECT COUNT(*) FROM products WHERE 1=1");
        
        if let Some(_status) = filter.status {}
        
        let row = sqlx::query(&query)
            .fetch_one(self.db.pool())
            .await?;
        
        let count: i64 = row.get(0);
        Ok(count)
    }
    
    async fn create(&self, request: CreateProductRequest) -> Result<crate::models::Product> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let price: f64 = request.price.try_into().unwrap_or(0.0);
        
        sqlx::query(
            r#"
            INSERT INTO products (
                id, title, slug, description, sku, price, compare_at_price, cost_price,
                currency, inventory_quantity, inventory_policy, inventory_management,
                continues_selling_when_out_of_stock, weight, weight_unit, requires_shipping,
                is_active, is_featured, seo_title, seo_description, created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?21)
            "#
        )
        .bind(id.to_string())
        .bind(request.title)
        .bind(request.slug)
        .bind(request.description)
        .bind(request.sku)
        .bind(price)
        .bind(request.compare_at_price.map(|d| d.try_into().unwrap_or(0.0)))
        .bind(request.cost_price.map(|d| d.try_into().unwrap_or(0.0)))
        .bind(request.currency)
        .bind(request.inventory_quantity)
        .bind(request.inventory_policy)
        .bind(request.inventory_management)
        .bind(request.continues_selling_when_out_of_stock)
        .bind(request.weight.map(|d| d.try_into().unwrap_or(0.0)))
        .bind(request.weight_unit)
        .bind(request.requires_shipping)
        .bind(request.is_active)
        .bind(request.is_featured)
        .bind(request.seo_title)
        .bind(request.seo_description)
        .bind(now)
        .execute(self.db.pool())
        .await?;
        
        self.find_by_id(id).await?.ok_or_else(||
            crate::Error::Database(sqlx::Error::RowNotFound)
        )
    }
    
    async fn update(&self, id: Uuid, request: UpdateProductRequest) -> Result<crate::models::Product> {
        let mut query = String::from("UPDATE products SET ");
        let mut sets = Vec::new();
        
        if let Some(title) = request.title {
            sets.push(format!("title = '{}'", title.replace("'", "''")));
        }
        if let Some(slug) = request.slug {
            sets.push(format!("slug = '{}'", slug.replace("'", "''")));
        }
        if let Some(price) = request.price {
            sets.push(format!("price = {}", price));
        }
        if let Some(is_active) = request.is_active {
            sets.push(format!("is_active = {}", if is_active { 1 } else { 0 }));
        }
        
        if sets.is_empty() {
            return Err(crate::Error::Validation("No fields to update".to_string()));
        }
        
        sets.push(format!("updated_at = '{}'", chrono::Utc::now().to_rfc3339()));
        query.push_str(&sets.join(", "));
        query.push_str(&format!(" WHERE id = '{}'", id));
        
        sqlx::query(&query)
            .execute(self.db.pool())
            .await?;
        
        self.find_by_id(id).await?.ok_or_else(||
            crate::Error::Database(sqlx::Error::RowNotFound)
        )
    }
    
    async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?1")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
    
    async fn find_variants(&self, product_id: Uuid) -> Result<Vec<crate::models::ProductVariant>> {
        // SQLite variants - simplified for now
        let _variants: Vec<ProductVariant> = sqlx::query_as::<_, ProductVariant>(
            "SELECT * FROM product_variants WHERE product_id = ?1 ORDER BY created_at"
        )
        .bind(product_id.to_string())
        .fetch_all(self.db.pool())
        .await?;
        
        // TODO: Convert to standard ProductVariant
        Ok(Vec::new())
    }
    
    async fn find_images(&self, product_id: Uuid) -> Result<Vec<crate::models::ProductImage>> {
        // SQLite images - simplified for now
        let _images: Vec<ProductImage> = sqlx::query_as::<_, ProductImage>(
            "SELECT * FROM product_images WHERE product_id = ?1 ORDER BY position"
        )
        .bind(product_id.to_string())
        .fetch_all(self.db.pool())
        .await?;
        
        // TODO: Convert to standard ProductImage
        Ok(Vec::new())
    }
}
