use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    Result,
    models::{CreateCustomerRequest, UpdateCustomerRequest},
    models::sqlite::{Customer},
};
use crate::repository::traits::CustomerRepositoryTrait;
use super::SqliteDb;

#[derive(Clone)]
pub struct SqliteCustomerRepository {
    db: SqliteDb,
}

impl SqliteCustomerRepository {
    pub fn new(db: SqliteDb) -> Self {
        Self { db }
    }
    
    fn convert_customer(&self, c: Customer) -> crate::models::Customer {
        c.into()
    }
}

#[async_trait]
impl CustomerRepositoryTrait for SqliteCustomerRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<crate::models::Customer>> {
        let customer: Option<Customer> = sqlx::query_as::<_, Customer>(
            "SELECT * FROM customers WHERE id = ?1"
        )
        .bind(id.to_string())
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(customer.map(|c| self.convert_customer(c)))
    }
    
    async fn find_by_email(&self, email: &str) -> Result<Option<crate::models::Customer>> {
        let customer: Option<Customer> = sqlx::query_as::<_, Customer>(
            "SELECT * FROM customers WHERE email = ?1"
        )
        .bind(email)
        .fetch_optional(self.db.pool())
        .await?;
        
        Ok(customer.map(|c| self.convert_customer(c)))
    }
    
    async fn list(&self) -> Result<Vec<crate::models::Customer>> {
        let customers: Vec<Customer> = sqlx::query_as::<_, Customer>(
            "SELECT * FROM customers ORDER BY created_at DESC"
        )
        .fetch_all(self.db.pool())
        .await?;
        
        Ok(customers.into_iter().map(|c| self.convert_customer(c)).collect())
    }
    
    async fn create(&self, request: CreateCustomerRequest) -> Result<crate::models::Customer> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO customers (id, email, first_name, last_name, phone, accepts_marketing, currency, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
            "#
        )
        .bind(id.to_string())
        .bind(request.email)
        .bind(request.first_name)
        .bind(request.last_name)
        .bind(request.phone)
        .bind(request.accepts_marketing)
        .bind(request.currency)
        .bind(now)
        .execute(self.db.pool())
        .await?;
        
        self.find_by_id(id).await?.ok_or_else(||
            crate::Error::Database(sqlx::Error::RowNotFound)
        )
    }
    
    async fn update(&self, id: Uuid, request: UpdateCustomerRequest) -> Result<crate::models::Customer> {
        let mut query = String::from("UPDATE customers SET ");
        let mut sets = Vec::new();
        
        if let Some(email) = request.email {
            sets.push(format!("email = '{}'", email.replace("'", "''")));
        }
        if let Some(first_name) = request.first_name {
            sets.push(format!("first_name = '{}'", first_name.replace("'", "''")));
        }
        if let Some(last_name) = request.last_name {
            sets.push(format!("last_name = '{}'", last_name.replace("'", "''")));
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
        let result = sqlx::query("DELETE FROM customers WHERE id = ?1")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;
        
        Ok(result.rows_affected() > 0)
    }
}
