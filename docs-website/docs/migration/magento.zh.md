# Magento 迁移到 R Commerce 指南

## 概述

由于其 EAV（实体-属性-值）模型、多个店铺视图和广泛的扩展生态系统，Magento 拥有所有主要电商平台中最复杂的数据库结构。本指南涵盖从 Magento 2.x（开源版或商业版）迁移到 R Commerce。

## 迁移前分析

### Magento 店铺审计

```bash
# 使用 Magento CLI (bin/magento)
php bin/magento info:adminuri
php bin/magento cache:status
php bin/magento indexer:status

# 获取店铺信息
php bin/magento config:show

# 列出所有模块
php bin/magento module:status
```

**从 MySQL：**

```sql
-- 连接到 Magento 数据库
mysql -u magento -p magento_db

-- 按类型统计实体
-- 产品
SELECT COUNT(*) AS product_count FROM catalog_product_entity WHERE type_id = 'simple';
SELECT COUNT(*) AS configurable_count FROM catalog_product_entity WHERE type_id = 'configurable';
SELECT COUNT(*) AS bundle_count FROM catalog_product_entity WHERE type_id = 'bundle';
SELECT COUNT(*) AS grouped_count FROM catalog_product_entity WHERE type_id = 'grouped';

-- 客户
SELECT COUNT(*) AS customer_count FROM customer_entity;
SELECT COUNT(DISTINCT email) AS unique_emails FROM customer_entity;

-- 订单
SELECT COUNT(*) AS order_count FROM sales_order;
SELECT COUNT(*) AS invoice_count FROM sales_invoice;
SELECT COUNT(*) AS shipment_count FROM sales_shipment;

-- 分类
SELECT COUNT(*) AS category_count FROM catalog_category_entity;

-- 店铺视图
SELECT COUNT(*) AS store_count FROM store;
SELECT COUNT(*) AS website_count FROM store_website;

-- 属性
SELECT COUNT(*) AS attribute_count FROM eav_attribute WHERE entity_type_id = 4; -- catalog_product

-- 扩展
SELECT COUNT(*) FROM setup_module WHERE module LIKE 'Company_%' OR module LIKE 'Vendor_%';
```

**分析产品结构的复杂查询：**

```sql
-- 分析可配置产品复杂度
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

-- 分析属性值分布
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

## Magento 数据结构理解

### EAV 模型概述

Magento 的 EAV 模型使查询变得复杂：

```
catalog_product_entity (主表)
  ├── entity_id
  ├── sku
  ├── type_id (simple, configurable, bundle, 等)
  ├── attribute_set_id
  └── [少数其他字段]

catalog_product_entity_varchar (用于 varchar 属性)
  ├── value_id
  ├── entity_id (外键到主表)
  ├── attribute_id (外键到 eav_attribute)
  ├── store_id
  └── value

catalog_product_entity_int (用于 integer 属性)
catalog_product_entity_decimal (用于 decimal 属性)
catalog_product_entity_text (用于 text 属性)
catalog_product_entity_datetime (用于 datetime 属性)

eav_attribute (定义所有属性)
  ├── attribute_id
  ├── entity_type_id (4 = catalog_product)
  ├── attribute_code
  ├── backend_type (varchar, int, decimal, text, datetime)
  ├── frontend_label
  └── [许多其他列]
```

### 在 Magento 中查询产品

```sql
-- 获取单个产品的所有数据需要连接多个表
SET @entity_id = 1234;

SELECT 
  e.entity_id,
  e.sku,
  e.type_id,
  e.sku AS name,
  
  -- 获取 name (varchar 属性)
  (SELECT pv.value 
   FROM catalog_product_entity_varchar pv 
   WHERE pv.entity_id = e.entity_id 
     AND pv.attribute_id = (SELECT attribute_id FROM eav_attribute WHERE attribute_code = 'name' AND entity_type_id = 4)
     AND pv.store_id = 0
  ) AS name,
  
  -- 获取 price (decimal 属性)
  (SELECT pd.value 
   FROM catalog_product_entity_decimal pd 
   WHERE pd.entity_id = e.entity_id 
     AND pd.attribute_id = (SELECT attribute_id FROM eav_attribute WHERE attribute_code = 'price' AND entity_type_id = 4)
     AND pd.store_id = 0
  ) AS price,
  
  -- 获取 description (text 属性)
  (SELECT pt.value 
   FROM catalog_product_entity_text pt 
   WHERE pt.entity_id = e.entity_id 
     AND pt.attribute_id = (SELECT attribute_id FROM eav_attribute WHERE attribute_code = 'description' AND entity_type_id = 4)
     AND pt.store_id = 0
  ) AS description

FROM catalog_product_entity e
WHERE e.entity_id = @entity_id;
```

### 理解属性集

Magento 将属性组织成"属性集"：

```sql
-- 列出所有属性集
SELECT 
  eas.attribute_set_id,
  eas.attribute_set_name,
  eet.entity_type_code
FROM eav_attribute_set eas
JOIN eav_entity_type eet ON eas.entity_type_id = eet.entity_type_id
WHERE eet.entity_type_code = 'catalog_product';

-- 默认属性集：
-- 4: Default
-- 9: Bag
-- 10: Bottom (用于裤子)
-- 11: Gear
-- 12: Sprite (用于 T 恤)
-- 13: Top (用于衬衫)
```

## 导出策略

### 方法 1：Magento 数据迁移工具方法

官方 Magento 方法使用迁移脚本：

```bash
# 安装 Magento 数据迁移工具
composer require magento/data-migration-tool:2.4.x

# 配置迁移
php bin/magento migrate:settings --reset vendor/magento/data-migration-tool/etc/opensource-to-opensource/1.9.4.5/config.xml
```

### 方法 2：带 EAV 处理的直接数据库导出

```sql
-- 处理 EAV 的综合产品导出查询
-- 由于 Magento 的结构，这非常复杂

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

### 方法 3：Magento API 导出

```bash
#!/bin/bash
# export-magento-api.sh

MAGENTO_URL="https://your-magento-store.com"
ACCESS_TOKEN="YOUR_INTEGRATION_ACCESS_TOKEN"

# 创建导出目录
mkdir -p magento-export

# 使用 Magento REST API 获取产品
curl -X GET "$MAGENTO_URL/rest/V1/products" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"searchCriteria":{"currentPage":1,"pageSize":100}}' \
  > magento-export/products-page-1.json

# 大型目录的分页
page=1
while true; do
  echo "获取第 $page 页..."
  
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
  
  # Magento API 速率限制
  sleep 2
done
```

## 高级 Python 迁移脚本

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
    """处理 Magento EAV 结构的辅助类"""
    
    def __init__(self, db_connection):
        self.db = db_connection
        self.attributes = {}
        self._load_attributes()
    
    def _load_attributes(self):
        """缓存所有产品属性"""
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
        """获取单个产品属性值"""
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
        """获取产品的所有属性"""
        attributes = {}
        
        # 查询所有属性表
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
        """执行完整的 Magento 到 R Commerce 迁移"""
        print("开始 Magento 到 R Commerce 迁移...")
        print(f"数据库：{self.db.server_host}")
        print(f"R Commerce：{self.rcommerce_url}")
        
        try:
            # 迁移前分析
            self.analyze_magento_structure()
            
            # 阶段 1：店铺配置
            print("\n=== 阶段 1：店铺配置 ===")
            self.migrate_store_configuration()
            
            # 阶段 2：分类
            print("\n=== 阶段 2：分类 ===")
            self.migrate_categories()
            
            # 阶段 3：属性（作为产品元字段）
            print("\n=== 阶段 3：产品属性 ===")
            self.migrate_attributes()
            
            # 阶段 4：先处理简单产品
            print("\n=== 阶段 4：简单产品 ===")
            self.migrate_products('simple')
            
            # 阶段 5：可配置产品
            print("\n=== 阶段 5：可配置产品 ===")
            self.migrate_products('configurable')
            
            # 阶段 6：其他产品类型
            print("\n=== 阶段 6：捆绑和分组产品 ===")
            self.migrate_products('bundle')
            self.migrate_products('grouped')
            
            # 阶段 7：客户
            print("\n=== 阶段 7：客户 ===")
            self.migrate_customers()
            
            # 阶段 8：订单（可选）
            if os.environ.get('MIGRATE_ORDERS'):
                print("\n=== 阶段 8：订单 ===")
                self.migrate_orders()
            
            # 保存日志
            self.save_migration_log()
            
            print("\n 迁移完成！")
            self.print_summary()
            
        except Exception as e:
            print(f"\n 迁移失败：{e}")
            import traceback
            traceback.print_exc()
            sys.exit(1)
        
        finally:
            self.db.close()
    
    def analyze_magento_structure(self):
        """分析 Magento 店铺结构"""
        print("分析 Magento 结构...")
        
        cursor = self.db.cursor(dictionary=True)
        
        # 按类型统计产品
        cursor.execute("""
            SELECT type_id, COUNT(*) as count
            FROM catalog_product_entity
            GROUP BY type_id
        """)
        
        product_types = cursor.fetchall()
        print("发现的产品类型：")
        for pt in product_types:
            print(f"  - {pt['type_id']}：{pt['count']} 个产品")
        
        # 统计店铺
        cursor.execute("SELECT COUNT(*) as count FROM store")
        store_count = cursor.fetchone()['count']
        print(f"店铺数量：{store_count}")
        
        # 统计客户组
        cursor.execute("SELECT COUNT(*) as count FROM customer_group")
        customer_group_count = cursor.fetchone()['count']
        print(f"客户组数量：{customer_group_count}")
        
        # 统计属性
        cursor.execute("""
            SELECT backend_type, COUNT(*) as count
            FROM eav_attribute
            WHERE entity_type_id = 4
            GROUP BY backend_type
        """)
        attributes = cursor.fetchall()
        print("产品属性：")
        for attr in attributes:
            print(f"  - {attr['backend_type']}：{attr['count']} 个属性")
        
        cursor.close()
    
    def migrate_store_configuration(self):
        """将店铺/网站配置迁移为元数据"""
        cursor = self.db.cursor(dictionary=True)
        
        # 获取所有店铺
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
            # 映射店铺配置
            store_config = {
                'store_id': store['store_id'],
                'store_name': store['store_name'],
                'store_code': store['store_code'],
                'website_id': store['website_id'],
                'website_name': store['website_name'],
                'website_code': store['website_code'],
                'group_name': store['group_name']
            }
            
            # 在 R Commerce 中，将此存储在系统级别
            self.migration_log.append({
                'type': 'store',
                'operation': 'map',
                'status': 'success',
                'source_id': store['store_id'],
                'config': store_config
            })
            
            print(f" 已映射店铺：{store['store_name']} ({store['store_code']})")
        
        cursor.close()
    
    def migrate_categories(self):
        """迁移 Magento 分类"""
        cursor = self.db.cursor(dictionary=True)
        
        # Magento 在嵌套集模型中存储分类
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
        
        # 从 EAV 获取分类名称
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
                    
                    # 在 R Commerce 中创建分类
                    response = requests.post(
                        f"{self.rcommerce_url}/v1/categories",
                        json=category_data,
                        headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                    )
                    
                    if response.status_code == 201:
                        print(f" 已迁移分类：{name}")
                        self.migration_log.append({
                            'type': 'category',
                            'operation': 'create',
                            'status': 'success',
                            'source_id': category['category_id'],
                            'target_id': response.json()['data']['id'],
                            'name': name
                        })
                    else:
                        print(f" 迁移分类 {name} 失败：{response.text}")
                        self.migration_log.append({
                            'type': 'category',
                            'operation': 'create',
                            'status': 'failed',
                            'source_id': category['category_id'],
                            'name': name,
                            'error': response.text
                        })
                    
                    # 速率限制
                    time.sleep(0.5)
                    
                except Exception as e:
                    print(f" 迁移分类 {name} 时出错：{e}")
        
        cursor.close()
    
    def migrate_attributes(self):
        """将 Magento 属性迁移为产品元字段"""
        cursor = self.db.cursor(dictionary=True)
        
        # 获取所有产品属性
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
                # 我们将这些存储为元数据模式定义
                attribute_definition = {
                    'attribute_id': attribute['attribute_id'],
                    'attribute_code': attribute['attribute_code'],
                    'frontend_label': attribute['frontend_label'],
                    'backend_type': attribute['backend_type'],
                    'frontend_input': attribute['frontend_input'],
                    'is_required': attribute['is_required'],
                    'is_user_defined': attribute['is_user_defined']
                }
                
                # 存储在迁移日志中供参考
                self.migration_log.append({
                    'type': 'attribute',
                    'operation': 'map',
                    'status': 'success',
                    'attribute_code': attribute['attribute_code'],
                    'definition': attribute_definition
                })
                
                print(f" 已映射属性：{attribute['attribute_code']}")
                
            except Exception as e:
                print(f" 映射属性 {attribute['attribute_code']} 时出错：{e}")
        
        cursor.close()
    
    def migrate_products(self, product_type: str):
        """迁移特定类型的产品"""
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
                # 获取此产品的所有属性
                attributes = self.eav_helper.get_all_product_attributes(product['entity_id'])
                
                # 根据产品类型转换
                if product_type == 'simple':
                    product_data = self.transform_simple_product(product, attributes)
                elif product_type == 'configurable':
                    product_data = self.transform_configurable_product(product, attributes)
                elif product_type == 'bundle':
                    product_data = self.transform_bundle_product(product, attributes)
                elif product_type == 'grouped':
                    product_data = self.transform_grouped_product(product, attributes)
                else:
                    print(f"⊘ 跳过不支持的类型：{product_type}")
                    continue
                
                # 在 R Commerce 中创建
                response = requests.post(
                    f"{self.rcommerce_url}/v1/products",
                    json=product_data,
                    headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                )
                
                if response.status_code == 201:
                    print(f" 已迁移 {product_type} 产品：{attributes.get('name', product['sku'])}")
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
                    print(f" 迁移 {product_type} 产品 {product['sku']} 失败：{response.text}")
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
                
                # 速率限制
                time.sleep(0.5)
                
            except Exception as e:
                print(f" 迁移 {product_type} 产品 {product['sku']} 时出错：{e}")
        
        cursor.close()
    
    def transform_simple_product(self, product: Dict, attributes: Dict) -> Dict:
        """转换简单产品"""
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
        """转换可配置产品及其变体"""
        # 获取可配置产品信息
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
        
        # 获取与此可配置产品关联的所有简单产品
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
        
        # 将可配置产品转换为主产品
        main_product = self.transform_simple_product(product, attributes)
        main_product['options'] = []
        main_product['variants'] = []
        
        # 添加可配置选项
        for config_attr in configurable_attributes:
            # 获取属性选项
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
        
        # 将子产品转换为变体
        for child in child_products:
            child_attributes = self.eav_helper.get_all_product_attributes(child['child_id'])
            variant = self.transform_simple_product(
                {'entity_id': child['child_id'], 'sku': child_attributes.get('sku'), 'type_id': 'simple'},
                child_attributes
            )
            
            # 覆盖父值
            variant['product_id'] = product['entity_id']  # 父产品 ID
            variant['options'] = {}
            
            # 提取变体选项值
            for config_attr in configurable_attributes:
                attr_code = config_attr['attribute_code']
                if attr_code in child_attributes:
                    variant['options'][attr_code] = child_attributes[attr_code]
            
            main_product['variants'].append(variant)
        
        return main_product
    
    def transform_bundle_product(self, product: Dict, attributes: Dict) -> Dict:
        """转换捆绑产品（R Commerce 原生不支持捆绑，映射到特殊产品类型）"""
        # Magento 中的捆绑产品很复杂 - 我们将它们创建为常规产品
        # 捆绑信息存储在 meta_data 中供未来捆绑实现
        
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
        
        # 在 meta_data 中存储捆绑信息
        main_product['meta_data']['magento']['bundle_options'] = bundle_options
        main_product['meta_data']['product_type_note'] = '捆绑产品 - 选项存储在 meta_data 中'
        
        return main_product
    
    def transform_grouped_product(self, product: Dict, attributes: Dict) -> Dict:
        """转换分组产品（简单产品的容器）"""
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
        main_product['meta_data']['product_type_note'] = '分组产品 - 子产品存储在 meta_data 中'
        
        return main_product
    
    def generate_slug(self, name: str) -> str:
        """生成 URL 友好的 slug"""
        import re
        return re.sub(r'[^a-z0-9]+', '-', name.lower()).strip('-')
    
    def getProductGallery(self, product_id: int):
        """获取产品图库图片"""
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
        """迁移 Magento 客户"""
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
                # 获取客户属性（firstname、lastname 等）
                attributes = self.eav_helper.get_all_product_attributes(customer['entity_id'], entity_type='customer')
                
                # 获取客户地址
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
                    }
                }
                
                # 在 R Commerce 中创建客户
                response = requests.post(
                    f"{self.rcommerce_url}/v1/customers",
                    json=customer_data,
                    headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                )
                
                if response.status_code == 201:
                    print(f" 已迁移客户：{customer['email']}")
                    self.migration_log.append({
                        'type': 'customer',
                        'operation': 'create',
                        'status': 'success',
                        'source_id': customer['entity_id'],
                        'target_id': response.json()['data']['id'],
                        'email': customer['email']
                    })
                else:
                    print(f" 迁移客户 {customer['email']} 失败：{response.text}")
                    self.migration_log.append({
                        'type': 'customer',
                        'operation': 'create',
                        'status': 'failed',
                        'source_id': customer['entity_id'],
                        'email': customer['email'],
                        'error': response.text
                    })
                
                # 速率限制
                time.sleep(0.5)
                
            except Exception as e:
                print(f" 迁移客户 {customer['email']} 时出错：{e}")
        
        cursor.close()
    
    def get_customer_addresses(self, customer_id: int):
        """获取客户地址"""
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
            SELECT 
                entity_id as address_id,
                parent_id as customer_id,
                created_at,
                updated_at
            FROM customer_address_entity
            WHERE parent_id = %s
        """, (customer_id,))
        
        addresses = cursor.fetchall()
        result = []
        
        for addr in addresses:
            # 获取地址属性
            addr_attributes = self.eav_helper.get_all_product_attributes(addr['address_id'], entity_type='customer_address')
            
            result.append({
                'first_name': addr_attributes.get('firstname', ''),
                'last_name': addr_attributes.get('lastname', ''),
                'company': addr_attributes.get('company', ''),
                'street1': addr_attributes.get('street', ''),
                'city': addr_attributes.get('city', ''),
                'state': addr_attributes.get('region', ''),
                'postal_code': addr_attributes.get('postcode', ''),
                'country': addr_attributes.get('country_id', ''),
                'phone': addr_attributes.get('telephone', ''),
                'is_default': addr_attributes.get('default_shipping', False) or addr_attributes.get('default_billing', False)
            })
        
        cursor.close()
        return result
    
    def save_migration_log(self):
        """保存迁移日志"""
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        filename = f'magento_migration_log_{timestamp}.json'
        
        with open(filename, 'w') as f:
            json.dump({
                'timestamp': timestamp,
                'migration_log': self.migration_log
            }, f, indent=2, default=str)
        
        print(f"\n迁移日志已保存到：{filename}")
    
    def print_summary(self):
        """打印迁移摘要"""
        total_products = len([l for l in self.migration_log if l['type'] == 'product'])
        successful_products = len([l for l in self.migration_log if l['type'] == 'product' and l['status'] == 'success'])
        failed_products = len([l for l in self.migration_log if l['type'] == 'product' and l['status'] == 'failed'])
        
        total_customers = len([l for l in self.migration_log if l['type'] == 'customer'])
        successful_customers = len([l for l in self.migration_log if l['type'] == 'customer' and l['status'] == 'success'])
        failed_customers = len([l for l in self.migration_log if l['type'] == 'customer' and l['status'] == 'failed'])
        
        print("\n=== 迁移摘要 ===")
        print(f"产品：{successful_products}/{total_products} 成功，{failed_products} 失败")
        print(f"客户：{successful_customers}/{total_customers} 成功，{failed_customers} 失败")

# 使用
if __name__ == '__main__':
    db_config = {
        'host': os.environ.get('MAGENTO_DB_HOST', 'localhost'),
        'user': os.environ.get('MAGENTO_DB_USER', 'magento'),
        'password': os.environ.get('MAGENTO_DB_PASS', 'password'),
        'database': os.environ.get('MAGENTO_DB_NAME', 'magento'),
        'charset': 'utf8mb4'
    }
    
    rcommerce_config = {
        'url': os.environ.get('RCOMMERCE_URL', 'https://api.yourstore.com'),
        'api_key': os.environ.get('RCOMMERCE_API_KEY', 'your_api_key')
    }
    
    migrator = MagentoMigrator(db_config, rcommerce_config)
    migrator.migrate_all()
```

## 迁移后步骤

1. **验证数据完整性**
   - 比较产品、客户、订单数量
   - 验证关键字段映射
   - 检查图片和媒体文件

2. **测试关键路径**
   - 产品浏览
   - 变体选择
   - 添加到购物车
   - 结账流程
   - 支付处理
   - 订单确认

3. **更新前端**
   - 更改 API 端点
   - 更新认证头
   - 测试所有 API 调用
   - 验证错误处理

4. **更新 Webhooks**
   - 重新配置支付提供商 Webhooks
   - 更新物流提供商集成
   - 测试通知系统
