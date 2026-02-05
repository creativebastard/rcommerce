# 数据模型

## 概述

R Commerce 使用全面的数据模型，专为大规模电商运营而设计。该模式针对 PostgreSQL 进行了优化，支持复杂的产品变体、订单管理和客户关系。

## 实体关系图

```
┌─────────────────┐          ┌─────────────────┐
│    Customer     │          │    Address      │
├─────────────────┤          ├─────────────────┤
│ id (UUID)       │◄─────────┤ id (UUID)       │
│ email           │          │ customer_id     │
│ first_name      │          │ street1         │
│ last_name       │          │ city            │
│ phone           │          │ state           │
│ created_at      │          │ postal_code     │
│ updated_at      │          │ country         │
└────────┬────────┘          └─────────────────┘
         │
         │ creates
         ▼
┌─────────────────┐
│     Order       │
├─────────────────┤          ┌─────────────────┐
│ id (UUID)       │          │   OrderNote     │
│ order_number    │◄─────────┤ id (UUID)       │
│ customer_id     │          │ order_id        │
│ total           │          │ content         │
│ status          │          │ created_at      │
│ created_at      │          └─────────────────┘
└────────┬────────┘
         │
         │ contains
         │
         ▼
┌─────────────────┐          ┌─────────────────┐
│ OrderLineItem   │          │    Payment      │
├─────────────────┤          ├─────────────────┤
│ id (UUID)       │          │ id (UUID)       │
│ order_id        │◄─────────┤ order_id        │
│ product_id      │          │ amount          │
│ quantity        │          │ status          │
│ unit_price      │          │ gateway         │
│ total           │          │ created_at      │
└─────────────────┘          └─────────────────┘

┌─────────────────┐          ┌─────────────────┐
│    Product      │          │ ProductVariant  │
├─────────────────┤          ├─────────────────┤
│ id (UUID)       │          │ id (UUID)       │
│ title           │          │ product_id      │
│ slug            │          │ sku             │
│ price           │◄─────────┤ price           │
│ inventory_qty   │          │ inventory_qty   │
│ status          │          │ created_at      │
│ created_at      │          └─────────────────┘
└─────────────────┘
```

## 核心实体

### Customer（客户）

客户实体代表店铺的注册用户。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| email | String | 唯一邮箱地址 |
| first_name | String | 客户名字 |
| last_name | String | 客户姓氏 |
| phone | String | 可选电话号码 |
| password_hash | String | Bcrypt 哈希密码 |
| is_verified | Boolean | 邮箱验证状态 |
| accepts_marketing | Boolean | 营销同意状态 |
| currency | Currency | 首选货币 |
| created_at | DateTime | 注册时间戳 |
| updated_at | DateTime | 最后更新时间戳 |

### Address（地址）

地址与客户关联，用于账单和配送。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| customer_id | UUID | 客户引用 |
| first_name | String | 收件人名字 |
| last_name | String | 收件人姓氏 |
| company | String | 可选公司名称 |
| street1 | String | 街道地址第一行 |
| street2 | String | 街道地址第二行 |
| city | String | 城市名称 |
| state | String | 州/省 |
| postal_code | String | 邮编 |
| country | String | ISO 国家代码 |
| phone | String | 联系电话 |
| is_default | Boolean | 默认地址标记 |

### Product（产品）

产品是目录中的核心可销售商品。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| title | String | 产品名称 |
| slug | String | URL 友好的标识符 |
| description | String | 产品描述 |
| sku | String | 库存单位 |
| product_type | Enum | 简单/可变/数字/捆绑 |
| price | Decimal | 基础价格 |
| compare_at_price | Decimal | 销售原价 |
| cost_price | Decimal | 成本价（用于利润计算）|
| currency | Currency | 价格货币 |
| inventory_quantity | Integer | 库存数量 |
| inventory_management | Boolean | 库存跟踪标记 |
| is_active | Boolean | 发布状态 |
| is_featured | Boolean | 精选产品标记 |
| requires_shipping | Boolean | 实体产品标记 |
| created_at | DateTime | 创建时间戳 |
| updated_at | DateTime | 最后更新时间戳 |

### Product Variant（产品变体）

变体代表产品的不同选项（尺寸、颜色等）。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| product_id | UUID | 父产品引用 |
| title | String | 变体名称 |
| sku | String | 唯一 SKU |
| price | Decimal | 变体特定价格 |
| inventory_quantity | Integer | 变体库存数量 |
| is_active | Boolean | 可购买状态 |

### Order（订单）

订单代表客户的购买交易。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| order_number | String | 人工可读的订单 ID |
| customer_id | UUID | 客户引用 |
| customer_email | String | 下单时的邮箱 |
| subtotal | Decimal | 行项目总和 |
| tax_amount | Decimal | 总税额 |
| shipping_amount | Decimal | 运费 |
| discount_amount | Decimal | 已应用折扣 |
| total_amount | Decimal | 最终总计 |
| currency | Currency | 订单货币 |
| status | Enum | pending/confirmed/processing/shipped/completed/cancelled |
| payment_status | Enum | pending/authorized/paid/failed/refunded |
| fulfillment_status | Enum | pending/processing/shipped/delivered |
| created_at | DateTime | 订单时间戳 |
| updated_at | DateTime | 最后更新时间戳 |

### Order Line Item（订单行项目）

行项目代表订单中的单个产品。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| order_id | UUID | 父订单引用 |
| product_id | UUID | 产品引用 |
| variant_id | UUID | 变体引用（可选）|
| title | String | 下单时的产品名称 |
| sku | String | 下单时的 SKU |
| quantity | Integer | 订购数量 |
| unit_price | Decimal | 单价 |
| total | Decimal | 行项目总计 |

### Payment（支付）

支付记录交易尝试和完成情况。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| order_id | UUID | 关联订单 |
| gateway | String | 支付提供商 |
| amount | Decimal | 支付金额 |
| currency | Currency | 支付货币 |
| status | Enum | pending/authorized/paid/failed/refunded |
| provider_id | String | 外部交易 ID |
| created_at | DateTime | 交易时间戳 |

### API Key（API 密钥）

API 密钥支持服务间认证。

| 字段 | 类型 | 描述 |
|------|------|------|
| id | UUID | 主键 |
| customer_id | UUID | 可选所有者引用 |
| key_prefix | String | 公共标识符 |
| key_hash | String | 哈希密钥 |
| name | String | 描述性名称 |
| scopes | Array | 权限范围 |
| is_active | Boolean | 密钥状态 |
| expires_at | DateTime | 可选过期时间 |
| last_used_at | DateTime | 最后使用时间戳 |
| created_at | DateTime | 创建时间戳 |

## 数据库模式

### PostgreSQL 类型

```sql
-- 订单状态枚举
CREATE TYPE order_status AS ENUM (
    'pending',
    'confirmed',
    'processing',
    'on_hold',
    'shipped',
    'completed',
    'cancelled',
    'refunded'
);

-- 支付状态枚举
CREATE TYPE payment_status AS ENUM (
    'pending',
    'authorized',
    'paid',
    'partially_refunded',
    'fully_refunded',
    'failed',
    'cancelled'
);

-- 产品类型枚举
CREATE TYPE product_type AS ENUM (
    'Simple',
    'Variable',
    'Subscription',
    'Digital',
    'Bundle'
);

-- 货币枚举
CREATE TYPE currency AS ENUM (
    'USD', 'EUR', 'GBP', 'JPY', 
    'AUD', 'CAD', 'CNY', 'HKD', 'SGD'
);
```

### 关键表

查看[完整模式文档](https://github.com/creativebastard/rcommerce/blob/main/docs/architecture/02-data-modeling.md)获取完整的 SQL 定义。

## 关系

### 客户关系
- 客户拥有多个地址
- 客户拥有多个订单
- 客户拥有多个 API 密钥

### 产品关系
- 产品拥有多个变体
- 产品拥有多个图片
- 产品属于多个分类
- 产品拥有多个订单行项目

### 订单关系
- 订单属于客户
- 订单拥有多个行项目
- 订单拥有多个支付记录
- 订单拥有多个配送记录
- 订单拥有多个备注

## 索引

性能关键索引：

```sql
-- 客户查询
CREATE INDEX idx_customers_email ON customers(email);
CREATE INDEX idx_customers_created_at ON customers(created_at DESC);

-- 产品搜索
CREATE UNIQUE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_products_status ON products(status);
CREATE INDEX idx_products_search ON products USING gin(
    to_tsvector('english', title || ' ' || COALESCE(description, ''))
);

-- 订单查询
CREATE INDEX idx_orders_customer_id ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);

-- API 密钥查询
CREATE INDEX idx_api_keys_prefix ON api_keys(key_prefix);
CREATE INDEX idx_api_keys_customer ON api_keys(customer_id);
```

## 数据完整性

### 约束
- 外键约束，关系使用 CASCADE
- 检查约束，确保价格和数量为正值
- 唯一约束，邮箱、slug、SKU 唯一
- 枚举验证，状态字段验证

### 触发器
- 自动更新 `updated_at` 时间戳
- 下单时自动调整库存
- 数据变更时记录审计日志

## 下一步

- [数据库抽象](./database-abstraction.zh.md) - 仓库模式
- [订单管理](./order-management.zh.md) - 订单生命周期
- [媒体存储](./media-storage.zh.md) - 文件处理
