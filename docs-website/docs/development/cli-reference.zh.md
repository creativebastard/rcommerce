# CLI 参考

R Commerce CLI (`rcommerce`) 提供用于服务器管理、数据库操作、API 密钥管理以及交互式产品/客户创建的命令。

## 全局选项

```bash
rcommerce [OPTIONS] <COMMAND>

选项：
  -c, --config <CONFIG>        配置文件路径
  -l, --log-level <LOG_LEVEL>  设置日志级别（debug、info、warn、error）
  -h, --help                   打印帮助信息
  -V, --version                打印版本信息
```

## 命令

### 服务器

启动 API 服务器：

```bash
rcommerce server [OPTIONS]

选项：
  -H, --host <HOST>      绑定地址 [默认值：0.0.0.0]
  -P, --port <PORT>      端口号 [默认值：8080]
      --skip-migrate     跳过自动数据库迁移
```

**示例：**

```bash
# 使用默认配置启动
rcommerce server

# 在自定义端口启动
rcommerce server -P 3000

# 不执行迁移启动
rcommerce server --skip-migrate
```

### 数据库

数据库管理命令：

```bash
rcommerce db <COMMAND>

命令：
  migrate    运行数据库迁移
  reset      重置数据库（危险 - 删除所有数据）
  seed       使用演示数据填充数据库
  status     显示数据库状态
```

**示例：**

```bash
# 运行迁移
rcommerce db migrate -c config.toml

# 检查数据库状态
rcommerce db status -c config.toml

# 重置数据库（带确认）
rcommerce db reset -c config.toml

# 填充演示数据
rcommerce db seed -c config.toml
```

### API 密钥管理

管理服务间认证的 API 密钥：

```bash
rcommerce api-key <COMMAND>

命令：
  list       列出所有 API 密钥
  create     创建新的 API 密钥
  get        获取 API 密钥详情
  revoke     吊销 API 密钥
  delete     永久删除 API 密钥
```

#### 列出 API 密钥

```bash
rcommerce api-key list [OPTIONS]

选项：
  -u, --customer-id <ID>  按客户 ID 筛选
```

**示例：**

```bash
rcommerce api-key list -c config.toml
```

输出：
```
API 密钥
前缀         名称                 权限范围                       激活状态   过期时间
------------------------------------------------------------------------------------------
aB3dEfGh     Production Backend   read, write                    ✓          永不过期
Xy9zZzZz     Test Key             read                           ✗          2024-12-31
```

#### 创建 API 密钥

```bash
rcommerce api-key create [OPTIONS]

选项：
  -u, --customer-id <ID>     客户 ID（系统密钥可选）
  -n, --name <NAME>          密钥名称/描述
  -s, --scopes <SCOPES>      权限范围（逗号分隔）[默认值：read]
  -e, --expires-days <DAYS>  过期天数（可选）
```

**示例：**

```bash
rcommerce api-key create \
  -c config.toml \
  --name "Production Backend" \
  --scopes "read,write"
```

输出：
```
✅ API 密钥创建成功！

重要：立即复制此密钥 - 它只会显示一次！

  密钥：aB3dEfGh.sEcReTkEy123456789

  前缀：      aB3dEfGh
  名称：      Production Backend
  权限范围：  read, write
  客户 ID：   System
  过期时间：  永不过期
```

#### 获取 API 密钥详情

```bash
rcommerce api-key get <PREFIX>
```

**示例：**

```bash
rcommerce api-key get -c config.toml aB3dEfGh
```

输出：
```
API 密钥详情
  前缀：       aB3dEfGh
  名称：       Production Backend
  权限范围：   read, write
  激活状态：   ✓ 是
  客户 ID：    System
  创建时间：   2024-01-31 10:30:00 UTC
  更新时间：   2024-01-31 10:30:00 UTC
  过期时间：   永不过期
  最后使用：   从未使用
```

#### 吊销 API 密钥

```bash
rcommerce api-key revoke [OPTIONS] <PREFIX>

选项：
  -r, --reason <REASON>  吊销原因
```

**示例：**

```bash
rcommerce api-key revoke \
  -c config.toml \
  aB3dEfGh \
  --reason "密钥泄露"
```

#### 删除 API 密钥

永久删除 API 密钥（不可恢复）：

```bash
rcommerce api-key delete [OPTIONS] <PREFIX>

选项：
      --force  跳过确认
```

**示例：**

```bash
# 带确认提示
rcommerce api-key delete -c config.toml aB3dEfGh

# 跳过确认
rcommerce api-key delete -c config.toml aB3dEfGh --force
```

### 产品管理

```bash
rcommerce product <COMMAND>

命令：
  list       列出产品
  create     创建产品（交互式）
  get        获取产品详情
  update     更新产品
  delete     删除产品
```

#### 列出产品

```bash
rcommerce product list -c config.toml
```

输出：
```
产品
ID                                    标题                          价格       货币       状态
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  Premium T-Shirt               29.99      USD        ✓ 激活
550e8400-e29b-41d4-a716-446655440001  Wireless Headphones           149.99     USD        ✓ 激活

总计：2 个产品
```

#### 创建产品（交互式）

```bash
rcommerce product create -c config.toml
```

此命令启动交互式提示，引导您完成产品创建：

```
📦 创建新产品
随时按 Ctrl+C 取消。

产品标题：Premium T-Shirt
URL 别名 [premium-t-shirt]: premium-t-shirt
产品类型：
  > 简单产品
    可变产品
    数字产品
    捆绑产品
价格：29.99
货币：
  > USD
    EUR
    GBP
    JPY
    AUD
    CAD
    CNY
    HKD
    SGD
SKU（可选）：TSHIRT-001
库存数量 [0]: 100
描述（可选）：High quality cotton t-shirt
激活产品？[Y/n]: y
标记为精选？[y/N]: n

📋 产品摘要
  标题：       Premium T-Shirt
  别名：       premium-t-shirt
  类型：       简单产品
  价格：       29.99 USD
  SKU：        TSHIRT-001
  库存：       100
  描述：       High quality cotton t-shirt
  激活：       是
  精选：       否

创建此产品？[Y/n]: y

✅ 产品创建成功！
  ID：    550e8400-e29b-41d4-a716-446655440000
  标题：  Premium T-Shirt
  别名：  premium-t-shirt
  价格：  29.99 USD
```

**交互式提示包括：**
- 产品标题（必填，最多 255 个字符）
- URL 别名（从标题自动生成，可编辑）
- 产品类型选择（简单产品/可变产品/数字产品/捆绑产品）
- 价格（数字验证）
- 货币选择（USD/EUR/GBP/JPY/AUD/CAD/CNY/HKD/SGD）
- SKU（可选，最多 100 个字符）
- 库存数量（默认值：0）
- 描述（可选）
- 激活状态（默认值：是）
- 精选状态（默认值：否）

#### 获取产品详情

```bash
rcommerce product get -c config.toml <product-id>
```

**示例：**

```bash
rcommerce product get -c config.toml 550e8400-e29b-41d4-a716-446655440000
```

输出：
```
产品详情
  ID：          550e8400-e29b-41d4-a716-446655440000
  标题：        Premium T-Shirt
  别名：        premium-t-shirt
  价格：        29.99 USD
  状态：        ✓ 激活
  库存：        100
  创建时间：    2024-01-31 10:30:00 UTC
  描述：        High quality cotton t-shirt
```

#### 删除产品

```bash
rcommerce product delete -c config.toml <product-id>
```

**示例：**

```bash
rcommerce product delete -c config.toml 550e8400-e29b-41d4-a716-446655440000
```

这将提示确认：
```
⚠️  删除产品
输入 'yes' 删除产品 '550e8400-e29b-41d4-a716-446655440000'：yes
✅ 产品 '550e8400-e29b-41d4-a716-446655440000' 已删除
```

### 订单管理

```bash
rcommerce order <COMMAND>

命令：
  list       列出订单
  get        获取订单详情
  create     创建测试订单
  update     更新订单状态
```

#### 列出订单

```bash
rcommerce order list -c config.toml
```

输出：
```
订单
ID                                    客户                 状态         总金额          创建时间
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  john@example.com     pending      149.99          2024-01-31
550e8400-e29b-41d4-a716-446655440001  jane@example.com     completed    299.98          2024-01-30

总计：2 个订单
```

### 客户管理

```bash
rcommerce customer <COMMAND>

命令：
  list       列出客户
  get        获取客户详情
  create     创建客户（交互式）
```

#### 列出客户

```bash
rcommerce customer list -c config.toml
```

输出：
```
客户
ID                                    邮箱                          姓名                 创建时间
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  john@example.com              John Doe             2024-01-31
550e8400-e29b-41d4-a716-446655440001  jane@example.com              Jane Smith           2024-01-30

总计：2 个客户
```

#### 创建客户（交互式）

```bash
rcommerce customer create -c config.toml
```

此命令启动交互式提示，引导您完成客户创建：

```
👤 创建新客户
随时按 Ctrl+C 取消。

邮箱地址：john@example.com
名字：John
姓氏：Doe
电话号码（可选）：+1234567890
首选货币：
  > USD
    EUR
    GBP
    JPY
    AUD
    CAD
    CNY
    HKD
    SGD
接收营销邮件？[y/N]: n
密码：********
确认密码：********

📋 客户摘要
  姓名：              John Doe
  邮箱：              john@example.com
  电话：              +1234567890
  货币：              USD
  接收营销：          否

创建此客户？[Y/n]: y

✅ 客户创建成功！
  ID：    550e8400-e29b-41d4-a716-446655440000
  姓名：  John Doe
  邮箱：  john@example.com
```

**交互式提示包括：**
- 邮箱地址（必填，已验证）
- 名字（必填，最多 100 个字符）
- 姓氏（必填，最多 100 个字符）
- 电话号码（可选）
- 首选货币选择
- 营销同意（默认值：否）
- 密码（最少 8 个字符，需确认）

#### 获取客户详情

```bash
rcommerce customer get -c config.toml <customer-id>
```

### 配置

显示加载的配置：

```bash
rcommerce config -c config.toml
```

### 导入

从外部平台或文件导入数据：

```bash
rcommerce import <COMMAND>

命令：
  platform   从电商平台导入（Shopify、WooCommerce 等）
  file       从文件导入（CSV、JSON、XML）
```

#### 从平台导入

直接从支持的电商平台导入数据：

```bash
rcommerce import platform <PLATFORM> [OPTIONS]

参数：
  <PLATFORM>    平台类型：shopify、woocommerce、magento、medusa

选项：
  -u, --api-url <URL>          API 端点 URL
  -k, --api-key <KEY>          API 密钥或访问令牌
      --api-secret <SECRET>    API 密钥（如需要）
  -e, --entities <ENTITIES>    逗号分隔列表：products,customers,orders [默认值：all]
      --limit <LIMIT>          每个实体最大导入记录数
      --dry-run                验证数据但不导入
```

**支持的平台：**

| 平台 | 状态 | 认证方式 | 实体 |
|------|------|----------|------|
| Shopify | ✅ 完整 | API 密钥 + 密码 | 产品、客户、订单 |
| WooCommerce | ✅ 完整 | 消费者密钥 + 密钥 | 产品、客户、订单 |
| Magento | 🚧 计划中 | OAuth/API 令牌 | 产品、客户、订单 |
| Medusa | 🚧 计划中 | API 令牌 | 产品、客户、订单 |

**示例：**

```bash
# 从 Shopify 导入所有数据
rcommerce import platform shopify \
  -c config.toml \
  --api-url https://your-store.myshopify.com \
  --api-key YOUR_API_KEY \
  --api-secret YOUR_API_PASSWORD

# 仅导入产品和客户（试运行）
rcommerce import platform shopify \
  -c config.toml \
  --api-url https://your-store.myshopify.com \
  --api-key YOUR_API_KEY \
  --api-secret YOUR_API_PASSWORD \
  --entities products,customers \
  --dry-run

# 从 WooCommerce 导入并限制数量
rcommerce import platform woocommerce \
  -c config.toml \
  --api-url https://your-store.com/wp-json/wc/v3 \
  --api-key YOUR_CONSUMER_KEY \
  --api-secret YOUR_CONSUMER_SECRET \
  --limit 100
```

**试运行模式：**

使用 `--dry-run` 验证数据而不实际导入：

```bash
rcommerce import platform shopify ... --dry-run
```

输出：
```
🧪 试运行模式 - 不会导入任何数据
从 Shopify 获取产品（试运行）...
验证：Premium T-Shirt
验证：Wireless Headphones
...

导入摘要（试运行）
========================
实体：products
  总计：     150
  创建：     150
  更新：     0
  跳过：     0
  错误：     0

✅ 验证完成。运行时不加 --dry-run 参数即可导入。
```

#### 从文件导入

从 CSV、JSON 或 XML 文件导入数据：

```bash
rcommerce import file [OPTIONS] --file <PATH> --format <FORMAT> --entity <ENTITY>

选项：
  -f, --file <PATH>        导入文件路径
  -F, --format <FORMAT>    文件格式：csv、json、xml
  -e, --entity <ENTITY>    实体类型：products、customers、orders
  -l, --limit <LIMIT>      最大导入记录数
      --dry-run            验证数据但不导入
```

**文件格式支持：**

| 格式 | 状态 | 描述 |
|------|------|------|
| CSV | ✅ 完整 | 带标题的逗号分隔值 |
| JSON | ✅ 完整 | JSON 对象数组 |
| XML | 🚧 计划中 | XML 文档格式 |

**CSV 格式：**

每种实体类型预期的列：

**产品：**
```csv
id,title,slug,description,price,compare_at_price,sku,inventory_quantity,status,product_type
TSHIRT-001,Premium T-Shirt,premium-t-shirt,High quality cotton,29.99,39.99,TSHIRT-001,100,active,physical
```

**客户：**
```csv
id,email,first_name,last_name,phone,address1,city,state,postal_code,country
cust-001,john@example.com,John,Doe,+1234567890,123 Main St,New York,NY,10001,US
```

**订单：**
```csv
id,order_number,customer_id,email,status,total,subtotal,tax_total,shipping_total
ORD-001,1001,cust-001,john@example.com,confirmed,59.98,54.99,4.99,0.00
```

**JSON 格式：**

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Premium T-Shirt",
    "slug": "premium-t-shirt",
    "description": "High quality cotton t-shirt",
    "price": "29.99",
    "sku": "TSHIRT-001",
    "inventory_quantity": 100,
    "status": "active"
  }
]
```

**示例：**

```bash
# 从 CSV 导入产品
rcommerce import file \
  -c config.toml \
  --file products.csv \
  --format csv \
  --entity products

# 从 JSON 导入客户（试运行）
rcommerce import file \
  -c config.toml \
  --file customers.json \
  --format json \
  --entity customers \
  --dry-run

# 带限制导入
rcommerce import file \
  -c config.toml \
  --file orders.csv \
  --format csv \
  --entity orders \
  --limit 50
```

#### 导入配置

导入设置也可以在 `config.toml` 中配置：

```toml
[import]
# 导入的默认批处理大小
batch_size = 100

# 出错时继续（跳过失败的记录）
continue_on_error = true

# 跳过现有记录（基于唯一标识符）
skip_existing = true

[import.shopify]
api_version = "2024-01"
# 商店特定设置

[import.woocommerce]
verify_ssl = true
```

### 环境变量

CLI 尊重以下环境变量：

| 变量 | 描述 |
|------|------|
| `RCOMMERCE_CONFIG` | 默认配置文件路径 |
| `RUST_LOG` | 日志级别（debug、info、warn、error） |

## 退出码

| 代码 | 含义 |
|------|------|
| 0 | 成功 |
| 1 | 一般错误 |
| 2 | 无效参数 |
| 3 | 数据库错误 |
| 4 | 配置错误 |

## 安全功能

CLI 包含多项安全功能：

### 防止 root 用户运行

出于安全原因，CLI 将拒绝以 root 用户运行：

```
❌ 错误：不允许以 root 运行！
   rcommerce CLI 不应以 root 身份运行。
   请以非特权用户运行。
```

### 配置文件权限

如果您的配置文件权限过于宽松，CLI 会发出警告：

```
⚠️  警告：配置文件可被全局读取
   路径：/etc/rcommerce/config.toml
   建议运行：chmod 600 /etc/rcommerce/config.toml
```

## 交互功能

CLI 使用 `dialoguer` crate 提供交互式提示：

- **输入验证**：实时验证并提供有用的错误信息
- **选择菜单**：使用方向键导航枚举和选项
- **确认提示**：带默认值的确认/取消
- **密码输入**：隐藏输入并确认匹配
- **摘要预览**：最终提交前审查所有数据

在交互式提示期间随时按 `Ctrl+C` 取消操作。

## 另请参阅

- [配置指南](../getting-started/configuration.md)
- [认证](../api-reference/authentication.md)
- [部署指南](../deployment/index.md)
