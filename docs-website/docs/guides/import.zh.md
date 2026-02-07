# 导入指南

本指南涵盖从各种电子商务平台导入数据到 R Commerce 的方法。

## 概述

R Commerce 提供多种导入方法：

- **平台迁移** - 直接从 Shopify、WooCommerce、Magento 导入
- **文件导入** - CSV、JSON、XML 文件上传
- **API 导入** - 程序化批量导入
- **CLI 工具** - 命令行导入工具

## 从 Shopify 导入

### 先决条件

1. Shopify 商店管理员访问权限
2. 具有 API 凭证的私有应用或自定义应用
3. 具有写入权限的 R Commerce API 密钥

### 从 Shopify 导出

#### 产品

```bash
# 使用 Shopify CLI
shopify theme pull

# 或通过 Admin API 导出
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/products.json?limit=250" \
  -H "X-Shopify-Access-Token: your_access_token"
```

#### 订单

```bash
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/orders.json?status=any&limit=250" \
  -H "X-Shopify-Access-Token: your_access_token"
```

#### 客户

```bash
curl -X GET "https://your-store.myshopify.com/admin/api/2024-01/customers.json?limit=250" \
  -H "X-Shopify-Access-Token: your_access_token"
```

### 导入到 R Commerce

```bash
# 导入产品
rcommerce import shopify products \
  --source products.json \
  --config config.toml \
  --batch-size 100

# 导入带客户的订单
rcommerce import shopify orders \
  --source orders.json \
  --import-customers \
  --config config.toml

# 导入所有内容
rcommerce import shopify all \
  --store your-store.myshopify.com \
  --token your_access_token \
  --config config.toml
```

### Shopify 导入选项

| 选项 | 说明 | 默认 |
|--------|-------------|---------|
| `--batch-size` | 每批记录数 | 100 |
| `--skip-images` | 跳过产品图片下载 | false |
| `--image-base-url` | 覆盖图片 URL 前缀 | - |
| `--currency-map` | 映射货币（USD:USDT） | - |
| `--skip-existing` | 跳过现有 SKU | false |
| `--update-existing` | 更新现有产品 | true |

## 从 WooCommerce 导入

### 从 WooCommerce 导出

#### 使用 WordPress CLI

```bash
# 导出产品
wp wc product list --format=json --user=admin > woocommerce_products.json

# 导出订单
wp wc shop_order list --format=json --user=admin > woocommerce_orders.json

# 导出客户
wp user list --role=customer --format=json > woocommerce_customers.json
```

#### 使用 REST API

```bash
# 获取 WooCommerce 凭证
CONSUMER_KEY=ck_your_key
CONSUMER_SECRET=cs_your_secret

# 导出产品
curl -X GET "https://yourstore.com/wp-json/wc/v3/products?per_page=100" \
  -u "$CONSUMER_KEY:$CONSUMER_SECRET" \
  -H "Content-Type: application/json"
```

### 导入到 R Commerce

> **注意：** 对于 WooCommerce 平台导入，请使用基础商店 URL（例如 `https://your-store.com`）。
> `/wp-json/wc/v3` 路径将由导入器自动添加。

```bash
# 从文件导入产品
rcommerce import woocommerce products \
  --source woocommerce_products.json \
  --config config.toml

# 直接从 WooCommerce API 导入
rcommerce import platform woocommerce \
  --api-url https://your-store.com \
  --api-key YOUR_CONSUMER_KEY \
  --api-secret YOUR_CONSUMER_SECRET \
  --config config.toml

# 带属性映射导入
rcommerce import woocommerce products \
  --source products.json \
  --attribute-map '{"pa_size":"size","pa_color":"color"}' \
  --config config.toml

# 导入订单
rcommerce import woocommerce orders \
  --source orders.json \
  --payment-gateway-map '{"bacs":"bank_transfer","cod":"cash_on_delivery"}' \
  --config config.toml
```

### WooCommerce 特定映射

```toml
[import.woocommerce]
# 属性映射
attribute_map = { pa_size = "size", pa_color = "color", pa_material = "material" }

# 类别映射
category_map = { "clothing" = "apparel", "electronics" = "tech" }

# 状态映射
order_status_map = { 
  "processing" = "confirmed",
  "completed" = "completed", 
  "cancelled" = "cancelled",
  "on-hold" = "pending"
}

# 支付网关映射
payment_gateway_map = {
  "bacs" = "bank_transfer",
  "cod" = "cash_on_delivery",
  "stripe" = "stripe",
  "paypal" = "paypal"
}
```

## 从 Magento 导入

### 从 Magento 导出

#### 使用 Magento CLI

```bash
# 导出产品
bin/magento export:products --format=json --output=products.json

# 导出订单
bin/magento export:orders --format=json --output=orders.json

# 导出客户
bin/magento export:customers --format=json --output=customers.json
```

#### 使用 REST API

```bash
# 获取管理员令牌
TOKEN=$(curl -X POST "https://magento.example.com/rest/V1/integration/admin/token" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"password"}' | tr -d '"')

# 导出产品
curl -X GET "https://magento.example.com/rest/V1/products?searchCriteria[pageSize]=500" \
  -H "Authorization: Bearer $TOKEN"
```

### 导入到 R Commerce

```bash
# 导入产品
rcommerce import magento products \
  --source products.json \
  --config config.toml \
  --batch-size 50

# 带商店视图映射导入
rcommerce import magento products \
  --source products.json \
  --store-view-map '{"default":"en","german":"de"}' \
  --config config.toml

# 导入带地址的客户
rcommerce import magento customers \
  --source customers.json \
  --include-addresses \
  --config config.toml
```

## CSV 文件格式

### 产品 CSV 格式

```csv
sku,name,description,price,currency,category,tags,image_url,inventory_quantity,weight
SKU001,Premium T-Shirt,High quality cotton t-shirt,29.99,USD,apparel,"cotton,summer",https://cdn.example.com/tshirt.jpg,100,0.5
SKU002,Wireless Headphones,Bluetooth headphones with noise canceling,149.99,USD,electronics,"audio,wireless",https://cdn.example.com/headphones.jpg,50,0.3
```

### 必需列

| 列 | 必需 | 说明 |
|--------|----------|-------------|
| `sku` | 是 | 唯一产品标识符 |
| `name` | 是 | 产品名称 |
| `price` | 是 | 产品价格 |
| `currency` | 是 | ISO 4217 货币代码 |

### 可选列

| 列 | 说明 |
|--------|-------------|
| `description` | 产品描述 |
| `short_description` | 简短描述 |
| `category` | 类别名称或 ID |
| `tags` | 逗号分隔的标签 |
| `image_url` | 主图 URL |
| `gallery_urls` | 逗号分隔的附加图片 |
| `inventory_quantity` | 库存数量 |
| `weight` | 产品重量（kg） |
| `dimensions` | LxWxH 格式（cm） |
| `status` | active/inactive/draft |

### 导入 CSV

```bash
# 基本导入
rcommerce import csv products \
  --file products.csv \
  --config config.toml

# 带自定义分隔符
rcommerce import csv products \
  --file products.csv \
  --delimiter ";" \
  --encoding utf-8 \
  --config config.toml

# 带列映射
rcommerce import csv products \
  --file products.csv \
  --column-map '{"product_code":"sku","product_name":"name"}' \
  --config config.toml

# 干运行预览
rcommerce import csv products \
  --file products.csv \
  --dry-run \
  --config config.toml
```

### 客户 CSV 格式

```csv
email,first_name,last_name,phone,accepts_marketing,address1,city,country,zip
john@example.com,John,Doe,+1234567890,true,123 Main St,New York,US,10001
jane@example.com,Jane,Smith,+0987654321,false,456 Oak Ave,Los Angeles,US,90210
```

### 订单 CSV 格式

```csv
order_number,customer_email,order_date,total,status,line_items
1001,john@example.com,2024-01-15T10:00:00Z,59.99,completed,"[{\"sku\":\"SKU001\",\"qty\":2,\"price\":29.99}]"
1002,jane@example.com,2024-01-16T14:30:00Z,149.99,processing,"[{\"sku\":\"SKU002\",\"qty\":1,\"price\":149.99}]"
```

## JSON/XML 导入

### JSON 导入

```bash
# 从 JSON 导入产品
rcommerce import json products \
  --file products.json \
  --config config.toml

# 带模式验证导入
rcommerce import json products \
  --file products.json \
  --schema product-schema.json \
  --config config.toml
```

#### JSON 产品格式

```json
{
  "products": [
    {
      "sku": "SKU001",
      "name": "Premium T-Shirt",
      "description": "High quality cotton t-shirt",
      "price": "29.99",
      "currency": "USD",
      "category": "apparel",
      "tags": ["cotton", "summer"],
      "images": [
        {
          "url": "https://cdn.example.com/tshirt.jpg",
          "alt": "T-Shirt Front",
          "position": 1
        }
      ],
      "inventory": {
        "quantity": 100,
        "track_inventory": true
      },
      "variants": [
        {
          "sku": "SKU001-S",
          "name": "Small",
          "price": "29.99",
          "options": {
            "size": "S"
          },
          "inventory_quantity": 25
        }
      ]
    }
  ]
}
```

### XML 导入

```bash
# 从 XML 导入产品
rcommerce import xml products \
  --file products.xml \
  --xpath "//product" \
  --config config.toml
```

#### XML 产品格式

```xml
<?xml version="1.0" encoding="UTF-8"?>
<products>
  <product>
    <sku>SKU001</sku>
    <name>Premium T-Shirt</name>
    <description>High quality cotton t-shirt</description>
    <price currency="USD">29.99</price>
    <category>apparel</category>
    <inventory>
      <quantity>100</quantity>
    </inventory>
  </product>
</products>
```

## CLI 导入命令

### 通用导入命令

```bash
rcommerce import [source] [entity] [options]
```

### 可用来源

| 来源 | 说明 |
|--------|-------------|
| `shopify` | Shopify 商店导入 |
| `woocommerce` | WooCommerce 导入 |
| `magento` | Magento 导入 |
| `csv` | CSV 文件导入 |
| `json` | JSON 文件导入 |
| `xml` | XML 文件导入 |

### 可用实体

| 实体 | 说明 |
|--------|-------------|
| `products` | 产品目录 |
| `customers` | 客户账户 |
| `orders` | 订单历史 |
| `coupons` | 优惠码 |
| `categories` | 产品类别 |
| `all` | 所有内容（仅平台导入） |

### 常用选项

| 选项 | 说明 |
|--------|-------------|
| `--config` | 配置文件路径 |
| `--source` | 源文件或 URL |
| `--batch-size` | 每批记录数 |
| `--dry-run` | 预览而不导入 |
| `--skip-existing` | 跳过重复记录 |
| `--update-existing` | 更新现有记录 |
| `--continue-on-error` | 跳过失败的记录 |
| `--mapping-file` | 字段映射配置 |

### 导入进度和监控

```bash
# 带详细输出导入
rcommerce import csv products --file products.csv --verbose

# 带进度条导入
rcommerce import csv products --file products.csv --progress

# 保存导入日志
rcommerce import csv products --file products.csv --log import.log

# 带 webhook 通知导入
rcommerce import csv products --file products.csv --webhook https://yoursite.com/webhooks/import
```

## 导入验证

### 导入前验证

```bash
# 验证而不导入
rcommerce import csv products --file products.csv --validate-only

# 检查常见问题
rcommerce import validate --file products.csv --type products
```

### 验证规则

- **SKU 唯一性** - 不允许重复 SKU
- **价格格式** - 有效的小数
- **货币代码** - 有效的 ISO 4217 代码
- **邮箱格式** - 客户有效的邮箱
- **必填字段** - 所有必填字段存在
- **图片 URL** - 有效的 URL 格式
- **类别存在** - 类别必须存在

### 导入后验证

```bash
# 验证导入计数
rcommerce import verify --import-id import_abc123

# 与源比较
rcommerce import verify --import-id import_abc123 --source products.csv
```

## 导入最佳实践

### 导入前

1. **备份您的数据库** - 批量导入前始终备份
2. **在暂存环境测试** - 先在非生产环境验证导入
3. **验证数据** - 清理和验证源数据
4. **准备映射** - 提前定义字段映射
5. **估计时间** - 大型导入可能需要数小时

### 导入期间

1. **使用批次** - 不要一次性导入所有内容
2. **监控进度** - 注意错误和警告
3. **检查资源** - 确保足够的 CPU/内存
4. **记录所有内容** - 保留详细的导入日志

### 导入后

1. **验证计数** - 比较源和目标计数
2. **检查样本** - 手动验证导入的记录
3. **测试功能** - 确保产品/订单正常工作
4. **更新搜索索引** - 如果需要则重建搜索

### 性能提示

```toml
[import]
# 增加批次大小以加快导入（如果内存允许）
batch_size = 500

# 导入期间禁用搜索索引
skip_search_index = true

# 导入期间禁用 webhooks
skip_webhooks = true

# 并行处理
workers = 4

# 禁用外键检查（PostgreSQL）
disable_constraints = true
```

## 故障排除

### 常见导入错误

**"重复 SKU"**
- 使用 `--skip-existing` 或 `--update-existing`
- 检查源数据中的重复项

**"无效的货币代码"**
- 使用 ISO 4217 代码（USD、EUR、GBP）
- 检查货币字段中的拼写错误

**"类别未找到"**
- 导入前创建类别
- 使用 `--auto-create-categories`

**"图片下载失败"**
- 检查图片 URL 是否可访问
- 如果不需要则使用 `--skip-images`
- 如果被阻止则配置图片代理

### 调试模式

```bash
# 启用调试日志
RUST_LOG=debug rcommerce import csv products --file products.csv

# 保存详细日志
rcommerce import csv products --file products.csv --debug --log debug.log
```

### 导入恢复

如果导入中途失败：

```bash
# 从最后一个成功的批次恢复
rcommerce import csv products --file products.csv --resume --import-id import_abc123

# 仅导入剩余记录
rcommerce import csv products --file products.csv --skip-existing
```

## 相关文档

- [Shopify 迁移](../migration/shopify.md)
- [WooCommerce 迁移](../migration/woocommerce.md)
- [Magento 迁移](../migration/magento.md)
- [CLI 参考](../development/cli-reference.md)
