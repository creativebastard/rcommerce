# 安装指南

本指南涵盖在各种平台上安装 R Commerce 的详细说明。

## 系统要求

### 最低要求

- **CPU**：2 核
- **内存**：2 GB
- **存储**：10 GB
- **操作系统**：Linux、macOS 或 FreeBSD

### 推荐要求

- **CPU**：4+ 核
- **内存**：4+ GB
- **存储**：50+ GB SSD
- **网络**：100 Mbps+ 带宽

## 前提条件

### 所有平台

```bash
# Rust 1.70+（从 https://rustup.rs 安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL 13+（必需）
# 请参阅下面的数据库设置

# 可选但推荐
# - pkg-config
# - OpenSSL 开发头文件
```

### 平台特定前提条件

**FreeBSD：**

```bash
# 安装系统包
pkg install -y \
  postgresql15-client \
  pkgconf \
  openssl \
  ca_root_nss

# 用于附加功能
pkg install -y \
  redis \
  nginx \
  haproxy
```

**Linux (Debian/Ubuntu)：**

```bash
# 安装构建依赖
apt-get update
apt-get install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  postgresql-client \
  libpq-dev
apt-get install -y \
  libmysqlclient-dev
```

**Linux (CentOS/RHEL/Fedora)：**

```bash
# 安装构建依赖
yum groupinstall -y "Development Tools"
yum install -y \
  openssl-devel \
  postgresql-devel \
  pkgconfig
```

**macOS：**

```bash
# 安装 Xcode 命令行工具
xcode-select --install

# 如果尚未安装 Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装依赖
brew install \
  postgresql@15 \
  pkg-config
```

## 构建步骤（所有平台）

```bash
# 克隆仓库
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# 构建发布二进制文件
cargo build --release

# 二进制文件位置
target/release/rcommerce

# 运行测试
cargo test --release

# 检查平台兼容性
cargo check --target x86_64-unknown-linux-gnu     # Linux x86_64
cargo check --target aarch64-unknown-linux-gnu    # Linux ARM64
cargo check --target x86_64-unknown-freebsd      # FreeBSD x86_64
cargo check --target aarch64-unknown-freebsd     # FreeBSD ARM64
cargo check --target x86_64-apple-darwin         # macOS x86_64
cargo check --target aarch64-apple-darwin        # macOS ARM64 (Apple Silicon)
```

## Docker 安装

### 使用 Docker Compose（推荐）

```bash
# 克隆仓库
git clone https://github.com/creativebastard/rcommerce.git
cd gocart

# 创建环境文件
cp .env.example .env
# 编辑 .env 进行配置

# 使用 Docker Compose 启动
docker-compose up -d

# 查看日志
docker-compose logs -f rcommerce

# 验证健康状态
curl http://localhost:8080/health
```

### 手动 Docker 构建

```dockerfile
# 构建阶段
FROM rust:1.75-slim as builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制清单并构建依赖
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --bin rcommerce || true

# 复制源代码
COPY . .

# 构建应用程序
RUN cargo build --release --bin rcommerce

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 创建服务用户
RUN groupadd -r rcommerce && useradd -r -g rcommerce rcommerce

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rcommerce /usr/local/bin/
COPY --from=builder /app/config/default.toml /etc/rcommerce/config.toml

# 创建目录
RUN mkdir -p /var/lib/rcommerce /var/log/rcommerce && \
    chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce

# 切换到服务用户
USER rcommerce

# 暴露端口
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# 启动应用程序
CMD ["rcommerce"]
```

构建并运行：

```bash
# 构建镜像
docker build -t rcommerce:latest .

# 运行容器
docker run -d \
  --name rcommerce \
  -p 8080:8080 \
  -v $(pwd)/config/production.toml:/etc/rcommerce/config.toml:ro \
  rcommerce:latest
```

## 二进制安装

### 下载预构建二进制文件

可用时，从发布页面下载预构建的二进制文件：

```bash
# 下载 Linux x86_64 版本
wget https://github.com/captainjez/gocart/releases/download/v0.1.0/rcommerce-linux-x86_64.tar.gz
tar -xzf rcommerce-linux-x86_64.tar.gz
sudo mv rcommerce /usr/local/bin/

# 下载 macOS ARM64 版本
wget https://github.com/captainjez/gocart/releases/download/v0.1.0/rcommerce-macos-arm64.tar.gz
tar -xzf rcommerce-macos-arm64.tar.gz
sudo mv rcommerce /usr/local/bin/
```

### 通过 Cargo 安装

```bash
# 从 crates.io 安装（发布时）
cargo install rcommerce-cli

# 验证安装
rcommerce --version
```

## 包管理器安装

### Homebrew (macOS/Linux)

```bash
# 使用 Homebrew（可用时）
brew tap rcommerce/tap
brew install rcommerce
```

### APT (Debian/Ubuntu)

```bash
# 添加仓库（可用时）
sudo apt-add-repository 'deb https://apt.rcommerce.app stable main'
sudo apt update
sudo apt install rcommerce
```

### FreeBSD pkg

```bash
# 使用 pkg（可用时）
sudo pkg install rcommerce

# 从 ports
cd /usr/ports/commerce/rcommerce && sudo make install
```

## 安装后配置

### 1. 创建配置目录

```bash
sudo mkdir -p /etc/rcommerce
sudo mkdir -p /var/lib/rcommerce
sudo mkdir -p /var/log/rcommerce
```

### 2. 创建服务用户

```bash
# Linux
sudo useradd -r -s /bin/false rcommerce
sudo chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce

# FreeBSD
sudo pw useradd rcommerce -d /var/lib/rcommerce -s /bin/false
sudo chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce
```

### 3. 设置配置

```bash
# 复制示例配置
sudo cp config/production.toml /etc/rcommerce/config.toml

# 编辑配置
sudo nano /etc/rcommerce/config.toml
```

### 4. 测试安装

```bash
# 测试配置
rcommerce config validate

# 测试数据库连接
rcommerce db check

# 运行服务器
rcommerce server start
```

## 故障排除

### 构建失败

**错误：`linker 'cc' not found`**

```bash
# 安装构建工具
# Ubuntu/Debian
sudo apt-get install build-essential

# CentOS/RHEL
sudo yum groupinstall "Development Tools"

# macOS
xcode-select --install
```

**错误：`openssl-sys` 构建失败**

```bash
# 安装 OpenSSL 开发头文件
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# CentOS/RHEL
sudo yum install openssl-devel pkgconfig

# macOS
brew install openssl pkg-config
export PKG_CONFIG_PATH="/opt/homebrew/opt/openssl/lib/pkgconfig"
```

**错误：`pq-sys` 构建失败 (PostgreSQL)**

```bash
# 安装 PostgreSQL 开发头文件
# Ubuntu/Debian
sudo apt-get install libpq-dev

# CentOS/RHEL
sudo yum install postgresql-devel

# macOS
brew install postgresql@15
```

### 运行时问题

**权限被拒绝**

```bash
# 修复权限
sudo chown -R rcommerce:rcommerce /var/lib/rcommerce /var/log/rcommerce
sudo chmod 750 /var/lib/rcommerce /var/log/rcommerce
```

**端口已被占用**

```bash
# 查找使用端口的进程
sudo lsof -i :8080

# 终止进程或在配置中更改端口
```

## 下一步

- [配置指南](configuration.zh.md) - 配置您的安装
- [开发指南](../development/index.md) - 设置开发环境
- [部署指南](../deployment/index.md) - 部署到生产环境
