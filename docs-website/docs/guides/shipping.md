# Shipping Setup Guide

This guide walks you through setting up shipping for your R Commerce store, from configuring carriers to creating shipping zones and rules.

## Overview

R Commerce provides a comprehensive shipping system with support for:

- **Multiple Carriers**: DHL, FedEx, UPS, USPS, and aggregators like EasyPost
- **Shipping Zones**: Geographic rate management
- **Rules Engine**: Conditional shipping logic
- **Real-time Rates**: Live carrier rate calculation
- **Label Generation**: Automated shipping labels

## Step 1: Configure Carrier Credentials

Add shipping configuration to your `config.toml`:

```toml
[shipping]
default_provider = "ups"

[shipping.ups]
api_key = "your_api_key"
username = "your_username"
password = "your_password"
account_number = "your_account"
test_mode = true  # Set to false for production

[shipping.fedex]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = true

[shipping.dhl]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = true

[shipping.easypost]
api_key = "your_api_key"
test_mode = true
```

### Getting Carrier Credentials

#### UPS

1. Register at [UPS Developer Kit](https://developer.ups.com/)
2. Request API access for your account
3. Generate API credentials in the developer portal

**Required Credentials:**
- API Key
- Username
- Password
- Account Number

**UPS Services Available:**
- UPS Ground
- UPS 3 Day Select
- UPS 2nd Day Air
- UPS Next Day Air
- UPS Worldwide Express

#### FedEx

1. Create account at [FedEx Developer Portal](https://developer.fedex.com/)
2. Register your application
3. Obtain API key and secret

**Required Credentials:**
- API Key
- API Secret
- Account Number
- Meter Number (for some services)

**FedEx Services Available:**
- FedEx Ground
- FedEx Express Saver
- FedEx 2Day
- FedEx Priority Overnight
- FedEx International Priority

#### DHL

1. Sign up at [DHL API Developer Portal](https://developer.dhl.com/)
2. Subscribe to the Express API
3. Get your API credentials

**Required Credentials:**
- API Key
- API Secret
- Account Number

**DHL Services Available:**
- DHL Express Worldwide
- DHL Express 9:00
- DHL Express 10:30
- DHL Express 12:00

#### USPS

1. Register at [USPS Web Tools](https://www.usps.com/business/web-tools-apis/)
2. Request API access
3. Receive credentials via email

**Required Credentials:**
- User ID
- Password (for some services)

**USPS Services Available:**
- First-Class Mail
- Priority Mail
- Priority Mail Express
- Parcel Select

#### EasyPost (Recommended for Multiple Carriers)

1. Create account at [EasyPost](https://www.easypost.com/)
2. Copy your API key from the dashboard
3. Add carrier accounts through EasyPost interface

**Required Credentials:**
- API Key (test or production)

**EasyPost Features:**
- Unified API for 100+ carriers
- Automatic carrier account management
- Address verification
- Insurance options

### API Credential Setup

Store credentials securely using environment variables:

```bash
# .env file
UPS_API_KEY=your_ups_key
UPS_USERNAME=your_ups_username
UPS_PASSWORD=your_ups_password
UPS_ACCOUNT=your_ups_account

FEDEX_API_KEY=your_fedex_key
FEDEX_SECRET=your_fedex_secret
FEDEX_ACCOUNT=your_fedex_account

DHL_API_KEY=your_dhl_key
DHL_SECRET=your_dhl_secret

EASYPOST_API_KEY=your_easypost_key
```

Reference in config:

```toml
[shipping.ups]
api_key = "${UPS_API_KEY}"
username = "${UPS_USERNAME}"
password = "${UPS_PASSWORD}"
account_number = "${UPS_ACCOUNT}"
test_mode = false
```

## Step 2: Set Up Shipping Zones

Shipping zones define geographic regions with specific rates. Create zones based on your shipping strategy:

### Example: Domestic and International Zones

```toml
[shipping.zones.domestic]
name = "United States"
countries = ["US"]

[shipping.zones.domestic.rates.standard]
name = "Standard Ground"
base_rate = 8.00
per_kg_rate = 1.00
free_shipping_threshold = 100.00

[shipping.zones.domestic.rates.express]
name = "Express"
base_rate = 15.00
per_kg_rate = 2.50

[shipping.zones.international]
name = "Rest of World"
countries = ["*"]  # All other countries
exclude = ["US"]

[shipping.zones.international.rates.international]
name = "International Standard"
base_rate = 35.00
per_kg_rate = 5.00
```

### Zone Configuration Options

| Option | Description | Example |
|--------|-------------|---------|
| `countries` | List of ISO country codes | `["US", "CA", "MX"]` |
| `regions` | Specific regions/states | `["CA", "NY", "TX"]` |
| `postal_codes` | Specific postal code ranges | `["10000-19999"]` |
| `exclude` | Countries to exclude | `["US"]` |

### Advanced Zone Example

```toml
# European Union zone
[shipping.zones.eu]
name = "European Union"
countries = ["DE", "FR", "IT", "ES", "NL", "BE", "AT"]

[shipping.zones.eu.rates.standard]
name = "EU Standard"
base_rate = 12.00
per_kg_rate = 2.00
delivery_days = [5, 7]

[shipping.zones.eu.rates.express]
name = "EU Express"
base_rate = 25.00
per_kg_rate = 4.00
delivery_days = [1, 3]

# Remote areas with higher rates
[shipping.zones.remote]
name = "Remote Areas"
countries = ["IS", "GL", "FO"]

[shipping.zones.remote.rates.standard]
name = "Remote Standard"
base_rate = 50.00
per_kg_rate = 10.00
```

## Step 3: Configure Shipping Rules

Shipping rules allow you to create conditional logic for shipping options.

### Common Rule Types

```toml
[shipping.rules.free_shipping]
name = "Free Shipping Over $100"
condition = "order_total >= 100"
action = "set_rate_to_zero"
priority = 100

[shipping.rules.heavy_items]
name = "Heavy Item Surcharge"
condition = "weight > 20"
action = "add_surcharge"
amount = 10.00

[shipping.rules.express_upgrade]
name = "Free Express for VIP Customers"
condition = "customer_tag == 'vip' AND order_total >= 200"
action = "upgrade_to_express"
```

### Available Conditions

| Condition | Description | Example |
|-----------|-------------|---------|
| `order_total` | Order subtotal amount | `order_total >= 100` |
| `weight` | Total order weight | `weight > 10` |
| `item_count` | Number of items | `item_count >= 5` |
| `customer_tag` | Customer tag/segment | `customer_tag == 'vip'` |
| `product_category` | Product category | `product_category == 'fragile'` |
| `destination_country` | Shipping destination | `destination_country == 'CA'` |

### Available Actions

| Action | Description | Parameters |
|--------|-------------|------------|
| `set_rate_to_zero` | Make shipping free | None |
| `add_surcharge` | Add extra fee | `amount` |
| `discount_rate` | Apply percentage discount | `percentage` |
| `upgrade_to_express` | Upgrade shipping method | None |
| `hide_method` | Hide a shipping option | `method_name` |
| `require_signature` | Require signature | None |

## Step 4: Configure Package Types

Define standard package sizes for your products:

```toml
[shipping.packages]

[shipping.packages.small_box]
name = "Small Box"
length = 20
width = 15
height = 10
unit = "cm"
max_weight = 2.0

[shipping.packages.medium_box]
name = "Medium Box"
length = 30
width = 25
height = 20
unit = "cm"
max_weight = 5.0

[shipping.packages.large_box]
name = "Large Box"
length = 50
width = 40
height = 30
unit = "cm"
max_weight = 20.0

[shipping.packages.flat_rate_envelope]
name = "Flat Rate Envelope"
length = 32
width = 24
height = 2
unit = "cm"
max_weight = 1.0
flat_rate = 8.50
```

## Step 5: Set Up Address Validation

Enable address validation to reduce shipping errors:

```toml
[shipping.address_validation]
enabled = true
provider = "easypost"  # or "ups", "fedex"
cache_results = true
cache_duration_hours = 24
```

## Step 6: Test Your Configuration

### Using the CLI

```bash
# Test shipping rates
rcommerce shipping test-rates \
  --from "123 Main St, New York, NY 10001, US" \
  --to "456 Oak Ave, Los Angeles, CA 90210, US" \
  --weight 5 \
  --providers ups,fedex

# Validate an address
rcommerce shipping validate-address \
  --address "789 Pine Rd, Chicago, IL 60601, US"

# Test label generation (test mode)
rcommerce shipping test-label \
  --provider ups \
  --service "ground" \
  --package medium_box
```

### Test Scenarios

Test these common scenarios before going live:

1. **Domestic Standard** - Standard ground shipping within the same country
2. **Domestic Express** - Express/overnight shipping
3. **International** - Shipping to different countries
4. **Heavy Items** - Items over weight thresholds
5. **Free Shipping** - Orders meeting free shipping criteria
6. **Remote Areas** - Shipping to remote/extended delivery areas

## Step 7: Go Live

### Pre-Launch Checklist

- [ ] All carrier credentials are for production (not test/sandbox)
- [ ] `test_mode = false` in all carrier configurations
- [ ] Shipping zones cover all destinations you ship to
- [ ] Rates are calculated correctly for all zones
- [ ] Free shipping thresholds are configured
- [ ] Address validation is enabled
- [ ] Shipping rules are tested and working
- [ ] Package types are defined
- [ ] Label printing is tested
- [ ] Tracking number format is validated

### Monitoring After Launch

Monitor these metrics in your dashboard:

| Metric | Target | Action if Off Target |
|--------|--------|---------------------|
| Rate Calculation Success | >99% | Check carrier API status |
| Address Validation Pass | >95% | Review address input fields |
| Label Generation Success | >99% | Verify carrier account balance |
| Average Shipping Cost | Track trend | Adjust rates if needed |

## Best Practices

### 1. Use Aggregators for Multiple Carriers

If you ship with multiple carriers, consider using EasyPost or ShipStation to simplify integration:

```toml
[shipping]
default_provider = "easypost"

[shipping.easypost]
api_key = "your_api_key"
test_mode = false
carriers = ["ups", "fedex", "usps"]  # Enabled carriers
```

### 2. Implement Fallback Rates

Configure fallback rates in case carrier APIs are unavailable:

```toml
[shipping.fallback]
enabled = true
domestic_rate = 10.00
international_rate = 40.00
max_weight_for_fallback = 50
```

### 3. Cache Shipping Rates

Cache rates for common routes to improve performance:

```toml
[shipping.cache]
enabled = true
ttl_seconds = 3600  # 1 hour
max_entries = 1000
```

### 4. Handle Volumetric Weight

Carriers charge by dimensional weight for large, light items. Ensure your products have accurate dimensions:

```toml
[shipping.volumetric_weight]
enabled = true
dimensional_factor = 5000  # Standard divisor for cm/kg
```

### 5. Set Up Shipping Notifications

Configure email notifications for shipping events:

```toml
[notifications.shipping]
ship_confirmation = true
delivery_confirmation = true
exception_alerts = true
template_prefix = "shipping_"
```

## Troubleshooting

### Common Issues

**"No shipping rates available"**
- Check carrier credentials are valid
- Verify shipping zone covers the destination
- Ensure package weight is within carrier limits
- Check carrier service availability for the route

**"Address validation failed"**
- Verify the address format matches the country
- Check for missing required fields (state, postal code)
- Try standardizing the address format

**"Label generation failed"**
- Verify carrier account is active and has balance
- Check package dimensions are within carrier limits
- Ensure customs information is provided for international shipments
- Verify the from address is valid

**Rates seem incorrect**
- Check dimensional weight calculation
- Verify package dimensions are accurate
- Review shipping zone configuration
- Check for conflicting shipping rules

### Debug Mode

Enable debug logging to troubleshoot issues:

```toml
[shipping.debug]
log_requests = true
log_responses = true
log_level = "debug"
```

## Next Steps

- [Configure Tax Settings](../guides/tax-setup.md)
- [Set Up Payment Gateways](../payment-gateways/index.md)
- [Configure Notifications](../guides/notifications.md)
- [API Reference: Shipping](../api-reference/shipping.md)
