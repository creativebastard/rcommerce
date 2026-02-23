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

# 可选：Zig 用于交叉编译
# macOS: brew install zig
# Linux: 参见 https://ziglang.org/download/
```

### 平台特定前提条件

**FreeBSD：**

```bash
# 安装系统包
pkg install -y \
  postgresql15-client \
  pkgconf \
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
  postgresql-client \
  libpq-dev
```

**Linux (CentOS/RHEL/Fedora)：**

```bash
# 安装构建依赖
yum groupinstall -y "Development Tools"
yum install -y \
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

# 可选：安装 Zig 用于交叉编译
brew install zig
```

## 构建步骤（所有平台）

### 快速构建

```bash
# 克隆仓库
git clone https://github.com/creativebastard/rcommerce.git
cd rcommerce

# 构建发布二进制文件
cargo build --release -p rcommerce-cli

# 二进制文件位置
target/release/rcommerce

# 运行测试
cargo test --release
```

### 交叉编译构建（推荐用于分发）

```bash
# 安装 cargo-zigbuild 用于交叉编译
cargo install cargo-zigbuild

# 添加交叉编译目标
rustup target add \
  aarch64-apple-darwin \
  x86_64-apple-darwin \
  x86_64-unknown-linux-gnu \
  aarch64-unknown-linux-gnu \
  x86_64-unknown-linux-musl \
  aarch64-unknown-linux-musl \
  x86_64-unknown-freebsd

# 为所有平台构建
./scripts/build-release.sh

# 或构建特定平台
./scripts/build-release.sh --macos-only      # 仅 macOS
./scripts/build-release.sh --linux-only      # Linux (GNU + MUSL)
./scripts/build-release.sh --musl-only       # 静态 Linux 二进制文件
./scripts/build-release.sh --freebsd-only    # 仅 FreeBSD
```

**构建输出：**

| 平台 | 二进制文件 | 大小 |
|------|------------|------|
| macOS ARM64 | `rcommerce-macos-arm64` | ~14 MB |
| macOS x86_64 | `rcommerce-macos-x86_64` | ~16 MB |
| macOS 通用 | `rcommerce-macos-universal` | ~30 MB |
| Linux x86_64 GNU | `rcommerce-x86_64-linux-gnu` | ~15 MB |
| Linux x86_64 MUSL | `rcommerce-x86_64-linux-static` | ~14 MB |
| Linux ARM64 GNU | `rcommerce-aarch64-linux-gnu` | ~13 MB |
| Linux ARM64 MUSL | `rcommerce-aarch64-linux-static` | ~12 MB |
| Linux ARMv7 | `rcommerce-armv7-linux` | ~12 MB |
| FreeBSD x86_64 | `rcommerce-x86_64-freebsd` | ~15 MB |

所有二进制文件都输出到 `dist/` 目录，并包含 SHA256 校验和。

## 初始设置

构建完成后，使用交互式设置向导配置您的实例：

```bash
# 运行设置向导
./target/release/rcommerce setup

# 向导将引导您完成：
# - 数据库配置和迁移
# - 从现有商店导入数据（可选）
# - 服务器、缓存和安全设置
# - TLS/SSL（包括 Let's Encrypt）
# - 支付网关和通知
```

**设置向导选项：**

```bash
# 保存配置到特定位置
./target/release/rcommerce setup -o /etc/rcommerce/config.toml

# 如果跳过向导，请手动配置：
# 1. 创建 config.toml（参见配置指南）
# 2. 运行迁移：./target/release/rcommerce db migrate -c config.toml
# 3. 启动服务器：./target/release/rcommerce server -c config.toml
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
wget https://github.com/creativebastard/rcommerce/releases/download/v0.1.0/rcommerce-x86_64-linux-static.tar.gz
tar -xzf rcommerce-x86_64-linux-static.tar.gz
sudo mv rcommerce /usr/local/bin/

# 下载 macOS ARM64 版本
wget https://github.com/creativebastard/rcommerce/releases/download/v0.1.0/rcommerce-macos-arm64.tar.gz
tar -xzf rcommerce-macos-arm64.tar.gz
sudo mv rcommerce /usr/local/bin/
```

### 通过 Cargo 安装

```bash
# 从源码安装
git clone https://github.com/creativebastard/rcommerce.git
cd rcommerce
cargo install --path crates/rcommerce-cli

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

**交叉编译错误**

```bash
# 确保为无数据库构建设置了 SQLX_OFFLINE
export SQLX_OFFLINE=true

# 确保已安装 Zig（用于 cargo-zigbuild）
zig version

# 如需，重新安装 cargo-zigbuild
cargo install cargo-zigbuild --force
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

**静态二进制文件无法在旧版 Linux 上运行**

MUSL 静态二进制文件应该可以在任何 Linux 系统上运行。如果遇到问题：

```bash
# 检查二进制文件类型
file rcommerce

# 尝试改用 GNU 版本（需要 glibc）
./rcommerce-x86_64-linux-gnu
```

## 下一步

- [配置指南](configuration.zh.md) - 配置您的安装
- [开发指南](../development/index.md) - 设置开发环境
- [部署指南](../deployment/index.md) - 部署到生产环境
