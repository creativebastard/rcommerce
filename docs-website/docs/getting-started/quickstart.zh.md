# 快速开始指南

通过本快速开始指南，在几分钟内启动并运行 R Commerce。

## 前提条件

在开始之前，请确保已安装以下软件：

- **Rust 1.70+** - [从 rustup.rs 安装](https://rustup.rs/)
- **PostgreSQL 13+**
- **Redis 6+** (可选，用于缓存)

## 安装

### 选项 1：从源码构建

```bash
# 克隆仓库
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# 构建项目
cargo build --release

# 二进制文件将位于：
# target/release/rcommerce
```

### 选项 2：Docker（快速开始推荐）

```bash
# 克隆仓库
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# 启动所有服务
docker-compose up -d

# 检查状态
docker-compose ps
```

## 配置

### 选项 1：交互式设置向导（推荐）

配置 R Commerce 最简单的方法是使用设置向导：

```bash
# 运行交互式设置向导
./target/release/rcommerce setup

# 或使用特定输出文件
./target/release/rcommerce setup -o config/production.toml
```

向导将引导您完成：
- 店铺配置（名称、货币）
- 数据库设置（PostgreSQL）
- 数据库迁移（处理现有数据库）
- 从 WooCommerce、Shopify、Magento 或 Medusa 导入可选数据
- 服务器、缓存和安全设置
- TLS/SSL 配置（包括 Let's Encrypt）
- 支付网关和电子邮件通知

### 选项 2：手动配置

创建 `config/development.toml` 文件：

```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
db_type = "Postgres"
host = "localhost"
port = 5432
username = "rcommerce_dev"
password = "devpass"
database = "rcommerce_dev"
pool_size = 5

[cache]
cache_type = "Memory"

[payment]
test_mode = true
```

### 数据库设置

**创建数据库（PostgreSQL）：**

```bash
# 创建数据库
psql -U postgres -c "CREATE DATABASE rcommerce_dev;"
psql -U postgres -c "CREATE USER rcommerce_dev WITH PASSWORD 'devpass';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce_dev TO rcommerce_dev;"
```

**运行迁移：**

```bash
# 如果使用设置向导，迁移会自动运行
# 否则，手动运行：
./target/release/rcommerce db migrate -c config.toml
```

## 运行服务器

### 开发模式

```bash
# 使用热重载运行
cargo watch -x run

# 或直接运行
cargo run

# 使用特定配置
cargo run -- --config config/development.toml
```

### 生产模式

```bash
# 构建发布二进制文件
cargo build --release

# 使用生产配置运行
./target/release/rcommerce --config config/production.toml
```

## 验证安装

### 健康检查

```bash
curl http://localhost:8080/health
```

预期响应：

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "database": "connected",
  "cache": "connected",
  "timestamp": "2024-01-23T14:13:35Z"
}
```

### 创建您的第一个产品

```bash
curl -X POST http://localhost:8080/api/v1/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "name": "Test Product",
    "slug": "test-product",
    "description": "A test product",
    "price": 29.99,
    "status": "active"
  }'
```

## 下一步

- [安装指南](installation.zh.md) - 详细的安装说明
- [配置指南](configuration.zh.md) - 完整的配置参考
- [API 参考](../api-reference/index.md) - 开始使用 API 构建
- [开发指南](../development/index.md) - 设置开发环境

## 故障排除

### 端口已被占用

```bash
# 查找使用端口 8080 的进程
lsof -i :8080

# 终止进程或使用不同端口
# 编辑 config/development.toml 并更改端口
```

### 数据库连接失败

```bash
# 检查 PostgreSQL 是否正在运行
pg_isready -h localhost -p 5432

# 检查凭据
psql -U rcommerce_dev -d rcommerce_dev -h localhost -W
```

### 构建错误

```bash
# 更新 Rust
rustup update

# 清理并重新构建
cargo clean
cargo build --release
```

## 获取帮助

- **文档**：浏览完整文档
- **GitHub Issues**：报告错误和请求功能
- **Discord**：加入社区获取实时帮助
