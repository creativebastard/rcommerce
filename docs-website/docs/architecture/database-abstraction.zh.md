# 数据库抽象

## 概述

R Commerce 使用仓库模式进行数据库访问，在业务逻辑和数据持久化之间提供清晰的分离。当前实现针对 PostgreSQL 14+ 进行了优化。

## 仓库模式

仓库模式将数据库操作抽象为 trait 接口：

```rust
#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>>;
    async fn create(&self, product: &Product) -> Result<Product>;
    async fn update(&self, product: &Product) -> Result<Product>;
    async fn delete(&self, id: Uuid) -> Result<bool>;
    async fn list(&self, pagination: Pagination) -> Result<Vec<Product>>;
}
```

## 技术栈

### SQLx

R Commerce 使用 SQLx 进行数据库访问：

- **编译时查询检查**：查询在编译时根据数据库模式进行验证
- **异步优先设计**：基于 Tokio 构建，支持高性能异步操作
- **类型安全结果**：查询结果自动映射到 Rust 结构体
- **连接池**：内置连接池管理

**示例：**
```rust
// 编译时检查的查询
let product = sqlx::query_as!(Product,
    "SELECT * FROM products WHERE id = $1",
    product_id
)
.fetch_optional(&pool)
.await?;
```

## PostgreSQL 特性

R Commerce 利用 PostgreSQL 特定功能：

### 枚举
```sql
CREATE TYPE order_status AS ENUM (
    'pending', 'confirmed', 'processing', 
    'shipped', 'completed', 'cancelled'
);
```

### JSONB 灵活数据
```sql
-- 元数据存储为 JSONB
metadata JSONB NOT NULL DEFAULT '{}'

-- 查询 JSONB 字段
SELECT * FROM products WHERE metadata->>'brand' = 'Nike';
```

### 全文搜索
```sql
-- 全文搜索索引
CREATE INDEX idx_products_search ON products 
USING gin(to_tsvector('english', title || ' ' || description));
```

### 数组
```sql
-- 标签数组字段
tags TEXT[]

-- 查询数组
SELECT * FROM products WHERE tags @> ARRAY['sale'];
```

## 连接池

在 `config.toml` 中配置连接池：

```toml
[database]
host = "localhost"
port = 5432
database = "rcommerce"
username = "rcommerce"
password = "secret"
pool_size = 20          # 池中最大连接数
min_connections = 5     # 最小空闲连接数
max_lifetime = 1800     # 连接最大生命周期（秒）
```

## 迁移

通过 CLI 管理数据库迁移：

```bash
# 运行待处理的迁移
rcommerce db migrate -c config.toml

# 检查迁移状态
rcommerce db status -c config.toml

# 重置数据库（仅开发环境）
rcommerce db reset -c config.toml
```

迁移文件存储在：
```
crates/rcommerce-core/migrations/
├── 001_initial_schema.sql
├── 002_add_api_keys.sql
└── ...
```

## 错误处理

数据库错误映射到应用程序错误：

```rust
pub enum Error {
    Database(sqlx::Error),
    NotFound(String),
    Validation(String),
    // ...
}
```

## 最佳实践

1. **尽可能使用编译时检查查询** 以确保类型安全
2. **使用事务** 处理多步骤操作
3. **基于查询模式策略性建立索引**
4. **使用连接池** 管理数据库连接
5. **监控慢查询** 并根据需要优化

## 另请参阅

- [数据模型](./data-model.zh.md) - 实体关系
- [订单管理](./order-management.zh.md) - 订单生命周期
