# 迁移指南

从主流电商平台迁移到 R Commerce 的综合指南。

## 可用的迁移指南

### [Shopify](./shopify.md)
从 Shopify 迁移，包括：
- 产品和变体迁移
- 客户和订单历史
- 主题和前端过渡
- SEO 保留
- 应用生态系统替代方案

### [WooCommerce](./woocommerce.md)
从 WooCommerce 迁移，包括：
- WordPress 集成移除
- 产品和属性迁移
- 客户和订单迁移
- 插件生态系统映射
- 支付网关过渡

### [Magento](./magento.md)
从 Magento 迁移，包括：
- 复杂产品结构迁移
- 客户组映射
- 订单和发票迁移
- 扩展生态系统替代方案
- B2B 功能迁移

### [Medusa](./medusa.md)
从 Medusa.js 迁移，包括：
- API 兼容性考虑
- 直接迁移路径
- 功能对比
- 迁移脚本示例

## 迁移策略

### 大爆炸式迁移
在计划停机期间一次性迁移所有数据。

**优点：**
- 执行简单
- 切换干净
- 复杂度较低

**缺点：**
- 需要停机时间
- 风险较高
- 回滚可能困难

**最适合：** 小型商店、淡季迁移、新上线项目

### 分阶段迁移
随时间分阶段迁移，实现零停机或最小停机。

**优点：**
- 零停机
- 风险较低
- 易于回滚
- 可测试每个阶段

**缺点：**
- 更复杂
- 时间线更长
- 需要同时运行双系统

**最适合：** 大型商店、高流量商店、复杂集成

## 导入工具

R Commerce 包含一个内置导入工具，用于从主流平台迁移：

### 快速开始

```bash
# 从 Shopify 导入
rcommerce import platform shopify \
  -c config.toml \
  --api-url https://your-store.myshopify.com \
  --api-key YOUR_API_KEY \
  --api-secret YOUR_API_PASSWORD

# 从 WooCommerce 导入
rcommerce import platform woocommerce \
  -c config.toml \
  --api-url https://your-store.com \
  --api-key YOUR_CONSUMER_KEY \
  --api-secret YOUR_CONSUMER_SECRET
```

### 支持的平台

| 平台 | 状态 | 实体 |
|----------|--------|----------|
| Shopify | ✅ 完整 | 产品、客户、订单 |
| WooCommerce | ✅ 完整 | 产品、客户、订单 |
| Magento | 🚧 计划中 | 产品、客户、订单 |
| Medusa | 🚧 计划中 | 产品、客户、订单 |

### 文件导入

从导出文件导入：

```bash
# CSV 导入
rcommerce import file -c config.toml --file products.csv --format csv --entity products

# JSON 导入
rcommerce import file -c config.toml --file customers.json --format json --entity customers
```

### 试运行模式

始终先使用 `--dry-run` 进行验证：

```bash
rcommerce import platform shopify ... --dry-run
```

这会验证所有数据而不修改您的数据库。

有关完整文档，请参阅 [CLI 参考](../development/cli-reference.md#import)。

## 迁移检查清单

### 迁移前
- [ ] 审计当前平台数据
- [ ] 清理未使用的产品/客户/订单
- [ ] 从当前平台导出所有数据
- [ ] 设置 R Commerce 环境
- [ ] 选择迁移策略
- [ ] 创建迁移时间表
- [ ] 准备回滚计划
- [ ] 使用 `--dry-run` 测试导入

### 迁移执行
- [ ] 导入产品和分类
- [ ] 导入客户
- [ ] 导入订单（可选，用于报表）
- [ ] 设置支付网关
- [ ] 配置配送
- [ ] 设置税务规则
- [ ] 测试结账流程
- [ ] 测试订单管理
- [ ] 测试客户账户

### 迁移后
- [ ] 验证数据完整性
- [ ] 测试所有集成
- [ ] 更新 DNS/指向域名
- [ ] 监控性能
- [ ] 测试备份/恢复
- [ ] 培训员工使用新系统
- [ ] 更新文档
- [ ] 归档旧平台数据

## 数据映射考虑事项

### 产品复杂度
不同平台处理产品的方式不同：

| 平台 | 产品模型 |
|----------|---------------|
| Shopify | 带变体的简单产品 |
| WooCommerce | 带属性的可变产品 |
| Magento | 复杂类型（可配置、捆绑、组合） |
| R Commerce | 带变体和属性的统一模型 |

### 客户账户
- **密码**：通常无法迁移（需要重置）
- **分组**：映射到 R Commerce 客户组
- **忠诚度**：可能需要自定义集成
- **订阅**：需要特殊处理

### 订单历史
- 订单编号方案可能不同
- 需要状态映射
- 部分退款/换货
- 多币种订单

### SEO 保留
对维持搜索排名至关重要：
- URL 重定向（必需！）
- 元数据迁移
- 站点地图结构
- 搜索引擎重新索引

## 常见陷阱

### 1. 低估时间
- 大型目录需要数天，而非数小时
- 订单历史迁移很慢
- 测试阶段通常比预期时间长

### 2. 数据丢失风险
- 迁移前始终备份
- 迁移后测试完整性
- 确认前保留原始数据

### 3. SEO 影响
- 缺少重定向 = 排名丢失
- 内容变更 = 重新索引时间
- 计划应对临时流量下降

### 4. 集成中断
- 支付 Webhook 需要更新
- 配送集成可能中断
- CRM/邮件营销连接
- 会计系统集成

### 5. 性能问题
- 新平台可能需要调优
- 不同的缓存策略
- 需要数据库优化

## 测试检查清单

- [ ] 产品目录完整（数量匹配）
- [ ] 产品详情准确（价格、图片、描述）
- [ ] 客户数据已迁移（邮箱、地址）
- [ ] 订单历史可访问
- [ ] 支付处理正常
- [ ] 配送计算准确
- [ ] 税务规则正确应用
- [ ] 邮件通知已发送
- [ ] Webhook 已接收
- [ ] 移动端响应式
- [ ] 管理功能正常
- [ ] 报表生成正确
- [ ] 备份和恢复已测试
- [ ] 性能可接受
- [ ] SEO 重定向正常

## 支持

如需迁移协助：

1. **文档**：查看平台特定指南
2. **社区**：[GitHub Discussions](https://github.com/creativebastard/rcommerce/discussions)
3. **专业服务**：联系 sales@rcommerce.app 获取企业迁移支持

## 另请参阅

- [架构概述](../architecture/overview.md)
- [部署指南](../deployment/index.md)
- [CLI 参考](../development/cli-reference.md)
