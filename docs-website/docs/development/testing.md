# Testing Guide

R Commerce includes comprehensive testing at multiple levels to ensure reliability and correctness.

## Test Types

### 1. Unit Tests

Unit tests are located within each crate's source files, using Rust's built-in test framework:

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

Run unit tests:
```bash
cargo test --lib
```

### 2. Integration Tests

Integration tests are located in `tests/` directories and test the full API:

```bash
# Run all integration tests
cargo test --test integration

# Run specific test
cargo test test_create_order
```

### 3. API Tests (MVP)

The MVP test suite validates core API functionality:

```bash
# Run MVP API tests
./scripts/test_api_mvp.sh

# Or run with curl directly
curl -s http://localhost:8080/health
curl -s http://localhost:8080/api/v1/products
```

### 4. End-to-End Tests

Full system tests including database operations:

```bash
# Run E2E test suite
./scripts/run_e2e_tests.sh

# Run complete system test
./scripts/test_complete_system.sh
```

## Test Organization

```
crates/
├── rcommerce-core/
│   └── src/
│       └── services/
│           └── product_service.rs      # Unit tests inline
├── rcommerce-api/
│   └── tests/
│       └── integration_tests.rs        # Integration tests
└── rcommerce-cli/
    └── tests/
        └── cli_tests.rs                # CLI tests
```

## Writing Tests

### Unit Test Example

```rust
// In your source file
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

### Integration Test Example

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

### Async Test Example

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_operation().await;
    assert!(result.is_ok());
}
```

## Test Database

Tests use a separate database configuration:

```toml
# config.test.toml
[database]
db_type = "Sqlite"
sqlite_path = ":memory:"
```

Run tests with test configuration:
```bash
export RCOMMERCE_CONFIG=config.test.toml
cargo test
```

## Mocking

Use `mockall` for mocking dependencies:

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

## Code Coverage

Generate test coverage reports:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View report
open tarpaulin-report.html
```

## Continuous Integration

Tests run automatically on:
- Every pull request
- Every push to main branch
- Daily scheduled runs

See `.github/workflows/` for CI configuration.

## Test Best Practices

1. **Test behavior, not implementation** - Tests should verify what code does, not how it does it
2. **One assertion per test** - Keep tests focused and readable
3. **Use descriptive names** - Test names should explain the scenario
4. **Arrange-Act-Assert** - Structure tests clearly
5. **Clean up after tests** - Don't leave test data in databases

## Debugging Tests

```bash
# Run with output
cargo test -- --nocapture

# Run specific test with output
cargo test test_name -- --nocapture

# Run with debugger
rust-gdb --args cargo test test_name
```

## Performance Testing

```bash
# Run benchmarks
cargo bench

# Profile test performance
cargo test --release -- --nocapture
```

## Load Testing

Use the included load testing script:

```bash
# Install k6
brew install k6

# Run load tests
k6 run scripts/load_test.js
```
