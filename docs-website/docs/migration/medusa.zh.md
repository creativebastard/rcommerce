# Medusa.js 迁移到 R Commerce 指南

## 概述

由于架构相似性，从 Medusa.js 迁移到 R Commerce 是最直接的迁移。两者都是基于现代技术栈构建的无头、API 优先平台。本指南侧重于利用 API 兼容性和直接数据映射。

## 迁移前分析

### 审计 Medusa.js 店铺

```javascript
// audit-medusa.js
const axios = require('axios');

const medusaClient = axios.create({
  baseURL: 'https://api.your-medusa-store.com',
  headers: {
    'x-medusa-access-token': process.env.MEDUSA_API_KEY
  }
});

async function auditMedusaStore() {
  console.log('审计 Medusa.js 店铺...');
  
  // 产品
  const products = await medusaClient.get('/store/products?limit=1');
  console.log(`产品：${products.data.count || '未知'}`);
  
  // 客户
  const customers = await medusaClient.get('/admin/customers?limit=1');
  console.log(`客户：${customers.data.count || '未知'}`);
  
  // 订单
  const orders = await medusaClient.get('/admin/orders?limit=1');
  console.log(`订单：${orders.data.count || '未知'}`);
  
  // 区域（Medusa 特定）
  const regions = await medusaClient.get('/admin/regions?limit=1');
  console.log(`区域：${regions.data.count || '未知'}`);
  
  // 销售渠道
  const channels = await medusaClient.get('/admin/sales-channels?limit=1');
  console.log(`销售渠道：${channels.data.count || '未知'}`);
  
  // 价格表
  const priceLists = await medusaClient.get('/admin/price-lists?limit=1');
  console.log(`价格表：${priceLists.data.count || '未知'}`);
  
  console.log('审计完成！');
}

auditMedusaStore().catch(console.error);
```

### Medusa.js 数据结构映射

| Medusa.js 概念 | R Commerce 等效 | 说明 |
|----------------|-----------------|------|
| **Regions** | Shipping Zones + Currencies | 直接映射 |
| **Sales Channels** | 不直接支持 | 使用多店铺或标签 |
| **Price Lists** | Customer Groups + Discounts | 类似功能 |
| **Swaps/Returns** | Returns API | R Commerce 有更简单的模型 |
| **Claims** | 不直接支持 | 使用自定义实现 |
| **Draft Orders** | Manual Orders | 类似功能 |
| **Published API Keys** | API Keys | R Commerce 有不同系统 |
| **Gift Cards** | Gift Cards | R Commerce 支持 |
| **Discount Conditions** | Discount Rules | R Commerce 有不同引擎 |

## 导出 Medusa.js 数据

### 使用 Medusa.js Admin API

```javascript
// export-medusa.js
const fs = require('fs').promises;
const path = require('path');

class MedusaExporter {
  constructor(config) {
    this.client = axios.create({
      baseURL: config.url,
      headers: {
        'x-medusa-access-token': config.api_key
      }
    });
    this.exportDir = path.join(process.cwd(), 'medusa-export');
  }
  
  async exportAll() {
    await fs.mkdir(this.exportDir, { recursive: true });
    
    console.log('导出 Medusa.js 数据...');
    
    await this.exportProducts();
    await this.exportCustomers();
    await this.exportOrders();
    await this.exportRegions();
    await this.exportDiscounts();
    await this.exportPriceLists();
    
    console.log('导出完成！');
  }
  
  async exportProducts() {
    console.log('导出产品...');
    let page = 0;
    let hasMore = true;
    const allProducts = [];
    
    while (hasMore) {
      const response = await this.client.get(`/admin/products`, {
        params: { limit: 100, offset: page * 100 }
      });
      
      const products = response.data.products;
      allProducts.push(...products);
      
      hasMore = products.length === 100;
      page++;
      
      console.log(`  - 第 ${page} 页：${products.length} 个产品`);
    }
    
    await fs.writeFile(
      path.join(this.exportDir, 'products.json'),
      JSON.stringify(allProducts, null, 2)
    );
    
    console.log(` 已导出 ${allProducts.length} 个产品`);
  }
  
  async exportCustomers() {
    console.log('导出客户...');
    let page = 0;
    let hasMore = true;
    const allCustomers = [];
    
    while (hasMore) {
      const response = await this.client.get(`/admin/customers`, {
        params: { limit: 100, offset: page * 100 }
      });
      
      const customers = response.data.customers;
      allCustomers.push(...customers);
      
      hasMore = customers.length === 100;
      page++;
      
      console.log(`  - 第 ${page} 页：${customers.length} 个客户`);
    }
    
    await fs.writeFile(
      path.join(this.exportDir, 'customers.json'),
      JSON.stringify(allCustomers, null, 2)
    );
    
    console.log(` 已导出 ${allCustomers.length} 个客户`);
  }
  
  async exportOrders() {
    console.log('导出订单...');
    let page = 0;
    let hasMore = true;
    const allOrders = [];
    
    while (hasMore) {
      const response = await this.client.get(`/admin/orders`, {
        params: { limit: 100, offset: page * 100 }
      });
      
      const orders = response.data.orders;
      allOrders.push(...orders);
      
      hasMore = orders.length === 100;
      page++;
      
      console.log(`  - 第 ${page} 页：${orders.length} 个订单`);
    }
    
    await fs.writeFile(
      path.join(this.exportDir, 'orders.json'),
      JSON.stringify(allOrders, null, 2)
    );
    
    console.log(` 已导出 ${allOrders.length} 个订单`);
  }
  
  async exportRegions() {
    console.log('导出区域...');
    const response = await this.client.get(`/admin/regions`);
    const regions = response.data.regions;
    
    await fs.writeFile(
      path.join(this.exportDir, 'regions.json'),
      JSON.stringify(regions, null, 2)
    );
    
    console.log(` 已导出 ${regions.length} 个区域`);
  }
  
  async exportDiscounts() {
    console.log('导出折扣...');
    let page = 0;
    let hasMore = true;
    const allDiscounts = [];
    
    while (hasMore) {
      const response = await this.client.get(`/admin/discounts`, {
        params: { limit: 100, offset: page * 100 }
      });
      
      const discounts = response.data.discounts;
      allDiscounts.push(...discounts);
      
      hasMore = discounts.length === 100;
      page++;
    }
    
    await fs.writeFile(
      path.join(this.exportDir, 'discounts.json'),
      JSON.stringify(allDiscounts, null, 2)
    );
    
    console.log(` 已导出 ${allDiscounts.length} 个折扣`);
  }
  
  async exportPriceLists() {
    console.log('导出价格表...');
    let page = 0;
    let hasMore = true;
    const allPriceLists = [];
    
    while (hasMore) {
      const response = await this.client.get(`/admin/price-lists`, {
        params: { limit: 100, offset: page * 100 }
      });
      
      const priceLists = response.data.price_lists;
      allPriceLists.push(...priceLists);
      
      hasMore = priceLists.length === 100;
      page++;
    }
    
    await fs.writeFile(
      path.join(this.exportDir, 'price-lists.json'),
      JSON.stringify(allPriceLists, null, 2)
    );
    
    console.log(` 已导出 ${allPriceLists.length} 个价格表`);
  }
}

// 使用
const exporter = new MedusaExporter({
  url: 'https://api.your-medusa-store.com',
  api_key: process.env.MEDUSA_API_KEY
});

exporter.exportAll().catch(console.error);
```

## 数据映射参考

### 产品映射（简单产品）

```javascript
// Medusa 产品 → R Commerce 产品
{
  // Medusa
  id: "prod_123",
  title: "T-Shirt",
  subtitle: "Premium Cotton",
  description: "High quality t-shirt",
  handle: "t-shirt-premium",
  is_giftcard: false,
  status: "published",
  images: [{ url: "...", metadata: {} }],
  thumbnail: "https://...",
  options: [{ id: "opt_1", title: "Size", values: ["S", "M", "L"] }],
  variants: [
    {
      id: "var_1",
      title: "S",
      prices: [{ currency_code: "usd", amount: 1999 }],
      options: [{ option_id: "opt_1", value: "S" }],
      inventory_quantity: 100,
      manage_inventory: true
    }
  ],
  tags: [{ value: "clothing" }],
  type: { value: "shirt" },
  collection_id: "col_123",
  
  // R Commerce 等效
  id: "generated_uuid",
  name: "T-Shirt",
  slug: "t-shirt-premium",
  description: "High quality t-shirt",
  sku: "TSHIRT-001",
  price: 19.99,
  status: "active", // published → active
  images: [{ url: "...", alt: "", position: 0 }],
  category_id: "category_uuid",
  tags: ["clothing", "shirt"], // tags + type
  inventory_quantity: 100, // 从默认变体
  inventory_policy: "deny",
  requires_shipping: true,
  is_gift_card: false,
  variants: [
    {
      id: "variant_uuid",
      title: "S",
      price: 19.99,
      inventory_quantity: 100,
      options: [{ name: "Size", value: "S" }]
    }
  ]
}
```

### 区域映射

Medusa 区域很复杂 - 它们处理货币、税费、支付提供商和配送：

```javascript
// Medusa 区域 → R Commerce 映射
{
  // Medusa 区域
  id: "reg_01",
  name: "United States",
  currency_code: "usd",
  tax_rate: 0,
  tax_code: null,
  gift_cards_taxable: true,
  automatic_taxes: true,
  payment_providers: ["stripe"],
  fulfillment_providers: ["manual"],
  countries: [{ iso_2: "us", display_name: "United States" }],
  
  // R Commerce 映射（多个配置）
  // 1. 货币配置
  ["currencies"]
  supported_currencies = ["USD"]
  default_currency = "USD"
  
  // 2. 税费配置
  ["tax_rates"]
  [[tax_rates]]
  name = "US Tax"
  rate = 0.0
  country = "US"
  
  // 3. 支付提供商
  ["payments.stripe"]
  enabled = true
  secret_key = "sk_..."
  
  // 4. 配送区域
  [[shipping.zones]]
  name = "United States"
  countries = ["US"]
}
```

### 客户组映射

```javascript
// Medusa 客户组 → R Commerce 客户组
{
  // Medusa
  id: "cgroup_01",
  name: "Wholesale",
  metadata: {},
  customers: [...],
  price_lists: [...],
  
  // R Commerce
  id: "generated_uuid",
  name: "Wholesale",
  description: "批发客户",
  meta_data: {
    medusa_id: "cgroup_01"
  }
}
```

## 迁移脚本

### 直接数据库迁移（推荐）

```javascript
// migrate-medusa-direct.js
const { Client } = require('pg'); // Medusa 使用 PostgreSQL
const axios = require('axios');

class DirectMedusaMigrator {
  constructor(medusaConfig, rcommerceConfig) {
    this.medusaDb = new Client(medusaConfig.database);
    this.rcommerceClient = axios.create({
      baseURL: rcommerceConfig.url,
      headers: { 'Authorization': `Bearer ${rcommerceConfig.api_key}` }
    });
  }
  
  async connect() {
    await this.medusaDb.connect();
    console.log('已连接到 Medusa 数据库');
  }
  
  async disconnect() {
    await this.medusaDb.end();
  }
  
  async migrateProducts() {
    console.log('直接迁移产品...');
    
    // 在单个查询中获取所有产品数据
    const query = `
      SELECT 
        p.id,
        p.title,
        p.subtitle,
        p.description,
        p.handle,
        p.thumbnail,
        p.type_id,
        p.collection_id,
        p.is_giftcard,
        p.status,
        p.discountable,
        p.created_at,
        p.updated_at,
        p.category_id,
        COALESCE(json_agg(DISTINCT jsonb_build_object('id', pi.id, 'url', pi.url, 'metadata', pi.metadata)) FILTER (WHERE pi.id IS NOT NULL), '[]') as images,
        COALESCE(json_agg(DISTINCT jsonb_build_object('id', pt.id, 'value', pt.value)) FILTER (WHERE pt.id IS NOT NULL), '[]') as tags,
        COALESCE(json_agg(DISTINCT jsonb_build_object('id', pov.id, 'value', pov.value)) FILTER (WHERE pov.id IS NOT NULL), '[]') as options,
        COALESCE(json_agg(DISTINCT jsonb_build_object(
          'id', pv.id,
          'title', pv.title,
          'sku', pv.sku,
          'barcode', pv.barcode,
          'inventory_quantity', pv.inventory_quantity,
          'manage_inventory', pv.manage_inventory,
          'allow_backorder', pv.allow_backorder,
          'variant_rank', pv.variant_rank,
          'metadata', pv.metadata,
          'prices', (SELECT json_agg(jsonb_build_object('currency', pp.currency_code, 'amount', pp.amount)) 
                     FROM product_variant pp WHERE pp.variant_id = pv.id),
          'options', (SELECT json_agg(jsonb_build_object('value', pvo.value, 'option_id', pvo.option_id)) 
                      FROM product_variant pvo WHERE pvo.variant_id = pv.id)
        )) FILTER (WHERE pv.id IS NOT NULL), '[]') as variants
      FROM product p
      LEFT JOIN product_image pi ON p.id = pi.product_id
      LEFT JOIN product_tag pt ON p.id = pt.product_id
      LEFT JOIN product_option pov ON p.id = pov.product_id
      LEFT JOIN product_variant pv ON p.id = pv.product_id
      GROUP BY p.id
    `;
    
    const result = await this.medusaDb.query(query);
    
    for (const product of result.rows) {
      try {
        // 转换为 R Commerce 格式
        const rcommerceProduct = {
          name: product.title,
          subtitle: product.subtitle,
          slug: product.handle,
          description: product.description,
          sku: product.variants?.[0]?.sku || null,
          price: product.variants?.[0]?.prices?.[0]?.amount ? 
                 product.variants[0].prices[0].amount / 100 : 0,
          status: product.status === 'published' ? 'active' : 'draft',
          is_gift_card: product.is_giftcard,
          images: product.images.map(img => ({
            url: img.url,
            alt: img.metadata?.alt || '',
            position: img.metadata?.position || 0
          })),
          tags: product.tags.map(tag => tag.value),
          inventory_quantity: product.variants?.[0]?.inventory_quantity || 0,
          inventory_policy: product.variants?.[0]?.allow_backorder ? 
                           'continue' : 'deny',
          variants: product.variants.map(variant => ({
            title: variant.title,
            sku: variant.sku,
            price: variant.prices?.[0]?.amount ? variant.prices[0].amount / 100 : 0,
            inventory_quantity: variant.inventory_quantity,
            options: variant.options.map(opt => ({
              name: product.options.find(o => o.id === opt.option_id)?.title || 'Option',
              value: opt.value
            }))
          })),
          meta_data: {
            medusa: {
              id: product.id,
              collection_id: product.collection_id,
              discountable: product.discountable,
              created_at: product.created_at,
              updated_at: product.updated_at
            }
          }
        };
        
        // 在 R Commerce 中创建
        const response = await this.rcommerceClient.post('/v1/products', rcommerceProduct);
        console.log(` 已迁移产品：${rcommerceProduct.name}`);
        
      } catch (error) {
        console.error(` 迁移产品失败：${product.title}`, error.message);
      }
    }
  }
  
  async migrateRegions() {
    console.log('迁移区域...');
    
    const query = `
      SELECT 
        r.id,
        r.name,
        r.currency_code,
        r.tax_rate,
        r.tax_code,
        r.automatic_taxes,
        r.gift_cards_taxable,
        r.payment_providers,
        r.fulfillment_providers,
        COALESCE(json_agg(DISTINCT jsonb_build_object('iso_2', rc.iso_2, 'display_name', rc.display_name)) 
                 FILTER (WHERE rc.iso_2 IS NOT NULL), '[]') as countries
      FROM region r
      LEFT JOIN region_countries rc_link ON r.id = rc_link.region_id
      LEFT JOIN country rc ON rc_link.country_code = rc.iso_2
      GROUP BY r.id
    `;
    
    const result = await this.medusaDb.query(query);
    
    for (const region of result.rows) {
      try {
        // 将区域映射到 R Commerce 配置
        const regionConfig = {
          name: region.name,
          currency: region.currency_code.toUpperCase(),
          countries: region.countries.map(c => c.iso_2),
          tax_config: {
            rate: parseFloat(region.tax_rate) || 0,
            automatic: region.automatic_taxes,
            gift_cards_taxable: region.gift_cards_taxable
          },
          payment_providers: region.payment_providers,
          fulfillment_providers: region.fulfillment_providers
        };
        
        // 存储区域配置（需要应用到 R Commerce 系统配置）
        console.log(` 已映射区域：${region.name} (${region.currency_code})`);
        
      } catch (error) {
        console.error(` 迁移区域失败：${region.name}`, error.message);
      }
    }
  }
}

// 使用
const migrator = new DirectMedusaMigrator({
  database: {
    host: process.env.MEDUSA_DB_HOST,
    database: process.env.MEDUSA_DB_NAME,
    user: process.env.MEDUSA_DB_USER,
    password: process.env.MEDUSA_DB_PASS
  }
}, {
  url: process.env.RCOMMERCE_URL,
  api_key: process.env.RCOMMERCE_API_KEY
});

migrator.connect()
  .then(() => migrator.migrateProducts())
  .then(() => migrator.migrateRegions())
  .then(() => migrator.disconnect())
  .catch(console.error);
```

## 迁移配置

```toml
# migration-medusa.toml

[migration]
strategy = "direct"  # 或 "compatibility_layer"
dry_run = false

[source.medusa]
api_url = "https://api.medusa-store.com"
api_key = "YOUR_MEDUSA_API_KEY"

[target.rcommerce]
api_url = "https://api.rcommerce-store.com"
api_key = "YOUR_RCOMMERCE_API_KEY"

[migration.mapping]
# 迁移功能标志
transfer_products = true
transfer_customers = true
transfer_orders = true
transfer_regions = true
transfer_discounts = true
transfer_price_lists = true

# 特殊映射
map_medusa_regions = true
map_price_lists_to_customer_groups = true
map_sales_channels_to_tags = true

# 可选：迁移到 R Commerce 兼容层
[compatibility]
enabled = false  # 设置为 true 以进行渐进式迁移
maintain_medusa_compatibility = false
```

## 迁移后步骤

1. **验证数据完整性**
   ```bash
   # 比较数量
   medusa_count=$(curl -s -H "x-medusa-access-token: $MEDUSA_KEY" $MEDUSA_URL/admin/products | jq '.count')
   rcommerce_count=$(curl -s -H "Authorization: Bearer $RC_KEY" $RC_URL/v1/products?per_page=1 | jq '.meta.pagination.total')
   
   echo "产品 - Medusa：$medusa_count，R Commerce：$rcommerce_count"
   ```

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
   ```javascript
   // 重新配置 Medusa admin 指向 R Commerce hooks
   const { data } = await medusa.admin.store.update({
     webhook_url: "https://api.rcommerce.app/webhooks/medusa"
   });
   ```

## 需要注意的差异

### Medusa.js 中 R Commerce 没有的功能

- **Sales Channels**：使用标签或客户组
- **多货币区域**：R Commerce 更简单的模型
- **Draft Orders**：R Commerce 有手动订单
- **Claims**：需要自定义实现
- **Swaps**：R Commerce 有更简单的退货系统
- **Published API Keys**：不同的认证模型

### R Commerce 中 Medusa.js 没有的功能

- **WooCommerce API 兼容性**：对现有集成有用
- **店小秘集成**：中国特定功能
- **高级欺诈检测**：内置系统
- **PostgreSQL 驱动**：基于 PostgreSQL 构建，确保可靠性和性能
- **跨平台部署**：FreeBSD 支持

由于 Medusa.js 和 R Commerce 之间的架构相似性，此迁移是最直接的。
