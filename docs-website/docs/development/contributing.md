# Contributing Guide

Thank you for your interest in contributing to R Commerce! This guide will help you get started.

## Code of Conduct

This project follows a standard code of conduct:
- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Respect differing viewpoints

## Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/rcommerce.git
cd rcommerce

# Add upstream remote
git remote add upstream https://github.com/creativebastard/rcommerce.git
```

### 2. Set Up Development Environment

Follow the [Local Development Setup](./local-setup.md) guide to configure your environment.

### 3. Create a Branch

```bash
# Sync with upstream
git fetch upstream
git checkout main
git merge upstream/main

# Create feature branch
git checkout -b feature/your-feature-name
```

## Development Workflow

### Making Changes

1. **Write code** following Rust best practices
2. **Add tests** for new functionality
3. **Update documentation** as needed
4. **Run tests** to ensure nothing breaks

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test --workspace

# Check for security issues
cargo audit
```

### Commit Guidelines

Use conventional commits format:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build process or auxiliary tool changes

Examples:
```
feat(products): add product variant support

fix(orders): correct tax calculation for discounts
docs(api): update authentication examples
```

### Before Submitting

```bash
# Ensure all tests pass
cargo test --workspace

# Check code formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build in release mode
cargo build --release
```

## Pull Request Process

### 1. Update Documentation

- Update relevant documentation in `docs/` or `docs-website/`
- Add examples if applicable
- Update CHANGELOG.md

### 2. Create Pull Request

```bash
# Push your branch
git push origin feature/your-feature-name
```

Then create a PR on GitHub with:
- Clear title describing the change
- Detailed description of what and why
- Reference any related issues
- Screenshots for UI changes

### 3. PR Review

- Address review comments
- Keep discussions focused and professional
- Be patient - maintainers are volunteers

### 4. Merge

Once approved, a maintainer will merge your PR. 

## Areas for Contribution

### High Priority

- **Payment Gateways**: Additional payment provider integrations
- **Shipping**: More shipping carrier integrations
- **Frontend**: Demo frontend improvements
- **Documentation**: User guides and tutorials

### Medium Priority

- **Performance**: Optimization and caching improvements
- **Testing**: Additional test coverage
- **CLI**: New CLI commands and features
- **Monitoring**: Metrics and observability

### Good First Issues

Look for issues labeled:
- `good first issue`
- `help wanted`
- `documentation`

## Project Structure

```
rcommerce/
├── crates/
│   ├── rcommerce-core/     # Core business logic
│   ├── rcommerce-api/      # HTTP API server
│   └── rcommerce-cli/      # Command line tool
├── docs/                    # Technical documentation
├── docs-website/           # User documentation website
├── scripts/                # Utility scripts
└── migrations/             # Database migrations
```

## Coding Standards

### Rust Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

- Use `snake_case` for functions and variables
- Use `CamelCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Document public APIs with doc comments

### Example

```rust
/// Calculates the total price including tax
/// 
/// # Arguments
/// 
/// * `price` - The base price
/// * `tax_rate` - The tax rate as a decimal (e.g., 0.10 for 10%)
/// 
/// # Returns
/// 
/// The total price with tax applied
/// 
/// # Example
/// 
/// ```
/// let total = calculate_total(dec!(100.00), dec!(0.10));
/// assert_eq!(total, dec!(110.00));
/// ```
pub fn calculate_total(price: Decimal, tax_rate: Decimal) -> Decimal {
    price * (Decimal::ONE + tax_rate)
}
```

### Error Handling

Use the project's error types:

```rust
use rcommerce_core::{Error, Result};

fn do_something() -> Result<Thing> {
    if invalid {
        return Err(Error::validation("Field is required"));
    }
    Ok(thing)
}
```

## Testing Requirements

- All new features must include tests
- Bug fixes should include regression tests
- Maintain or improve code coverage
- Integration tests for API endpoints

## Documentation

### Code Documentation

- Document all public functions, structs, and traits
- Include examples in doc comments
- Explain complex algorithms

### User Documentation

Update relevant docs in:
- `docs/` - Technical documentation
- `docs-website/` - User-facing documentation
- `README.md` - Project overview

## Security

Report security vulnerabilities privately:
- Email: security@rcommerce.app
- Do not open public issues for security bugs

## Getting Help

- **GitHub Discussions**: For questions and ideas
- **Discord**: [Join our community](https://discord.gg/rcommerce)
- **Issues**: For bug reports and feature requests

## Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Given credit in documentation

Thank you for contributing to R Commerce!
