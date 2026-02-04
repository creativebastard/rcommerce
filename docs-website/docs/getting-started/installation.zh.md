# 安装指南

本指南涵盖在各种平台上安装 R Commerce 的详细说明。

## 系统要求

### 最低要求

- **CPU**: 2 核
- **内存**: 2 GB
- **存储**: 10 GB
- **操作系统**: Linux、macOS 或 FreeBSD

### 推荐要求

- **CPU**: 4+ 核
- **内存**: 4+ GB
- **存储**: 50+ GB SSD
- **网络**: 100 Mbps+ 带宽

## 前提条件

### 所有平台

```bash
# Rust 1.70+ (从 https://rustup.rs 安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL 13+ (必需)
# 请参阅下面的数据库设置

# 可选但推荐
# - pkg-config
# - OpenSSL 开发头文件
```

### 平台特定前提条件

**FreeBSD:**

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

**Linux (Debian/Ubuntu):**

```bash
# 安装构建依赖
apt-get update
apt-get install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  postgresql-client \
  libpq-dev
```

**Linux (CentOS/RHEL/Fedora):**

```bash
# 安装构建依赖
yum groupinstall -y "Development Tools"
yum install -y \
  openssl-devel \
  postgresql-devel \
  pkgconfig
```

**macOS:**

```bash
# 安装 Xcode 命令行工具
xcode-select --install

# 安装 Homebrew（如果尚未安装）
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

# 构建项目
cargo build --release

# 二进制文件将位于：
# target/release/rcommerce
```

## 验证安装

```bash
# 检查版本
./target/release/rcommerce --version

# 运行健康检查
./target/release/rcommerce health

# 测试配置
./target/release/rcommerce config --check
```

## 数据库设置

### PostgreSQL

```bash
# 创建数据库
createdb rcommerce

# 创建用户
psql -c "CREATE USER rcommerce WITH PASSWORD 'your_password';"
psql -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce TO rcommerce;"
```

## 故障排除

### 编译错误

**错误**: `linker 'cc' not found`

**解决方案**: 安装 C 编译器
```bash
# Debian/Ubuntu
apt-get install build-essential

# CentOS/RHEL
yum groupinstall "Development Tools"

# macOS
xcode-select --install
```

**错误**: `openssl-sys` 构建失败

**解决方案**: 安装 OpenSSL 开发包
```bash
# Debian/Ubuntu
apt-get install libssl-dev

# CentOS/RHEL
yum install openssl-devel

# macOS
brew install openssl
```

### 运行时错误

**错误**: `cannot connect to database`

**解决方案**: 
1. 检查 PostgreSQL 是否正在运行
2. 验证连接字符串
3. 确认防火墙规则

## 下一步

- [配置指南](configuration.md) - 配置您的安装
- [快速开始](quickstart.md) - 启动并运行
- [部署指南](../deployment/docker.md) - 生产部署
