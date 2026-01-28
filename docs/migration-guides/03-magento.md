# Magento to R commerce Migration Guide

## Overview

Magento has the most complex database structure of any major ecommerce platform due to its EAV (Entity-Attribute-Value) model, multiple store views, and extensive extension ecosystem. This guide covers migrating from Magento 2.x (Open Source or Commerce) to R commerce.

## Pre-Migration Analysis

### Magento Store Audit

```bash
# Using Magento CLI (bin/magento)
php bin/magento info:adminuri
php bin/magento cache:status
php bin/magento indexer:status

# Get store information
php bin/magento config:show

# List all modules
php bin/magento module:status
```

**From MySQL:**

```sql
-- Connect to Magento database
mysql -u magento -p magento_db

-- Count entities by type
-- Products
SELECT COUNT(*) AS product_count FROM catalog_product_entity WHERE type_id = 'simple';
SELECT COUNT(*) AS configurable_count FROM catalog_product_entity WHERE type_id = 'configurable';
SELECT COUNT(*) AS bundle_count FROM catalog_product_entity WHERE type_id = 'bundle';
SELECT COUNT(*) AS grouped_count FROM catalog_product_entity WHERE type_id = 'grouped';

-- Customers
SELECT COUNT(*) AS customer_count FROM customer_entity;
SELECT COUNT(DISTINCT email) AS unique_emails FROM customer_entity;

-- Orders
SELECT COUNT(*) AS order_count FROM sales_order;
SELECT COUNT(*) AS invoice_count FROM sales_invoice;
SELECT COUNT(*) AS shipment_count FROM sales_shipment;

-- Categories
SELECT COUNT(*) AS category_count FROM catalog_category_entity;

-- Store Views
SELECT COUNT(*) AS store_count FROM store;
SELECT COUNT(*) AS website_count FROM store_website;

-- Attributes
SELECT COUNT(*) AS attribute_count FROM eav_attribute WHERE entity_type_id = 4; -- catalog_product

-- Extensions
SELECT COUNT(*) FROM setup_module WHERE module LIKE 'Company_%' OR module LIKE 'Vendor_%';
```

**Complex Query to Analyze Product Structure:**

```sql
-- Analyze configurable product complexity
SELECT 
  cpe.sku,
  COUNT(cpe_child.entity_id) AS child_count,
  COUNT(DISTINCT ea.attribute_code) AS attribute_count
FROM catalog_product_entity cpe
JOIN catalog_product_super_link cpsl ON cpe.entity_id = cpsl.parent_id
JOIN catalog_product_entity cpe_child ON cpsl.product_id = cpe_child.entity_id
JOIN catalog_product_entity_int cpei ON cpe_child.entity_id = cpei.entity_id
JOIN eav_attribute ea ON cpei.attribute_id = ea.attribute_id
WHERE cpe.type_id = 'configurable'
GROUP BY cpe.entity_id
ORDER BY child_count DESC
LIMIT 20;

-- Analyze attribute value distribution
SELECT 
  ea.attribute_code,
  ea.frontend_label,
  COUNT(DISTINCT cpei.value) AS unique_values,
  COUNT(cpei.value) AS total_values
FROM eav_attribute ea
JOIN catalog_product_entity_int cpei ON ea.attribute_id = cpei.attribute_id
WHERE ea.entity_type_id = 4
GROUP BY ea.attribute_id
ORDER BY unique_values DESC
LIMIT 20;
```

## Magento Data Structure Understanding

### EAV Model Overview

Magento's EAV model makes querying complex:

```
catalog_product_entity (main table)
  ├── entity_id
  ├── sku
  ├── type_id (simple, configurable, bundle, etc.)
  ├── attribute_set_id
  └── [few other fields]

catalog_product_entity_varchar (for varchar attributes)
  ├── value_id
  ├── entity_id (FK to main table)
  ├── attribute_id (FK to eav_attribute)
  ├── store_id
  └── value

catalog_product_entity_int (for integer attributes)
catalog_product_entity_decimal (for decimal attributes)
catalog_product_entity_text (for text attributes)
catalog_product_entity_datetime (for datetime attributes)

eav_attribute (defines all attributes)
  ├── attribute_id
  ├── entity_type_id (4 = catalog_product)
  ├── attribute_code
  ├── backend_type (varchar, int, decimal, text, datetime)
  ├── frontend_label
  └── [many other columns]
```

### Querying a Product in Magento

```sql
-- Get all data for a single product requires joining multiple tables
SET @entity_id = 1234;

SELECT 
  e.entity_id,
  e.sku,
  e.type_id,
  e.sku AS name,
  
  -- Get name (varchar attribute)
  (SELECT pv.value 
   FROM catalog_product_entity_varchar pv 
   WHERE pv.entity_id = e.entity_id 
     AND pv.attribute_id = (SELECT attribute_id FROM eav_attribute WHERE attribute_code = 'name' AND entity_type_id = 4)
     AND pv.store_id = 0
  ) AS name,
  
  -- Get price (decimal attribute)
  (SELECT pd.value 
   FROM catalog_product_entity_decimal pd 
   WHERE pd.entity_id = e.entity_id 
     AND pd.attribute_id = (SELECT attribute_id FROM eav_attribute WHERE attribute_code = 'price' AND entity_type_id = 4)
     AND pd.store_id = 0
  ) AS price,
  
  -- Get description (text attribute)
  (SELECT pt.value 
   FROM catalog_product_entity_text pt 
   WHERE pt.entity_id = e.entity_id 
     AND pt.attribute_id = (SELECT attribute_id FROM eav_attribute WHERE attribute_code = 'description' AND entity_type_id = 4)
     AND pt.store_id = 0
  ) AS description

FROM catalog_product_entity e
WHERE e.entity_id = @entity_id;
```

### Understanding Attribute Sets

Magento organizes attributes into "attribute sets":

```sql
-- List all attribute sets
SELECT 
  eas.attribute_set_id,
  eas.attribute_set_name,
  eet.entity_type_code
FROM eav_attribute_set eas
JOIN eav_entity_type eet ON eas.entity_type_id = eet.entity_type_id
WHERE eet.entity_type_code = 'catalog_product';

-- Default attribute sets:
-- 4: Default
-- 9: Bag
-- 10: Bottom (for pants)
-- 11: Gear
-- 12: Sprite (for t-shirts)
-- 13: Top (for shirts)
```

## Export Strategy

### Method 1: Magento Data Migration Tool Approach

The official Magento approach uses migration scripts:

```bash
# Install Magento Data Migration Tool
composer require magento/data-migration-tool:2.4.x

# Configure migration
php bin/magento migrate:settings --reset vendor/magento/data-migration-tool/etc/opensource-to-opensource/1.9.4.5/config.xml
```

### Method 2: Direct Database Export with EAV Handling

```sql
-- Comprehensive product export query that handles EAV
-- This is extremely complex due to Magento's structure

SET @sql = NULL;

SELECT
  GROUP_CONCAT(DISTINCT
    CONCAT(
      'MAX(IF(ea.attribute_code = ''',
      ea.attribute_code,
      ''', pv.value, NULL)) AS ',
      ea.attribute_code
    )
  ) INTO @sql
FROM eav_attribute ea
WHERE ea.entity_type_id = 4
  AND ea.attribute_code IN (
    'name', 'price', 'description', 'short_description',
    'sku', 'weight', 'status', 'visibility', 'tax_class_id',
    'meta_title', 'meta_description', 'url_key'
  );

SET @sql = CONCAT(
'SELECT 
  e.entity_id,
  e.sku,
  e.type_id,
  e.attribute_set_id,
  ', @sql, '
FROM catalog_product_entity e
LEFT JOIN eav_attribute ea ON ea.entity_type_id = 4
LEFT JOIN catalog_product_entity_varchar pv ON e.entity_id = pv.entity_id 
  AND ea.attribute_id = pv.attribute_id 
  AND pv.store_id = 0
WHERE e.type_id = ''simple''
GROUP BY e.entity_id'
);

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;
```

### Method 3: Magento API Export

```bash
#!/bin/bash
# export-magento-api.sh

MAGENTO_URL="https://your-magento-store.com"
ACCESS_TOKEN="YOUR_INTEGRATION_ACCESS_TOKEN"

# Create export directory
mkdir -p magento-export

# Get products using Magento REST API
curl -X GET "$MAGENTO_URL/rest/V1/products" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"searchCriteria":{"currentPage":1,"pageSize":100}}' \
  > magento-export/products-page-1.json

# Pagination for large catalogs
page=1
while true; do
  echo "Fetching page $page..."
  
  response=$(curl -s -X GET "$MAGENTO_URL/rest/V1/products" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json" \
    -d "{\"searchCriteria\":{\"currentPage\":$page,\"pageSize\":100}}")
  
  total_count=$(echo $response | jq '.total_count')
  items_count=$(echo $response | jq '.items | length')
  
  if [ "$items_count" -eq 0 ]; then
    break
  fi
  
  echo $response > magento-export/products-page-$page.json
  page=$((page + 1))
  
  # Magento API rate limiting
  sleep 2
done
```

## Advanced Python Migration Script

```python
#!/usr/bin/env python3
# migrate-magento.py

import mysql.connector
import requests
import json
import os
import sys
import time
from typing import List, Dict, Optional
import xml.etree.ElementTree as ET

class MagentoEAVHelper:
    """Helper to handle Magento's EAV structure"""
    
    def __init__(self, db_connection):
        self.db = db_connection
        self.attributes = {}
        self._load_attributes()
    
    def _load_attributes(self):
        """Cache all product attributes"""
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT 
                attribute_id,
                attribute_code,
                backend_type,
                frontend_label,
                frontend_input
            FROM eav_attribute 
            WHERE entity_type_id = 4
        """)
        
        for row in cursor:
            self.attributes[row['attribute_code']] = row
            self.attributes[row['attribute_id']] = row
        
        cursor.close()
    
    def get_product_attribute(self, product_id: int, attribute_code: str, store_id: int = 0):
        """Get a single product attribute value"""
        if attribute_code not in self.attributes:
            return None
        
        attr = self.attributes[attribute_code]
        backend_type = attr['backend_type']
        table_map = {
            'varchar': 'catalog_product_entity_varchar',
            'int': 'catalog_product_entity_int',
            'decimal': 'catalog_product_entity_decimal',
            'text': 'catalog_product_entity_text',
            'datetime': 'catalog_product_entity_datetime'
        }
        
        if backend_type not in table_map:
            return None
        
        table = table_map[backend_type]
        
        cursor = self.db.cursor()
        cursor.execute(f"""
            SELECT value FROM {table}
            WHERE entity_id = %s 
              AND attribute_id = %s 
              AND store_id = %s
        """, (product_id, attr['attribute_id'], store_id))
        
        result = cursor.fetchone()
        cursor.close()
        
        return result[0] if result else None
    
    def get_all_product_attributes(self, product_id: int, store_id: int = 0):
        """Get all attributes for a product"""
        attributes = {}
        
        # Query all attribute tables
        tables = [
            'catalog_product_entity_varchar',
            'catalog_product_entity_int',
            'catalog_product_entity_decimal',
            'catalog_product_entity_text',
            'catalog_product_entity_datetime'
        ]
        
        for table in tables:
            cursor = self.db.cursor(dictionary=True)
            cursor.execute(f"""
                SELECT attribute_id, value FROM {table}
                WHERE entity_id = %s AND store_id = %s
            """, (product_id, store_id))
            
            for row in cursor:
                if row['attribute_id'] in self.attributes:
                    attr_code = self.attributes[row['attribute_id']]['attribute_code']
                    attributes[attr_code] = row['value']
            
            cursor.close()
        
        return attributes

class MagentoMigrator:
    def __init__(self, db_config, rcommerce_config):
        self.db = mysql.connector.connect(**db_config)
        self.eav_helper = MagentoEAVHelper(self.db)
        self.rcommerce_url = rcommerce_config['url']
        self.rcommerce_key = rcommerce_config['api_key']
        self.migration_log = []
        self.store_mapping = {}
        self.attribute_set_mapping = {}
    
    def migrate_all(self):
        """Execute complete Magento to R commerce migration"""
        print("Starting Magento to R commerce migration...")
        print(f"Database: {self.db.server_host}")
        print(f"R commerce: {self.rcommerce_url}")
        
        try:
            # Pre-migration analysis
            self.analyze_magento_structure()
            
            # Phase 1: Store configuration
            print("\n=== Phase 1: Store Configuration ===")
            self.migrate_store_configuration()
            
            # Phase 2: Categories
            print("\n=== Phase 2: Categories ===")
            self.migrate_categories()
            
            # Phase 3: Attributes (as product metafields)
            print("\n=== Phase 3: Product Attributes ===")
            self.migrate_attributes()
            
            # Phase 4: Simple products first
            print("\n=== Phase 4: Simple Products ===")
            self.migrate_products('simple')
            
            # Phase 5: Configurable products
            print("\n=== Phase 5: Configurable Products ===")
            self.migrate_products('configurable')
            
            # Phase 6: Other product types
            print("\n=== Phase 6: Bundle and Grouped Products ===")
            self.migrate_products('bundle')
            self.migrate_products('grouped')
            
            # Phase 7: Customers
            print("\n=== Phase 7: Customers ===")
            self.migrate_customers()
            
            # Phase 8: Orders (optional)
            if os.environ.get('MIGRATE_ORDERS'):
                print("\n=== Phase 8: Orders ===")
                self.migrate_orders()
            
            # Save log
            self.save_migration_log()
            
            print("\n Migration completed!")
            self.print_summary()
            
        except Exception as e:
            print(f"\n Migration failed: {e}")
            import traceback
            traceback.print_exc()
            sys.exit(1)
        
        finally:
            self.db.close()
    
    def analyze_magento_structure(self):
        """Analyze Magento store structure"""
        print("Analyzing Magento structure...")
        
        cursor = self.db.cursor(dictionary=True)
        
        # Count products by type
        cursor.execute("""
            SELECT type_id, COUNT(*) as count
            FROM catalog_product_entity
            GROUP BY type_id
        """)
        
        product_types = cursor.fetchall()
        print("Product types found:")
        for pt in product_types:
            print(f"  - {pt['type_id']}: {pt['count']} products")
        
        # Count stores
        cursor.execute("SELECT COUNT(*) as count FROM store")
        store_count = cursor.fetchone()['count']
        print(f"Store count: {store_count}")
        
        # Count customer groups
        cursor.execute("SELECT COUNT(*) as count FROM customer_group")
        customer_group_count = cursor.fetchone()['count']
        print(f"Customer group count: {customer_group_count}")
        
        # Count attributes
        cursor.execute("""
            SELECT backend_type, COUNT(*) as count
            FROM eav_attribute
            WHERE entity_type_id = 4
            GROUP BY backend_type
        """)
        attributes = cursor.fetchall()
        print("Product attributes:")
        for attr in attributes:
            print(f"  - {attr['backend_type']}: {attr['count']} attributes")
        
        cursor.close()
    
    def migrate_store_configuration(self):
        """Migrate store/website configuration as metadata"""
        cursor = self.db.cursor(dictionary=True)
        
        # Get all stores
        cursor.execute("""
            SELECT 
                s.store_id,
                s.name as store_name,
                s.code as store_code,
                s.website_id,
                w.name as website_name,
                w.code as website_code,
                g.name as group_name
            FROM store s
            JOIN store_website w ON s.website_id = w.website_id
            JOIN store_group g ON s.group_id = g.group_id
        """)
        
        stores = cursor.fetchall()
        
        for store in stores:
            # Map store configuration
            store_config = {
                'store_id': store['store_id'],
                'store_name': store['store_name'],
                'store_code': store['store_code'],
                'website_id': store['website_id'],
                'website_name': store['website_name'],
                'website_code': store['website_code'],
                'group_name': store['group_name']
            }
            
            # In R commerce, store this at system level
            self.migration_log.append({
                'type': 'store',
                'operation': 'map',
                'status': 'success',
                'source_id': store['store_id'],
                'config': store_config
            })
            
            print(f" Mapped store: {store['store_name']} ({store['store_code']})")
        
        cursor.close()
    
    def migrate_categories(self):
        """Migrate Magento categories"""
        cursor = self.db.cursor(dictionary=True)
        
        # Magento stores categories in nested set model
        cursor.execute("""
            SELECT 
                entity_id as category_id,
                parent_id,
                level,
                path,
                position
            FROM catalog_category_entity
            ORDER BY level, position
        """)
        
        categories = cursor.fetchall()
        
        # Get category names from EAV
        for category in categories:
            name = self.eav_helper.get_product_attribute(
                category['category_id'], 
                'name', 
                entity_type='catalog_category'
            )
            
            description = self.eav_helper.get_product_attribute(
                category['category_id'], 
                'description', 
                entity_type='catalog_category'
            )
            
            if name:
                try:
                    category_data = {
                        'name': name,
                        'slug': self.generateSlug(name),
                        'description': description or '',
                        'meta_data': {
                            'magento': {
                                'category_id': category['category_id'],
                                'parent_id': category['parent_id'],
                                'level': category['level'],
                                'path': category['path'],
                                'position': category['position']
                            }
                        }
                    }
                    
                    # Create category in R commerce
                    response = requests.post(
                        f"{self.rcommerce_url}/v1/categories",
                        json=category_data,
                        headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                    )
                    
                    if response.status_code == 201:
                        print(f" Migrated category: {name}")
                        self.migration_log.append({
                            'type': 'category',
                            'operation': 'create',
                            'status': 'success',
                            'source_id': category['category_id'],
                            'target_id': response.json()['data']['id'],
                            'name': name
                        })
                    else:
                        print(f" Failed to migrate category {name}: {response.text}")
                        self.migration_log.append({
                            'type': 'category',
                            'operation': 'create',
                            'status': 'failed',
                            'source_id': category['category_id'],
                            'name': name,
                            'error': response.text
                        })
                    
                    # Rate limiting
                    time.sleep(0.5)
                    
                except Exception as e:
                    print(f" Error migrating category {name}: {e}")
        
        cursor.close()
    
    def migrate_attributes(self):
        """Migrate Magento attributes as product metafields"""
        cursor = self.db.cursor(dictionary=True)
        
        # Get all product attributes
        cursor.execute("""
            SELECT 
                attribute_id,
                attribute_code,
                frontend_label,
                backend_type,
                frontend_input,
                is_required,
                is_user_defined
            FROM eav_attribute
            WHERE entity_type_id = 4
              AND is_user_defined = 1
            ORDER BY attribute_id
        """)
        
        attributes = cursor.fetchall()
        
        for attribute in attributes:
            try:
                # We'll store these as a metadata schema definition
                attribute_definition = {
                    'attribute_id': attribute['attribute_id'],
                    'attribute_code': attribute['attribute_code'],
                    'frontend_label': attribute['frontend_label'],
                    'backend_type': attribute['backend_type'],
                    'frontend_input': attribute['frontend_input'],
                    'is_required': attribute['is_required'],
                    'is_user_defined': attribute['is_user_defined']
                }
                
                # Store in migration log for reference
                self.migration_log.append({
                    'type': 'attribute',
                    'operation': 'map',
                    'status': 'success',
                    'attribute_code': attribute['attribute_code'],
                    'definition': attribute_definition
                })
                
                print(f" Mapped attribute: {attribute['attribute_code']}")
                
            except Exception as e:
                print(f" Error mapping attribute {attribute['attribute_code']}: {e}")
        
        cursor.close()
    
    def migrate_products(self, product_type: str):
        """Migrate products of a specific type"""
        cursor = self.db.cursor(dictionary=True)
        
        cursor.execute("""
            SELECT 
                e.entity_id,
                e.sku,
                e.type_id,
                e.attribute_set_id
            FROM catalog_product_entity e
            WHERE e.type_id = %s
            ORDER BY e.entity_id
        """, (product_type,))
        
        products = cursor.fetchall()
        
        for product in products:
            try:
                # Get all attributes for this product
                attributes = self.eav_helper.get_all_product_attributes(product['entity_id'])
                
                # Transform based on product type
                if product_type == 'simple':
                    product_data = self.transform_simple_product(product, attributes)
                elif product_type == 'configurable':
                    product_data = self.transform_configurable_product(product, attributes)
                elif product_type == 'bundle':
                    product_data = self.transform_bundle_product(product, attributes)
                elif product_type == 'grouped':
                    product_data = self.transform_grouped_product(product, attributes)
                else:
                    print(f"⊘ Skipping unsupported type: {product_type}")
                    continue
                
                # Create in R commerce
                response = requests.post(
                    f"{self.rcommerce_url}/v1/products",
                    json=product_data,
                    headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                )
                
                if response.status_code == 201:
                    print(f" Migrated {product_type} product: {attributes.get('name', product['sku'])}")
                    self.migration_log.append({
                        'type': 'product',
                        'product_type': product_type,
                        'operation': 'create',
                        'status': 'success',
                        'source_id': product['entity_id'],
                        'sku': product['sku'],
                        'target_id': response.json()['data']['id'],
                        'name': attributes.get('name', product['sku'])
                    })
                else:
                    print(f" Failed to migrate {product_type} product {product['sku']}: {response.text}")
                    self.migration_log.append({
                        'type': 'product',
                        'product_type': product_type,
                        'operation': 'create',
                        'status': 'failed',
                        'source_id': product['entity_id'],
                        'sku': product['sku'],
                        'name': attributes.get('name', product['sku']),
                        'error': response.text
                    })
                
                # Rate limiting
                time.sleep(0.5)
                
            except Exception as e:
                print(f" Error migrating {product_type} product {product['sku']}: {e}")
        
        cursor.close()
    
    def transform_simple_product(self, product: Dict, attributes: Dict) -> Dict:
        """Transform simple product"""
        return {
            'name': attributes.get('name', f"Product {product['sku']}"),
            'slug': attributes.get('url_key') or self.generate_slug(attributes.get('name', '')),
            'description': attributes.get('description', ''),
            'short_description': attributes.get('short_description', ''),
            'sku': product['sku'],
            'price': float(attributes.get('price', 0) or 0),
            'compare_at_price': float(attributes.get('msrp', 0) or 0) if attributes.get('msrp') else None,
            'cost': float(attributes.get('cost', 0) or 0) if attributes.get('cost') else None,
            'inventory_quantity': int(attributes.get('qty', 0) or 0),
            'inventory_policy': 'deny' if attributes.get('is_in_stock') == '0' else 'continue',
            'weight': float(attributes.get('weight', 0) or 0) if attributes.get('weight') else None,
            'status': 'active' if attributes.get('status') == '1' else 'draft',
            'is_taxable': attributes.get('tax_class_id') != '0',
            'requires_shipping': attributes.get('weight') and float(attributes.get('weight', 0)) > 0,
            'meta_data': {
                'magento': {
                    'entity_id': product['entity_id'],
                    'type_id': product['type_id'],
                    'attribute_set_id': product['attribute_set_id'],
                    'visibility': attributes.get('visibility'),
                    'tax_class_id': attributes.get('tax_class_id'),
                    'is_salable': attributes.get('is_salable'),
                    'stock_status': attributes.get('stock_status')
                },
                'original_attributes': attributes
            }
        }
    
    def transform_configurable_product(self, product: Dict, attributes: Dict) -> Dict:
        """Transform configurable product and its variants"""
        # Get configurable product information
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT 
                cpsa.product_super_attribute_id,
                cpsa.attribute_id,
                ea.attribute_code
            FROM catalog_product_super_attribute cpsa
            JOIN eav_attribute ea ON cpsa.attribute_id = ea.attribute_id
            WHERE cpsa.product_id = %s
        """, (product['entity_id'],))
        
        configurable_attributes = cursor.fetchall()
        cursor.close()
        
        # Get all simple products associated with this configurable
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT 
                cpsl.product_id as child_id,
                cpsl.parent_id
            FROM catalog_product_super_link cpsl
            WHERE cpsl.parent_id = %s
        """, (product['entity_id'],))
        
        child_products = cursor.fetchall()
        cursor.close()
        
        # Transform configurable as main product
        main_product = self.transform_simple_product(product, attributes)
        main_product['options'] = []
        main_product['variants'] = []
        
        # Add configurable options
        for config_attr in configurable_attributes:
            # Get attribute options
            cursor = self.db.cursor(dictionary=True)
            cursor.execute("""
                SELECT 
                  eaov.option_id,
                  eaov.value
                FROM eav_attribute_option eao
                JOIN eav_attribute_option_value eaov ON eao.option_id = eaov.option_id
                WHERE eao.attribute_id = %s
                  AND eaov.store_id = 0
            """, (config_attr['attribute_id'],))
            
            options = cursor.fetchall()
            cursor.close()
            
            main_product['options'].append({
                'name': config_attr['attribute_code'],
                'position': 0,
                'values': [opt['value'] for opt in options]
            })
        
        # Transform child products as variants
        for child in child_products:
            child_attributes = self.eav_helper.get_all_product_attributes(child['child_id'])
            variant = self.transform_simple_product(
                {'entity_id': child['child_id'], 'sku': child_attributes.get('sku'), 'type_id': 'simple'},
                child_attributes
            )
            
            # Override parent values
            variant['product_id'] = product['entity_id']  # Parent product ID
            variant['options'] = {}
            
            # Extract variant option values
            for config_attr in configurable_attributes:
                attr_code = config_attr['attribute_code']
                if attr_code in child_attributes:
                    variant['options'][attr_code] = child_attributes[attr_code]
            
            main_product['variants'].append(variant)
        
        return main_product
    
    def transform_bundle_product(self, product: Dict, attributes: Dict) -> Dict:
        """Transform bundle product (R commerce doesn't support bundles natively, map to special product type)"""
        # Bundle products in Magento are complex - we'll create them as regular products
        # with bundle information stored in meta_data for future bundle implementation
        
        main_product = self.transform_simple_product(product, attributes)
        
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT 
                bpo.option_id,
                bpo.parent_id,
                bpo.required,
                bpo.type,
                bpo.position,
                bps.selection_id,
                bps.product_id as child_product_id,
                bps.selection_price_type,
                bps.selection_price_value,
                bps.selection_qty,
                bps.selection_can_change_qty,
                bps.position as selection_position
            FROM catalog_product_bundle_option bpo
            LEFT JOIN catalog_product_bundle_selection bps ON bpo.option_id = bps.option_id
            WHERE bpo.parent_id = %s
        """, (product['entity_id'],))
        
        bundle_options = cursor.fetchall()
        cursor.close()
        
        # Store bundle information in meta_data
        main_product['meta_data']['magento']['bundle_options'] = bundle_options
        main_product['meta_data']['product_type_note'] = 'Bundle product - options stored in meta_data'
        
        return main_product
    
    def transform_grouped_product(self, product: Dict, attributes: Dict) -> Dict:
        """Transform grouped product (container for simple products)"""
        main_product = self.transform_simple_product(product, attributes)
        
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT linked_product_id
            FROM catalog_product_link
            WHERE product_id = %s AND link_type_id = 3
        """, (product['entity_id'],))
        
        grouped_products = cursor.fetchall()
        cursor.close()
        
        main_product['meta_data']['magento']['grouped_children'] = [p['linked_product_id'] for p in grouped_products]
        main_product['meta_data']['product_type_note'] = 'Grouped product - child products stored in meta_data'
        
        return main_product
    
    def generate_slug(self, name: str) -> str:
        """Generate URL-friendly slug"""
        import re
        return re.sub(r'[^a-z0-9]+', '-', name.lower()).strip('-')
    
    def getProductGallery(self, product_id: int):
        """Get product gallery images"""
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT 
                gallery.value_id,
                gallery.value as image_path,
                gallery.media_type,
                gallery.position,
                gallery.disabled,
                gallery.label
            FROM catalog_product_entity_media_gallery gallery
            JOIN catalog_product_entity_media_gallery_value_to_entity entity_link 
                ON gallery.value_id = entity_link.value_id
            WHERE entity_link.entity_id = %s
            ORDER BY gallery.position
        """, (product_id,))
        
        gallery = cursor.fetchall()
        cursor.close()
        
        return gallery
    
    def migrate_customers(self):
        """Migrate Magento customers"""
        cursor = self.db.cursor(dictionary=True)
        
        cursor.execute("""
            SELECT 
                e.entity_id,
                e.email,
                e.created_at,
                e.group_id,
                cg.customer_group_code
            FROM customer_entity e
            JOIN customer_group cg ON e.group_id = cg.customer_group_id
            ORDER BY e.entity_id
        """)
        
        customers = cursor.fetchall()
        
        for customer in customers:
            try:
                # Get customer attributes (firstname, lastname, etc.)
                attributes = self.eav_helper.get_all_product_attributes(customer['entity_id'], entity_type='customer')
                
                # Get customer addresses
                addresses = self.get_customer_addresses(customer['entity_id'])
                
                customer_data = {
                    'email': customer['email'],
                    'first_name': attributes.get('firstname', ''),
                    'last_name': attributes.get('lastname', ''),
                    'phone': attributes.get('phone', None),
                    'accepts_marketing': attributes.get('is_subscribed', False),
                    'meta_data': {
                        'magento': {
                            'entity_id': customer['entity_id'],
                            'group_id': customer['group_id'],
                            'customer_group_code': customer['customer_group_code'],
                            'created_at': customer['created_at'],
                            'original_attributes': attributes
                        }
                    },
                    'addresses': addresses
                }
                
                # Create in R commerce
                response = requests.post(
                    f"{self.rcommerce_url}/v1/customers",
                    json=customer_data,
                    headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                )
                
                if response.status_code == 201:
                    print(f" Migrated customer: {customer['email']}")
                    self.migration_log.append({
                        'type': 'customer',
                        'operation': 'create',
                        'status': 'success',
                        'source_id': customer['entity_id'],
                        'target_id': response.json()['data']['id'],
                        'email': customer['email']
                    })
                else:
                    print(f" Failed to migrate customer {customer['email']}: {response.text}")
                    self.migration_log.append({
                        'type': 'customer',
                        'operation': 'create',
                        'status': 'failed',
                        'source_id': customer['entity_id'],
                        'email': customer['email'],
                        'error': response.text
                    })
                
                # Rate limiting
                time.sleep(0.5)
                
            except Exception as e:
                print(f" Error migrating customer {customer['email']}: {e}")
        
        cursor.close()
    
    def get_customer_addresses(self, customer_id: int) -> List[Dict]:
        """Get all customer addresses"""
        cursor = self.db.cursor(dictionary=True)
        
        cursor.execute("""
            SELECT 
                ea.entity_id as address_id,
                ea.parent_id as customer_id,
                ea.is_active
            FROM customer_address_entity ea
            WHERE ea.parent_id = %s
        """, (customer_id,))
        
        addresses = cursor.fetchall()
        address_list = []
        
        for address in addresses:
            # Get address attributes
            addr_attrs = self.eav_helper.get_all_product_attributes(address['address_id'], entity_type='customer_address')
            
            address_list.append({
                'first_name': addr_attrs.get('firstname', ''),
                'last_name': addr_attrs.get('lastname', ''),
                'company': addr_attrs.get('company'),
                'street1': addr_attrs.get('street'),
                'street2': None,  # Magento combines street lines
                'city': addr_attrs.get('city'),
                'state': addr_attrs.get('region'),
                'postal_code': addr_attrs.get('postcode'),
                'country': addr_attrs.get('country_id'),
                'phone': addr_attrs.get('telephone'),
                'is_default': addr_attrs.get('default_billing') == '1' or addr_attrs.get('default_shipping') == '1',
                'meta_data': {
                    'magento': {
                        'address_id': address['address_id'],
                        'address_type': addr_attrs.get('address_type'),
                        'vat_id': addr_attrs.get('vat_id')
                    }
                }
            })
        
        cursor.close()
        return address_list
    
    def migrate_orders(self):
        """Migrate Magento orders"""
        # This is a simplified version - real order migration is much more complex
        cursor = self.db.cursor(dictionary=True)
        
        cursor.execute("""
            SELECT 
                o.entity_id as order_id,
                o.increment_id as order_number,
                o.customer_email,
                o.customer_firstname,
                o.customer_lastname,
                o.store_id,
                o.created_at,
                o.updated_at,
                o.status,
                o.state,
                o.grand_total,
                o.subtotal,
                o.tax_amount,
                o.shipping_amount,
                o.discount_amount,
                o.total_qty_ordered,
                o.shipping_method,
                o.shipping_description
            FROM sales_order o
            ORDER BY o.entity_id
            LIMIT 1000  -- Limit for testing
        """)
        
        orders = cursor.fetchall()
        
        for order in orders:
            try:
                # Get order items
                items = self.get_order_items(order['order_id'])
                
                # Get order addresses
                billing_address = self.get_order_address(order['order_id'], 'billing')
                shipping_address = self.get_order_address(order['order_id'], 'shipping')
                
                # Transform order
                order_data = {
                    'order_number': order['order_number'],
                    'customer_email': order['customer_email'],
                    'customer_first_name': order['customer_firstname'],
                    'customer_last_name': order['customer_lastname'],
                    'subtotal': float(order['subtotal'] or 0),
                    'tax_amount': float(order['tax_amount'] or 0),
                    'shipping_amount': float(order['shipping_amount'] or 0),
                    'discount_amount': float(order['discount_amount'] or 0),
                    'total': float(order['grand_total'] or 0),
                    'status': self.mapOrderStatus(order['status']),
                    'billing_address': billing_address,
                    'shipping_address': shipping_address,
                    'line_items': [self.transformOrderItem(item) for item in items],
                    'meta_data': {
                        'magento': {
                            'order_id': order['order_id'],
                            'store_id': order['store_id'],
                            'state': order['state'],
                            'shipping_method': order['shipping_method'],
                            'shipping_description': order['shipping_description'],
                            'total_qty_ordered': order['total_qty_ordered']
                        }
                    }
                }
                
                # Create order in R commerce
                response = requests.post(
                    f"{self.rcommerce_url}/v1/orders",
                    json=order_data,
                    headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                )
                
                if response.status_code == 201:
                    print(f" Migrated order: {order['order_number']}")
                    self.migration_log.append({
                        'type': 'order',
                        'operation': 'create',
                        'status': 'success',
                        'source_id': order['order_id'],
                        'target_id': response.json()['data']['id'],
                        'order_number': order['order_number']
                    })
                else:
                    print(f" Failed to migrate order {order['order_number']}: {response.text}")
                
                time.sleep(0.5)
                
            except Exception as e:
                print(f" Error migrating order {order['order_number']}: {e}")
        
        cursor.close()
    
    def get_order_items(self, order_id: int) -> List[Dict]:
        """Get all items for an order"""
        cursor = self.db.cursor(dictionary=True)
        
        cursor.execute("""
            SELECT 
                oi.item_id,
                oi.product_id,
                oi.product_type,
                oi.sku,
                oi.name,
                oi.qty_ordered,
                oi.price,
                oi.base_price,
                oi.original_price,
                oi.tax_amount,
                oi.discount_amount,
                oi.row_total
            FROM sales_order_item oi
            WHERE oi.order_id = %s
        """, (order_id,))
        
        items = cursor.fetchall()
        cursor.close()
        return items
    
    def get_order_address(self, order_id: int, address_type: str) -> Dict:
        """Get billing or shipping address for an order"""
        address_type_id = 1 if address_type == 'billing' else 2
        
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT 
                ea.entity_id as address_id,
                ea.parent_id
            FROM sales_order_address ea
            WHERE ea.parent_id = %s AND ea.address_type = %s
        """, (order_id, address_type[0].upper()))  # 'B' or 'S'
        
        address_data = cursor.fetchone()
        cursor.close()
        
        if not address_data:
            return {}
        
        # Get address attributes
        addr_attrs = self.eav_helper.get_all_product_attributes(
            address_data['address_id'], 
            entity_type='sales_order_address'
        )
        
        return {
            'first_name': addr_attrs.get('firstname', ''),
            'last_name': addr_attrs.get('lastname', ''),
            'company': addr_attrs.get('company'),
            'street1': addr_attrs.get('street'),
            'street2': None,  # Magento combines
            'city': addr_attrs.get('city'),
            'state': addr_attrs.get('region'),
            'postal_code': addr_attrs.get('postcode'),
            'country': addr_attrs.get('country_id'),
            'phone': addr_attrs.get('telephone')
        }
    
    def mapOrderStatus(self, magento_status: str) -> str:
        """Map Magento order status to R commerce status"""
        status_map = {
            'pending': 'pending',
            'processing': 'processing',
            'complete': 'completed',
            'closed': 'completed',
            'canceled': 'cancelled',
            'holded': 'on_hold',
            'payment_review': 'on_hold',
            'fraud': 'fraud_review'
        }
        return status_map.get(magento_status, 'pending')
    
    def transformOrderItem(self, item: Dict) -> Dict:
        """Transform order item"""
        return {
            'product_id': f"magento_{item['product_id']}",  # Reference to imported product
            'name': item['name'],
            'sku': item['sku'],
            'quantity': float(item['qty_ordered'] or 0),
            'unit_price': float(item['price'] or 0),
            'tax_amount': float(item['tax_amount'] or 0),
            'discount_amount': float(item['discount_amount'] or 0),
            'total': float(item['row_total'] or 0),
            'meta_data': {
                'magento': {
                    'item_id': item['item_id'],
                    'product_type': item['product_type'],
                    'original_price': item['original_price']
                }
            }
        }
    
    def save_migration_log(self):
        """Save complete migration log"""
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        filename = f'magento_migration_log_{timestamp}.json'
        
        summary = {
            'products': {'success': 0, 'failed': 0},
            'categories': {'success': 0, 'failed': 0},
            'customers': {'success': 0, 'failed': 0},
            'orders': {'success': 0, 'failed': 0}
        }
        
        for log_entry in self.migration_log:
            item_type = log_entry['type']
            status = log_entry['status']
            
            if item_type in summary:
                if status == 'success':
                    summary[item_type]['success'] += 1
                else:
                    summary[item_type]['failed'] += 1
        
        with open(filename, 'w') as f:
            json.dump({
                'timestamp': timestamp,
                'summary': summary,
                'details': self.migration_log
            }, f, indent=2)
        
        print(f"Migration log saved to {filename}")
        print(f"Summary: {json.dumps(summary, indent=2)}")
    
    def print_summary(self):
        """Print migration summary"""
        summary = {
            'products': {'success': 0, 'failed': 0},
            'categories': {'success': 0, 'failed': 0},
            'customers': {'success': 0, 'failed': 0},
            'orders': {'success': 0, 'failed': 0}
        }
        
        for log_entry in self.migration_log:
            item_type = log_entry['type']
            status = log_entry['status']
            
            if item_type in summary:
                if status == 'success':
                    summary[item_type]['success'] += 1
                else:
                    summary[item_type]['failed'] += 1
        
        print("\n" + "="*50)
        print("MIGRATION SUMMARY")
        print("="*50)
        for item_type, counts in summary.items():
            total = counts['success'] + counts['failed']
            print(f"{item_type.capitalize()}: {counts['success']}/{total} succeeded")
        print("="*50)

# Usage
if __name__ == '__main__':
    db_config = {
        'host': os.environ.get('MAGENTO_DB_HOST', 'localhost'),
        'user': os.environ.get('MAGENTO_DB_USER', 'magento'),
        'password': os.environ.get('MAGENTO_DB_PASS', 'magento'),
        'database': os.environ.get('MAGENTO_DB_NAME', 'magento'),
        'charset': 'utf8mb4',
        'collation': 'utf8mb4_unicode_ci'
    }
    
    rcommerce_config = {
        'url': os.environ.get('RCOMMERCE_URL', 'https://api.yourstore.com'),
        'api_key': os.environ.get('RCOMMERCE_API_KEY', 'your_api_key')
    }
    
    migrator = MagentoMigrator(db_config, rcommerce_config)
    migrator.migrate_all()
```

## Handling Enterprise Features

If migrating from Magento Commerce (Enterprise):

### Customer Segments

```sql
-- Customer segments (Magento Commerce feature)
SELECT 
    segment_id,
    name,
    description,
    is_active,
    conditions_serialized
FROM customer_segment;

-- Map to R commerce customer groups
```

### CMS Blocks and Pages

```sql
-- CMS content
SELECT 
    block_id,
    title,
    identifier,
    content,
    is_active,
    store_id
FROM cms_block;

SELECT 
    page_id,
    title,
    identifier,
    content_heading,
    content,
    is_active
FROM cms_page;
```

### Advanced Inventory (MSI)

For Magento Commerce with Multiple Source Inventory:

```sql
-- Inventory sources
SELECT * FROM inventory_source;

-- Stock items with source
SELECT 
    ssi.sku,
    iss.source_code,
    iss.quantity,
    iss.status
FROM inventory_source_item iss
JOIN catalog_product_entity pe ON iss.sku = pe.sku
JOIN inventory_stock_item ssi ON pe.entity_id = ssi.product_id;
```

## Post-Migration Checklist

### Critical Verification

```bash
# 1. Verify product counts
mag_product_count=$(mysql -u magento -p -e "SELECT COUNT(*) FROM catalog_product_entity" magento_db)
rc_product_count=$(curl -s -H "Authorization: Bearer $RC_KEY" $RC_URL/v1/products?per_page=1 | jq '.meta.pagination.total')

# 2. Verify categories
mag_category_count=$(mysql -u magento -p -e "SELECT COUNT(*) FROM catalog_category_entity" magento_db)
rc_category_count=$(curl -s -H "Authorization: Bearer $RC_KEY" $RC_URL/v1/categories?per_page=1 | jq '.meta.pagination.total')

# 3. Verify customers
mag_customer_count=$(mysql -u magento -p -e "SELECT COUNT(*) FROM customer_entity" magento_db)
rc_customer_count=$(curl -s -H "Authorization: Bearer $RC_KEY" $RC_URL/v1/customers?per_page=1 | jq '.meta.pagination.total')

echo "Product Count - Magento: $mag_product_count, R commerce: $rc_product_count"
echo "Category Count - Magento: $mag_category_count, R commerce: $rc_category_count"
echo "Customer Count - Magento: $mag_customer_count, R commerce: $rc_customer_count"
```

### R commerce Feature Testing

- [ ] Product search and filtering works
- [ ] Configurable product selection works
- [ ] Categories display correctly
- [ ] Customer login functional
- [ ] Cart and checkout process works
- [ ] Payment gateways configured
- [ ] Shipping methods set up
- [ ] Tax rules applied correctly
- [ ] Email notifications send
- [ ] Order management functional

This migration guide addresses the complexity of Magento's architecture, including EAV handling, multi-store configurations, and enterprise features that require special attention during migration to R commerce.
