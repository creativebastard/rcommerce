# Shopify 迁移到 R Commerce 指南

## 概述

本指南提供使用兼容层和直接迁移方法从 Shopify 迁移到 R Commerce 的分步说明。

## 迁移前检查清单

### Shopify 店铺审计

```bash
# 统计产品数量
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/products/count.json" \
  -H "X-Shopify-Access-Token: YOUR_ADMIN_API_TOKEN"

# 统计客户数量
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/customers/count.json" \
  -H "X-Shopify-Access-Token: YOUR_ADMIN_API_TOKEN"

# 统计订单数量
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/orders/count.json" \
  -H "X-Shopify-Access-Token: YOUR_ADMIN_API_TOKEN" \
  -H "Content-Type: application/json"
```

### 导出 Shopify 数据

#### 方法 1：使用 Shopify Admin API

```bash
#!/bin/bash
# export-shopify.sh

SHOP_DOMAIN="your-store.myshopify.com"
API_TOKEN="YOUR_ADMIN_API_TOKEN"

# 创建导出目录
mkdir -p shopify-export

# 导出产品（带分页）
echo "导出产品..."
curl -s -X GET "https://${SHOP_DOMAIN}/admin/api/2024-01/products.json?limit=250" \
  -H "X-Shopify-Access-Token: ${API_TOKEN}" | \
  jq '.' > shopify-export/products.json

# 导出客户
echo "导出客户..."
curl -s -X GET "https://${SHOP_DOMAIN}/admin/api/2024-01/customers.json?limit=250" \
  -H "X-Shopify-Access-Token: ${API_TOKEN}" | \
  jq '.' > shopify-export/customers.json

# 导出订单
echo "导出订单..."
curl -s -X GET "https://${SHOP_DOMAIN}/admin/api/2024-01/orders.json?status=any&limit=250" \
  -H "X-Shopify-Access-Token: ${API_TOKEN}" | \
  jq '.' > shopify-export/orders.json

# 导出集合/智能集合
echo "导出集合..."
curl -s -X GET "https://${SHOP_DOMAIN}/admin/api/2024-01/smart_collections.json?limit=250" \
  -H "X-Shopify-Access-Token: ${API_TOKEN}" | \
  jq '.' > shopify-export/collections.json

# 导出自定义集合
curl -s -X GET "https://${SHOP_DOMAIN}/admin/api/2024-01/custom_collections.json?limit=250" \
  -H "X-Shopify-Access-Token: ${API_TOKEN}" | \
  jq '.' >> shopify-export/collections.json

echo "导出完成！"
```

#### 方法 2：使用 Shopify CLI

```bash
# 安装 Shopify CLI
npm install -g @shopify/cli

# 登录您的店铺
shopify login --store your-store.myshopify.com

# 导出产品
shopify product list --json > products.json

# 导出客户
shopify customer list --json > customers.json
```

## 迁移策略

### 选项 1：兼容层（零停机）

此方法使用 R Commerce 的 Shopify 兼容层同时运行两个平台。

```toml
# config/compatibility.toml
[compatibility]
enabled = true
source_platform = "shopify"

[compatibility.shopify]
api_key = "YOUR_SHOPIFY_API_KEY"
store_domain = "your-store.myshopify.com"
shared_inventory = true
sync_orders = true
```

**优势：**
- 零停机迁移
- 逐步测试 R Commerce 功能
- 保留 Shopify 作为备份
- 平台间库存同步

**设置：**

1. 部署带兼容层的 R Commerce
2. 配置 Shopify API 凭证
3. 启用双向同步
4. 逐步将功能迁移到 R Commerce
5. 准备就绪时更新 DNS

### 选项 2：大爆炸迁移

此方法一次性迁移所有数据并完全切换。

## 数据映射

### 产品数据映射

| Shopify 字段 | R Commerce 字段 | 转换 |
|--------------|-----------------|------|
| `id` | `meta_data.shopify_id` | 存储引用 |
| `title` | `name` | 直接映射 |
| `body_html` | `description` | 直接映射 |
| `vendor` | `meta_data.vendor` | 存储为元数据 |
| `product_type` | `category` | 映射到分类 |
| `tags` | `tags` | 直接映射 |
| `handle` | `slug` | 直接映射 |
| `variants` | `variants` | 见下文 |
| `images` | `images` | 带位置映射 |
| `status` | `status` | `active`→`active`, `draft`→`draft` |
| `published_scope` | `meta_data.published_scope` | 存储为元数据 |

### 变体映射

```javascript
// Shopify 变体 → R Commerce 变体
{
  // Shopify
  "id": "gid://shopify/ProductVariant/123",
  "sku": "SHIRT-RED-L",
  "price": "29.99",
  "compare_at_price": "39.99",
  "inventory_quantity": 100,
  "inventory_policy": "deny",
  "option1": "Red",
  "option2": "Large",
  "barcode": "1234567890",
  "weight": 0.2,
  "weight_unit": "kg",
  "image_id": "gid://shopify/ProductImage/456",
  
  // R Commerce
  "sku": "SHIRT-RED-L",
  "price": 29.99,
  "compare_at_price": 39.99,
  "cost": null, // Shopify 没有成本字段
  "inventory_quantity": 100,
  "inventory_policy": "deny", // Shopify: deny/continue
  "weight": 0.2,
  "weight_unit": "kg",
  "barcode": "1234567890"
}
```

### 客户映射

| Shopify 字段 | R Commerce 字段 | 说明 |
|--------------|-----------------|------|
| `id` | `meta_data.shopify_id` | 仅引用 |
| `email` | `email` | 主要标识符 |
| `first_name` | `first_name` | 直接映射 |
| `last_name` | `last_name` | 直接映射 |
| `phone` | `phone` | 直接映射 |
| `state` | `status` | `enabled` → `active` |
| `tags` | `tags` | 直接映射 |
| `addresses` | `addresses` | 数组映射 |
| `orders_count` | `orders_count` | 直接映射 |
| `total_spent` | `total_spent` | 直接映射 |
| `email_marketing_consent` | `accepts_marketing` | 映射同意状态 |

### 订单映射

**重要说明：**
- Shopify 订单创建后不可变
- R Commerce 允许订单编辑（有限制）
- 财务状态映射至关重要
- 配送状态映射到 R Commerce 状态

```javascript
// Shopify 订单 → R Commerce 订单
{
  // 将 financial_status 映射到 R Commerce payment_status
  shopify: {
    financial_status: "paid" // "pending", "authorized", "partially_refunded", "refunded", "voided"
  },
  
  r_commerce: {
    payment_status: "paid" // "pending", "authorized", "paid", "partially_refunded", "fully_refunded", "failed"
  }
}
```

## 迁移脚本

### 产品迁移 Ruby 脚本

```ruby
#!/usr/bin/env ruby
# migrate_products.rb

require 'json'
require 'net/http'
require 'uri'

class ShopifyToRCommerce
  def initialize(shopify_domain, shopify_token, rcommerce_url, rcommerce_key)
    @shopify_domain = shopify_domain
    @shopify_token = shopify_token
    @rcommerce_url = rcommerce_url
    @rcommerce_key = rcommerce_key
    
    @http = Net::HTTP.new(URI.parse(@rcommerce_url).host, URI.parse(@rcommerce_url).port)
    @http.use_ssl = true
  end
  
  def migrate_products
    shopify_products = fetch_shopify_products
    
    shopify_products.each do |shopify_product|
      begin
        rcommerce_product = transform_product(shopify_product)
        create_rcommerce_product(rcommerce_product)
        puts " 已迁移产品：#{shopify_product['title']}"
      rescue => e
        puts " 迁移产品失败：#{shopify_product['title']} - #{e.message}"
      end
    end
  end
  
  private
  
  def fetch_shopify_products
    uri = URI("https://#{@shopify_domain}/admin/api/2024-01/products.json")
    uri.query = URI.encode_www_form(limit: 250)
    
    request = Net::HTTP::Get.new(uri)
    request['X-Shopify-Access-Token'] = @shopify_token
    
    response = @http.request(request)
    
    if response.code == '200'
      JSON.parse(response.body)['products']
    else
      raise "获取产品失败：#{response.code} - #{response.body}"
    end
  end
  
  def transform_product(shopify_product)
    {
      name: shopify_product['title'],
      slug: shopify_product['handle'],
      description: shopify_product['body_html'],
      short_description: shopify_product['body_html']&.truncate(200),
      sku: shopify_product['variants'].first['sku'],
      price: shopify_product['variants'].first['price'].to_f,
      compare_at_price: shopify_product['variants'].first['compare_at_price']&.to_f,
      inventory_quantity: shopify_product['variants'].first['inventory_quantity'],
      inventory_policy: shopify_product['variants'].first['inventory_policy'],
      status: map_status(shopify_product['status']),
      weight: shopify_product['variants'].first['weight']&.to_f,
      weight_unit: shopify_product['variants'].first['weight_unit'],
      tags: shopify_product['tags'],
      meta_data: {
        shopify_id: shopify_product['id'],
        vendor: shopify_product['vendor'],
        product_type: shopify_product['product_type'],
        published_at: shopify_product['published_at']
      },
      variants: transform_variants(shopify_product['variants']),
      images: transform_images(shopify_product['images'])
    }
  end
  
  def transform_variants(shopify_variants)
    shopify_variants.map do |variant|
      {
        sku: variant['sku'],
        price: variant['price'].to_f,
        compare_at_price: variant['compare_at_price']&.to_f,
        inventory_quantity: variant['inventory_quantity'],
        inventory_policy: variant['inventory_policy'],
        weight: variant['weight']&.to_f,
        weight_unit: variant['weight_unit'],
        barcode: variant['barcode'],
        meta_data: {
          shopify_id: variant['id'],
          option1: variant['option1'],
          option2: variant['option2'],
          option3: variant['option3']
        }
      }
    end
  end
  
  def transform_images(shopify_images)
    shopify_images.map do |image|
      {
        url: image['src'],
        alt: image['alt'],
        position: image['position']
      }
    end
  end
  
  def map_status(shopify_status)
    case shopify_status
    when 'active'
      'active'
    when 'draft'
      'draft'
    when 'archived'
      'archived'
    else
      'draft'
    end
  end
  
  def create_rcommerce_product(product_data)
    uri = URI("#{@rcommerce_url}/v1/products")
    
    request = Net::HTTP::Post.new(uri)
    request['Authorization'] = "Bearer #{@rcommerce_key}"
    request['Content-Type'] = 'application/json'
    request.body = product_data.to_json
    
    response = @http.request(request)
    
    if response.code != '201'
      raise "创建产品失败：#{response.code} - #{response.body}"
    end
    
    JSON.parse(response.body)
  end
end

# 使用
migrator = ShopifyToRCommerce.new(
  'your-store.myshopify.com',
  'YOUR_SHOPIFY_ADMIN_TOKEN',
  'https://api.yourstore.com',
  'YOUR_RCOMMERCE_API_KEY'
)

migrator.migrate_products
```

### 客户迁移 Node.js 脚本

```javascript
// migrate-customers.js
const axios = require('axios');
const fs = require('fs').promises;

class ShopifyCustomerMigrator {
  constructor(shopifyDomain, shopifyToken, rcommerceUrl, rcommerceKey) {
    this.shopifyClient = axios.create({
      baseURL: `https://${shopifyDomain}/admin/api/2024-01`,
      headers: {
        'X-Shopify-Access-Token': shopifyToken,
        'Content-Type': 'application/json'
      }
    });
    
    this.rcommerceClient = axios.create({
      baseURL: rcommerceUrl,
      headers: {
        'Authorization': `Bearer ${rcommerceKey}`,
        'Content-Type': 'application/json'
      }
    });
    
    this.migrationLog = [];
  }
  
  async migrateAll() {
    try {
      console.log('从 Shopify 获取客户...');
      const customers = await this.fetchAllCustomers();
      
      console.log(`找到 ${customers.length} 个客户。开始迁移...`);
      
      for (const [index, customer] of customers.entries()) {
        try {
          await this.migrateCustomer(customer);
          console.log(` 已迁移 ${index + 1}/${customers.length}：${customer.email}`);
        } catch (error) {
          console.error(` 迁移 ${customer.email} 失败：`, error.message);
          this.migrationLog.push({
            customer: customer.email,
            status: 'failed',
            error: error.message
          });
        }
      }
      
      await this.saveMigrationLog();
      console.log('迁移完成！');
      
    } catch (error) {
      console.error('迁移失败：', error);
    }
  }
  
  async fetchAllCustomers() {
    let allCustomers = [];
    let pageInfo = null;
    
    do {
      const params = {
        limit: 250,
        ...(pageInfo && { page_info: pageInfo })
      };
      
      const response = await this.shopifyClient.get('/customers.json', { params });
      allCustomers = allCustomers.concat(response.data.customers);
      
      // 从 Link header 获取下一页
      const linkHeader = response.headers.link;
      if (linkHeader) {
        const nextLink = linkHeader.split(',').find(link => link.includes('rel="next"'));
        if (nextLink) {
          const match = nextLink.match(/page_info=([^>]+)/);
          pageInfo = match ? match[1] : null;
        } else {
          pageInfo = null;
        }
      } else {
        pageInfo = null;
      }
    } while (pageInfo);
    
    return allCustomers;
  }
  
  transformCustomer(shopifyCustomer) {
    const defaultAddress = shopifyCustomer.default_address;
    
    return {
      email: shopifyCustomer.email,
      first_name: shopifyCustomer.first_name,
      last_name: shopifyCustomer.last_name,
      phone: shopifyCustomer.phone,
      accepts_marketing: shopifyCustomer.email_marketing_consent?.state === 'subscribed',
      meta_data: {
        shopify_id: shopifyCustomer.id,
        state: shopifyCustomer.state,
        orders_count: shopifyCustomer.orders_count,
        total_spent: shopifyCustomer.total_spent,
        tax_exempt: shopifyCustomer.tax_exempt,
        tags: shopifyCustomer.tags
      },
      addresses: shopifyCustomer.addresses.map(addr => ({
        first_name: addr.first_name,
        last_name: addr.last_name,
        company: addr.company,
        street1: addr.address1,
        street2: addr.address2,
        city: addr.city,
        state: addr.province,
        postal_code: addr.zip,
        country: addr.country_code,
        phone: addr.phone,
        is_default: addr.default
      }))
    };
  }
  
  async migrateCustomer(shopifyCustomer) {
    const rcommerceCustomer = this.transformCustomer(shopifyCustomer);
    
    try {
      const response = await this.rcommerceClient.post('/v1/customers', rcommerceCustomer);
      
      this.migrationLog.push({
        email: shopifyCustomer.email,
        status: 'success',
        rcommerce_id: response.data.data.id,
        shopify_id: shopifyCustomer.id
      });
      
      return response.data;
    } catch (error) {
      if (error.response?.data?.error?.code === 'customer_duplicate_email') {
        // 处理重复客户
        console.log(`客户 ${shopifyCustomer.email} 已存在，跳过...`);
        this.migrationLog.push({
          email: shopifyCustomer.email,
          status: 'skipped',
          reason: 'duplicate'
        });
      } else {
        throw error;
      }
    }
  }
  
  async saveMigrationLog() {
    await fs.writeFile(
      'customer-migration-log.json',
      JSON.stringify(this.migrationLog, null, 2)
    );
  }
}

// 使用
const migrator = new ShopifyCustomerMigrator(
  'your-store.myshopify.com',
  process.env.SHOPIFY_ADMIN_TOKEN,
  'https://api.yourstore.com',
  process.env.RCOMMERCE_API_KEY
);

migrator.migrateAll().catch(console.error);
```

## SEO 和重定向

### URL 重定向映射

创建从 Shopify URL 到 R Commerce URL 的重定向：

```ruby
#!/usr/bin/env ruby
# generate_redirects.rb

require 'json'

# 加载导出的产品
products = JSON.parse(File.read('shopify-export/products.json'))

# 生成 Nginx 重定向规则
File.open('nginx-redirects.conf', 'w') do |file|
  products.each do |product|
    shopify_handle = product['handle']
    shopify_id = product['id']
    
    # 假设您有 Shopify ID 到 R Commerce ID 的映射
    # 您需要从迁移日志中创建此映射
    rcommerce_id = get_rcommerce_product_id(shopify_id)
    
    next unless rcommerce_id
    
    # 生成重定向
    file.puts "# #{product['title']} 的重定向"
    file.puts "location = /products/#{shopify_handle} {"
    file.puts "  return 301 /products/#{rcommerce_id};"
    file.puts "}"
    file.puts ""
  end
end
```

### Nginx 重定向配置

```nginx
# 添加到您的 Nginx 配置
server {
    listen 80;
    server_name your-store.myshopify.com;
    
    # 产品重定向
    location ~ ^/products/(.+)$ {
        return 301 https://your-new-store.com/products/$1;
    }
    
    # 集合重定向
    location ~ ^/collections/(.+)$ {
        return 301 https://your-new-store.com/collections/$1;
    }
    
    # 页面重定向
    location ~ ^/pages/(.+)$ {
        return 301 https://your-new-store.com/pages/$1;
    }
    
    # 博客重定向
    location ~ ^/blogs/(.+)$ {
        return 301 https://your-new-store.com/blog/$1;
    }
    
    # 购物车重定向
    location /cart {
        return 301 https://your-new-store.com/cart;
    }
    
    # 结账重定向
    location /checkout {
        return 301 https://your-new-store.com/checkout;
    }
    
    # 账户重定向
    location ~ ^/account(.+)$ {
        return 301 https://your-new-store.com/account$1;
    }
}
```

## 迁移后检查清单

迁移后验证以下内容：

### 产品
- [ ] 所有产品已迁移（数量匹配）
- [ ] 产品详情正确（名称、描述、价格）
- [ ] 图片迁移成功
- [ ] 变体/选项正确
- [ ] 库存数量准确
- [ ] SKU 保留
- [ ] SEO 数据（元标题、描述）已迁移
- [ ] 产品 URL 重定向正确

### 客户
- [ ] 所有客户已迁移（数量匹配）
- [ ] 邮箱地址保留
- [ ] 配送地址正确
- [ ] 客户组恢复（如适用）
- [ ] 营销偏好保留
- [ ] 发送密码重置邮件

### 订单
- [ ] 订单历史可访问（如已迁移）
- [ ] 订单号保留（或已创建重定向映射）
- [ ] 订单详情准确
- [ ] 支付历史完整
- [ ] 配送数据准确
- [ ] 退款信息保留

### 设置
- [ ] 支付网关已配置
- [ ] 配送方式已设置
- [ ] 税率已配置
- [ ] 邮件模板已自定义
- [ ] 通知偏好已设置

### 集成
- [ ] 支付 Webhooks 已配置
- [ ] 物流提供商已连接
- [ ] 邮件营销已集成
- [ ] 分析跟踪正常工作
- [ ] CRM 已连接

### 测试
- [ ] 店铺前台正确加载
- [ ] 产品可搜索
- [ ] 加入购物车功能正常
- [ ] 结账流程可用
- [ ] 支付处理正常
- [ ] 发送订单确认
- [ ] 账户登录功能正常
- [ ] 密码重置功能正常
- [ ] 发送邮件通知

### 性能
- [ ] 页面加载时间可接受（<3秒）
- [ ] 搜索快速返回结果
- [ ] 结账在高负载下表现良好
- [ ] 数据库查询已优化

## 回滚计划

如果迁移失败：

1. **恢复 DNS 变更**：指回 Shopify
2. **保留 Shopify 店铺**：不要立即取消
3. **数据备份**：保留导出的数据
4. **记录问题**：记录出错内容
5. **修复后重试**：解决问题后重试

## 技巧和最佳实践

### 1. 从小开始
- 先迁移几个产品
- 测试结账流程
- 验证集成
- 然后进行完整迁移

### 2. 使用 Shopify Plus（如可用）
- 更高的 API 速率限制
- 批量操作 API
- 更好的支持

### 3. 精心安排迁移时间
- 工作日上午效果好
- 避开节假日季
- 大型店铺预留 2-4 小时

### 4. 监控性能
- 跟踪 API 速率限制
- 监控数据库性能
- 注意慢查询

### 5. 保留 SEO
- 重定向每个 URL
- 提交新站点地图
- 监控搜索控制台
- 预期 2-4 周排名波动

### 6. 客户沟通
- 提前通知客户
- 解释新平台的优势
- 提供清晰的说明
- 过渡期提供支持

### 7. 员工培训
- 培训新管理界面
- 记录新流程
- 测试订单管理工作流
- 准备客服脚本

## 故障排除

### 速率限制问题

```bash
# 检查当前速率限制
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/shop.json" \
  -H "X-Shopify-Access-Token: YOUR_TOKEN" \
  -i | grep -i "X-Shopify-Shop-Api-Call-Limit"

# 请求之间添加延迟
sleep 0.5

# 对大数据集使用批量操作
```

### 内存问题

```bash
# 分小块处理
# 不要一次性处理所有产品，每次处理 100 个

# 使用分页
page = 1
per_page = 100
loop do
  products = fetch_products(page: page, per_page: per_page)
  break if products.empty?
  
  products.each { |p| migrate_product(p) }
  page += 1
end
```

### 数据验证失败

```bash
# 记录验证错误
each product do
  begin
    migrate_product(product)
  rescue ValidationError => e
    puts "验证失败 #{product['id']}：#{e.message}"
    puts "数据：#{product.inspect}"
    next # 跳到下一个
  end
end
```

## 支持和资源

- **Shopify API 文档**：https://shopify.dev/docs/api/admin
- **Shopify API 速率限制**：https://shopify.dev/docs/api/usage/rate-limits
- **R Commerce API 文档**：参见 docs/api/01-api-design.md
- **R Commerce Discord**：https://discord.gg/rcommerce
- **专业帮助**：migration@rcommerce.app

## 示例：带监控的完整迁移

```javascript
// complete-migration.js
const { EventEmitter } = require('events');
const fs = require('fs').promises;

class MonitoredMigration extends EventEmitter {
  constructor(config) {
    super();
    this.config = config;
    this.stats = {
      products: { success: 0, failed: 0, skipped: 0 },
      customers: { success: 0, failed: 0, skipped: 0 },
      orders: { success: 0, failed: 0, skipped: 0 }
    };
    this.startTime = Date.now();
  }
  
  async run() {
    this.emit('start', this.stats);
    
    try {
      // 阶段 1：产品
      await this.migrateProducts();
      
      // 阶段 2：客户
      await this.migrateCustomers();
      
      // 阶段 3：订单（可选）
      if (this.config.migrateOrders) {
        await this.migrateOrders();
      }
      
      this.emit('complete', this.stats);
      
    } catch (error) {
      this.emit('error', error);
      throw error;
    }
  }
  
  async migrateProducts() {
    this.emit('phase-start', { phase: 'products' });
    
    try {
      // 带监控的增强产品迁移
      // ... 带进度更新的实现
    } finally {
      this.emit('phase-complete', { 
        phase: 'products',
        stats: this.stats.products 
      });
    }
  }
}

// 带监控的使用
const migration = new MonitoredMigration(config);

migration.on('start', (stats) => {
  console.log('迁移开始：', stats);
});

migration.on('phase-start', ({ phase }) => {
  console.log(`开始 ${phase} 迁移...`);
});

migration.on('phase-complete', ({ phase, stats }) => {
  console.log(`${phase} 迁移完成：`, stats);
  
  // 发送通知
  sendSlackNotification(`${phase} 迁移：${JSON.stringify(stats)}`);
});

migration.on('error', (error) => {
  console.error('迁移失败：', error);
  sendSlackNotification(`❌ 迁移失败：${error.message}`);
});

migration.on('complete', (stats) => {
  console.log('迁移成功完成！', stats);
  sendSlackNotification(` 迁移完成：${JSON.stringify(stats)}`);
});

migration.run().catch(console.error);
```

本指南提供了从 Shopify 迁移到 R Commerce 的全面方法。根据您的具体需求和店铺复杂度调整脚本和配置。
