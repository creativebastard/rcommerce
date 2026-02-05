# 本地开发设置

本指南将引导您完成 R Commerce 本地开发环境的设置。

## 前提条件

- **Rust 1.70+**（从 [rustup.rs](https://rustup.rs/) 安装）
- **PostgreSQL 13+** 或 **MySQL 8+** 或 **SQLite 3+**
- **Redis 6+**（可选，用于缓存）
- **Git**

## 快速开始

### 1. 克隆仓库

```bash
git clone https://github.com/creativebastard/rcommerce.git
cd rcommerce
```

### 2. 设置数据库

#### 选项 A：PostgreSQL（推荐）

```bash
# macOS
brew install postgresql@15
brew services start postgresql@15

# 创建数据库
psql -U postgres -c "CREATE DATABASE rcommerce_dev;"
psql -U postgres -c "CREATE USER rcommerce_dev WITH PASSWORD 'devpass';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce_dev TO rcommerce_dev;"
```

#### 选项 B：SQLite（开发最简单）

无需设置 - SQLite 将自动创建文件。

### 3. 配置环境

```bash
# 复制示例配置
cp config.example.toml config.development.toml

# 编辑配置文件
# 根据需要更新数据库连接设置
```

### 4. 构建和运行

```bash
# 构建项目
cargo build --release

# 运行数据库迁移
cargo run --bin rcommerce -- db migrate

# 启动服务器
cargo run --bin rcommerce -- server
```

API 将在 `http://localhost:8080` 可用。

### 5. 验证安装

```bash
# 健康检查
curl http://localhost:8080/health

# API 信息
curl http://localhost:8080/
```

## 开发配置

### 数据库配置

**PostgreSQL：**
```toml
[database]
db_type = "Postgres"
host = "localhost"
port = 5432
database = "rcommerce_dev"
username = "rcommerce_dev"
password = "devpass"
```

**SQLite：**
```toml
[database]
db_type = "Sqlite"
sqlite_path = "./rcommerce_dev.db"
```

### 可选：Redis 缓存

```bash
# 安装 Redis
brew install redis  # macOS
brew services start redis

# 在 config.toml 中启用
[cache]
cache_type = "Redis"
redis_url = "redis://localhost:6379"
```

## 运行测试

```bash
# 运行所有测试
cargo test --workspace

# 运行特定 crate 的测试
cargo test -p rcommerce-core

# 带输出运行
cargo test -- --nocapture
```

## 开发工具

### 热重载

```bash
# 安装 cargo-watch
cargo install cargo-watch

# 监视变更并重新构建
cargo watch -x 'run --bin rcommerce -- server'
```

### 代码质量

```bash
# 格式化代码
cargo fmt

# 运行 linter
cargo clippy

# 检查安全漏洞
cargo audit
```

## IDE 设置

### VS Code

推荐扩展：
- **rust-analyzer** - Rust 语言支持
- **Even Better TOML** - TOML 文件支持
- **CodeLLDB** - 调试支持

### IntelliJ / RustRover

- 安装 Rust 插件
- 作为 Cargo 项目导入
- 为 `cargo run` 和 `cargo test` 设置运行配置

## 故障排除

### 构建失败

**错误：`ld: library not found for -lpq`**
```bash
# macOS：安装 PostgreSQL 客户端库
brew install libpq
brew link libpq --force
```

**错误：`sqlx` 编译时检查失败**
```bash
# 为无数据库构建设置 SQLX_OFFLINE
export SQLX_OFFLINE=true
cargo build
```

### 数据库连接问题

**PostgreSQL 连接被拒绝**
```bash
# 检查 PostgreSQL 是否正在运行
brew services list | grep postgresql

# 重启 PostgreSQL
brew services restart postgresql@15
```

### 端口已被占用

```bash
# 查找使用端口 8080 的进程
lsof -i :8080

# 终止进程或在 config.toml 中更改端口
[server]
port = 8081
```

## 下一步

- [CLI 参考](./cli-reference.zh.md) - 了解 CLI 命令
- [测试指南](./testing.zh.md) - 编写和运行测试
- [贡献指南](./contributing.zh.md) - 为项目做出贡献
