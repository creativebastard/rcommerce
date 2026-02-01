# CLI Reference

The R Commerce CLI (`rcommerce`) provides commands for server management, database operations, API key administration, and interactive product/customer creation.

## Global Options

```bash
rcommerce [OPTIONS] <COMMAND>

Options:
  -c, --config <CONFIG>        Configuration file path
  -l, --log-level <LOG_LEVEL>  Set log level (debug, info, warn, error)
  -h, --help                   Print help
  -V, --version                Print version
```

## Commands

### Server

Start the API server:

```bash
rcommerce server [OPTIONS]

Options:
  -H, --host <HOST>      Bind address [default: 0.0.0.0]
  -P, --port <PORT>      Port number [default: 8080]
      --skip-migrate     Skip automatic database migration
```

**Examples:**

```bash
# Start with default config
rcommerce server

# Start on custom port
rcommerce server -P 3000

# Start without migrations
rcommerce server --skip-migrate
```

### Database

Database management commands:

```bash
rcommerce db <COMMAND>

Commands:
  migrate    Run database migrations
  reset      Reset database (DANGEROUS - deletes all data)
  seed       Seed database with demo data
  status     Show database status
```

**Examples:**

```bash
# Run migrations
rcommerce db migrate -c config.toml

# Check database status
rcommerce db status -c config.toml

# Reset database (with confirmation)
rcommerce db reset -c config.toml

# Seed with demo data
rcommerce db seed -c config.toml
```

### API Key Management

Manage API keys for service-to-service authentication:

```bash
rcommerce api-key <COMMAND>

Commands:
  list       List all API keys
  create     Create a new API key
  get        Get API key details
  revoke     Revoke an API key
  delete     Delete an API key permanently
```

#### List API Keys

```bash
rcommerce api-key list [OPTIONS]

Options:
  -u, --customer-id <ID>  Filter by customer ID
```

**Example:**

```bash
rcommerce api-key list -c config.toml
```

Output:
```
API Keys
Prefix       Name                 Scopes                         Active     Expires
------------------------------------------------------------------------------------------
aB3dEfGh     Production Backend   read, write                    ✓          Never
Xy9zZzZz     Test Key             read                           ✗          2024-12-31
```

#### Create API Key

```bash
rcommerce api-key create [OPTIONS]

Options:
  -u, --customer-id <ID>     Customer ID (optional for system keys)
  -n, --name <NAME>          Key name/description
  -s, --scopes <SCOPES>      Scopes (comma-separated) [default: read]
  -e, --expires-days <DAYS>  Expiration in days (optional)
```

**Example:**

```bash
rcommerce api-key create \
  -c config.toml \
  --name "Production Backend" \
  --scopes "read,write"
```

Output:
```
✅ API Key created successfully!

IMPORTANT: Copy this key now - it won't be shown again!

  Key: aB3dEfGh.sEcReTkEy123456789

  Prefix:      aB3dEfGh
  Name:        Production Backend
  Scopes:      read, write
  Customer ID: System
  Expires:     Never
```

#### Get API Key Details

```bash
rcommerce api-key get <PREFIX>
```

**Example:**

```bash
rcommerce api-key get -c config.toml aB3dEfGh
```

Output:
```
API Key Details
  Prefix:       aB3dEfGh
  Name:         Production Backend
  Scopes:       read, write
  Active:       ✓ Yes
  Customer ID:  System
  Created:      2024-01-31 10:30:00 UTC
  Updated:      2024-01-31 10:30:00 UTC
  Expires:      Never
  Last Used:    Never
```

#### Revoke API Key

```bash
rcommerce api-key revoke [OPTIONS] <PREFIX>

Options:
  -r, --reason <REASON>  Reason for revocation
```

**Example:**

```bash
rcommerce api-key revoke \
  -c config.toml \
  aB3dEfGh \
  --reason "Key compromised"
```

#### Delete API Key

Permanently delete an API key (irreversible):

```bash
rcommerce api-key delete [OPTIONS] <PREFIX>

Options:
      --force  Skip confirmation
```

**Example:**

```bash
# With confirmation prompt
rcommerce api-key delete -c config.toml aB3dEfGh

# Skip confirmation
rcommerce api-key delete -c config.toml aB3dEfGh --force
```

### Product Management

```bash
rcommerce product <COMMAND>

Commands:
  list       List products
  create     Create a product (interactive)
  get        Get product details
  update     Update a product
  delete     Delete a product
```

#### List Products

```bash
rcommerce product list -c config.toml
```

Output:
```
Products
ID                                    Title                          Price      Currency   Status
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  Premium T-Shirt                29.99      USD        ✓ Active
550e8400-e29b-41d4-a716-446655440001  Wireless Headphones            149.99     USD        ✓ Active

Total: 2 products
```

#### Create Product (Interactive)

```bash
rcommerce product create -c config.toml
```

This command launches an interactive prompt that guides you through product creation:

```
📦 Create New Product
Press Ctrl+C to cancel at any time.

Product title: Premium T-Shirt
URL slug [premium-t-shirt]: premium-t-shirt
Product type:
  > Simple
    Variable
    Digital
    Bundle
Price: 29.99
Currency:
  > USD
    EUR
    GBP
    JPY
    AUD
    CAD
    CNY
    HKD
    SGD
SKU (optional): TSHIRT-001
Inventory quantity [0]: 100
Description (optional): High quality cotton t-shirt
Make product active? [Y/n]: y
Mark as featured? [y/N]: n

📋 Product Summary
  Title:       Premium T-Shirt
  Slug:        premium-t-shirt
  Type:        Simple
  Price:       29.99 USD
  SKU:         TSHIRT-001
  Inventory:   100
  Description: High quality cotton t-shirt
  Active:      Yes
  Featured:    No

Create this product? [Y/n]: y

✅ Product created successfully!
  ID:    550e8400-e29b-41d4-a716-446655440000
  Title: Premium T-Shirt
  Slug:  premium-t-shirt
  Price: 29.99 USD
```

**Interactive prompts include:**
- Product title (required, max 255 chars)
- URL slug (auto-generated from title, editable)
- Product type selection (Simple/Variable/Digital/Bundle)
- Price (numeric validation)
- Currency selection (USD/EUR/GBP/JPY/AUD/CAD/CNY/HKD/SGD)
- SKU (optional, max 100 chars)
- Inventory quantity (default: 0)
- Description (optional)
- Active status (default: Yes)
- Featured status (default: No)

#### Get Product Details

```bash
rcommerce product get -c config.toml <product-id>
```

**Example:**

```bash
rcommerce product get -c config.toml 550e8400-e29b-41d4-a716-446655440000
```

Output:
```
Product Details
  ID:          550e8400-e29b-41d4-a716-446655440000
  Title:       Premium T-Shirt
  Slug:        premium-t-shirt
  Price:       29.99 USD
  Status:      ✓ Active
  Inventory:   100
  Created:     2024-01-31 10:30:00 UTC
  Description: High quality cotton t-shirt
```

#### Delete Product

```bash
rcommerce product delete -c config.toml <product-id>
```

**Example:**

```bash
rcommerce product delete -c config.toml 550e8400-e29b-41d4-a716-446655440000
```

This will prompt for confirmation:
```
⚠️  Product deletion
Type 'yes' to delete product '550e8400-e29b-41d4-a716-446655440000': yes
✅ Product '550e8400-e29b-41d4-a716-446655440000' deleted
```

### Order Management

```bash
rcommerce order <COMMAND>

Commands:
  list       List orders
  get        Get order details
  create     Create a test order
  update     Update order status
```

#### List Orders

```bash
rcommerce order list -c config.toml
```

Output:
```
Orders
ID                                    Customer             Status       Total           Created
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  john@example.com     pending      149.99          2024-01-31
550e8400-e29b-41d4-a716-446655440001  jane@example.com     completed    299.98          2024-01-30

Total: 2 orders
```

### Customer Management

```bash
rcommerce customer <COMMAND>

Commands:
  list       List customers
  get        Get customer details
  create     Create a customer (interactive)
```

#### List Customers

```bash
rcommerce customer list -c config.toml
```

Output:
```
Customers
ID                                    Email                          Name                 Created
----------------------------------------------------------------------------------------------------
550e8400-e29b-41d4-a716-446655440000  john@example.com               John Doe             2024-01-31
550e8400-e29b-41d4-a716-446655440001  jane@example.com               Jane Smith           2024-01-30

Total: 2 customers
```

#### Create Customer (Interactive)

```bash
rcommerce customer create -c config.toml
```

This command launches an interactive prompt that guides you through customer creation:

```
👤 Create New Customer
Press Ctrl+C to cancel at any time.

Email address: john@example.com
First name: John
Last name: Doe
Phone number (optional): +1234567890
Preferred currency:
  > USD
    EUR
    GBP
    JPY
    AUD
    CAD
    CNY
    HKD
    SGD
Accepts marketing emails? [y/N]: n
Password: ********
Confirm password: ********

📋 Customer Summary
  Name:              John Doe
  Email:             john@example.com
  Phone:             +1234567890
  Currency:          USD
  Accepts Marketing: No

Create this customer? [Y/n]: y

✅ Customer created successfully!
  ID:    550e8400-e29b-41d4-a716-446655440000
  Name:  John Doe
  Email: john@example.com
```

**Interactive prompts include:**
- Email address (required, validated)
- First name (required, max 100 chars)
- Last name (required, max 100 chars)
- Phone number (optional)
- Preferred currency selection
- Marketing consent (default: No)
- Password (min 8 chars, with confirmation)

#### Get Customer Details

```bash
rcommerce customer get -c config.toml <customer-id>
```

### Configuration

Display the loaded configuration:

```bash
rcommerce config -c config.toml
```

## Environment Variables

The CLI respects these environment variables:

| Variable | Description |
|----------|-------------|
| `RCOMMERCE_CONFIG` | Default config file path |
| `RUST_LOG` | Log level (debug, info, warn, error) |

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Database error |
| 4 | Configuration error |

## Security Features

The CLI includes several security features:

### Root User Prevention

The CLI will refuse to run as the root user for security reasons:

```
❌ ERROR: Running as root is not allowed!
   The rcommerce CLI should not be run as root for security reasons.
   Please run as a non-privileged user.
```

### Config File Permissions

The CLI warns if your config file has overly permissive permissions:

```
⚠️  WARNING: Config file is world-readable
   Path: /etc/rcommerce/config.toml
   Consider running: chmod 600 /etc/rcommerce/config.toml
```

## Interactive Features

The CLI uses the `dialoguer` crate to provide interactive prompts for:

- **Input validation**: Real-time validation with helpful error messages
- **Selection menus**: Arrow key navigation for enums and options
- **Confirmation prompts**: Yes/no confirmations with defaults
- **Password input**: Hidden input with confirmation matching
- **Summary preview**: Review all data before final submission

Press `Ctrl+C` at any time during interactive prompts to cancel the operation.

## See Also

- [Configuration Guide](../getting-started/configuration.md)
- [Authentication](../api-reference/authentication.md)
- [Deployment Guide](../deployment/index.md)
