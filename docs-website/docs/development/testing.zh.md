# 测试指南

R Commerce 包含多层次的全面测试，以确保可靠性和正确性。

## 测试类型

### 1. 单元测试

单元测试位于每个 crate 的源文件中，使用 Rust 内置的测试框架：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_creation() {
        let product = Product::new("Test Product", dec!(29.99));
        assert_eq!(product.title, "Test Product");
        assert_eq!(product.price, dec!(29.99));
    }
}
```

运行单元测试：
```bash
cargo test --lib
```

### 2. 集成测试

集成测试位于 `tests/` 目录中，测试完整的 API：

```bash
# 运行所有集成测试
cargo test --test integration

# 运行特定测试
cargo test test_create_order
```

### 3. API 测试（MVP）

MVP 测试套件验证核心 API 功能：

```bash
# 运行 MVP API 测试
./scripts/test_api_mvp.sh

# 或直接使用 curl 运行
curl -s http://localhost:8080/health
curl -s http://localhost:8080/api/v1/products
```

### 4. 端到端测试

包含数据库操作的完整系统测试：

```bash
# 运行 E2E 测试套件
./scripts/run_e2e_tests.sh

# 运行完整系统测试
./scripts/test_complete_system.sh
```

## 测试组织

```
crates/
├── rcommerce-core/
│   └── src/
│       └── services/
│           └── product_service.rs      # 内联单元测试
├── rcommerce-api/
│   └── tests/
│       └── integration_tests.rs        # 集成测试
└── rcommerce-cli/
    └── tests/
        └── cli_tests.rs                # CLI 测试
```

## 编写测试

### 单元测试示例

```rust
// 在您的源文件中
pub fn calculate_tax(amount: Decimal, rate: Decimal) -> Decimal {
    (amount * rate).round_dp(2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_calculate_tax() {
        let amount = dec!(100.00);
        let rate = dec!(0.10);
        let tax = calculate_tax(amount, rate);
        assert_eq!(tax, dec!(10.00));
    }

    #[test]
    fn test_calculate_tax_rounding() {
        let amount = dec!(99.99);
        let rate = dec!(0.10);
        let tax = calculate_tax(amount, rate);
        assert_eq!(tax, dec!(10.00));
    }
}
```

### 集成测试示例

```rust
// tests/product_api_tests.rs
use reqwest::Client;

#[tokio::test]
async fn test_list_products() {
    let client = Client::new();
    let response = client
        .get("http://localhost:8080/api/v1/products")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    
    let products: Vec<Product> = response
        .json()
        .await
        .expect("Failed to parse response");
    
    assert!(!products.is_empty());
}
```

### 异步测试示例

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_operation().await;
    assert!(result.is_ok());
}
```

## 测试数据库

测试使用单独的数据库配置：

```toml
# config.test.toml
[database]
db_type = "Sqlite"
sqlite_path = ":memory:"
```

使用测试配置运行测试：
```bash
export RCOMMERCE_CONFIG=config.test.toml
cargo test
```

## Mocking

使用 `mockall` 进行依赖模拟：

```rust
use mockall::mock;

mock! {
    PaymentGateway {}
    
    #[async_trait]
    impl PaymentGateway for PaymentGateway {
        async fn process_payment(&self, amount: Decimal) -> Result<PaymentResult, Error>;
    }
}

#[tokio::test]
async fn test_payment_processing() {
    let mut mock = MockPaymentGateway::new();
    mock.expect_process_payment()
        .returning(|_| Ok(PaymentResult::Success));
    
    let service = PaymentService::new(mock);
    let result = service.process(dec!(100.00)).await;
    
    assert!(result.is_ok());
}
```

## 代码覆盖率

生成测试覆盖率报告：

```bash
# 安装 tarpaulin
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html

# 查看报告
open tarpaulin-report.html
```

## 持续集成

测试在以下情况自动运行：
- 每个 Pull Request
- 每次推送到 main 分支
- 每日定时运行

查看 `.github/workflows/` 获取 CI 配置。

## 测试最佳实践

1. **测试行为，而非实现** - 测试应验证代码做什么，而非如何做
2. **每个测试一个断言** - 保持测试专注和可读
3. **使用描述性名称** - 测试名称应解释场景
4. **Arrange-Act-Assert** - 清晰地组织测试
5. **测试后清理** - 不要在数据库中留下测试数据

## 调试测试

```bash
# 带输出运行
cargo test -- --nocapture

# 带输出运行特定测试
cargo test test_name -- --nocapture

# 使用调试器运行
rust-gdb --args cargo test test_name
```

## 性能测试

```bash
# 运行基准测试
cargo bench

# 分析测试性能
cargo test --release -- --nocapture
```

## 负载测试

使用包含的负载测试脚本：

```bash
# 安装 k6
brew install k6

# 运行负载测试
k6 run scripts/load_test.js
```
