# 贡献指南

感谢您对贡献 R Commerce 的兴趣！本指南将帮助您入门。

## 行为准则

本项目遵循标准的行为准则：
- 尊重和包容
- 欢迎新成员
- 专注于建设性反馈
- 尊重不同观点

## 入门

### 1. Fork 和克隆

```bash
# 在 GitHub 上 Fork 仓库，然后克隆您的 Fork
git clone https://github.com/YOUR_USERNAME/rcommerce.git
cd rcommerce

# 添加上游远程仓库
git remote add upstream https://github.com/creativebastard/rcommerce.git
```

### 2. 设置开发环境

按照[本地开发设置](./local-setup.zh.md)指南配置您的环境。

### 3. 创建分支

```bash
# 与上游同步
git fetch upstream
git checkout main
git merge upstream/main

# 创建功能分支
git checkout -b feature/your-feature-name
```

## 开发工作流

### 进行更改

1. **编写代码** 遵循 Rust 最佳实践
2. **添加测试** 用于新功能
3. **更新文档** 根据需要
4. **运行测试** 确保没有破坏任何东西

```bash
# 格式化代码
cargo fmt

# 运行 linter
cargo clippy

# 运行测试
cargo test --workspace

# 检查安全问题
cargo audit
```

### 提交指南

使用约定式提交格式：

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

类型：
- `feat`：新功能
- `fix`：Bug 修复
- `docs`：文档变更
- `style`：代码样式变更（格式化）
- `refactor`：代码重构
- `test`：测试变更
- `chore`：构建过程或辅助工具变更

示例：
```
feat(products): 添加产品变体支持

fix(orders): 修正折扣的税费计算
docs(api): 更新认证示例
```

### 提交前

```bash
# 确保所有测试通过
cargo test --workspace

# 检查代码格式
cargo fmt --check

# 运行 clippy
cargo clippy -- -D warnings

# 以发布模式构建
cargo build --release
```

## Pull Request 流程

### 1. 更新文档

- 更新 `docs/` 或 `docs-website/` 中的相关文档
- 如适用，添加示例
- 更新 CHANGELOG.md

### 2. 创建 Pull Request

```bash
# 推送您的分支
git push origin feature/your-feature-name
```

然后在 GitHub 上创建 PR，包含：
- 清晰描述变更的标题
- 详细描述做了什么以及为什么
- 引用任何相关问题
- UI 变更的截图

### 3. PR 审核

- 处理审核意见
- 保持讨论专注和专业
- 保持耐心 - 维护者是志愿者

### 4. 合并

一旦批准，维护者将合并您的 PR。

## 贡献领域

### 高优先级

- **支付网关**：额外的支付提供商集成
- **物流**：更多物流承运商集成
- **前端**：演示前端改进
- **文档**：用户指南和教程

### 中优先级

- **性能**：优化和缓存改进
- **测试**：额外的测试覆盖
- **CLI**：新的 CLI 命令和功能
- **监控**：指标和可观测性

### 适合新手的 Issue

查找带有以下标签的 issue：
- `good first issue`
- `help wanted`
- `documentation`

## 项目结构

```
rcommerce/
├── crates/
│   ├── rcommerce-core/     # 核心业务逻辑
│   ├── rcommerce-api/      # HTTP API 服务器
│   └── rcommerce-cli/      # 命令行工具
├── docs/                    # 技术文档
├── docs-website/           # 用户文档网站
├── scripts/                # 实用脚本
└── migrations/             # 数据库迁移
```

## 编码标准

### Rust 风格

遵循 [Rust API 指南](https://rust-lang.github.io/api-guidelines/)：

- 函数和变量使用 `snake_case`
- 类型和 trait 使用 `CamelCase`
- 常量使用 `SCREAMING_SNAKE_CASE`
- 用文档注释记录公共 API

### 示例

```rust
/// 计算包含税费的总价
/// 
/// # 参数
/// 
/// * `price` - 基础价格
/// * `tax_rate` - 税率，小数形式（例如 10% 为 0.10）
/// 
/// # 返回
/// 
/// 应用税费后的总价
/// 
/// # 示例
/// 
/// ```
/// let total = calculate_total(dec!(100.00), dec!(0.10));
/// assert_eq!(total, dec!(110.00));
/// ```
pub fn calculate_total(price: Decimal, tax_rate: Decimal) -> Decimal {
    price * (Decimal::ONE + tax_rate)
}
```

### 错误处理

使用项目的错误类型：

```rust
use rcommerce_core::{Error, Result};

fn do_something() -> Result<Thing> {
    if invalid {
        return Err(Error::validation("Field is required"));
    }
    Ok(thing)
}
```

## 测试要求

- 所有新功能必须包含测试
- Bug 修复应包含回归测试
- 保持或提高代码覆盖率
- API 端点的集成测试

## 文档

### 代码文档

- 记录所有公共函数、结构体和 trait
- 在文档注释中包含示例
- 解释复杂算法

### 用户文档

在以下位置更新相关文档：
- `docs/` - 技术文档
- `docs-website/` - 用户面向文档
- `README.md` - 项目概述

## 安全性

私下报告安全漏洞：
- 邮箱：security@rcommerce.app
- 不要为安全 bug 打开公共 issue

## 获取帮助

- **GitHub Discussions**：用于问题和想法
- **Discord**：[加入我们的社区](https://discord.gg/rcommerce)
- **Issues**：用于 bug 报告和功能请求

## 致谢

贡献者将被：
- 列入 CONTRIBUTORS.md
- 在发布说明中提及
- 在文档中致谢

感谢您为 R Commerce 做出贡献！
