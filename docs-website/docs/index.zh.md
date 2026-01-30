# R Commerce

高性能无头电商平台，使用 Rust 构建。

<div class="hero-buttons" markdown>

[快速开始](getting-started/quickstart.md){ .md-button .md-button--primary }
[API 参考](api-reference/index.md){ .md-button }
[GitHub](https://github.com/creativebastard/rcommerce){ .md-button }

</div>

## 为什么选择 R Commerce？

<div class="grid cards" markdown>

-   :material-rocket-launch:{ .lg .middle } __极致性能__

    ---

    亚毫秒级响应时间，每秒处理 10万+ 请求。Rust 的速度，Node.js 的开发效率。

-   :material-scale-balance:{ .lg .middle } __企业级__

    ---

    内置多租户、角色权限管理、审计日志。符合 SOC2 和 GDPR 标准。

-   :material-code-json:{ .lg .middle } __API 优先__

    ---

    GraphQL 和 REST API。Webhook、实时订阅、完整 SDK 支持。

-   :material-postgresql:{ .lg .middle } __PostgreSQL 驱动__

    ---

    基于 PostgreSQL 构建，提供可靠性、性能和完整的 ACID 合规性。

</div>

## 核心功能

- **产品管理** - 多规格、数字产品、订阅、捆绑包
- **订单处理** - 多阶段结账、库存预留、拆分发货
- **支付网关** - Stripe、PayPal、微信支付、支付宝、Airwallex、Braintree
- **客户管理** - 分段、忠诚度、推荐计划
- **库存管理** - 多仓库、实时追踪、自动补货
- **营销工具** - 优惠券、折扣、购物车价格规则

## 性能指标

| 指标 | 数值 |
|------|------|
| API 响应时间 | < 10ms (p99) |
| 吞吐量 | 100,000+ RPS |
| 内存占用 | < 100MB |
| 启动时间 | < 1 秒 |
| 数据库连接 | 连接池化，自动故障转移 |

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/creativebastard/rcommerce.git
cd rcommerce

# 构建项目
cargo build --release

# 运行服务器
./target/release/rcommerce server
```

## 系统要求

- **Rust** 1.70+
- **PostgreSQL** 13+
- **Redis** 6+ (可选，用于缓存)

## 许可证

MIT 许可证 - 查看 [LICENSE](https://github.com/creativebastard/rcommerce/blob/main/LICENSE) 文件了解详情。

## 社区

- [GitHub Discussions](https://github.com/creativebastard/rcommerce/discussions)
- [问题反馈](https://github.com/creativebastard/rcommerce/issues)
