# API 密钥指南

本指南涵盖在 R Commerce 中创建、管理和使用 API 密钥所需了解的一切内容。

## 概述

API 密钥为服务间通信提供安全、长期有效的认证方式。与为用户会话设计的 JWT 令牌不同，API 密钥适用于：

- 后端服务集成
- Webhook 处理器
- ETL 和数据同步流程
- 第三方集成
- 自动化脚本和工具

## 创建 API 密钥

### 基本创建

使用 CLI 创建 API 密钥：

```bash
rcommerce api-key create --name "My Integration"
```

**输出：**
```
✅ API Key created successfully!
Key: aB3dEfGh.sEcReTkEy123456789abcdef1234567
Prefix: aB3dEfGh
Scopes: read
Created: 2026-01-15T10:30:00Z
Expires: Never

⚠️  IMPORTANT: Store this key securely. It will not be shown again!
```

### 创建时指定范围

创建时指定范围：

```bash
rcommerce api-key create \
  --name "Product Sync Service" \
  --scopes "products:read,products:write"
```

### 创建时设置过期时间

设置过期日期以增强安全性：

```bash
rcommerce api-key create \
  --name "Temporary Integration" \
  --scopes "orders:read" \
  --expires "2026-12-31T23:59:59Z"
```

### 创建时设置速率限制

限制每分钟请求数：

```bash
rcommerce api-key create \
  --name "Limited Access Key" \
  --scopes "products:read" \
  --rate-limit 100
```

## 管理 API 密钥

### 列出所有密钥

查看系统中的所有 API 密钥：

```bash
rcommerce api-key list
```

**输出：**
```
┌──────────┬──────────────────────┬─────────────────┬─────────┬──────────────────────┐
│ Prefix   │ Name                 │ Scopes          │ Status  │ Last Used            │
├──────────┼──────────────────────┼─────────────────┼─────────┼──────────────────────┤
│ aB3dEfGh │ Product Sync Service │ products:write  │ Active  │ 2026-01-20T14:30:00Z │
│ Xy9ZaBcD │ Mobile App Backend   │ read            │ Active  │ 2026-01-20T15:45:00Z │
│ Mn7OpQrS │ Old Integration      │ write           │ Revoked │ 2026-01-10T09:00:00Z │
└──────────┴──────────────────────┴─────────────────┴─────────┴──────────────────────┘
```

### 查看密钥详情

获取特定密钥的详细信息：

```bash
rcommerce api-key get aB3dEfGh
```

**输出：**
```
API Key Details
===============
Prefix:           aB3dEfGh
Name:             Product Sync Service
Scopes:           products:read, products:write
Status:           Active
Created:          2026-01-15T10:30:00Z
Created By:       admin@example.com
Expires:          Never
Rate Limit:       1000 req/min
Last Used:        2026-01-20T14:30:00Z
Last Used IP:     203.0.113.42
Revoked:          -
Revoke Reason:    -
```

### 撤销 API 密钥

撤销密钥以立即禁用它：

```bash
rcommerce api-key revoke aB3dEfGh --reason "Key compromised"
```

**输出：**
```
✅ API Key revoked successfully
Prefix: aB3dEfGh
Revoked at: 2026-01-20T16:00:00Z
Reason: Key compromised
```

> **注意：** 已撤销的密钥无法重新激活。如需使用请创建新密钥。

### 删除 API 密钥

从系统中永久删除密钥：

```bash
rcommerce api-key delete aB3dEfGh
```

**确认：**
```
⚠️  WARNING: This action cannot be undone!
Are you sure you want to delete API key 'aB3dEfGh'? [y/N]: y
✅ API Key deleted successfully
```

> **注意：** 建议在删除前撤销密钥以保留审计记录。

## 使用 API 密钥

### 在 HTTP 请求中

在 Authorization 请求头中包含 API 密钥：

```bash
curl -X GET "https://api.rcommerce.app/api/v1/products" \
  -H "Authorization: Bearer aB3dEfGh.sEcReTkEy123456789abcdef1234567"
```

或不使用 Bearer 前缀：

```bash
curl -X GET "https://api.rcommerce.app/api/v1/products" \
  -H "Authorization: aB3dEfGh.sEcReTkEy123456789abcdef1234567"
```

### 在 JavaScript/TypeScript 中

```javascript
const API_KEY = process.env.RCOMMERCE_API_KEY;

async function getProducts() {
  const response = await fetch('https://api.rcommerce.app/api/v1/products', {
    headers: {
      'Authorization': `Bearer ${API_KEY}`,
      'Content-Type': 'application/json'
    }
  });
  
  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
  
  return await response.json();
}
```

### 在 Python 中

```python
import os
import requests

API_KEY = os.environ.get('RCOMMERCE_API_KEY')
BASE_URL = 'https://api.rcommerce.app'

headers = {
    'Authorization': f'Bearer {API_KEY}',
    'Content-Type': 'application/json'
}

# 获取产品
response = requests.get(f'{BASE_URL}/api/v1/products', headers=headers)
products = response.json()

# 创建订单
order_data = {
    'customer_id': '123e4567-e89b-12d3-a456-426614174000',
    'items': [
        {'product_id': '123e4567-e89b-12d3-a456-426614174001', 'quantity': 2}
    ]
}
response = requests.post(f'{BASE_URL}/api/v1/orders', json=order_data, headers=headers)
```

### 在 Go 中

```go
package main

import (
    "net/http"
    "os"
)

func main() {
    apiKey := os.Getenv("RCOMMERCE_API_KEY")
    
    req, err := http.NewRequest("GET", "https://api.rcommerce.app/api/v1/products", nil)
    if err != nil {
        panic(err)
    }
    
    req.Header.Set("Authorization", "Bearer "+apiKey)
    
    client := &http.Client{}
    resp, err := client.Do(req)
    if err != nil {
        panic(err)
    }
    defer resp.Body.Close()
    
    // 处理响应...
}
```

### 在 PHP 中

```php
<?php
$apiKey = getenv('RCOMMERCE_API_KEY');

$ch = curl_init('https://api.rcommerce.app/api/v1/products');
curl_setopt($ch, CURLOPT_HTTPHEADER, [
    'Authorization: Bearer ' . $apiKey,
    'Content-Type: application/json'
]);
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

$response = curl_exec($ch);
curl_close($ch);

$products = json_decode($response, true);
?>
```

## 安全最佳实践

### 1. 安全存储密钥

**使用环境变量：**

```bash
# .env 文件
RCOMMERCE_API_KEY=aB3dEfGh.sEcReTkEy123456789abcdef1234567
```

```python
# Python
import os
api_key = os.environ.get('RCOMMERCE_API_KEY')
```

```javascript
// Node.js
const apiKey = process.env.RCOMMERCE_API_KEY;
```

**切勿：**
- ❌ 在源代码中硬编码密钥
- ❌ 将密钥提交到版本控制
- ❌ 在日志或错误消息中记录密钥
- ❌ 通过电子邮件或聊天分享密钥

### 2. 使用最小范围

仅授予必要的权限：

```bash
# 良好：特定访问
rcommerce api-key create --name "Product Viewer" --scopes "products:read"

# 避免：不需要时授予过宽访问权限
rcommerce api-key create --name "Product Viewer" --scopes "read"
```

### 3. 定期轮换密钥

建立密钥轮换计划：

```bash
# 1. 创建新密钥
rcommerce api-key create --name "Product Sync - Q1 2026" --scopes "products:write"

# 2. 在应用中更新为新密钥

# 3. 测试新密钥

# 4. 撤销旧密钥
rcommerce api-key revoke <old_prefix> --reason "Scheduled rotation"
```

**建议轮换计划：**
- 生产环境密钥：每 90 天
- 预发布环境密钥：每 180 天
- 开发环境密钥：按需

### 4. 监控密钥使用

定期检查密钥活动：

```bash
# 列出带有最后使用信息的密钥
rcommerce api-key list

# 获取特定密钥的详细使用情况
rcommerce api-key get <prefix>
```

**注意：**
- 长时间未使用的密钥（考虑撤销的候选）
- `last_used_ip` 中的意外 IP 地址
- 使用量的突然激增

### 5. 为不同环境使用单独的密钥

```bash
# 开发环境
rcommerce api-key create --name "Dev - Product Sync" --scopes "products:write"

# 预发布环境
rcommerce api-key create --name "Staging - Product Sync" --scopes "products:write"

# 生产环境
rcommerce api-key create --name "Prod - Product Sync" --scopes "products:read"
```

### 6. 设置适当的速率限制

保护您的系统免受意外滥用：

```bash
# 高流量服务
rcommerce api-key create --name "Data Sync" --scopes "write" --rate-limit 10000

# 低流量 Webhook
rcommerce api-key create --name "Webhook Handler" --scopes "orders:write" --rate-limit 100

# 偶尔运行的脚本
rcommerce api-key create --name "Daily Report" --scopes "read" --rate-limit 60
```

### 7. 立即撤销泄露的密钥

如果密钥泄露：

```bash
# 1. 立即撤销
rcommerce api-key revoke <prefix> --reason "Compromised - accidentally committed to GitHub"

# 2. 创建新密钥
rcommerce api-key create --name "Replacement Key" --scopes "<same_scopes>"

# 3. 更新您的应用

# 4. 审查访问日志中的未授权使用
```

## 用例示例

### 示例 1：移动应用的只读 API 密钥

**场景：** 移动应用需要展示产品但不应修改数据。

```bash
# 创建密钥
rcommerce api-key create \
  --name "Mobile App - Production" \
  --scopes "products:read" \
  --rate-limit 1000
```

**在移动应用中使用：**
```javascript
// React Native 示例
const API_KEY = Config.RCOMMERCE_API_KEY; // 来自环境变量

async function fetchProducts() {
  const response = await fetch(`${API_URL}/api/v1/products`, {
    headers: {
      'Authorization': `Bearer ${API_KEY}`
    }
  });
  return await response.json();
}
```

**安全考虑：**
- 移动应用无法安全存储 API 密钥
- 考虑为生产环境移动应用使用后端代理或 JWT 认证
- 此示例适用于内部/企业应用

### 示例 2：Webhook 处理器 API 密钥

**场景：** 支付网关发送需要更新订单的 Webhooks。

```bash
# 创建具有最小所需范围的密钥
rcommerce api-key create \
  --name "Payment Gateway Webhook" \
  --scopes "orders:read,orders:write,payments:read,payments:write,webhooks:write" \
  --rate-limit 500
```

**Webhook 处理器实现：**
```python
from flask import Flask, request
import os

app = Flask(__name__)
API_KEY = os.environ.get('RCOMMERCE_API_KEY')

def verify_webhook_signature(payload, signature):
    # 验证来自支付网关的 webhook 签名
    pass

@app.route('/webhooks/payment', methods=['POST'])
def handle_payment_webhook():
    # 验证 webhook 真实性
    signature = request.headers.get('X-Webhook-Signature')
    if not verify_webhook_signature(request.data, signature):
        return 'Invalid signature', 401
    
    # 处理 webhook
    event = request.json
    
    if event['type'] == 'payment.success':
        # 使用 R Commerce API 更新订单
        order_id = event['data']['order_id']
        update_data = {
            'payment_status': 'paid',
            'transaction_id': event['data']['transaction_id']
        }
        
        response = requests.patch(
            f'https://api.rcommerce.app/api/v1/orders/{order_id}',
            json=update_data,
            headers={'Authorization': f'Bearer {API_KEY}'}
        )
        
        if response.ok:
            return 'OK', 200
        return 'Failed to update order', 500
    
    return 'Event ignored', 200
```

### 示例 3：库存管理集成

**场景：** 仓库管理系统需要同步库存水平。

```bash
# 创建密钥
rcommerce api-key create \
  --name "WMS Integration" \
  --scopes "inventory:read,inventory:write,products:read,orders:read" \
  --rate-limit 5000
```

**同步脚本：**
```python
import requests
import os
from datetime import datetime

API_KEY = os.environ.get('RCOMMERCE_API_KEY')
BASE_URL = 'https://api.rcommerce.app'
HEADERS = {'Authorization': f'Bearer {API_KEY}'}

def sync_inventory():
    # 从 R Commerce 获取所有产品
    response = requests.get(f'{BASE_URL}/api/v1/products', headers=HEADERS)
    products = response.json()
    
    for product in products:
        # 从 WMS 获取当前库存
        wms_stock = get_wms_stock(product['sku'])
        
        # 更新 R Commerce 库存
        if wms_stock != product['inventory_quantity']:
            requests.put(
                f'{BASE_URL}/api/v1/inventory/{product["id"]}',
                json={'quantity': wms_stock},
                headers=HEADERS
            )
            print(f"Updated {product['sku']}: {product['inventory_quantity']} -> {wms_stock}")

def get_wms_stock(sku):
    # 查询您的 WMS 获取当前库存水平
    pass

if __name__ == '__main__':
    print(f"Starting inventory sync at {datetime.now()}")
    sync_inventory()
    print("Sync complete")
```

### 示例 4：多服务架构

**场景：** 多个微服务需要不同级别的访问权限。

```bash
# 产品服务 - 管理产品目录
rcommerce api-key create \
  --name "Service: Product Manager" \
  --scopes "products:write,inventory:read" \
  --rate-limit 2000

# 订单服务 - 处理订单
rcommerce api-key create \
  --name "Service: Order Processor" \
  --scopes "orders:write,customers:read,products:read,payments:write" \
  --rate-limit 5000

# 通知服务 - 发送邮件
rcommerce api-key create \
  --name "Service: Notifications" \
  --scopes "orders:read,customers:read" \
  --rate-limit 1000

# 分析服务 - 生成报告
rcommerce api-key create \
  --name "Service: Analytics" \
  --scopes "read,reports:write" \
  --rate-limit 500
```

## 故障排除

### 401 未授权

**原因：** API 密钥无效或缺失

**解决方案：**
1. 验证 API 密钥是否包含在 Authorization 请求头中
2. 检查密钥是否已被撤销：
   ```bash
   rcommerce api-key get <prefix>
   ```
3. 确保密钥未过期

### 403 禁止访问

**原因：** 请求的操作权限不足

**解决方案：**
1. 检查您的密钥范围：
   ```bash
   rcommerce api-key get <prefix>
   ```
2. 验证端点所需的范围
3. 如需创建具有适当范围的新密钥

### 429 请求过多

**原因：** 超出速率限制

**解决方案：**
1. 检查您的密钥速率限制：
   ```bash
   rcommerce api-key get <prefix>
   ```
2. 在客户端实现指数退避
3. 如需请求更高的速率限制：
   ```bash
   # 创建具有更高限制的新密钥
   rcommerce api-key create --name "High Volume Key" --scopes "write" --rate-limit 10000
   ```

### 创建后密钥无法使用

**原因：** 可能存在延迟或密钥复制不正确

**解决方案：**
1. 验证您复制了完整的密钥，包括前缀和点号
2. 检查多余的空白字符
3. 确保您使用的是正确的环境（开发/预发布/生产）

## 配置选项

### 在 config.toml 中

```toml
[security]
# 密钥前缀长度（默认：8）
api_key_prefix_length = 8

# 密钥密钥部分长度（默认：32）
api_key_secret_length = 32

# 新密钥的默认速率限制（可选）
api_key_default_rate_limit = 1000
```

## 下一步

- [范围参考](../api-reference/scopes.md) - 完整的范围文档
- [认证](../api-reference/authentication.md) - 认证方法
- [CLI 参考](../development/cli-reference.md) - 所有 CLI 命令
- [错误代码](../api-reference/errors.md) - 错误处理参考
