//! JSON file importer

use crate::import::{
    error::{ImportError, ImportResult},
    types::{ImportConfig, ImportProgress, ImportStats},
    EntityType, FileImporter,
};
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;

/// JSON file importer
pub struct JsonImporter;

impl JsonImporter {
    pub fn new() -> Self {
        Self
    }

    /// Extract array from JSON value
    fn extract_array(value: Value) -> ImportResult<Vec<Value>> {
        match value {
            Value::Array(arr) => Ok(arr),
            Value::Object(obj) => {
                // Try to find an array field
                for (_, v) in obj {
                    if let Value::Array(arr) = v {
                        return Ok(arr);
                    }
                }
                Err(ImportError::Parse(
                    "JSON object does not contain an array field".to_string(),
                ))
            }
            _ => Err(ImportError::Parse(
                "JSON root must be an array or object containing an array".to_string(),
            )),
        }
    }
}

impl Default for JsonImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FileImporter for JsonImporter {
    fn format(&self) -> &'static str {
        "json"
    }

    async fn import_file(
        &self,
        file_path: &Path,
        entity_type: EntityType,
        _config: &ImportConfig,
        progress: &(dyn Fn(ImportProgress) + Send + Sync),
    ) -> ImportResult<ImportStats> {
        let content = tokio::fs::read_to_string(file_path).await?;
        let json: Value = serde_json::from_str(&content)?;
        let records = Self::extract_array(json)?;
        let total = records.len();

        progress(ImportProgress {
            stage: entity_type.to_string(),
            current: 0,
            total,
            message: format!("Importing {} {} records from JSON...", total, entity_type),
        });

        let mut stats = ImportStats {
            total,
            ..Default::default()
        };

        for (i, record) in records.iter().enumerate() {
            progress(ImportProgress {
                stage: entity_type.to_string(),
                current: i + 1,
                total,
                message: format!("Processing record {}/{}", i + 1, total),
            });

            // Validate record structure based on entity type
            let is_valid = match entity_type {
                EntityType::Products => {
                    record.get("title").is_some() && record.get("price").is_some()
                }
                EntityType::Customers => {
                    record.get("email").is_some()
                }
                EntityType::Orders => {
                    record.get("order_number").is_some() || record.get("id").is_some()
                }
            };

            if is_valid {
                // In real implementation, insert into database
                stats.created += 1;
            } else {
                stats.errors += 1;
                stats.error_details.push(format!(
                    "Record {}: Missing required fields",
                    i + 1
                ));
            }
        }

        Ok(stats)
    }
}

/// Example JSON structures for each entity type
pub mod examples {
    use serde_json::json;

    /// Example product JSON structure
    pub fn product() -> serde_json::Value {
        json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "title": "Premium Cotton T-Shirt",
            "slug": "premium-cotton-t-shirt",
            "description": "High-quality organic cotton t-shirt",
            "price": "29.99",
            "compare_at_price": "39.99",
            "sku": "TSHIRT-001",
            "inventory_quantity": 100,
            "status": "active",
            "product_type": "physical",
            "tags": ["clothing", "organic"],
            "variants": [
                {
                    "id": "var-001",
                    "title": "Small / Black",
                    "sku": "TSHIRT-001-S-BLK",
                    "price": "29.99",
                    "inventory_quantity": 50
                }
            ]
        })
    }

    /// Example customer JSON structure
    pub fn customer() -> serde_json::Value {
        json!({
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "email": "customer@example.com",
            "first_name": "John",
            "last_name": "Doe",
            "phone": "+1234567890",
            "addresses": [
                {
                    "address1": "123 Main St",
                    "city": "New York",
                    "state": "NY",
                    "postal_code": "10001",
                    "country": "US"
                }
            ]
        })
    }

    /// Example order JSON structure
    pub fn order() -> serde_json::Value {
        json!({
            "id": "ord-123456",
            "order_number": "1001",
            "customer_id": "123e4567-e89b-12d3-a456-426614174000",
            "email": "customer@example.com",
            "status": "confirmed",
            "total": "59.98",
            "subtotal": "54.99",
            "tax_total": "4.99",
            "shipping_total": "0.00",
            "line_items": [
                {
                    "product_id": "550e8400-e29b-41d4-a716-446655440000",
                    "title": "Premium Cotton T-Shirt",
                    "quantity": 2,
                    "price": "29.99",
                    "total": "59.98"
                }
            ]
        })
    }
}
