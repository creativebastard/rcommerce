# 架构文档

本章节提供关于 R Commerce 架构和设计决策的深入技术文档。

## 概述

R Commerce 采用模块化、API 优先的架构设计，注重性能、可靠性和部署便捷性。

## 架构文档

| 文档 | 描述 |
|------|------|
| [01-overview.md](01-overview.md) | 架构愿景、为什么选择 Rust、为什么选择无头架构 |
| [02-data-modeling.md](02-data-modeling.md) | 核心数据模型和数据库模式 |
| [04-database-abstraction.md](04-database-abstraction.md) | 数据库层和仓库模式 |
| [05-payment-architecture.md](05-payment-architecture.md) | 支付网关集成设计 |
| [06-shipping-integration.md](06-shipping-integration.md) | 物流提供商架构 |
| [07-order-management.md](07-order-management.md) | 订单生命周期和状态管理 |
| [08-compatibility-layer.md](08-compatibility-layer.md) | 平台兼容性和迁移方案 |
| [09-product-types-and-subscriptions.md](09-product-types-and-subscriptions.md) | 产品类型和订阅处理 |
| [09-media-storage.md](09-media-storage.md) | 媒体文件存储和 CDN 集成 |
| [10-notifications.md](10-notifications.md) | 邮件、短信和 Webhook 通知 |
| [11-dunning-payment-retry.md](11-dunning-payment-retry.md) | 失败支付重试逻辑 |
| [12-redis-caching.md](12-redis-caching.md) | Redis 缓存策略 |

## 设计原则

1. **模块化** - 基于插件的架构
2. **配置即代码** - TOML/JSON 配置
3. **API 完整性** - 100% 功能通过 API 提供
4. **PostgreSQL 支持** - 基于 PostgreSQL 构建，支持可靠性和性能
5. **可观测性** - 内置日志和指标

## 相关文档

- [API 文档](../api/index.md) - REST/GraphQL API 参考
- [部署指南](../deployment/index.md) - 安装和部署
- [开发指南](../development/developer-guide.md) - 贡献和开发
