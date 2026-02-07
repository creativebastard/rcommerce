# 数据库迁移

数据库迁移管理 R Commerce 中的架构更改，允许您对数据库结构进行版本控制并在环境中一致地应用更改。

## 概述

R Commerce 使用基于 SQL 的迁移，具有以下特点：

- **版本控制** - 在 Git 中跟踪架构更改
- **可逆** - 每个迁移都有回滚能力
- **原子性** - 每个迁移在事务中运行
- **幂等** - 多次运行安全

## 启动时运行迁移

### 自动迁移

在配置中启用自动迁移：

```toml
[database]
url = "postgres://user:pass@localhost/rcommerce"

# 启动时运行待处理的迁移
run_migrations_on_startup = true

# 迁移设置
[migrations]
# 包含迁移文件的目录
path = "./migrations"

# 如果迁移失败则启动失败
strict = true

# 记录迁移执行
verbose = true
```

### 手动迁移

使用 CLI 手动运行迁移：

```bash
# 运行所有待处理的迁移
rcommerce db migrate

# 使用特定配置运行迁移
rcommerce db migrate -c /path/to/config.toml

# 检查迁移状态
rcommerce db status

# 查看待处理的迁移
rcommerce db pending
```

## 创建自定义迁移

### 迁移文件命名

迁移文件遵循以下命名约定：

```
{version}_{description}.sql
```

示例：

```
001_initial_schema.sql
002_add_user_preferences.sql
003_create_inventory_table.sql
```

### 迁移文件结构

每个迁移文件包含向上和向下迁移：

```sql
-- 向上迁移（应用更改）
-- 004_add_product_tags.sql

CREATE TABLE product_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    tag VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(product_id, tag)
);

CREATE INDEX idx_product_tags_product_id ON product_tags(product_id);
CREATE INDEX idx_product_tags_tag ON product_tags(tag);

-- 向下迁移（回滚更改）
-- @DOWN

DROP INDEX IF EXISTS idx_product_tags_tag;
DROP INDEX IF EXISTS idx_product_tags_product_id;
DROP TABLE IF EXISTS product_tags;
```

### 迁移模板

使用 CLI 生成新迁移：

```bash
# 创建新的迁移文件
rcommerce db create-migration add_customer_loyalty_points

# 输出：Created migrations/005_add_customer_loyalty_points.sql
```

生成的模板：

```sql
-- 迁移：add_customer_loyalty_points
-- 创建于：2024-01-15T10:30:00Z

-- 向上迁移


-- @DOWN

-- 向下迁移

```

### 迁移最佳实践

#### 1. 保持迁移小巧

将大型更改拆分为多个迁移：

```sql
-- 005_create_order_items.sql
CREATE TABLE order_items (...);

-- 006_add_order_item_indexes.sql
CREATE INDEX idx_order_items_order_id ON order_items(order_id);
CREATE INDEX idx_order_items_product_id ON order_items(product_id);
```

#### 2. 使迁移可逆

始终提供向下迁移：

```sql
-- 向上
ALTER TABLE customers ADD COLUMN phone VARCHAR(20);

-- @DOWN
ALTER TABLE customers DROP COLUMN IF EXISTS phone;
```

#### 3. 处理现有数据

在架构更改中考虑数据迁移：

```sql
-- 添加带默认值的新列
ALTER TABLE products ADD COLUMN status VARCHAR(20) DEFAULT 'active';

-- 更新现有记录
UPDATE products SET status = 'active' WHERE status IS NULL;

-- 使列非空
ALTER TABLE products ALTER COLUMN status SET NOT NULL;
```

#### 4. 使用事务

默认情况下迁移在事务中运行。对于无法在事务中运行的操作（如并发创建索引），使用：

```sql
-- @NO_TRANSACTION

CREATE INDEX CONCURRENTLY idx_products_name ON products(name);
```

#### 5. 避免破坏性更改

不要删除列，而是考虑：

```sql
-- 而不是：ALTER TABLE products DROP COLUMN old_field;

-- 1. 添加弃用通知
COMMENT ON COLUMN products.old_field IS 'DEPRECATED: Will be removed in v2.0';

-- 2. 使可为空
ALTER TABLE products ALTER COLUMN old_field DROP NOT NULL;

-- 3. 在稍后迁移中删除
```

## 数据库重置程序

### 开发重置

将数据库重置为初始状态：

```bash
# 重置数据库（删除并重新创建）
rcommerce db reset

# 带新迁移重置
rcommerce db reset --with-migrations

# 重置并用测试数据填充
rcommerce db reset --seed
```

### 迁移回滚

回滚特定迁移：

```bash
# 回滚最后一个迁移
rcommerce db rollback

# 回滚特定数量的迁移
rcommerce db rollback --steps 3

-- 回滚到特定版本
rcommerce db rollback --to 003

-- 回滚特定迁移
rcommerce db rollback --migration 005_add_feature
```

### 强制迁移状态

在极少数情况下，您可能需要强制迁移状态：

```bash
-- 将迁移标记为已应用（不运行它）
rcommerce db force --version 005

-- 将迁移标记为待处理
rcommerce db unforce --version 005

-- 重置迁移跟踪（谨慎使用！）
rcommerce db reset-tracking
```

## 迁移最佳实践

### 开发工作流

1. **在代码更改前创建迁移**
   ```bash
   rcommerce db create-migration add_feature_x
   ```

2. **先编写迁移**
   - 定义架构更改
   - 在本地测试迁移

3. **编写应用程序代码**
   - 使用新架构实现功能

4. **测试迁移**
   ```bash
   rcommerce db reset --with-migrations
   ```

5. **提交迁移和代码**
   ```bash
   git add migrations/ src/
   git commit -m "Add feature X with migration"
   ```

### 团队协作

#### 迁移冲突

当两个开发人员创建相同版本的迁移时：

```bash
-- 检查迁移状态
rcommerce db status

-- 如果存在冲突，重新编号迁移
mv migrations/005_add_feature.sql migrations/006_add_feature.sql

-- 更新迁移跟踪
rcommerce db force --version 005
rcommerce db migrate
```

#### 代码审查清单

- [ ] 迁移有向上和向下部分
- [ ] 迁移是可逆的
- [ ] 没有未经弃用的破坏性更改
- [ ] 为外键添加了索引
- [ ] 处理了数据迁移（如果需要）
- [ ] 迁移已在本地测试

### 生产部署

#### 部署前

1. **备份数据库**
   ```bash
   pg_dump -Fc rcommerce > backup_$(date +%Y%m%d).dump
   ```

2. **在暂存环境测试迁移**
   ```bash
   rcommerce db migrate --dry-run
   ```

3. **检查迁移持续时间**
   ```sql
   -- 估计大型表更改的时间
   EXPLAIN ANALYZE ALTER TABLE large_table ADD COLUMN new_col VARCHAR(100);
   ```

#### 部署步骤

1. **部署应用程序**（禁用迁移）
2. **手动运行迁移**
   ```bash
   rcommerce db migrate
   ```
3. **验证迁移成功**
   ```bash
   rcommerce db status
   ```
4. **启用应用程序流量**

#### 零停机迁移

对于大型表，使用非阻塞迁移：

```sql
-- @NO_TRANSACTION

-- 创建索引而不锁定表
CREATE INDEX CONCURRENTLY idx_orders_created_at ON orders(created_at);
```

添加不带默认值的列（快速）：

```sql
-- 步骤 1：添加可为空的列（快速）
ALTER TABLE products ADD COLUMN new_feature BOOLEAN;

-- 步骤 2：在应用程序代码中设置默认值

-- 步骤 3：批量回填数据

-- 步骤 4：使非空（稍后迁移）
```

## 故障排除

### 迁移失败

**"迁移已应用"**

```bash
-- 检查当前状态
rcommerce db status

-- 如果手动修复则强制标记为已应用
rcommerce db force --version 005
```

**"迁移校验和不匹配"**

```bash
-- 如果迁移在应用后被修改
rcommerce db verify --fix

-- 或重置并重新应用
rcommerce db rollback --to 004
rcommerce db migrate
```

**"锁定超时"**

```sql
-- 检查锁
SELECT * FROM pg_locks WHERE NOT granted;

-- 终止阻塞进程
SELECT pg_terminate_backend(pid);
```

### 常见问题

**失败的迁移使数据库处于不一致状态：**

```bash
-- 检查迁移状态
rcommerce db status

-- 回滚失败的迁移
rcommerce db rollback

-- 修复迁移文件
vim migrations/005_failed_migration.sql

-- 重试
rcommerce db migrate
```

**长时间运行的迁移：**

```bash
-- 监控进度
watch -n 5 'rcommerce db status'

-- 检查 PostgreSQL 活动
psql -c "SELECT * FROM pg_stat_activity WHERE state = 'active';"
```

### 迁移调试

启用详细日志：

```bash
-- 调试模式
RUST_LOG=debug rcommerce db migrate

-- SQL 日志
RUST_LOG=sqlx=debug rcommerce db migrate
```

查看迁移历史：

```bash
-- 列出已应用的迁移
rcommerce db history

-- 显示特定迁移详情
rcommerce db show 005
```

## 迁移参考

### CLI 命令

| 命令 | 说明 |
|---------|-------------|
| `rcommerce db migrate` | 运行待处理的迁移 |
| `rcommerce db rollback` | 回滚迁移 |
| `rcommerce db status` | 显示迁移状态 |
| `rcommerce db pending` | 列出待处理的迁移 |
| `rcommerce db history` | 显示已应用的迁移 |
| `rcommerce db create-migration` | 创建新的迁移文件 |
| `rcommerce db reset` | 重置数据库 |
| `rcommerce db force` | 强制迁移状态 |
| `rcommerce db verify` | 验证迁移完整性 |

### 迁移元数据表

R Commerce 在 `_migrations` 表中跟踪迁移：

```sql
SELECT * FROM _migrations;

-- 输出：
-- version | name                    | applied_at
-- ---------+-------------------------+------------------------
-- 001      | initial_schema          | 2024-01-01 00:00:00+00
-- 002      | add_user_preferences    | 2024-01-02 00:00:00+00
-- 003      | create_inventory_table  | 2024-01-03 00:00:00+00
```

## 相关文档

- [数据库配置](../getting-started/configuration.md)
- [本地开发设置](./local-setup.md)
- [生产部署](../deployment/binary.md)
