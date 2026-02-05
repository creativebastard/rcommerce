# WooCommerce 迁移到 R Commerce 指南

## 概述

从 WooCommerce 迁移到 R Commerce 需要处理 WordPress 集成、插件数据和 WooCommerce 特定功能。本指南涵盖直接数据库迁移和基于 API 的迁移方法。

## 迁移前分析

### 审计 WooCommerce 店铺

**从 WordPress 管理后台：**
```sql
-- 获取产品数量
SELECT COUNT(*) FROM wp_posts WHERE post_type = 'product' AND post_status = 'publish';

-- 获取客户数量  
SELECT COUNT(DISTINCT user_id) FROM wp_wc_customer_lookup;

-- 获取订单数量
SELECT COUNT(*) FROM wp_posts WHERE post_type = 'shop_order';

-- 获取插件信息
SELECT option_name, option_value FROM wp_options WHERE option_name LIKE '%woocommerce%';

-- 获取激活的插件
SELECT option_value FROM wp_options WHERE option_name = 'active_plugins';
```

**使用 WP-CLI：**
```bash
# 安装 WP-CLI（如未安装）
wp --version

# 获取产品数量
wp wc product list --user=1 --format=count

# 获取客户数量
wp wc customer list --user=1 --format=count

# 获取订单数量
wp wc order list --user=1 --format=count --status=any

# 导出所有产品
wp wc product list --user=1 --format=json > products.json

# 列出所有插件
wp plugin list --status=active --format=json
```

## 导出策略

### 选项 1：直接数据库导出

```bash
#!/bin/bash
# export-woocommerce-db.sh

DB_NAME="wordpress"
DB_USER="wp_user"
DB_PASS="wp_password"

# 创建导出目录
mkdir -p woocommerce-export

# 导出产品
echo "导出产品..."
mysql -u $DB_USER -p$DB_PASS $DB_NAME -e "
SELECT 
  p.ID,
  p.post_title as name,
  p.post_content as description,
  p.post_excerpt as short_description,
  p.post_status,
  p.post_date,
  pm.meta_value as sku,
  pm_price.meta_value as price,
  pm_regular_price.meta_value as regular_price,
  pm_sale_price.meta_value as sale_price,
  pm_stock.meta_value as stock_quantity,
  pm_stock_status.meta_value as stock_status,
  pm_weight.meta_value as weight,
  pm_length.meta_value as length,
  pm_width.meta_value as width,
  pm_height.meta_value as height
FROM wp_posts p
LEFT JOIN wp_postmeta pm ON p.ID = pm.post_id AND pm.meta_key = '_sku'
LEFT JOIN wp_postmeta pm_price ON p.ID = pm_price.post_id AND pm_price.meta_key = '_price'
LEFT JOIN wp_postmeta pm_regular_price ON p.ID = pm_regular_price.post_id AND pm_regular_price.meta_key = '_regular_price'
LEFT JOIN wp_postmeta pm_sale_price ON p.ID = pm_sale_price.post_id AND pm_sale_price.meta_key = '_sale_price'
LEFT JOIN wp_postmeta pm_stock ON p.ID = pm_stock.post_id AND pm_stock.meta_key = '_stock'
LEFT JOIN wp_postmeta pm_stock_status ON p.ID = pm_stock_status.post_id AND pm_stock_status.meta_key = '_stock_status'
LEFT JOIN wp_postmeta pm_weight ON p.ID = pm_weight.post_id AND pm_weight.meta_key = '_weight'
LEFT JOIN wp_postmeta pm_length ON p.ID = pm_length.post_id AND pm_length.meta_key = '_length'
LEFT JOIN wp_postmeta pm_width ON p.ID = pm_width.post_id AND pm_width.meta_key = '_width'
LEFT JOIN wp_postmeta pm_height ON p.ID = pm_height.post_id AND pm_height.meta_key = '_height'
WHERE p.post_type = 'product' 
  AND p.post_status IN ('publish', 'draft')
" > woocommerce-export/products.csv

# 导出产品分类
echo "导出分类..."
mysql -u $DB_USER -p$DB_PASS $DB_NAME -e "
SELECT 
  t.term_id,
  t.name,
  t.slug,
  tt.parent,
  tt.description,
  tx.taxonomy
FROM wp_terms t
JOIN wp_term_taxonomy tx ON t.term_id = tx.term_id
JOIN wp_term_relationships tr ON tx.term_taxonomy_id = tr.term_taxonomy_id
JOIN wp_posts p ON tr.object_id = p.ID
WHERE tx.taxonomy IN ('product_cat', 'product_tag')
  AND p.post_type = 'product'
GROUP BY t.term_id
" > woocommerce-export/categories.csv

# 导出客户
echo "导出客户..."
mysql -u $DB_USER -p$DB_PASS $DB_NAME -e "
SELECT 
  u.ID as user_id,
  u.user_email as email,
  u.user_registered as created_at,
  m_first_name.meta_value as first_name,
  m_last_name.meta_value as last_name,
  m_billing_first_name.meta_value as billing_first_name,
  m_billing_last_name.meta_value as billing_last_name,
  m_billing_company.meta_value as billing_company,
  m_billing_address_1.meta_value as billing_address_1,
  m_billing_address_2.meta_value as billing_address_2,
  m_billing_city.meta_value as billing_city,
  m_billing_state.meta_value as billing_state,
  m_billing_postcode.meta_value as billing_postcode,
  m_billing_country.meta_value as billing_country,
  m_billing_phone.meta_value as billing_phone,
  m_shipping_first_name.meta_value as shipping_first_name,
  m_shipping_last_name.meta_value as shipping_last_name,
  m_shipping_company.meta_value as shipping_company,
  m_shipping_address_1.meta_value as shipping_address_1,
  m_shipping_address_2.meta_value as shipping_address_2,
  m_shipping_city.meta_value as shipping_city,
  m_shipping_state.meta_value as shipping_state,
  m_shipping_postcode.meta_value as shipping_postcode,
  m_shipping_country.meta_value as shipping_country
FROM wp_users u
LEFT JOIN wp_usermeta m_first_name ON u.ID = m_first_name.user_id AND m_first_name.meta_key = 'first_name'
LEFT JOIN wp_usermeta m_last_name ON u.ID = m_last_name.user_id AND m_last_name.meta_key = 'last_name'
LEFT JOIN wp_usermeta m_billing_first_name ON u.ID = m_billing_first_name.user_id AND m_billing_first_name.meta_key = 'billing_first_name'
LEFT JOIN wp_usermeta m_billing_last_name ON u.ID = m_billing_last_name.user_id AND m_billing_last_name.meta_key = 'billing_last_name'
LEFT JOIN wp_usermeta m_billing_company ON u.ID = m_billing_company.user_id AND m_billing_company.meta_key = 'billing_company'
LEFT JOIN wp_usermeta m_billing_address_1 ON u.ID = m_billing_address_1.user_id AND m_billing_address_1.meta_key = 'billing_address_1'
LEFT JOIN wp_usermeta m_billing_address_2 ON u.ID = m_billing_address_2.user_id AND m_billing_address_2.meta_key = 'billing_address_2'
LEFT JOIN wp_usermeta m_billing_city ON u.ID = m_billing_city.user_id AND m_billing_city.meta_key = 'billing_city'
LEFT JOIN wp_usermeta m_billing_state ON u.ID = m_billing_state.user_id AND m_billing_state.meta_key = 'billing_state'
LEFT JOIN wp_usermeta m_billing_postcode ON u.ID = m_billing_postcode.user_id AND m_billing_postcode.meta_key = 'billing_postcode'
LEFT JOIN wp_usermeta m_billing_country ON u.ID = m_billing_country.user_id AND m_billing_country.meta_key = 'billing_country'
LEFT JOIN wp_usermeta m_billing_phone ON u.ID = m_billing_phone.user_id AND m_billing_phone.meta_key = 'billing_phone'
LEFT JOIN wp_usermeta m_shipping_first_name ON u.ID = m_shipping_first_name.user_id AND m_shipping_first_name.meta_key = 'shipping_first_name'
LEFT JOIN wp_usermeta m_shipping_last_name ON u.ID = m_shipping_last_name.user_id AND m_shipping_last_name.meta_key = 'shipping_last_name'
LEFT JOIN wp_usermeta m_shipping_company ON u.ID = m_shipping_company.user_id AND m_shipping_company.meta_key = 'shipping_company'
LEFT JOIN wp_usermeta m_shipping_address_1 ON u.ID = m_shipping_address_1.user_id AND m_shipping_address_1.meta_key = 'shipping_address_1'
LEFT JOIN wp_usermeta m_shipping_address_2 ON u.ID = m_shipping_address_2.user_id AND m_shipping_address_2.meta_key = 'shipping_address_2'
LEFT JOIN wp_usermeta m_shipping_city ON u.ID = m_shipping_city.user_id AND m_shipping_city.meta_key = 'shipping_city'
LEFT JOIN wp_usermeta m_shipping_state ON u.ID = m_shipping_state.user_id AND m_shipping_state.meta_key = 'shipping_state'
LEFT JOIN wp_usermeta m_shipping_postcode ON u.ID = m_shipping_postcode.user_id AND m_shipping_postcode.meta_key = 'shipping_postcode'
LEFT JOIN wp_usermeta m_shipping_country ON u.ID = m_shipping_country.user_id AND m_shipping_country.meta_key = 'shipping_country'
WHERE u.ID IN (SELECT user_id FROM wp_wc_customer_lookup)
" > woocommerce-export/customers.csv

echo "导出完成！"
```

### 选项 2：使用 WooCommerce REST API

```bash
#!/bin/bash
# export-woocommerce-api.sh

# 认证
CONSUMER_KEY="your_consumer_key"
CONSUMER_SECRET="your_consumer_secret"
SHOP_URL="https://your-store.com"

# 创建导出目录
mkdir -p woocommerce-export

# 获取产品
# 注意：WooCommerce API 有速率限制（取决于主机）
curl -u "${CONSUMER_KEY}:${CONSUMER_SECRET}" \
  "${SHOP_URL}/wp-json/wc/v3/products?per_page=100" \
  > woocommerce-export/products.json

# 带分页获取产品
page=1
while true; do
  echo "获取产品第 $page 页..."
  response=$(curl -s -u "${CONSUMER_KEY}:${CONSUMER_SECRET}" \
    "${SHOP_URL}/wp-json/wc/v3/products?per_page=100&page=${page}")
  
  if [ "$(echo "$response" | jq 'length')" -eq 0 ]; then
    break
  fi
  
  echo "$response" >> woocommerce-export/products-page-${page}.json
  page=$((page + 1))
  
  # WooCommerce API 速率限制
  sleep 1
done

echo "产品导出完成！"
```

## 数据映射

### WordPress 用户到 R Commerce 客户

WooCommerce 将客户存储为带有额外元数据的 WordPress 用户：

```php
<?php
// migrate-customers.php

class WooCommerceCustomerMigrator {
  public function transformCustomer($wpUser) {
    // 核心 WordPress 数据
    $customer = [
      'email' => $wpUser->user_email,
      'first_name' => get_user_meta($wpUser->ID, 'first_name', true),
      'last_name' => get_user_meta($wpUser->ID, 'last_name', true),
      'created_at' => $wpUser->user_registered,
      'meta_data' => [
        'wordpress_id' => $wpUser->ID,
        'username' => $wpUser->user_login,
        'nickname' => get_user_meta($wpUser->ID, 'nickname', true),
        'description' => get_user_meta($wpUser->ID, 'description', true),
        'rich_editing' => get_user_meta($wpUser->ID, 'rich_editing', true),
        'syntax_highlighting' => get_user_meta($wpUser->ID, 'syntax_highlighting', true),
        'comment_shortcuts' => get_user_meta($wpUser->ID, 'comment_shortcuts', true),
        'admin_color' => get_user_meta($wpUser->ID, 'admin_color', true),
        'use_ssl' => get_user_meta($wpUser->ID, 'use_ssl', true),
        'show_admin_bar_front' => get_user_meta($wpUser->ID, 'show_admin_bar_front', true),
        'locale' => get_user_meta($wpUser->ID, 'locale', true),
        'wp_capabilities' => get_user_meta($wpUser->ID, 'wp_capabilities', true),
        'wp_user_level' => get_user_meta($wpUser->ID, 'wp_user_level', true),
        'dismissed_wp_pointers' => get_user_meta($wpUser->ID, 'dismissed_wp_pointers', true),
        'session_tokens' => get_user_meta($wpUser->ID, 'session_tokens', true),
        'last_update' => get_user_meta($wpUser->ID, 'last_update', true)
      ]
    ];
    
    // WooCommerce 特定数据
    $customer['billing_address'] = $this->getBillingAddress($wpUser->ID);
    $customer['shipping_address'] = $this->getShippingAddress($wpUser->ID);
    $customer['accepts_marketing'] = $this->getMarketingConsent($wpUser->ID);
    
    // 额外的 WooCommerce 数据
    $customer['meta_data']['woocommerce'] = [
      'paying_customer' => get_user_meta($wpUser->ID, 'paying_customer', true),
      'billing_email' => get_user_meta($wpUser->ID, 'billing_email', true),
      'shipping_email' => get_user_meta($wpUser->ID, 'shipping_email', true),
      'last_update' => get_user_meta($wpUser->ID, 'last_update', true)
    ];
    
    return $customer;
  }
  
  private function getBillingAddress($userId) {
    return [
      'first_name' => get_user_meta($userId, 'billing_first_name', true),
      'last_name' => get_user_meta($userId, 'billing_last_name', true),
      'company' => get_user_meta($userId, 'billing_company', true),
      'address1' => get_user_meta($userId, 'billing_address_1', true),
      'address2' => get_user_meta($userId, 'billing_address_2', true),
      'city' => get_user_meta($userId, 'billing_city', true),
      'state' => get_user_meta($userId, 'billing_state', true),
      'postal_code' => get_user_meta($userId, 'billing_postcode', true),
      'country' => get_user_meta($userId, 'billing_country', true),
      'phone' => get_user_meta($userId, 'billing_phone', true)
    ];
  }
  
  private function getShippingAddress($userId) {
    return [
      'first_name' => get_user_meta($userId, 'shipping_first_name', true),
      'last_name' => get_user_meta($userId, 'shipping_last_name', true),
      'company' => get_user_meta($userId, 'shipping_company', true),
      'address1' => get_user_meta($userId, 'shipping_address_1', true),
      'address2' => get_user_meta($userId, 'shipping_address_2', true),
      'city' => get_user_meta($userId, 'shipping_city', true),
      'state' => get_user_meta($userId, 'shipping_state', true),
      'postal_code' => get_user_meta($userId, 'shipping_postcode', true),
      'country' => get_user_meta($userId, 'shipping_country', true)
    ];
  }
  
  private function getMarketingConsent($userId) {
    // WooCommerce 以各种方式存储营销同意
    // 检查常用插件或内置方法
    
    $marketing = get_user_meta($userId, 'marketing_opt_in', true);
    if ($marketing !== '') {
      return $marketing === 'yes';
    }
    
    // 检查用户是否已购买（表示某种同意）
    $orders = wc_get_orders([
      'customer_id' => $userId,
      'limit' => 1
    ]);
    
    return !empty($orders);
  }
}
```

### 产品变体处理

WooCommerce 使用产品变体的方式与 Shopify 不同：

```php
<?php
// migrate-variable-products.php

class WooCommerceVariableProductMigrator {
  public function transformProduct($productId) {
    $product = wc_get_product($productId);
    
    if ($product->get_type() === 'variable') {
      return $this->transformVariableProduct($product);
    } else {
      return $this->transformSimpleProduct($product);
    }
  }
  
  private function transformVariableProduct($product) {
    $baseProduct = [
      'name' => $product->get_name(),
      'slug' => $product->get_slug(),
      'description' => $product->get_description(),
      'short_description' => $product->get_short_description(),
      'status' => $product->get_status(),
      'category_id' => $this->getPrimaryCategory($product),
      'tags' => $this->getProductTags($product),
      'meta_data' => $this->getProductMeta($product),
      'images' => $this->getProductImages($product)
    ];
    
    // 处理属性（在 R Commerce 中变为选项）
    $attributes = $product->get_attributes();
    $baseProduct['options'] = $this->transformAttributes($attributes);
    
    // 转换变体
    $variations = $product->get_children();
    $baseProduct['variants'] = [];
    
    foreach ($variations as $variationId) {
      $variation = wc_get_product($variationId);
      $baseProduct['variants'][] = $this->transformVariation($variation, $attributes);
    }
    
    return $baseProduct;
  }
  
  private function transformVariation($variation, $parentAttributes) {
    $variant = [
      'sku' => $variation->get_sku(),
      'price' => $variation->get_price(),
      'regular_price' => $variation->get_regular_price(),
      'sale_price' => $variation->get_sale_price(),
      'inventory_quantity' => $variation->get_stock_quantity(),
      'inventory_policy' => $variation->get_stock_status(),
      'weight' => $variation->get_weight(),
      'dimensions' => [
        'length' => $variation->get_length(),
        'width' => $variation->get_width(),
        'height' => $variation->get_height()
      ],
      'meta_data' => [
        'woocommerce' => [
          'variation_id' => $variation->get_id(),
          'virtual' => $variation->is_virtual(),
          'downloadable' => $variation->is_downloadable()
        ]
      ]
    ];
    
    // 将变体属性映射到 R Commerce 选项
    foreach ($variation->get_variation_attributes() as $attr => $value) {
      // 从 'attribute_pa_color' 转换为 'Color'
      $attrName = str_replace('attribute_', '', $attr);
      $variant['options'] = $variant['options'] || [];
      $variant['options'][$attrName] = $value;
    }
    
    return $variant;
  }
  
  private function transformAttributes($attributes) {
    $options = [];
    
    foreach ($attributes as $attribute) {
      $options[] = [
        'name' => $attribute->get_name(),
        'position' => $attribute->get_position(),
        'values' => $attribute->get_options()
      ];
    }
    
    return $options;
  }
  
  // ... 额外的辅助方法
}
```

## 迁移脚本（Python）

```python
#!/usr/bin/env python3
# migrate-woocommerce.py

import mysql.connector
import requests
import json
import os
import sys
import time
from datetime import datetime
from typing import List, Dict

class WooCommerceMigrator:
    def __init__(self, db_config, rcommerce_config):
        self.db = mysql.connector.connect(**db_config)
        self.rcommerce_url = rcommerce_config['url']
        self.rcommerce_key = rcommerce_config['api_key']
        self.migration_log = []
        
    def migrate_all(self):
        """执行完整迁移"""
        print("开始 WooCommerce 到 R Commerce 迁移...")
        
        try:
            # 阶段 1：分类
            print("\n=== 阶段 1：迁移分类 ===")
            self.migrate_categories()
            
            # 阶段 2：产品
            print("\n=== 阶段 2：迁移产品 ===")
            self.migrate_products()
            
            # 阶段 3：客户
            print("\n=== 阶段 3：迁移客户 ===")
            self.migrate_customers()
            
            # 阶段 4：订单（可选）
            if os.environ.get('MIGRATE_ORDERS'):
                print("\n=== 阶段 4：迁移订单 ===")
                self.migrate_orders()
            
            print("\n 迁移成功完成！")
            self.save_migration_log()
            
        except Exception as e:
            print(f"\n 迁移失败：{e}")
            sys.exit(1)
        
        finally:
            self.db.close()
    
    def migrate_categories(self):
        """将 WooCommerce 产品分类迁移到 R Commerce"""
        cursor = self.db.cursor(dictionary=True)
        
        query = """
        SELECT 
            t.term_id as id,
            t.name,
            t.slug,
            tt.description,
            tt.parent,
            tx.taxonomy
        FROM wp_terms t
        JOIN wp_term_taxonomy tx ON t.term_id = tx.term_id
        WHERE tx.taxonomy IN ('product_cat', 'product_tag')
        """
        
        cursor.execute(query)
        categories = cursor.fetchall()
        
        for category in categories:
            try:
                # 转换分类
                category_data = {
                    'name': category['name'],
                    'slug': category['slug'],
                    'description': category['description'],
                    'meta_data': {
                        'woocommerce': {
                            'term_id': category['id'],
                            'parent': category['parent'],
                            'taxonomy': category['taxonomy']
                        }
                    }
                }
                
                # 在 R Commerce 中创建
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
                
                # 速率限制
                time.sleep(0.5)
                
            except Exception as e:
                print(f" 迁移分类 {category['name']} 时出错：{e}")
                self.migration_log.append({
                    'type': 'category',
                    'operation': 'create',
                    'status': 'error',
                    'source_id': category['id'],
                    'name': category['name'],
                    'error': str(e)
                })
    
    def migrate_products(self):
        """将 WooCommerce 产品迁移到 R Commerce"""
        cursor = self.db.cursor(dictionary=True)
        
        # 获取所有产品
        query = """
        SELECT 
            p.ID as id,
            p.post_title as name,
            p.post_content as description,
            p.post_excerpt as short_description,
            p.post_status,
            p.post_date,
            p.post_modified,
            pm_sku.meta_value as sku,
            pm_price.meta_value as price,
            pm_regular_price.meta_value as regular_price,
            pm_sale_price.meta_value as sale_price,
            pm_stock.meta_value as stock_quantity,
            pm_stock_status.meta_value as stock_status,
            pm_weight.meta_value as weight,
            pm_length.meta_value as length,
            pm_width.meta_value as width,
            pm_height.meta_value as height,
            pm_virtual.meta_value as virtual,
            pm_downloadable.meta_value as downloadable,
            pm_tax_status.meta_value as tax_status,
            pm_tax_class.meta_value as tax_class
        FROM wp_posts p
        LEFT JOIN wp_postmeta pm_sku ON p.ID = pm_sku.post_id AND pm_sku.meta_key = '_sku'
        LEFT JOIN wp_postmeta pm_price ON p.ID = pm_price.post_id AND pm_price.meta_key = '_price'
        LEFT JOIN wp_postmeta pm_regular_price ON p.ID = pm_regular_price.post_id AND pm_regular_price.meta_key = '_regular_price'
        LEFT JOIN wp_postmeta pm_sale_price ON p.ID = pm_sale_price.post_id AND pm_sale_price.meta_key = '_sale_price'
        LEFT JOIN wp_postmeta pm_stock ON p.ID = pm_stock.post_id AND pm_stock.meta_key = '_stock'
        LEFT JOIN wp_postmeta pm_stock_status ON p.ID = pm_stock_status.post_id AND pm_stock_status.meta_key = '_stock_status'
        LEFT JOIN wp_postmeta pm_weight ON p.ID = pm_weight.post_id AND pm_weight.meta_key = '_weight'
        LEFT JOIN wp_postmeta pm_length ON p.ID = pm_length.post_id AND pm_length.meta_key = '_length'
        LEFT JOIN wp_postmeta pm_width ON p.ID = pm_width.post_id AND pm_width.meta_key = '_width'
        LEFT JOIN wp_postmeta pm_height ON p.ID = pm_height.post_id AND pm_height.meta_key = '_height'
        LEFT JOIN wp_postmeta pm_virtual ON p.ID = pm_virtual.post_id AND pm_virtual.meta_key = '_virtual'
        LEFT JOIN wp_postmeta pm_downloadable ON p.ID = pm_downloadable.post_id AND pm_downloadable.meta_key = '_downloadable'
        LEFT JOIN wp_postmeta pm_tax_status ON p.ID = pm_tax_status.post_id AND pm_tax_status.meta_key = '_tax_status'
        LEFT JOIN wp_postmeta pm_tax_class ON p.ID = pm_tax_class.post_id AND pm_tax_class.meta_key = '_tax_class'
        WHERE p.post_type = 'product'
        AND p.post_status IN ('publish', 'draft', 'private')
        ORDER BY p.ID
        """
        
        cursor.execute(query)
        products = cursor.fetchall()
        
        for product in products:
            try:
                # 获取产品分类/标签
                categories = self.getProductCategories(product['id'])
                tags = self.getProductTags(product['id'])
                images = self.getProductImages(product['id'])
                
                # 转换产品数据
                product_data = {
                    'name': product['name'],
                    'slug': self.generateSlug(product['name']),
                    'description': product['description'] or '',
                    'short_description': product['short_description'] or '',
                    'sku': product['sku'],
                    'price': float(product['price'] or 0),
                    'compare_at_price': float(product['regular_price'] or 0) if product['sale_price'] else None,
                    'inventory_quantity': int(product['stock_quantity'] or 0),
                    'inventory_policy': 'deny' if product['stock_status'] == 'outofstock' else 'continue',
                    'weight': float(product['weight'] or 0) if product['weight'] else None,
                    'length': float(product['length'] or 0) if product['length'] else None,
                    'width': float(product['width'] or 0) if product['width'] else None,
                    'height': float(product['height'] or 0) if product['height'] else None,
                    'status': self.mapProductStatus(product['post_status']),
                    'category_id': categories[0]['id'] if categories else None,
                    'tags': tags,
                    'images': images,
                    'is_taxable': product['tax_status'] != 'none',
                    'requires_shipping': product['virtual'] != 'yes',
                    'meta_data': {
                        'woocommerce': {
                            'product_id': product['id'],
                            'post_date': product['post_date'],
                            'post_modified': product['post_modified'],
                            'tax_class': product['tax_class'],
                            'stock_status': product['stock_status']
                        }
                    }
                }
                
                # 处理可变产品
                if self.isVariableProduct(product['id']):
                    product_data['variants'] = self.migrateVariations(product['id'])
                    product_data['options'] = self.getProductAttributes(product['id'])
                
                # 在 R Commerce 中创建
                response = requests.post(
                    f"{self.rcommerce_url}/v1/products",
                    json=product_data,
                    headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                )
                
                if response.status_code == 201:
                    print(f" 已迁移产品：{product['name']}")
                    self.migration_log.append({
                        'type': 'product',
                        'operation': 'create',
                        'status': 'success',
                        'source_id': product['id'],
                        'target_id': response.json()['data']['id'],
                        'name': product['name']
                    })
                else:
                    print(f" 迁移产品 {product['name']} 失败：{response.text}")
                    self.migration_log.append({
                        'type': 'product',
                        'operation': 'create',
                        'status': 'failed',
                        'source_id': product['id'],
                        'name': product['name'],
                        'error': response.text
                    })
                
                # 速率限制
                time.sleep(0.5)
                
            except Exception as e:
                print(f" 迁移产品 {product['name']} 时出错：{e}")
                self.migration_log.append({
                    'type': 'product',
                    'operation': 'create',
                    'status': 'error',
                    'source_id': product['id'],
                    'name': product['name'],
                    'error': str(e)
                })
    
    def getProductCategories(self, productId):
        """获取产品的分类"""
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
        SELECT t.term_id as id, t.name, t.slug
        FROM wp_terms t
        JOIN wp_term_taxonomy tx ON t.term_id = tx.term_id
        JOIN wp_term_relationships tr ON tx.term_taxonomy_id = tr.term_taxonomy_id
        WHERE tr.object_id = %s AND tx.taxonomy = 'product_cat'
        """, (productId,))
        return cursor.fetchall()
    
    def getProductTags(self, productId):
        """获取产品的标签"""
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
        SELECT t.name
        FROM wp_terms t
        JOIN wp_term_taxonomy tx ON t.term_id = tx.term_id
        JOIN wp_term_relationships tr ON tx.term_taxonomy_id = tr.term_taxonomy_id
        WHERE tr.object_id = %s AND tx.taxonomy = 'product_tag'
        """, (productId,))
        return [row['name'] for row in cursor.fetchall()]
    
    def getProductImages(self, productId):
        """获取产品的图片"""
        cursor = self.db.cursor(dictionary=True)
        cursor.execute("""
        SELECT pm_image.meta_value as image_url, pm_alt.meta_value as alt_text
        FROM wp_postmeta pm_image
        LEFT JOIN wp_postmeta pm_alt ON pm_image.post_id = pm_alt.post_id AND pm_alt.meta_key = '_wp_attachment_image_alt'
        WHERE pm_image.post_id IN (
            SELECT pm.meta_value
            FROM wp_postmeta pm
            WHERE pm.post_id = %s AND pm.meta_key = '_thumbnail_id'
        )
        """, (productId,))
        return cursor.fetchall()
    
    def isVariableProduct(self, productId):
        """检查产品是否为可变产品"""
        cursor = self.db.cursor()
        cursor.execute(
            "SELECT COUNT(*) FROM wp_posts WHERE post_parent = %s AND post_type = 'product_variation'",
            (productId,)
        )
        return cursor.fetchone()[0] > 0
    
    def mapProductStatus(self, wpStatus):
        """将 WordPress 发布状态映射到 R Commerce 状态"""
        status_map = {
            'publish': 'active',
            'draft': 'draft',
            'private': 'archived',
            'pending': 'draft'
        }
        return status_map.get(wpStatus, 'draft')
    
    def generateSlug(self, name):
        """生成 URL 友好的 slug"""
        return name.lower().replace(' ', '-')
    
    def save_migration_log(self):
        """保存迁移日志到文件"""
        timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
        filename = f'migration_log_{timestamp}.json'
        
        with open(filename, 'w') as f:
            json.dump({
                'timestamp': timestamp,
                'summary': {
                    'total_products': len([l for l in self.migration_log if l['type'] == 'product']),
                    'successful_products': len([l for l in self.migration_log if l['type'] == 'product' and l['status'] == 'success']),
                    'failed_products': len([l for l in self.migration_log if l['type'] == 'product' and l['status'] == 'failed']),
                    'total_categories': len([l for l in self.migration_log if l['type'] == 'category']),
                    'successful_categories': len([l for l in self.migration_log if l['type'] == 'category' and l['status'] == 'success']),
                },
                'details': self.migration_log
            }, f, indent=2)
        
        print(f"迁移日志已保存到 {filename}")

# 使用
if __name__ == '__main__':
    db_config = {
        'host': os.environ.get('DB_HOST', 'localhost'),
        'user': os.environ.get('DB_USER', 'wp_user'),
        'password': os.environ.get('DB_PASS', 'wp_password'),
        'database': os.environ.get('DB_NAME', 'wordpress'),
        'charset': 'utf8mb4'
    }
    
    rcommerce_config = {
        'url': os.environ.get('RCOMMERCE_URL', 'https://api.yourstore.com'),
        'api_key': os.environ.get('RCOMMERCE_API_KEY', 'your_api_key')
    }
    
    migrator = WooCommerceMigrator(db_config, rcommerce_config)
    migrator.migrate_all()
```

## 插件数据迁移

WooCommerce 将扩展数据存储在各个位置：

### 常见插件数据位置

```sql
-- WooCommerce 订阅
SELECT * FROM wp_woocommerce_susbcriptions WHERE status IN ('active', 'on-hold');

-- WooCommerce 预订
SELECT * FROM wp_wc_bookings WHERE status IN ('paid', 'confirmed');

-- WooCommerce 会员
SELECT * FROM wp_wc_memberships_user_memberships WHERE status = 'active';

-- Yoast SEO
SELECT * FROM wp_postmeta WHERE meta_key LIKE '_yoast_%' AND post_id IN (SELECT ID FROM wp_posts WHERE post_type = 'product');

-- Advanced Custom Fields
SELECT * FROM wp_postmeta WHERE meta_key LIKE 'field_%' AND post_id IN (SELECT ID FROM wp_posts WHERE post_type = 'product');
```

### 插件迁移策略

```php
<?php
// migrate-plugin-data.php

class WooCommercePluginDataMigrator {
  
  public function migrateSubscriptionData($rcommerceProductId) {
    global $wpdb;
    
    // 检查 WooCommerce 订阅是否激活
    if (!class_exists('WC_Subscriptions')) {
      return null;
    }
    
    $subscriptions = $wpdb->get_results("
      SELECT s.*, p.post_parent as product_id
      FROM {$wpdb->prefix}woocommerce_subscriptions s
      JOIN {$wpdb->posts} p ON s.product_id = p.ID
      WHERE p.post_parent = %d
      AND s.status IN ('active', 'on-hold')
    ", $rcommerceProductId);
    
    foreach ($subscriptions as $subscription) {
      // 为 R Commerce 转换订阅数据
      $subscription_data = [
        'product_id' => $rcommerceProductId,
        'interval' => $subscription->billing_interval,
        'period' => $subscription->billing_period, // day, week, month, year
        'price' => $subscription->billing_amount,
        'trial_period' => $subscription->trial_period,
        'trial_interval' => $subscription->trial_interval,
        'meta_data' => [
          'woocommerce' => [
            'subscription_id' => $subscription->id,
            'status' => $subscription->status,
            'start_date' => $subscription->start_date,
            'expiry_date' => $subscription->expiry_date,
            'end_date' => $subscription->end_date,
            'requires_manual_renewal' => $subscription->requires_manual_renewal,
            'billing_period' => $subscription->billing_period,
            'billing_interval' => $subscription->billing_interval,
            'next_payment_date' => $subscription->next_payment_date
          ]
        ]
      ];
      
      // 存储在 R Commerce meta_data 中以供未来订阅实现
      update_post_meta($rcommerceProductId, '_subscription_data', $subscription_data);
    }
  }
  
  public function migrateBookingData($rcommerceProductId) {
    if (!class_exists('WC_Bookings')) {
      return null;
    }
    
    global $wpdb;
    
    $bookable_product = new WC_Product_Booking($rcommerceProductId);
    
    $booking_data = [
      'duration_type' => $bookable_product->get_duration_type(),
      'duration' => $bookable_product->get_duration(),
      'duration_unit' => $bookable_product->get_duration_unit(),
      'calendar_display_mode' => $bookable_product->get_calendar_display_mode(),
      'requires_confirmation' => $bookable_product->get_requires_confirmation(),
      'can_be_cancelled' => $bookable_product->get_can_be_cancelled(),
      'cancel_limit' => $bookable_product->get_cancel_limit(),
      'cancel_limit_unit' => $bookable_product->get_cancel_limit_unit(),
      'min_date' => $bookable_product->get_min_date(),
      'max_date' => $bookable_product->get_max_date(),
      'max_date_value' => $bookable_product->get_max_date_value(),
      'max_date_unit' => $bookable_product->get_max_date_unit(),
      'buffer_period' => $bookable_product->get_buffer_period(),
      'availability_rules' => $bookable_product->get_availability_rules()
    ];
    
    // 存储预订数据以供未来实现
    update_post_meta($rcommerceProductId, '_booking_data', $booking_data);
  }
  
  public function migrateACFData($rcommerceProductId, $woocommerceProductId) {
    // Advanced Custom Fields 数据
    if (!function_exists('get_field')) {
      return;
    }
    
    $acf_fields = get_fields($woocommerceProductId);
    
    if ($acf_fields) {
      update_post_meta($rcommerceProductId, '_acf_data', $acf_fields);
    }
  }
}
```

## 迁移后清理

迁移后：

```bash
#!/bin/bash
# cleanup-woocommerce.sh

echo "清理 WooCommerce 安装..."

# 创建备份
echo "创建完整备份..."
wp db export wordpress-pre-rcommerce-backup.sql

# 禁用 WooCommerce
echo "停用 WooCommerce..."
wp plugin deactivate woocommerce
wp plugin deactivate woocommerce-*

# 重定向所有流量
echo "设置重定向..."
cp wp-config.php wp-config.php.backup

# 添加重定向到 R Commerce
cat >> wp-config.php << 'EOF'
// 将所有 WooCommerce 页面重定向到 R Commerce
if (strpos($_SERVER['REQUEST_URI'], '/shop') === 0 ||
    strpos($_SERVER['REQUEST_URI'], '/product') === 0 ||
    strpos($_SERVER['REQUEST_URI'], '/cart') === 0 ||
    strpos($_SERVER['REQUEST_URI'], '/checkout') === 0) {
    wp_redirect('https://your-new-store.com' . $_SERVER['REQUEST_URI'], 301);
    exit;
}
EOF

echo "清理完成！"
```

## WordPress 集成移除

要完全与 WordPress 断开连接：

```php
<?php
// wp-config-rcommerce-transition.php

// 仅保留 WordPress 用于内容，移除所有电商功能

// 完全禁用 WooCommerce
define('WC_ABSPATH', '');
define('WOOCOMMERCE_ABSPATH', '');

// 阻止 WooCommerce 加载
add_action('plugins_loaded', function() {
    remove_action('plugins_loaded', 'woocommerce_init', 10);
}, 0);

// 重定向所有 WooCommerce 端点
add_action('init', function() {
    $woocommerce_pages = array(
        'shop' => '/shop/',
        'cart' => '/cart/',
        'checkout' => '/checkout/',
        'myaccount' => '/my-account/',
        'terms' => '/terms-and-conditions/'
    );
    
    foreach ($woocommerce_pages as $page => $url) {
        $page_id = wc_get_page_id($page);
        if ($page_id > 0) {
            wp_delete_post($page_id, true);
        }
    }
});

// 添加 JavaScript 立即重定向
add_action('wp_head', function() {
    ?>
    <script>
    if (window.location.pathname.indexOf('/shop') === 0 ||
        window.location.pathname.indexOf('/product') === 0 ||
        window.location.pathname.indexOf('/cart') === 0 ||
        window.location.pathname.indexOf('/checkout') === 0) {
        window.location.href = 'https://your-new-store.com' + window.location.pathname;
    }
    </script>
    <?php
});
```
