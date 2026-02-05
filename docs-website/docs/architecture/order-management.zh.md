# 订单管理

## 概述

R Commerce 中的订单管理系统处理客户购买的完整生命周期，从购物车到配送完成。

## 订单生命周期

```
┌─────────┐    ┌──────────┐    ┌───────────┐    ┌──────────┐    ┌───────────┐
│ Pending │───▶│Confirmed │───▶│Processing │───▶│ Shipped  │───▶│Completed  │
└─────────┘    └──────────┘    └───────────┘    └──────────┘    └───────────┘
     │                                                    │
     │              ┌──────────┐                          │
     └─────────────▶│ Cancelled│◀─────────────────────────┘
                    └──────────┘
```

### 状态定义

| 状态 | 描述 |
|------|------|
| **Pending** | 订单已创建，等待支付确认 |
| **Confirmed** | 已收到付款，准备处理 |
| **Processing** | 订单正在准备发货 |
| **On Hold** | 需要人工审核 |
| **Shipped** | 订单已发货，运输中 |
| **Completed** | 订单已送达，已完成 |
| **Cancelled** | 订单已取消（发货前）|
| **Refunded** | 订单已退款（取消或退货后）|

## 订单结构

### 订单头
包含摘要信息：
- 客户详情
- 账单/配送地址
- 财务总计
- 状态跟踪

### 行项目
订单中的单个产品：
- 产品引用
- 变体（尺寸、颜色等）
- 数量
- 定价（单价、折扣、税费）

### 支付
支付交易：
- 使用的网关
- 金额和货币
- 状态（pending、authorized、paid、failed、refunded）
- 交易 ID

### 配送
配送记录：
- 承运商和服务
- 跟踪号
- 已发货商品
- 状态更新

## 订单处理流程

### 1. 订单创建
```rust
// 从购物车创建订单
let order = order_service
    .create_from_cart(cart_id, customer_id)
    .await?;
```

### 2. 支付处理
```rust
// 处理支付
let payment = payment_service
    .process(order.id, payment_method)
    .await?;

// 更新订单状态
order_service.update_status(order.id, OrderStatus::Confirmed).await?;
```

### 3. 库存预留
- 订单确认时预留库存
- 订单取消时释放预留
- 配送完成时调整库存

### 4. 配送
```rust
// 创建配送
let fulfillment = fulfillment_service
    .create(order.id, items_to_ship)
    .await?;

// 生成运单
let label = shipping_service
    .create_label(fulfillment.id, carrier)
    .await?;
```

## 订单备注

员工可以向订单添加备注：
- 内部备注（仅员工可见）
- 客户可见备注
- 系统生成备注（状态变更等）

## 订单编辑

订单在发货前可以编辑：
- 添加/删除商品
- 更改数量
- 更新地址
- 应用折扣

## 欺诈检测

基础欺诈评分：
- 订单价值阈值
- 速度检查
- 地址验证
- 风险评分集成

## Webhook 事件

订单事件触发 Webhooks：
- `order.created`
- `order.paid`
- `order.shipped`
- `order.completed`
- `order.cancelled`

## 另请参阅

- [数据模型](./data-model.zh.md) - 订单实体关系
- [数据库抽象](./database-abstraction.zh.md) - 数据访问模式
