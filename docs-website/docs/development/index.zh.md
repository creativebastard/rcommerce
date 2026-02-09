# 开发文档

本章节包含面向使用或贡献 R Commerce 的开发者的资源。

## 开发者资源

| 文档 | 描述 |
|------|------|
| [developer-guide.md](developer-guide.md) | 完整的开发环境搭建指南 |
| [development-roadmap.md](development-roadmap.md) | 项目路线图和时间线 |
| [cli-reference.md](cli-reference.md) | 完整的 CLI 命令参考 |
| [configuration-reference.md](configuration-reference.md) | 配置选项说明 |
| [contributing.md](../CONTRIBUTING.md) | 贡献指南 |

## 开发者快速入门

### 1. 克隆仓库

```bash
git clone https://github.com/creativebastard/rcommerce.git
cd rcommerce
```

### 2. 安装依赖

```bash
# Rust（最新稳定版）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL
# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt-get install postgresql
```

### 3. 设置数据库

```bash
# 创建数据库
psql -U postgres -c "CREATE DATABASE rcommerce;"
psql -U postgres -c "CREATE USER rcommerce WITH PASSWORD 'password';"
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rcommerce TO rcommerce;"
```

### 4. 配置

```bash
cp config.example.toml config.toml
# 编辑 config.toml 添加您的设置
```

### 5. 运行

```bash
cargo run --bin rcommerce -- server
```

## 测试

```bash
# 运行所有测试
cargo test --workspace

# 带输出运行
cargo test -- --nocapture

# 检查代码格式
cargo fmt --check

# 运行代码检查工具
cargo clippy
```

## 项目结构

```
rcommerce/
├── crates/
│   ├── rcommerce-core/     # 核心库
│   ├── rcommerce-api/      # HTTP API 服务器
│   └── rcommerce-cli/      # CLI 工具
├── docs/                    # 文档
├── migrations/              # 数据库迁移
└── scripts/                 # 实用脚本
```

## 相关文档

- [架构](../architecture/overview.md) - 系统架构
- [API 文档](../api-reference/index.md) - API 参考
- [部署](../deployment/index.md) - 部署指南
