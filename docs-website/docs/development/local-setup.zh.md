# 本地开发设置

本指南将引导您完成 R Commerce 本地开发环境的设置。

## 前提条件

- **Rust 1.70+**（从 [rustup.rs](https://rustup.rs/) 安装）
- **PostgreSQL 13+**
- **Redis 6+**（可选，用于缓存）
- **Git**
- **Zig**（可选，用于交叉编译）

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

## 交叉编译

要从开发机器为其他平台构建二进制文件：

### 交叉编译的前提条件

```bash
# 安装 Zig（交叉编译链接器）
brew install zig

# 安装 cargo-zigbuild
cargo install cargo-zigbuild

# 添加交叉编译目标
rustup target add \
  x86_64-unknown-linux-gnu \
  aarch64-unknown-linux-gnu \
  x86_64-unknown-linux-musl \
  aarch64-unknown-linux-musl \
  x86_64-unknown-freebsd
```

### 使用构建脚本

```bash
# 为所有平台构建
./scripts/build-release.sh

# 仅构建 macOS 目标
./scripts/build-release.sh --macos-only

# 仅构建 Linux 目标
./scripts/build-release.sh --linux-only

# 构建特定目标
./scripts/build-release.sh x86_64-unknown-linux-musl
```

### 手动交叉编译

```bash
# 为 Linux x86_64 构建（需要 cargo-zigbuild）
SQLX_OFFLINE=true cargo zigbuild --release --target x86_64-unknown-linux-musl -p rcommerce-cli

# 为 Linux ARM64 构建
SQLX_OFFLINE=true cargo zigbuild --release --target aarch64-unknown-linux-musl -p rcommerce-cli

# 为 FreeBSD 构建
SQLX_OFFLINE=true cargo zigbuild --release --target x86_64-unknown-freebsd -p rcommerce-cli
```

二进制文件将位于 `target/<target-triple>/release/rcommerce`。

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

**OpenSSL 交叉编译错误**

R Commerce 使用 rustls（纯 Rust TLS）而不是 OpenSSL，因此不应该出现 OpenSSL 链接问题。如果遇到 TLS 相关的构建错误：

```bash
# 确保您使用的是 rustls 功能标志
grep -r "rustls" Cargo.toml crates/*/Cargo.toml
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
