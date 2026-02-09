# Magento 迁移到 R Commerce 指南

## 概述

由于其 EAV（实体-属性-值）模型、多个店铺视图和广泛的扩展生态系统，Magento 拥有所有主要电商平台中最复杂的数据库结构。本指南涵盖使用基于 API 的方法从 Magento 2.x（开源版或商业版）迁移到 R Commerce。

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

# 获取产品数量
php bin/magento info:product-count

# 获取客户数量（通过自定义命令或 API）
```

**使用 Magento REST API：**

```bash
# 获取访问令牌
ACCESS_TOKEN=$(curl -X POST "https://your-magento-store.com/rest/V1/integration/admin/token" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}' | tr -d '"')

# 获取产品数量
curl -X GET "https://your-magento-store.com/rest/V1/products?searchCriteria[pageSize]=1" \
  -H "Authorization: Bearer $ACCESS_TOKEN" | jq '.search_criteria.total_count'

# 获取客户数量
curl -X GET "https://your-magento-store.com/rest/V1/customers/search?searchCriteria[pageSize]=1" \
  -H "Authorization: Bearer $ACCESS_TOKEN" | jq '.search_criteria.total_count'

# 获取订单数量
curl -X GET "https://your-magento-store.com/rest/V1/orders?searchCriteria[pageSize]=1" \
  -H "Authorization: Bearer $ACCESS_TOKEN" | jq '.search_criteria.total_count'

# 获取店铺信息
curl -X GET "https://your-magento-store.com/rest/V1/store/storeConfigs" \
  -H "Authorization: Bearer $ACCESS_TOKEN"
```

## Magento 数据结构理解

### EAV 模型概述

Magento 的 EAV 模型使数据访问变得复杂。使用 REST API 时，这种复杂性被抽象化：

```
catalog_product_entity (主表)
  ├── entity_id
  ├── sku
  ├── type_id (simple, configurable, bundle, 等)
  ├── attribute_set_id
  └── [少数其他字段]

EAV 属性表（通过 API 访问）：
  - catalog_product_entity_varchar (用于 varchar 属性)
  - catalog_product_entity_int (用于 integer 属性)
  - catalog_product_entity_decimal (用于 decimal 属性)
  - catalog_product_entity_text (用于 text 属性)
  - catalog_product_entity_datetime (用于 datetime 属性)

eav_attribute (定义所有属性)
  ├── attribute_id
  ├── entity_type_id (4 = catalog_product)
  ├── attribute_code
  ├── backend_type (varchar, int, decimal, text, datetime)
  ├── frontend_label
  └── [许多其他列]
```

### 理解属性集

Magento 将属性组织成"属性集"。这些可以通过 API 检索：

```bash
# 获取属性集
curl -X GET "https://your-magento-store.com/rest/V1/products/attribute-sets/sets/list?searchCriteria[pageSize]=100" \
  -H "Authorization: Bearer $ACCESS_TOKEN"
```

## 导出策略

### 方法 1：Magento REST API 导出（推荐）

```bash
#!/bin/bash
# export-magento-api.sh

MAGENTO_URL="https://your-magento-store.com"
ACCESS_TOKEN="YOUR_INTEGRATION_ACCESS_TOKEN"

# 创建导出目录
mkdir -p magento-export

# 使用 Magento REST API 获取产品
echo "导出产品..."
page=1
while true; do
  echo "获取产品第 $page 页..."
  
  response=$(curl -s -X GET "$MAGENTO_URL/rest/V1/products?searchCriteria[currentPage]=$page&searchCriteria[pageSize]=100" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json")
  
  items_count=$(echo $response | jq '.items | length')
  
  if [ "$items_count" -eq 0 ]; then
    break
  fi
  
  echo $response > magento-export/products-page-$page.json
  page=$((page + 1))
  
  # Magento API 速率限制
  sleep 2
done

# 导出分类
echo "导出分类..."
curl -X GET "$MAGENTO_URL/rest/V1/categories" \
  -H "Authorization: Bearer $ACCESS_TOKEN" \
  -H "Content-Type: application/json" > magento-export/categories.json

# 导出客户
echo "导出客户..."
page=1
while true; do
  echo "获取客户第 $page 页..."
  
  response=$(curl -s -X GET "$MAGENTO_URL/rest/V1/customers/search?searchCriteria[currentPage]=$page&searchCriteria[pageSize]=100" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json")
  
  items_count=$(echo $response | jq '.items | length')
  
  if [ "$items_count" -eq 0 ]; then
    break
  fi
  
  echo $response > magento-export/customers-page-$page.json
  page=$((page + 1))
  sleep 2
done

# 导出订单
echo "导出订单..."
page=1
while true; do
  echo "获取订单第 $page 页..."
  
  response=$(curl -s -X GET "$MAGENTO_URL/rest/V1/orders?searchCriteria[currentPage]=$page&searchCriteria[pageSize]=100" \
    -H "Authorization: Bearer $ACCESS_TOKEN" \
    -H "Content-Type: application/json")
  
  items_count=$(echo $response | jq '.items | length')
  
  if [ "$items_count" -eq 0 ]; then
    break
  fi
  
  echo $response > magento-export/orders-page-$page.json
  page=$((page + 1))
  sleep 2
done

echo "导出完成！"
```

### 方法 2：Magento 数据迁移工具方法

官方 Magento 方法使用迁移脚本：

```bash
# 安装 Magento 数据迁移工具
composer require magento/data-migration-tool:2.4.x

# 配置迁移
php bin/magento migrate:settings --reset vendor/magento/data-migration-tool/etc/opensource-to-opensource/1.9.4.5/config.xml
```

### 方法 3：通过 Magento 后台 CSV 导出

1. 前往 **系统 > 数据传输 > 导出**
2. 选择实体类型（产品、客户、订单）
3. 选择导出格式（CSV）
4. 根据需要配置字段过滤器
5. 点击**继续**

## 高级 Python 迁移脚本

```python
#!/usr/bin/env python3
# migrate-magento.py

import requests
import json
import os
import sys
import time
from typing import List, Dict, Optional
from datetime import datetime

class MagentoAPIHelper:
    """处理 Magento REST API 交互的辅助类"""
    
    def __init__(self, base_url: str, access_token: str):
        self.base_url = base_url.rstrip('/')
        self.access_token = access_token
        self.headers = {
            'Authorization': f'Bearer {access_token}',
            'Content-Type': 'application/json'
        }
    
    def get(self, endpoint: str, params: dict = None) -> dict:
        """向 Magento API 发起 GET 请求"""
        url = f"{self.base_url}/rest/V1/{endpoint}"
        response = requests.get(url, headers=self.headers, params=params)
        response.raise_for_status()
        return response.json()
    
    def get_all_pages(self, endpoint: str, page_size: int = 100) -> List[dict]:
        """获取分页端点的所有页面"""
        all_items = []
        page = 1
        
        while True:
            params = {
                'searchCriteria[currentPage]': page,
                'searchCriteria[pageSize]': page_size
            }
            
            try:
                response = self.get(endpoint, params)
                items = response.get('items', [])
                
                if not items:
                    break
                
                all_items.extend(items)
                print(f"  获取第 {page} 页，得到 {len(items)} 个项目")
                
                # 检查是否已获取所有项目
                total_count = response.get('search_criteria', {}).get('total_count', 0)
                if len(all_items) >= total_count:
                    break
                
                page += 1
                time.sleep(1)  # 速率限制
                
            except Exception as e:
                print(f"获取第 {page} 页时出错：{e}")
                break
        
        return all_items

class MagentoMigrator:
    def __init__(self, magento_config: dict, rcommerce_config: dict):
        self.magento = MagentoAPIHelper(
            magento_config['url'],
            magento_config['access_token']
        )
        self.rcommerce_url = rcommerce_config['url']
        self.rcommerce_key = rcommerce_config['api_key']
        self.migration_log = []
        self.store_mapping = {}
        self.attribute_mapping = {}
    
    def migrate_all(self):
        """执行完整的 Magento 到 R Commerce 迁移"""
        print("开始 Magento 到 R Commerce 迁移...")
        print(f"Magento：{self.magento.base_url}")
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
    
    def analyze_magento_structure(self):
        """分析 Magento 店铺结构"""
        print("分析 Magento 结构...")
        
        try:
            # 按类型获取产品数量
            product_types = {}
            products = self.magento.get_all_pages('products', page_size=1)
            total_products = len(products)
            print(f"找到的产品总数：{total_products}")
            
            # 获取店铺信息
            stores = self.magento.get('store/storeConfigs')
            print(f"店铺配置：{len(stores)}")
            
            # 获取属性集
            attr_sets = self.magento.get('products/attribute-sets/sets/list?searchCriteria[pageSize]=100')
            print(f"属性集：{len(attr_sets.get('items', []))}")
            
        except Exception as e:
            print(f"警告：无法分析结构：{e}")
    
    def migrate_store_configuration(self):
        """将店铺/网站配置迁移为元数据"""
        try:
            stores = self.magento.get('store/storeConfigs')
            
            for store in stores:
                store_config = {
                    'store_id': store.get('id'),
                    'store_code': store.get('code'),
                    'website_id': store.get('website_id'),
                    'website_name': store.get('website_name'),
                    'base_url': store.get('base_url'),
                    'base_currency': store.get('base_currency_code'),
                    'timezone': store.get('timezone')
                }
                
                self.migration_log.append({
                    'type': 'store',
                    'operation': 'map',
                    'status': 'success',
                    'source_id': store.get('id'),
                    'config': store_config
                })
                
                print(f" 已映射店铺：{store.get('code')}")
                
        except Exception as e:
            print(f"迁移店铺配置时出错：{e}")
    
    def migrate_categories(self):
        """迁移 Magento 分类"""
        try:
            categories_data = self.magento.get('categories')
            
            def process_category(category, parent_id=None):
                try:
                    category_data = {
                        'name': category['name'],
                        'slug': self.generate_slug(category['name']),
                        'description': '',  # 如有自定义属性将填充
                        'meta_data': {
                            'magento': {
                                'category_id': category['id'],
                                'parent_id': parent_id,
                                'path': category.get('path', ''),
                                'level': category.get('level', 0)
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
                        print(f" 已迁移分类：{category['name']}")
                        self.migration_log.append({
                            'type': 'category',
                            'operation': 'create',
                            'status': 'success',
                            'source_id': category['id'],
                            'target_id': response.json()['data']['id'],
                            'name': category['name']
                        })
                    else:
                        print(f" 迁移分类 {category['name']} 失败：{response.text}")
                        self.migration_log.append({
                            'type': 'category',
                            'operation': 'create',
                            'status': 'failed',
                            'source_id': category['id'],
                            'name': category['name'],
                            'error': response.text
                        })
                    
                    time.sleep(0.5)
                    
                    # 处理子分类
                    for child in category.get('children_data', []):
                        process_category(child, category['id'])
                        
                except Exception as e:
                    print(f" 迁移分类 {category.get('name')} 时出错：{e}")
            
            # 从根分类的子分类开始
            for category in categories_data.get('children_data', []):
                process_category(category)
                
        except Exception as e:
            print(f"分类迁移时出错：{e}")
    
    def migrate_attributes(self):
        """将 Magento 属性迁移为产品元字段模式"""
        try:
            attributes = self.magento.get_all_pages('products/attributes', page_size=100)
            
            for attr in attributes:
                try:
                    attribute_definition = {
                        'attribute_id': attr['attribute_id'],
                        'attribute_code': attr['attribute_code'],
                        'frontend_label': attr.get('default_frontend_label', ''),
                        'frontend_input': attr.get('frontend_input', ''),
                        'is_required': attr.get('is_required', False),
                        'is_user_defined': attr.get('is_user_defined', False)
                    }
                    
                    self.migration_log.append({
                        'type': 'attribute',
                        'operation': 'map',
                        'status': 'success',
                        'attribute_code': attr['attribute_code'],
                        'definition': attribute_definition
                    })
                    
                    self.attribute_mapping[attr['attribute_code']] = attribute_definition
                    print(f" 已映射属性：{attr['attribute_code']}")
                    
                except Exception as e:
                    print(f" 映射属性时出错：{e}")
                    
        except Exception as e:
            print(f"属性迁移时出错：{e}")
    
    def migrate_products(self, product_type: str):
        """迁移特定类型的产品"""
        try:
            # 获取所有产品并按类型过滤
            all_products = self.magento.get_all_pages('products', page_size=100)
            products = [p for p in all_products if p.get('type_id') == product_type]
            
            print(f"找到 {len(products)} 个 {product_type} 产品")
            
            for product in products:
                try:
                    # 获取完整产品详情
                    product_detail = self.magento.get(f"products/{product['sku']}")
                    
                    # 根据产品类型转换
                    if product_type == 'simple':
                        product_data = self.transform_simple_product(product_detail)
                    elif product_type == 'configurable':
                        product_data = self.transform_configurable_product(product_detail)
                    elif product_type == 'bundle':
                        product_data = self.transform_bundle_product(product_detail)
                    elif product_type == 'grouped':
                        product_data = self.transform_grouped_product(product_detail)
                    else:
                        print(f" 跳过不支持的类型：{product_type}")
                        continue
                    
                    # 在 R Commerce 中创建
                    response = requests.post(
                        f"{self.rcommerce_url}/v1/products",
                        json=product_data,
                        headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                    )
                    
                    if response.status_code == 201:
                        print(f" 已迁移 {product_type} 产品：{product.get('name', product['sku'])}")
                        self.migration_log.append({
                            'type': 'product',
                            'product_type': product_type,
                            'operation': 'create',
                            'status': 'success',
                            'source_id': product.get('id'),
                            'sku': product['sku'],
                            'target_id': response.json()['data']['id'],
                            'name': product.get('name', product['sku'])
                        })
                    else:
                        print(f" 迁移 {product_type} 产品 {product['sku']} 失败：{response.text}")
                        self.migration_log.append({
                            'type': 'product',
                            'product_type': product_type,
                            'operation': 'create',
                            'status': 'failed',
                            'source_id': product.get('id'),
                            'sku': product['sku'],
                            'name': product.get('name', product['sku']),
                            'error': response.text
                        })
                    
                    time.sleep(0.5)
                    
                except Exception as e:
                    print(f" 迁移 {product_type} 产品 {product.get('sku', 'unknown')} 时出错：{e}")
                    
        except Exception as e:
            print(f"{product_type} 产品迁移时出错：{e}")
    
    def transform_simple_product(self, product: dict) -> dict:
        """从 Magento API 响应转换简单产品"""
        custom_attrs = {attr['attribute_code']: attr['value'] 
                       for attr in product.get('custom_attributes', [])}
        
        return {
            'name': product.get('name', f"Product {product['sku']}"),
            'slug': custom_attrs.get('url_key') or self.generate_slug(product.get('name', '')),
            'description': custom_attrs.get('description', ''),
            'short_description': custom_attrs.get('short_description', ''),
            'sku': product['sku'],
            'price': float(custom_attrs.get('price', 0) or 0),
            'compare_at_price': float(custom_attrs.get('msrp', 0) or 0) if custom_attrs.get('msrp') else None,
            'cost': float(custom_attrs.get('cost', 0) or 0) if custom_attrs.get('cost') else None,
            'inventory_quantity': int(custom_attrs.get('qty', 0) or 0),
            'inventory_policy': 'deny' if custom_attrs.get('is_in_stock') == '0' else 'continue',
            'weight': float(custom_attrs.get('weight', 0) or 0) if custom_attrs.get('weight') else None,
            'status': 'active' if product.get('status') == 1 else 'draft',
            'is_taxable': custom_attrs.get('tax_class_id') != '0',
            'requires_shipping': bool(custom_attrs.get('weight') and float(custom_attrs.get('weight', 0)) > 0),
            'images': [{'url': media['file'], 'position': media.get('position', 0)} 
                      for media in product.get('media_gallery_entries', [])],
            'meta_data': {
                'magento': {
                    'entity_id': product.get('id'),
                    'type_id': product.get('type_id'),
                    'attribute_set_id': product.get('attribute_set_id'),
                    'visibility': custom_attrs.get('visibility'),
                    'tax_class_id': custom_attrs.get('tax_class_id'),
                    'custom_attributes': custom_attrs
                }
            }
        }
    
    def transform_configurable_product(self, product: dict) -> dict:
        """转换可配置产品及其变体"""
        main_product = self.transform_simple_product(product)
        main_product['options'] = []
        main_product['variants'] = []
        
        # 获取可配置产品选项
        try:
            for option in product.get('extension_attributes', {}).get('configurable_product_options', []):
                option_values = [str(v['value_index']) for v in option.get('values', [])]
                main_product['options'].append({
                    'name': option.get('label', ''),
                    'position': option.get('position', 0),
                    'values': option_values
                })
            
            # 获取子产品（变体）
            child_skus = product.get('extension_attributes', {}).get('configurable_product_links', [])
            for child_sku in child_skus:
                try:
                    child_product = self.magento.get(f"products/{child_sku}")
                    variant = self.transform_simple_product(child_product)
                    variant['options'] = {}
                    
                    # 从自定义属性中提取变体选项值
                    for attr in child_product.get('custom_attributes', []):
                        if attr['attribute_code'] in [opt['name'] for opt in main_product['options']]:
                            variant['options'][attr['attribute_code']] = attr['value']
                    
                    main_product['variants'].append(variant)
                    time.sleep(0.2)
                except Exception as e:
                    print(f"  获取变体 {child_sku} 时出错：{e}")
                    
        except Exception as e:
            print(f" 处理可配置选项时出错：{e}")
        
        return main_product
    
    def transform_bundle_product(self, product: dict) -> dict:
        """转换捆绑产品"""
        main_product = self.transform_simple_product(product)
        
        # 捆绑选项存储在 meta_data 中
        bundle_options = product.get('extension_attributes', {}).get('bundle_product_options', [])
        main_product['meta_data']['magento']['bundle_options'] = bundle_options
        main_product['meta_data']['product_type_note'] = '捆绑产品 - 选项存储在 meta_data 中'
        
        return main_product
    
    def transform_grouped_product(self, product: dict) -> dict:
        """转换分组产品"""
        main_product = self.transform_simple_product(product)
        
        # 分组产品链接
        grouped_links = product.get('extension_attributes', {}).get('grouped_product_links', [])
        main_product['meta_data']['magento']['grouped_children'] = grouped_links
        main_product['meta_data']['product_type_note'] = '分组产品 - 子产品存储在 meta_data 中'
        
        return main_product
    
    def migrate_customers(self):
        """迁移 Magento 客户"""
        try:
            customers = self.magento.get_all_pages('customers/search', page_size=100)
            
            for customer in customers:
                try:
                    customer_data = self.transform_customer(customer)
                    
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
                            'source_id': customer['id'],
                            'target_id': response.json()['data']['id'],
                            'email': customer['email']
                        })
                    else:
                        print(f" 迁移客户 {customer['email']} 失败：{response.text}")
                        self.migration_log.append({
                            'type': 'customer',
                            'operation': 'create',
                            'status': 'failed',
                            'source_id': customer['id'],
                            'email': customer['email'],
                            'error': response.text
                        })
                    
                    time.sleep(0.5)
                    
                except Exception as e:
                    print(f" 迁移客户 {customer.get('email', 'unknown')} 时出错：{e}")
                    
        except Exception as e:
            print(f"客户迁移时出错：{e}")
    
    def transform_customer(self, customer: dict) -> dict:
        """转换 Magento 客户"""
        addresses = customer.get('addresses', [])
        default_billing = next((a for a in addresses if a.get('default_billing')), addresses[0] if addresses else {})
        default_shipping = next((a for a in addresses if a.get('default_shipping')), addresses[0] if addresses else {})
        
        return {
            'email': customer['email'],
            'first_name': customer.get('firstname', ''),
            'last_name': customer.get('lastname', ''),
            'phone': default_billing.get('telephone', ''),
            'accepts_marketing': customer.get('extension_attributes', {}).get('is_subscribed', False),
            'billing_address': {
                'first_name': default_billing.get('firstname', ''),
                'last_name': default_billing.get('lastname', ''),
                'company': default_billing.get('company', ''),
                'address1': ' '.join(default_billing.get('street', [])[:1]),
                'address2': ' '.join(default_billing.get('street', [])[1:]),
                'city': default_billing.get('city', ''),
                'state': default_billing.get('region', {}).get('region', ''),
                'postal_code': default_billing.get('postcode', ''),
                'country': default_billing.get('country_id', ''),
                'phone': default_billing.get('telephone', '')
            },
            'shipping_address': {
                'first_name': default_shipping.get('firstname', ''),
                'last_name': default_shipping.get('lastname', ''),
                'company': default_shipping.get('company', ''),
                'address1': ' '.join(default_shipping.get('street', [])[:1]),
                'address2': ' '.join(default_shipping.get('street', [])[1:]),
                'city': default_shipping.get('city', ''),
                'state': default_shipping.get('region', {}).get('region', ''),
                'postal_code': default_shipping.get('postcode', ''),
                'country': default_shipping.get('country_id', ''),
                'phone': default_shipping.get('telephone', '')
            },
            'meta_data': {
                'magento': {
                    'customer_id': customer['id'],
                    'group_id': customer.get('group_id'),
                    'store_id': customer.get('store_id'),
                    'website_id': customer.get('website_id')
                }
            }
        }
    
    def migrate_orders(self):
        """迁移 Magento 订单"""
        try:
            orders = self.magento.get_all_pages('orders', page_size=50)  # 订单使用较小的批次
            
            for order in orders:
                try:
                    order_data = self.transform_order(order)
                    
                    response = requests.post(
                        f"{self.rcommerce_url}/v1/orders",
                        json=order_data,
                        headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                    )
                    
                    if response.status_code == 201:
                        print(f" 已迁移订单：{order.get('increment_id', order['entity_id'])}")
                        self.migration_log.append({
                            'type': 'order',
                            'operation': 'create',
                            'status': 'success',
                            'source_id': order['entity_id'],
                            'target_id': response.json()['data']['id'],
                            'order_number': order.get('increment_id')
                        })
                    else:
                        print(f" 迁移订单 {order.get('increment_id')} 失败：{response.text}")
                    
                    time.sleep(0.5)
                    
                except Exception as e:
                    print(f" 迁移订单 {order.get('increment_id', 'unknown')} 时出错：{e}")
                    
        except Exception as e:
            print(f"订单迁移时出错：{e}")
    
    def transform_order(self, order: dict) -> dict:
        """转换 Magento 订单"""
        return {
            'order_number': order.get('increment_id', str(order['entity_id'])),
            'customer_email': order.get('customer_email', ''),
            'customer_first_name': order.get('customer_firstname', ''),
            'customer_last_name': order.get('customer_lastname', ''),
            'subtotal': float(order.get('subtotal', 0) or 0),
            'tax_amount': float(order.get('tax_amount', 0) or 0),
            'shipping_amount': float(order.get('shipping_amount', 0) or 0),
            'discount_amount': float(order.get('discount_amount', 0) or 0),
            'total': float(order.get('grand_total', 0) or 0),
            'status': self.map_order_status(order.get('status', 'pending')),
            'billing_address': self.transform_order_address(order.get('billing_address', {})),
            'shipping_address': self.transform_order_address(order.get('extension_attributes', {}).get('shipping_assignments', [{}])[0].get('shipping', {}).get('address', {})),
            'line_items': [self.transform_order_item(item) for item in order.get('items', [])],
            'meta_data': {
                'magento': {
                    'order_id': order['entity_id'],
                    'store_id': order.get('store_id'),
                    'state': order.get('state'),
                    'shipping_method': order.get('shipping_method'),
                    'shipping_description': order.get('shipping_description'),
                    'coupon_code': order.get('coupon_code')
                }
            }
        }
    
    def transform_order_address(self, address: dict) -> dict:
        """转换订单地址"""
        return {
            'first_name': address.get('firstname', ''),
            'last_name': address.get('lastname', ''),
            'company': address.get('company', ''),
            'address1': ' '.join(address.get('street', [])[:1]),
            'address2': ' '.join(address.get('street', [])[1:]),
            'city': address.get('city', ''),
            'state': address.get('region', ''),
            'postal_code': address.get('postcode', ''),
            'country': address.get('country_id', ''),
            'phone': address.get('telephone', '')
        }
    
    def transform_order_item(self, item: dict) -> dict:
        """转换订单项目"""
        return {
            'product_id': f"magento_{item.get('product_id')}",
            'name': item.get('name', ''),
            'sku': item.get('sku', ''),
            'quantity': float(item.get('qty_ordered', 0) or 0),
            'unit_price': float(item.get('price', 0) or 0),
            'tax_amount': float(item.get('tax_amount', 0) or 0),
            'discount_amount': float(item.get('discount_amount', 0) or 0),
            'total': float(item.get('row_total', 0) or 0),
            'meta_data': {
                'magento': {
                    'item_id': item.get('item_id'),
                    'product_type': item.get('product_type'),
                    'original_price': item.get('original_price')
                }
            }
        }
    
    def map_order_status(self, magento_status: str) -> str:
        """将 Magento 订单状态映射到 R Commerce 状态"""
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
    
    def generate_slug(self, name: str) -> str:
        """生成 URL 友好的 slug"""
        import re
        return re.sub(r'[^a-z0-9]+', '-', name.lower()).strip('-')
    
    def save_migration_log(self):
        """保存完整迁移日志"""
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
        
        print(f"迁移日志已保存到 {filename}")
        print(f"摘要：{json.dumps(summary, indent=2)}")
    
    def print_summary(self):
        """打印迁移摘要"""
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
        print("迁移摘要")
        print("="*50)
        for item_type, counts in summary.items():
            total = counts['success'] + counts['failed']
            print(f"{item_type.capitalize()}：{counts['success']}/{total} 成功")
        print("="*50)

# 使用
if __name__ == '__main__':
    magento_config = {
        'url': os.environ.get('MAGENTO_URL', 'https://your-magento-store.com'),
        'access_token': os.environ.get('MAGENTO_ACCESS_TOKEN', 'your_access_token')
    }
    
    rcommerce_config = {
        'url': os.environ.get('RCOMMERCE_URL', 'https://api.yourstore.com'),
        'api_key': os.environ.get('RCOMMERCE_API_KEY', 'your_api_key')
    }
    
    migrator = MagentoMigrator(magento_config, rcommerce_config)
    migrator.migrate_all()
```

## 处理企业版功能

如果从 Magento Commerce（企业版）迁移：

### 客户细分

```bash
# 通过 API 导出客户细分
curl -X GET "https://your-magento-store.com/rest/V1/customerSegments" \
  -H "Authorization: Bearer $ACCESS_TOKEN"
```

### CMS 块和页面

```bash
# 导出 CMS 块
curl -X GET "https://your-magento-store.com/rest/V1/cmsBlock/search?searchCriteria[pageSize]=100" \
  -H "Authorization: Bearer $ACCESS_TOKEN"

# 导出 CMS 页面
curl -X GET "https://your-magento-store.com/rest/V1/cmsPage/search?searchCriteria[pageSize]=100" \
  -H "Authorization: Bearer $ACCESS_TOKEN"
```

### 高级库存 (MSI)

```bash
# 获取库存源
curl -X GET "https://your-magento-store.com/rest/V1/inventory/sources" \
  -H "Authorization: Bearer $ACCESS_TOKEN"

# 获取带源的库存项目
curl -X GET "https://your-magento-store.com/rest/V1/inventory/source-items?searchCriteria[pageSize]=100" \
  -H "Authorization: Bearer $ACCESS_TOKEN"
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
