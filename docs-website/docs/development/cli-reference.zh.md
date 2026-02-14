# CLI 参考

R Commerce CLI (`rcommerce`) 提供用于服务器管理、数据库操作、API 密钥管理以及交互式产品/客户创建的命令。

## 全局选项

```bash
rcommerce [OPTIONS] <COMMAND>

选项：
  -c, --config <CONFIG>        配置文件路径
  -l, --log-level <LOG_LEVEL>  设置日志级别（debug、info、warn、error）
  -h, --help                   打印帮助
  -V, --version                打印版本
```

## 交互式 Shell（命令行界面）

`shell` 命令启动一个交互式 REPL（读取-求值-输出循环），用于管理您的 R Commerce 安装：

```bash
rcommerce shell -c config.toml
```

这提供了一个命令行界面，用于列出产品、订单、客户等，无需离开终端。

### Shell 命令

进入 shell 后，您可以使用以下命令：

| 命令 | 描述 | 示例 |
|---------|-------------|---------|
| `help`, `h`, `?` | 显示可用命令 | `help` |
| `exit`, `quit`, `q` | 退出 shell | `exit` |
| `clear`, `cls` | 清屏 | `clear` |
| `dashboard`, `dash`, `d` | 显示仪表板概览 | `dashboard` |
| `status`, `st` | 显示数据库状态 | `status` |
| `list <entity> [limit]` | 列出实体 | `list products 10` |
| `get <entity> <id>` | 获取实体详情 | `get product abc-123` |
| `create <entity>` | 创建新实体 | `create product` |
| `delete <entity> <id>` | 删除实体 | `delete customer xyz-789` |
| `search <entity> <query>` | 搜索实体 | `search products laptop` |

**实体快捷方式：**
- `p` → product(s)（产品）
- `o` → order(s)（订单）
- `c` → customer(s)（客户）
- `k`, `keys` → api-keys（API 密钥）

### Shell 示例

```
$ rcommerce shell -c config.toml

╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║           🛒 R Commerce 交互式 Shell                          ║
║                                                               ║
║     输入 'help' 查看可用命令或 'exit' 退出                    ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝

rcommerce> dashboard

📊 仪表板

关键指标：
  产品数量：            150
  订单数量：            42
  客户数量：            28
  总收入：              $12,450.00

最近订单：
  ID                                    客户                  状态       总计         创建时间
  ----------------------------------------------------------------------------------------------------
  550e8400-e29b-41d4-a716-446655440000  john@example.com      已完成     $299.99      2024-01-31
  550e8400-e29b-41d4-a716-446655440001  jane@example.com      待处理     $149.50      2024-01-30

rcommerce> list products 5

产品（显示 5 个）
  ID                                    标题                          价格       货币       状态
  ----------------------------------------------------------------------------------------------------
  550e8400-e29b-41d4-a716-446655440000  Premium T-Shirt               29.99      USD        ✓ 激活
  550e8400-e29b-41d4-a716-446655440001  Wireless Headphones           149.99     USD        ✓ 激活

rcommerce> search products laptop

匹配 'laptop' 的产品（3 个）
  ID                                    标题                          价格       货币       状态
  ----------------------------------------------------------------------------------------------------
  550e8400-e29b-41d4-a716-446655440002  Gaming Laptop Pro             1299.99    USD        ✓ 激活

rcommerce> exit

再见：再见！👋
```

### Shell 中的交互式创建

Shell 支持交互式创建产品和客户：

```
rcommerce> create product

📦 创建新产品
产品标题：Premium T-Shirt
URL slug [premium-t-shirt]: premium-t-shirt
产品类型：
  > Simple
    Variable
    Digital
    Bundle
价格：29.99
...

✓ 产品创建成功！
  ID：    550e8400-e29b-41d4-a716-446655440000
  标题：  Premium T-Shirt
```

## 命令

### Server（服务器）

启动 API 服务器：

```bash
rcommerce server [OPTIONS]

选项：
  -H, --host <HOST>      绑定地址 [默认：0.0.0.0]
  -P, --port <PORT>      端口号 [默认：8080]
      --skip-migrate     跳过自动数据库迁移
```

**示例：**

```bash
# 使用默认配置启动
rcommerce server

# 在自定义端口启动
rcommerce server -P 3000

# 不迁移启动
rcommerce server --skip-migrate
```

### Database（数据库）

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

### API Key 管理

管理服务间认证的 API 密钥：

```bash
rcommerce api-key <COMMAND>

命令：
  list       列出所有 API 密钥
  create     创建新的 API 密钥
  get        获取 API 密钥详情
  revoke     撤销 API 密钥
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
  -s, --scopes <SCOPES>      权限范围（逗号分隔）[默认：read]
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

重要：立即复制此密钥 - 不会再次显示！

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

#### 撤销 API 密钥

```bash
rcommerce api-key revoke [OPTIONS] <PREFIX>

选项：
  -r, --reason <REASON>  撤销原因
```

**示例：**

```bash
rcommerce api-key revoke \
  -c config.toml \
  aB3dEfGh \
  --reason "密钥已泄露"
```

#### 删除 API 密钥

永久删除 API 密钥（不可逆）：

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

### Product 管理

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
ID                                    标题                          价格      货币   状态
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  Premium T-Shirt                29.99      USD        ✓ Active
550e8400-e29b-41d4-a716-446655440001  Wireless Headphones            149.99     USD        ✓ Active

总计：2 个产品
```

#### 创建产品（交互式）

```bash
rcommerce product create -c config.toml
```

此命令启动交互式提示，引导您完成产品创建：

```
📦 创建新产品
按 Ctrl+C 随时取消。

产品标题：Premium T-Shirt
URL slug [premium-t-shirt]: premium-t-shirt
产品类型：
  > Simple
    Variable
    Digital
    Bundle
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
库存数量 [0]：100
描述（可选）：High quality cotton t-shirt
激活产品？[Y/n]: y
标记为精选？[y/N]: n

📋 产品摘要
  标题：       Premium T-Shirt
  Slug：       premium-t-shirt
  类型：       Simple
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
  Slug：  premium-t-shirt
  价格：  29.99 USD
```

**交互式提示包括：**
- 产品标题（必填，最多 255 字符）
- URL slug（从标题自动生成，可编辑）
- 产品类型选择（Simple/Variable/Digital/Bundle）
- 价格（数字验证）
- 货币选择（USD/EUR/GBP/JPY/AUD/CAD/CNY/HKD/SGD）
- SKU（可选，最多 100 字符）
- 库存数量（默认：0）
- 描述（可选）
- 激活状态（默认：是）
- 精选状态（默认：否）

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
  Slug：        premium-t-shirt
  价格：        29.99 USD
  状态：        ✓ Active
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
⚠️  产品删除
输入 'yes' 删除产品 '550e8400-e29b-41d4-a716-446655440000'：yes
✅ 产品 '550e8400-e29b-41d4-a716-446655440000' 已删除
```

### Order 管理

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
ID                                    客户                 状态       总计           创建时间
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  john@example.com     pending    149.99         2024-01-31
550e8400-e29b-41d4-a716-446655440001  jane@example.com     completed  299.98         2024-01-30

总计：2 个订单
```

### Customer 管理

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
按 Ctrl+C 随时取消。

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
接受营销邮件？[y/N]: n
密码：********
确认密码：********

📋 客户摘要
  姓名：              John Doe
  邮箱：              john@example.com
  电话：              +1234567890
  货币：              USD
  接受营销：          否

创建此客户？[Y/n]: y

✅ 客户创建成功！
  ID：    550e8400-e29b-41d4-a716-446655440000
  姓名：  John Doe
  邮箱：  john@example.com
```

**交互式提示包括：**
- 邮箱地址（必填，已验证）
- 名字（必填，最多 100 字符）
- 姓氏（必填，最多 100 字符）
- 电话号码（可选）
- 首选货币选择
- 营销同意（默认：否）
- 密码（最少 8 字符，带确认）

#### 获取客户详情

```bash
rcommerce customer get -c config.toml <customer-id>
```

### Configuration（配置）

显示加载的配置：

```bash
rcommerce config -c config.toml
```

### Import（导入）

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
  -e, --entities <ENTITIES>    逗号分隔列表：products,customers,orders [默认：all]
      --limit <LIMIT>          每实体最大导入记录数
      --dry-run                验证数据而不导入
```

**支持的平台：**

| 平台 | 状态 | 认证方式 | 实体 |
|------|------|----------|------|
| Shopify | ✅ 完整 | API Key + Password | Products、Customers、Orders |
| WooCommerce | ✅ 完整 | Consumer Key + Secret | Products、Customers、Orders |
| Magento | 🚧 计划中 | OAuth/API Token | Products、Customers、Orders |
| Medusa | 🚧 计划中 | API Token | Products、Customers、Orders |

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

# 从 WooCommerce 导入（带限制）
rcommerce import platform woocommerce \
  -c config.toml \
  --api-url https://your-store.com \
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
🧪 试运行模式 - 不会导入数据
从 Shopify 获取产品（试运行）...
验证：Premium T-Shirt
验证：Wireless Headphones
...

导入摘要（试运行）
========================
实体：products
  总计：     150
  已创建：   150
  已更新：   0
  已跳过：   0
  错误：     0

✅ 验证完成。不带 --dry-run 运行以导入。
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
      --dry-run            验证数据而不导入
```

**文件格式支持：**

| 格式 | 状态 | 描述 |
|------|------|------|
| CSV | ✅ 完整 | 带标题的逗号分隔值 |
| JSON | ✅ 完整 | JSON 对象数组 |
| XML | 🚧 计划中 | XML 文档格式 |

**CSV 格式：**

每个实体类型的预期列：

**Products：**
```csv
id,title,slug,description,price,compare_at_price,sku,inventory_quantity,status,product_type
TSHIRT-001,Premium T-Shirt,premium-t-shirt,High quality cotton,29.99,39.99,TSHIRT-001,100,active,physical
```

**Customers：**
```csv
id,email,first_name,last_name,phone,address1,city,state,postal_code,country
cust-001,john@example.com,John,Doe,+1234567890,123 Main St,New York,NY,10001,US
```

**Orders：**
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
# 导入的默认批次大小
batch_size = 100

# 出错时继续（跳过失败记录）
continue_on_error = true

# 跳过现有记录（基于唯一标识符）
skip_existing = true

[import.shopify]
api_version = "2024-01"
# 店铺特定设置

[import.woocommerce]
verify_ssl = true
```

### 环境变量

CLI 尊重以下环境变量：

| 变量 | 描述 |
|------|------|
| `RCOMMERCE_CONFIG` | 默认配置文件路径 |
| `RUST_LOG` | 日志级别（debug、info、warn、error）|

## 退出码

| 代码 | 含义 |
|------|------|
| 0 | 成功 |
| 1 | 一般错误 |
| 2 | 无效参数 |
| 3 | 数据库错误 |
| 4 | 配置错误 |

## 安全特性

CLI 包含多项安全特性：

### Root 用户阻止

出于安全原因，CLI 将拒绝以 root 用户运行：

```
❌ 错误：不允许以 root 运行！
   rcommerce CLI 不应以 root 运行。
   请以非特权用户运行。
```

### 配置文件权限

如果配置文件权限过于宽松，CLI 会发出警告：

```
⚠️  警告：配置文件可被所有人读取
   路径：/etc/rcommerce/config.toml
   建议运行：chmod 600 /etc/rcommerce/config.toml
```

## 交互特性

CLI 使用 `dialoguer` crate 提供交互式提示：

- **输入验证**：实时验证和有用的错误消息
- **选择菜单**：使用方向键导航枚举和选项
- **确认提示**：带默认值的 是/否 确认
- **密码输入**：带确认匹配的隐藏输入
- **摘要预览**：最终提交前查看所有数据

在交互式提示期间随时按 `Ctrl+C` 取消操作。

## 另请参阅

- [配置指南](../getting-started/configuration.md)
- [认证](../api-reference/authentication.md)
- [部署指南](../deployment/index.md)
