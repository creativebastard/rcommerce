# CLI Reference

The R Commerce CLI (`rcommerce`) provides commands for server management, database operations, and API key administration.

## Global Options

```bash
rcommerce [OPTIONS] <COMMAND>

Options:
  -c, --config <CONFIG>        Configuration file path
  -l, --log-level <LOG_LEVEL>  Set log level (debug, info, warn, error)
  -h, --help                   Print help
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
  list    List products
  create  Create a product
  get     Get product details
  update  Update a product
  delete  Delete a product
```

### Order Management

```bash
rcommerce order <COMMAND>

Commands:
  list    List orders
  get     Get order details
  create  Create a test order
  update  Update order status
```

### Customer Management

```bash
rcommerce customer <COMMAND>

Commands:
  list    List customers
  get     Get customer details
  create  Create a customer
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

## See Also

- [Configuration Guide](../getting-started/configuration.md)
- [Authentication](../api-reference/authentication.md)
- [Deployment Guide](../deployment/index.md)
