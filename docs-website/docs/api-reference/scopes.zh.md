# API 权限范围参考

R Commerce 使用细粒度的基于权限范围的 API 密钥权限系统。权限范围定义了 API 密钥可以访问哪些资源以及可以执行哪些操作。

## 权限范围格式

权限范围遵循以下格式：`resource:action`

```
products:read
│         │
│         └── 操作: read、write 或 admin
└─────────── 资源: products、orders、customers 等
```

### 全局权限范围

全局权限范围适用于所有资源：

- `read` - 对所有资源的读取访问权限
- `write` - 对所有资源的写入访问权限（包含 read）
- `admin` - 对所有资源的完全管理访问权限（包含 read 和 write）

### 资源特定权限范围

资源特定权限范围适用于单一资源类型：

- `products:read` - 仅读取产品
- `products:write` - 仅写入产品（包含读取）
- `orders:read` - 仅读取订单
- `orders:write` - 仅写入订单（包含读取）

## 可用资源

以下资源可用于权限范围：

| 资源 | 描述 | 典型使用场景 |
|------|------|-------------|
| `products` | 产品目录管理 | 产品同步、库存更新 |
| `orders` | 订单管理 | 订单处理、履约 |
| `customers` | 客户数据 | CRM 集成、客户查询 |
| `carts` | 购物车 | 购物车持久化、废弃购物车恢复 |
| `coupons` | 折扣优惠券 | 促销管理 |
| `payments` | 支付记录 | 支付对账、退款 |
| `inventory` | 库存跟踪 | 库存管理、仓库同步 |
| `webhooks` | Webhook 配置 | Webhook 管理、事件处理 |
| `users` | 用户账户 | 用户管理、访问控制 |
| `settings` | 系统设置 | 配置管理 |
| `reports` | 分析报告 | 报告集成、数据导出 |
| `imports` | 数据导入 | 批量产品导入、迁移 |
| `exports` | 数据导出 | 数据备份、报告 |

## 可用操作

三个操作级别可用：

| 操作 | 描述 | HTTP 方法 |
|------|------|----------|
| `read` | 查看资源 | GET |
| `write` | 创建、更新、删除资源 | POST、PUT、PATCH、DELETE |
| `admin` | 管理操作 | 所有方法 + 仅管理员端点 |

### 权限层级

操作遵循分层权限模型：

```
admin
 └── write
      └── read
```

- **`write`** 包含 **`read`** 权限
- **`admin`** 包含 **`read`** 和 **`write`** 权限

示例：具有 `products:write` 权限的密钥可以：
- ✅ 读取产品 (GET /api/v1/products)
- ✅ 创建产品 (POST /api/v1/products)
- ✅ 更新产品 (PUT /api/v1/products/:id)
- ✅ 删除产品 (DELETE /api/v1/products/:id)

## 权限范围组合

### 单一资源访问

```bash
# 仅读取产品
--scopes "products:read"

# 完全访问产品
--scopes "products:write"

# 产品管理访问
--scopes "products:admin"
```

### 多资源访问

```bash
# 读取产品，写入订单
--scopes "products:read,orders:write"

# 读取产品和客户，完全访问订单
--scopes "products:read,customers:read,orders:write"
```

### 通配符权限范围

```bash
# 读取所有资源
--scopes "read"

# 写入所有资源（包含读取）
--scopes "write"

# 管理所有资源
--scopes "admin"
```

### 混合权限范围

```bash
# 读取所有资源，但仅写入订单
--scopes "read,orders:write"

# 读取产品，完全访问其他所有内容
--scopes "products:read,write"
```

## 权限范围预设

R Commerce 为常见使用场景提供预定义的权限范围集：

### 只读访问

```bash
--scopes "read"
```

授予所有资源的读取访问权限。适用于：
- 报告工具
- 分析仪表板
- 数据同步（只读）
- 审计系统

### 读写访问

```bash
--scopes "read,write"
```

授予所有资源的读写访问权限。适用于：
- 全功能集成
- 管理仪表板
- 数据迁移工具

### 管理员访问

```bash
--scopes "admin"
```

授予所有资源的完全管理访问权限。适用于：
- 系统管理员
- 完整平台访问
- 紧急恢复工具

### 产品只读

```bash
--scopes "products:read"
```

仅授予产品的只读访问权限。适用于：
- 产品目录展示
- 产品搜索集成
- 价格比较工具

### 产品完全访问

```bash
--scopes "products:read,products:write"
```

授予产品的完全访问权限。适用于：
- 产品管理工具
- 库存管理系统
- PIM（产品信息管理）集成

### 订单只读

```bash
--scopes "orders:read"
```

仅授予订单的只读访问权限。适用于：
- 订单跟踪系统
- 报告工具
- 客户服务仪表板

### 订单完全访问

```bash
--scopes "orders:read,orders:write"
```

授予订单的完全访问权限。适用于：
- 订单管理系统
- 履约集成
- 客户服务工具

### 客户访问

```bash
--scopes "products:read,orders:read,orders:write,carts:read,carts:write,customers:read,customers:write,payments:read,payments:write"
```

授予面向客户操作所需的访问权限。适用于：
- 移动应用程序
- 客户门户
- 前端应用程序

### Webhook 处理器

```bash
--scopes "webhooks:write,orders:read,orders:write,payments:read,payments:write"
```

授予 Webhook 处理的访问权限。适用于：
- 支付网关 Webhook
- 第三方集成
- 事件处理系统

### 库存管理器

```bash
--scopes "inventory:read,inventory:write,products:read,orders:read"
```

授予库存管理的访问权限。适用于：
- 仓库管理系统
- 库存同步
- 库存水平监控

## 权限范围验证

创建或更新 API 密钥时，权限范围会被验证以确保：

1. **有效的资源名称** - 必须是可用资源之一
2. **有效的操作名称** - 必须是 `read`、`write` 或 `admin`
3. **有效的格式** - 必须遵循 `resource:action` 或是全局权限范围

### 无效权限范围示例

```bash
# 无效资源
--scopes "invalid_resource:read"
# 错误：未知资源: invalid_resource

# 无效操作
--scopes "products:execute"
# 错误：未知操作: execute

# 无效格式
--scopes "products-read"
# 错误：无效权限范围格式: products-read

# 部分过多
--scopes "products:read:extra"
# 错误：无效权限范围格式: products:read:extra
```

## 权限范围检查

API 会自动检查每个请求的权限范围：

### 示例：检查读取访问权限

```rust
// 检查 API 密钥是否可以读取产品
if api_key_auth.can_read(Resource::Products) {
    // 允许访问
}
```

### 示例：检查写入访问权限

```rust
// 检查 API 密钥是否可以写入订单
if api_key_auth.can_write(Resource::Orders) {
    // 允许创建/更新订单
}
```

### 示例：检查管理员访问权限

```rust
// 检查 API 密钥是否具有管理员访问权限
if api_key_auth.is_admin() {
    // 允许管理操作
}
```

## 权限范围最佳实践

### 1. 最小权限原则

仅授予必要的最小权限：

```bash
# 良好：特定访问
--scopes "products:read,orders:read"

# 避免：在不需要时授予过宽的访问权限
--scopes "write"
```

### 2. 为不同服务使用单独的密钥

为不同服务使用不同的 API 密钥：

```bash
# 产品同步服务的密钥
rcommerce api-key create --name "Product Sync" --scopes "products:read,products:write"

# 订单处理服务的密钥  
rcommerce api-key create --name "Order Processor" --scopes "orders:read,orders:write"

# 报告服务的密钥
rcommerce api-key create --name "Reporting" --scopes "read"
```

### 3. 环境特定密钥

为不同环境创建单独的密钥：

```bash
# 开发环境
rcommerce api-key create --name "Dev - Product Sync" --scopes "products:write"

# 预发布环境
rcommerce api-key create --name "Staging - Product Sync" --scopes "products:write"

# 生产环境
rcommerce api-key create --name "Prod - Product Sync" --scopes "products:read"
```

### 4. 定期权限范围审计

定期审查 API 密钥权限范围：

```bash
# 列出所有密钥及其权限范围
rcommerce api-key list

# 检查特定密钥详情
rcommerce api-key get <prefix>
```

### 5. 记录密钥用途

使用描述性名称并记录用途：

```bash
rcommerce api-key create \
  --name "Mobile App - Production" \
  --scopes "products:read,orders:read,orders:write,carts:read,carts:write"
# 文档说明: 用于 iOS 和 Android 应用的面向客户操作
```

## 常见权限范围模式

### 电商平台集成

```bash
--scopes "products:read,products:write,orders:read,orders:write,customers:read,customers:write,inventory:read,inventory:write"
```

### 支付网关集成

```bash
--scopes "orders:read,orders:write,payments:read,payments:write,webhooks:write"
```

### ERP 集成

```bash
--scopes "products:read,products:write,orders:read,orders:write,inventory:read,inventory:write,customers:read,customers:write"
```

### 营销自动化

```bash
--scopes "customers:read,orders:read,coupons:read,coupons:write,products:read"
```

### 分析和报告

```bash
--scopes "read,reports:read"
```

## 故障排除

### 403 禁止访问错误

如果您收到 403 错误，请检查您的 API 密钥是否具有所需的权限范围：

```bash
# 检查您的密钥权限范围
rcommerce api-key get <prefix>
```

与调用端点所需的权限范围进行比较。

### 权限范围变更

如果您需要不同的权限范围，必须创建新的 API 密钥：

```bash
# 撤销旧密钥
rcommerce api-key revoke <old_prefix> --reason "Replacing with scoped key"

# 创建具有正确权限范围的新密钥
rcommerce api-key create --name "Updated Key" --scopes "correct:scopes"
```

> **注意：** 出于安全原因，无法修改现有密钥的权限范围。

## 下一步

- [认证](authentication.zh.md) - 认证方法和使用
- [API 密钥指南](../guides/api-keys.md) - 管理 API 密钥
- [CLI 参考](../development/cli-reference.md) - CLI 命令参考
