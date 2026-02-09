# WooCommerce 迁移到 R Commerce 指南

## 概述

从 WooCommerce 迁移到 R Commerce 需要处理 WordPress 集成、插件数据和 WooCommerce 特定功能。本指南涵盖基于 API 的迁移方法。

## 迁移前分析

### 审计 WooCommerce 店铺

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

**使用 WooCommerce REST API：**
```bash
# 获取店铺信息
curl -u "consumer_key:consumer_secret" \
  "https://your-store.com/wp-json/wc/v3/system_status"

# 获取产品数量（检查 X-WP-Total 响应头）
curl -I -u "consumer_key:consumer_secret" \
  "https://your-store.com/wp-json/wc/v3/products?per_page=1"

# 获取客户数量
curl -I -u "consumer_key:consumer_secret" \
  "https://your-store.com/wp-json/wc/v3/customers?per_page=1"

# 获取订单数量
curl -I -u "consumer_key:consumer_secret" \
  "https://your-store.com/wp-json/wc/v3/orders?per_page=1"
```

## 导出策略

### 选项 1：使用 WooCommerce REST API（推荐）

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

# 导出客户
echo "导出客户..."
page=1
while true; do
  echo "获取客户第 $page 页..."
  response=$(curl -s -u "${CONSUMER_KEY}:${CONSUMER_SECRET}" \
    "${SHOP_URL}/wp-json/wc/v3/customers?per_page=100&page=${page}")
  
  if [ "$(echo "$response" | jq 'length')" -eq 0 ]; then
    break
  fi
  
  echo "$response" >> woocommerce-export/customers-page-${page}.json
  page=$((page + 1))
  sleep 1
done

# 导出订单（可选）
echo "导出订单..."
page=1
while true; do
  echo "获取订单第 $page 页..."
  response=$(curl -s -u "${CONSUMER_KEY}:${CONSUMER_SECRET}" \
    "${SHOP_URL}/wp-json/wc/v3/orders?per_page=100&page=${page}")
  
  if [ "$(echo "$response" | jq 'length')" -eq 0 ]; then
    break
  fi
  
  echo "$response" >> woocommerce-export/orders-page-${page}.json
  page=$((page + 1))
  sleep 1
done

# 导出分类
echo "导出分类..."
curl -u "${CONSUMER_KEY}:${CONSUMER_SECRET}" \
  "${SHOP_URL}/wp-json/wc/v3/products/categories?per_page=100" \
  > woocommerce-export/categories.json

echo "导出完成！"
```

### 选项 2：使用 WP-CLI 导出

```bash
#!/bin/bash
# export-woocommerce-wpcli.sh

# 导出产品到 JSON
wp wc product list --user=1 --format=json > woocommerce-export/products.json

# 导出客户
wp wc customer list --user=1 --format=json > woocommerce-export/customers.json

# 导出订单
wp wc order list --user=1 --format=json --status=any > woocommerce-export/orders.json

# 导出产品分类
wp wc product_cat list --user=1 --format=json > woocommerce-export/categories.json

# 导出产品标签
wp wc product_tag list --user=1 --format=json > woocommerce-export/tags.json
```

### 选项 3：通过 WooCommerce 后台 CSV 导出

1. 前往 **WooCommerce > 产品**
2. 点击**导出**按钮
3. 选择要导出的列
4. 选择"导出为 CSV"
5. 对订单重复操作（WooCommerce > 订单 > 导出）

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

import requests
import json
import os
import sys
import time
from datetime import datetime
from typing import List, Dict

class WooCommerceMigrator:
    def __init__(self, wc_config, rcommerce_config):
        self.wc_url = wc_config['url']
        self.wc_key = wc_config['consumer_key']
        self.wc_secret = wc_config['consumer_secret']
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
    
    def _wc_get(self, endpoint: str, params: dict = None) -> dict:
        """向 WooCommerce API 发起认证 GET 请求"""
        url = f"{self.wc_url}/wp-json/wc/v3/{endpoint}"
        response = requests.get(url, auth=(self.wc_key, self.wc_secret), params=params)
        response.raise_for_status()
        return response.json()
    
    def migrate_categories(self):
        """将 WooCommerce 产品分类迁移到 R Commerce"""
        page = 1
        while True:
            categories = self._wc_get('products/categories', {'per_page': 100, 'page': page})
            if not categories:
                break
            
            for category in categories:
                try:
                    category_data = {
                        'name': category['name'],
                        'slug': category['slug'],
                        'description': category.get('description', ''),
                        'meta_data': {
                            'woocommerce': {
                                'category_id': category['id'],
                                'parent': category.get('parent', 0),
                                'display': category.get('display', 'default'),
                                'image': category.get('image', {})
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
            
            page += 1
    
    def migrate_products(self):
        """将 WooCommerce 产品迁移到 R Commerce"""
        page = 1
        while True:
            products = self._wc_get('products', {'per_page': 100, 'page': page})
            if not products:
                break
            
            for product in products:
                try:
                    product_data = self.transform_product(product)
                    
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
                    print(f" 迁移产品 {product.get('name', 'unknown')} 时出错：{e}")
                    self.migration_log.append({
                        'type': 'product',
                        'operation': 'create',
                        'status': 'error',
                        'source_id': product.get('id'),
                        'name': product.get('name', 'unknown'),
                        'error': str(e)
                    })
            
            page += 1
    
    def transform_product(self, product: dict) -> dict:
        """将 WooCommerce 产品转换为 R Commerce 格式"""
        product_data = {
            'name': product['name'],
            'slug': product['slug'],
            'description': product.get('description', ''),
            'short_description': product.get('short_description', ''),
            'sku': product.get('sku', ''),
            'price': float(product.get('price', 0)),
            'compare_at_price': float(product.get('regular_price', 0)) if product.get('sale_price') else None,
            'inventory_quantity': product.get('stock_quantity', 0) or 0,
            'inventory_policy': 'deny' if product.get('stock_status') == 'outofstock' else 'continue',
            'weight': float(product.get('weight', 0)) if product.get('weight') else None,
            'status': self.map_product_status(product['status']),
            'tags': [tag['name'] for tag in product.get('tags', [])],
            'images': [{'url': img['src'], 'alt': img.get('alt', '')} for img in product.get('images', [])],
            'is_taxable': product.get('tax_status') != 'none',
            'requires_shipping': not product.get('virtual', False),
            'meta_data': {
                'woocommerce': {
                    'product_id': product['id'],
                    'type': product.get('type', 'simple'),
                    'tax_class': product.get('tax_class', ''),
                    'stock_status': product.get('stock_status', ''),
                    'manage_stock': product.get('manage_stock', False),
                    'backorders': product.get('backorders', 'no'),
                    'sold_individually': product.get('sold_individually', False)
                }
            }
        }
        
        # 处理可变产品
        if product.get('type') == 'variable':
            product_data['options'] = self.transform_attributes(product.get('attributes', []))
            product_data['variants'] = self.migrate_variations(product['id'])
        
        return product_data
    
    def transform_attributes(self, attributes: list) -> list:
        """将 WooCommerce 属性转换为 R Commerce 选项"""
        options = []
        for attr in attributes:
            if attr.get('variation', False):
                options.append({
                    'name': attr['name'],
                    'position': attr.get('position', 0),
                    'values': attr.get('options', [])
                })
        return options
    
    def migrate_variations(self, product_id: int) -> list:
        """获取并转换产品变体"""
        variations = self._wc_get(f'products/{product_id}/variations', {'per_page': 100})
        variants = []
        
        for variation in variations:
            variant = {
                'sku': variation.get('sku', ''),
                'price': float(variation.get('price', 0)),
                'regular_price': float(variation.get('regular_price', 0)),
                'sale_price': float(variation.get('sale_price', 0)) if variation.get('sale_price') else None,
                'inventory_quantity': variation.get('stock_quantity', 0) or 0,
                'inventory_policy': 'deny' if variation.get('stock_status') == 'outofstock' else 'continue',
                'weight': float(variation.get('weight', 0)) if variation.get('weight') else None,
                'options': {attr['name']: attr['option'] for attr in variation.get('attributes', [])},
                'meta_data': {
                    'woocommerce': {
                        'variation_id': variation['id'],
                        'virtual': variation.get('virtual', False),
                        'downloadable': variation.get('downloadable', False)
                    }
                }
            }
            variants.append(variant)
        
        return variants
    
    def migrate_customers(self):
        """将 WooCommerce 客户迁移到 R Commerce"""
        page = 1
        while True:
            customers = self._wc_get('customers', {'per_page': 100, 'page': page})
            if not customers:
                break
            
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
            
            page += 1
    
    def transform_customer(self, customer: dict) -> dict:
        """将 WooCommerce 客户转换为 R Commerce 格式"""
        billing = customer.get('billing', {})
        shipping = customer.get('shipping', {})
        
        return {
            'email': customer['email'],
            'first_name': customer.get('first_name', ''),
            'last_name': customer.get('last_name', ''),
            'phone': billing.get('phone', ''),
            'accepts_marketing': customer.get('is_paying_customer', False),
            'billing_address': {
                'first_name': billing.get('first_name', ''),
                'last_name': billing.get('last_name', ''),
                'company': billing.get('company', ''),
                'address1': billing.get('address_1', ''),
                'address2': billing.get('address_2', ''),
                'city': billing.get('city', ''),
                'state': billing.get('state', ''),
                'postal_code': billing.get('postcode', ''),
                'country': billing.get('country', ''),
                'phone': billing.get('phone', '')
            },
            'shipping_address': {
                'first_name': shipping.get('first_name', ''),
                'last_name': shipping.get('last_name', ''),
                'company': shipping.get('company', ''),
                'address1': shipping.get('address_1', ''),
                'address2': shipping.get('address_2', ''),
                'city': shipping.get('city', ''),
                'state': shipping.get('state', ''),
                'postal_code': shipping.get('postcode', ''),
                'country': shipping.get('country', '')
            },
            'meta_data': {
                'woocommerce': {
                    'customer_id': customer['id'],
                    'username': customer.get('username', ''),
                    'role': customer.get('role', ''),
                    'is_paying_customer': customer.get('is_paying_customer', False)
                }
            }
        }
    
    def migrate_orders(self):
        """将 WooCommerce 订单迁移到 R Commerce"""
        page = 1
        while True:
            orders = self._wc_get('orders', {'per_page': 100, 'page': page})
            if not orders:
                break
            
            for order in orders:
                try:
                    order_data = self.transform_order(order)
                    
                    response = requests.post(
                        f"{self.rcommerce_url}/v1/orders",
                        json=order_data,
                        headers={'Authorization': f'Bearer {self.rcommerce_key}'}
                    )
                    
                    if response.status_code == 201:
                        print(f" 已迁移订单：{order['id']}")
                        self.migration_log.append({
                            'type': 'order',
                            'operation': 'create',
                            'status': 'success',
                            'source_id': order['id'],
                            'target_id': response.json()['data']['id']
                        })
                    else:
                        print(f" 迁移订单 {order['id']} 失败：{response.text}")
                    
                    time.sleep(0.5)
                    
                except Exception as e:
                    print(f" 迁移订单 {order.get('id', 'unknown')} 时出错：{e}")
            
            page += 1
    
    def transform_order(self, order: dict) -> dict:
        """将 WooCommerce 订单转换为 R Commerce 格式"""
        return {
            'order_number': str(order['id']),
            'customer_email': order.get('billing', {}).get('email', ''),
            'customer_first_name': order.get('billing', {}).get('first_name', ''),
            'customer_last_name': order.get('billing', {}).get('last_name', ''),
            'status': self.map_order_status(order['status']),
            'subtotal': float(order.get('subtotal', 0)),
            'tax_amount': float(order.get('total_tax', 0)),
            'shipping_amount': float(order.get('shipping_total', 0)),
            'discount_amount': float(order.get('discount_total', 0)),
            'total': float(order.get('total', 0)),
            'billing_address': {
                'first_name': order.get('billing', {}).get('first_name', ''),
                'last_name': order.get('billing', {}).get('last_name', ''),
                'company': order.get('billing', {}).get('company', ''),
                'address1': order.get('billing', {}).get('address_1', ''),
                'address2': order.get('billing', {}).get('address_2', ''),
                'city': order.get('billing', {}).get('city', ''),
                'state': order.get('billing', {}).get('state', ''),
                'postal_code': order.get('billing', {}).get('postcode', ''),
                'country': order.get('billing', {}).get('country', ''),
                'phone': order.get('billing', {}).get('phone', '')
            },
            'shipping_address': {
                'first_name': order.get('shipping', {}).get('first_name', ''),
                'last_name': order.get('shipping', {}).get('last_name', ''),
                'company': order.get('shipping', {}).get('company', ''),
                'address1': order.get('shipping', {}).get('address_1', ''),
                'address2': order.get('shipping', {}).get('address_2', ''),
                'city': order.get('shipping', {}).get('city', ''),
                'state': order.get('shipping', {}).get('state', ''),
                'postal_code': order.get('shipping', {}).get('postcode', ''),
                'country': order.get('shipping', {}).get('country', '')
            },
            'line_items': [self.transform_line_item(item) for item in order.get('line_items', [])],
            'meta_data': {
                'woocommerce': {
                    'order_id': order['id'],
                    'order_key': order.get('order_key', ''),
                    'payment_method': order.get('payment_method', ''),
                    'payment_method_title': order.get('payment_method_title', ''),
                    'transaction_id': order.get('transaction_id', ''),
                    'date_created': order.get('date_created', ''),
                    'date_modified': order.get('date_modified', '')
                }
            }
        }
    
    def transform_line_item(self, item: dict) -> dict:
        """转换订单行项目"""
        return {
            'product_id': str(item.get('product_id', '')),
            'name': item.get('name', ''),
            'sku': item.get('sku', ''),
            'quantity': item.get('quantity', 0),
            'unit_price': float(item.get('price', 0)),
            'total': float(item.get('total', 0)),
            'meta_data': {
                'woocommerce': {
                    'variation_id': item.get('variation_id'),
                    'tax_class': item.get('tax_class', '')
                }
            }
        }
    
    def map_product_status(self, wc_status: str) -> str:
        """将 WooCommerce 产品状态映射到 R Commerce 状态"""
        status_map = {
            'publish': 'active',
            'draft': 'draft',
            'private': 'archived',
            'pending': 'draft'
        }
        return status_map.get(wc_status, 'draft')
    
    def map_order_status(self, wc_status: str) -> str:
        """将 WooCommerce 订单状态映射到 R Commerce 状态"""
        status_map = {
            'pending': 'pending',
            'processing': 'processing',
            'on-hold': 'on_hold',
            'completed': 'completed',
            'cancelled': 'cancelled',
            'refunded': 'refunded',
            'failed': 'failed'
        }
        return status_map.get(wc_status, 'pending')
    
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
    wc_config = {
        'url': os.environ.get('WC_URL', 'https://your-store.com'),
        'consumer_key': os.environ.get('WC_CONSUMER_KEY', 'your_consumer_key'),
        'consumer_secret': os.environ.get('WC_CONSUMER_SECRET', 'your_consumer_secret')
    }
    
    rcommerce_config = {
        'url': os.environ.get('RCOMMERCE_URL', 'https://api.yourstore.com'),
        'api_key': os.environ.get('RCOMMERCE_API_KEY', 'your_api_key')
    }
    
    migrator = WooCommerceMigrator(wc_config, rcommerce_config)
    migrator.migrate_all()
```

## 插件数据迁移

WooCommerce 将扩展数据存储在各个位置。通过 REST API 或 WP-CLI 访问这些数据：

### 常见插件数据访问

**WooCommerce 订阅：**
```bash
# 使用 WP-CLI 导出订阅
wp wc shop_subscription list --user=1 --format=json > subscriptions.json
```

**WooCommerce 预订：**
```bash
# 导出预订
wp wc booking list --user=1 --format=json > bookings.json
```

**WooCommerce 会员：**
```bash
# 导出会员
wp wc user_membership list --user=1 --format=json > memberships.json
```

**Yoast SEO：**
```php
<?php
// 通过 WordPress 函数访问 Yoast SEO 数据
$yoast_title = get_post_meta($product_id, '_yoast_wpseo_title', true);
$yoast_description = get_post_meta($product_id, '_yoast_wpseo_metadesc', true);
$yoast_focus_keyword = get_post_meta($product_id, '_yoast_wpseo_focuskw', true);
```

**Advanced Custom Fields：**
```php
<?php
// 通过 WordPress 函数访问 ACF 字段
if (function_exists('get_fields')) {
    $acf_fields = get_fields($product_id);
    // 处理 ACF 字段以进行迁移
}
```

### 插件迁移策略

```php
<?php
// migrate-plugin-data.php

class WooCommercePluginDataMigrator {
  
  public function migrateSubscriptionData($rcommerceProductId, $woocommerceProductId) {
    // 检查 WooCommerce 订阅是否激活
    if (!class_exists('WC_Subscriptions')) {
      return null;
    }
    
    // 通过 API 或 WP-CLI 导出获取此产品的订阅
    $subscriptions = $this->getProductSubscriptions($woocommerceProductId);
    
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
  
  public function migrateBookingData($rcommerceProductId, $woocommerceProductId) {
    if (!class_exists('WC_Bookings')) {
      return null;
    }
    
    $bookable_product = new WC_Product_Booking($woocommerceProductId);
    
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
