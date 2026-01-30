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
git clone https://gitee.com/captainjez/gocart.git
cd gocart

# 构建项目
cargo build --release

# 二进制文件将位于：
# target/release/rcommerce
```

### 选项 2：Docker（快速开始推荐）

```bash
# 使用 Docker Compose 启动
docker-compose up -d

# 这将启动：
# - R Commerce API (端口 8080)
# - PostgreSQL 数据库
# - Redis 缓存
```

## 数据库设置

### PostgreSQL

```bash
# 创建数据库
createdb rcommerce_dev

# 创建用户并授权
psql rcommerce_dev << EOF
CREATE USER rcommerce_dev WITH PASSWORD 'devpass';
GRANT ALL PRIVILEGES ON DATABASE rcommerce_dev TO rcommerce_dev;
EOF
```

### 3. 运行迁移

```bash
# 运行数据库迁移
cargo run -- migrate run
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
# 使用发布构建
./target/release/rcommerce server --config config/production.toml
```

## 验证安装

```bash
# 健康检查
curl http://localhost:8080/health

# 预期响应：OK

# 获取 API 信息
curl http://localhost:8080/
```

## 下一步

- [配置指南](configuration.md) - 自定义您的安装
- [API 参考](../api-reference/index.md) - 探索 API
- [部署指南](../deployment/docker.md) - 生产部署

## 故障排除

### 数据库连接错误

确保 PostgreSQL 正在运行且凭据正确：

```bash
# 测试连接
psql -h localhost -U rcommerce_dev -d rcommerce_dev
```

### 端口冲突

如果端口 8080 已被占用：

```bash
# 使用不同端口
cargo run -- server -p 8081
```

### 构建错误

确保您使用的是最新稳定版 Rust：

```bash
rustup update
rustc --version  # 应为 1.70+
```
