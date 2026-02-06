# Shipping Integration

R Commerce provides a comprehensive shipping system with support for multiple carriers, real-time rate calculation, and automated label generation.

## Overview

The shipping module supports:

- **Direct Carrier Integrations**: DHL, FedEx, UPS, USPS
- **Third-Party Aggregators**: EasyPost, ShipStation
- **Weight-Based Calculations**: Actual and volumetric weight
- **Shipping Zones**: Geographic rate management
- **Rules Engine**: Conditional shipping logic
- **Multi-Carrier Rate Shopping**: Compare rates across providers

## Supported Carriers

### Direct Carriers

| Carrier | Services | International |
|---------|----------|---------------|
| DHL Express | EXPRESS_WORLDWIDE, EXPRESS_12:00, ECONOMY_SELECT | Yes |
| FedEx | Ground, 2Day, Overnight, International Priority | Yes |
| UPS | Ground, 2nd Day Air, Next Day Air, Worldwide Saver | Yes |
| USPS | Ground Advantage, Priority Mail, Priority Express | Limited |

### Aggregators

| Provider | Features |
|----------|----------|
| EasyPost | 100+ carriers, address verification, insurance |
| ShipStation | Order management, batch processing, inventory sync |

## Configuration

Add shipping configuration to your `config.toml`:

```toml
[shipping]
default_provider = "ups"

[shipping.ups]
api_key = "your_api_key"
username = "your_username"
password = "your_password"
account_number = "your_account"
test_mode = false

[shipping.fedex]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = false

[shipping.dhl]
api_key = "your_api_key"
api_secret = "your_secret"
account_number = "your_account"
test_mode = false

[shipping.easypost]
api_key = "your_api_key"
test_mode = false
```

## Weight Calculations

### Volumetric Weight

Carriers charge based on the greater of actual weight or volumetric (dimensional) weight:

**Formula**: `(Length × Width × Height) / Dimensional Factor`

| Provider | Factor (cm/kg) | Factor (in/lb) |
|----------|----------------|----------------|
| DHL | 5000 | - |
| FedEx | 5000 | 139 |
| UPS | 5000 | 139 |
| USPS | - | 166 |

### Example Calculation

```rust
use rcommerce_core::shipping::calculation::VolumetricWeightCalculator;

// Calculate volumetric weight
let calc = VolumetricWeightCalculator::standard_international();
let volumetric_weight = calc.calculate(
    Decimal::from(50),  // length cm
    Decimal::from(40),  // width cm
    Decimal::from(30),  // height cm
);

// volumetric_weight = (50 × 40 × 30) / 5000 = 12 kg
```

### Chargeable Weight

The chargeable weight is the maximum of actual and volumetric weight:

```rust
let calc = VolumetricWeightCalculator::standard_international();
let chargeable = calc.calculate_chargeable_weight(&package);

println!("Actual: {} kg", chargeable.actual_weight);
println!("Volumetric: {:?} kg", chargeable.volumetric_weight);
println!("Chargeable: {} kg", chargeable.chargeable_weight);
```

## Getting Shipping Rates

### Single Carrier

```rust
use rcommerce_core::shipping::{UpsProvider, RateOptions, Package};

let ups = UpsProvider::new(api_key, username, password, account);

let package = Package::new(Decimal::from(5), "kg")
    .with_dimensions(Decimal::from(30), Decimal::from(20), Decimal::from(15), "cm");

let options = RateOptions::default();

let rates = ups.get_rates(&from_address, &to_address, &package, &options).await?;

for rate in rates {
    println!("{}: ${} ({} days)", 
        rate.service_name, 
        rate.total_cost,
        rate.delivery_days.unwrap_or(0)
    );
}
```

### Multi-Carrier Rate Shopping

```rust
use rcommerce_core::shipping::{ShippingProviderFactory, ShippingRateAggregator};

let mut factory = ShippingProviderFactory::new();
factory.register(Box::new(ups));
factory.register(Box::new(fedex));
factory.register(Box::new(dhl));

let aggregator = ShippingRateAggregator::new(factory);

let rates = aggregator.get_all_rates(
    &from_address,
    &to_address,
    &package,
    &RateOptions::default(),
).await?;

// Rates sorted by total cost
```

## Shipping Zones

Define geographic zones with specific rates:

```rust
use rcommerce_core::shipping::zones::{ShippingZone, ZoneRate, ZoneCalculator};

// Create domestic zone
let domestic = ShippingZone::new("domestic", "United States")
    .with_country("US")
    .with_rate(
        ZoneRate::new("Standard Ground", Decimal::from(8), Decimal::from(1))
            .with_free_shipping_threshold(Decimal::from(100))
    );

// Create international zone
let international = ShippingZone::new("international", "Rest of World")
    .with_rate(
        ZoneRate::new("International", Decimal::from(35), Decimal::from(5))
    );

// Calculate shipping
let mut calculator = ZoneCalculator::new();
calculator.add_zone(domestic);
calculator.add_zone(international);

if let Some((cost, rate)) = calculator.calculate_shipping(&address, weight, subtotal) {
    println!("Shipping: ${} via {}", cost, rate.name);
}
```

## Shipping Rules

Create conditional shipping logic:

```rust
use rcommerce_core::shipping::rules::{ShippingRule, RuleCondition, RuleAction};

// Free shipping for orders over $100
let free_shipping = ShippingRule::new(
    "Free Shipping",
    RuleCondition::OrderTotal { 
        min: Some(Decimal::from(100)), 
        max: None 
    },
    RuleAction::FreeShipping,
).with_priority(100);

// 20% discount for express shipping
let express_discount = ShippingRule::new(
    "Express Discount",
    RuleCondition::ShippingMethod { 
        methods: vec!["express".to_string()] 
    },
    RuleAction::DiscountShipping { percentage: Decimal::from(20) },
);

// Apply rules
let mut engine = ShippingRuleEngine::new();
engine.add_rule(free_shipping);
engine.add_rule(express_discount);

engine.apply_rules(&order, &mut rates);
```

## Creating Shipments

```rust
let shipment = provider.create_shipment(
    &from_address,
    &to_address,
    &package,
    "PRIORITY_OVERNIGHT",  // service code
    Some(&customs_info),   // for international
).await?;

println!("Tracking: {}", shipment.tracking_number.unwrap());
println!("Label: {}", shipment.label_url.unwrap());
```

## Tracking Shipments

### Track by Number

```rust
let tracking = provider.track_shipment("1Z999AA10123456784").await?;

println!("Status: {}", tracking.status.description());
for event in &tracking.events {
    println!("{} - {} at {:?}", 
        event.timestamp, 
        event.description,
        event.location
    );
}
```

### Detect Carrier from Tracking Number

```rust
use rcommerce_core::shipping::carriers::detect_carrier_from_tracking;

let carrier = detect_carrier_from_tracking("1Z999AA10123456784");
// Returns: Some("ups")
```

## International Shipping

### Customs Information

```rust
use rcommerce_core::shipping::{CustomsInfo, CustomsItem, ContentsType, NonDeliveryOption};

let customs = CustomsInfo {
    contents_type: ContentsType::Merchandise,
    contents_description: "Electronic components".to_string(),
    non_delivery_option: NonDeliveryOption::Return,
    customs_items: vec![
        CustomsItem {
            description: "Circuit board".to_string(),
            quantity: 2,
            value: Decimal::from(50),
            currency: "USD".to_string(),
            weight: Some(Decimal::from(1)),
            weight_unit: Some("kg".to_string()),
            hs_tariff_number: Some("8517.62.00".to_string()),
            origin_country: "US".to_string(),
        }
    ],
    declaration_value: Decimal::from(100),
    declaration_currency: "USD".to_string(),
};
```

## Packaging

### Predefined Package Types

```rust
use rcommerce_core::shipping::packaging::PackageRegistry;

let registry = PackageRegistry::new();

// Get USPS flat rate boxes
let usps_boxes = registry.get_by_carrier("USPS");

// Find best flat rate for items
let calculator = PackagingCalculator::new();
if let Some(package_type) = calculator.find_best_flat_rate(&items, Some("USPS")) {
    println!("Use {} at ${}", 
        package_type.name,
        package_type.flat_rate.unwrap()
    );
}
```

## Testing

Use test mode during development:

```rust
let ups = UpsProvider::new(api_key, username, password, account)
    .with_test_mode(true);

// All API calls go to sandbox
// Labels are test labels (not valid for shipping)
```

## API Endpoints

### Get Shipping Rates

```http
POST /api/v1/shipping/rates
Content-Type: application/json

{
  "from_address": {
    "first_name": "John",
    "last_name": "Doe",
    "address1": "123 Main St",
    "city": "New York",
    "state": "NY",
    "zip": "10001",
    "country": "US"
  },
  "to_address": {
    "first_name": "Jane",
    "last_name": "Smith",
    "address1": "456 Oak Ave",
    "city": "Los Angeles",
    "state": "CA",
    "zip": "90210",
    "country": "US"
  },
  "package": {
    "weight": 5.0,
    "weight_unit": "kg",
    "length": 30,
    "width": 20,
    "height": 15,
    "dimension_unit": "cm"
  },
  "providers": ["ups", "fedex"]
}
```

### Create Shipment

```http
POST /api/v1/shipping/shipments
Content-Type: application/json

{
  "order_id": "550e8400-e29b-41d4-a716-446655440000",
  "provider": "ups",
  "service_code": "02",
  "package": {
    "weight": 5.0,
    "weight_unit": "kg"
  }
}
```

### Track Shipment

```http
GET /api/v1/shipping/tracking/1Z999AA10123456784
```

## Best Practices

1. **Cache Rates**: Cache shipping rates for common routes to reduce API calls
2. **Use Test Mode**: Always use test mode in development/staging
3. **Handle Errors**: Carriers may have outages; implement fallback logic
4. **Validate Addresses**: Use address validation before creating shipments
5. **Monitor Costs**: Track shipping costs by carrier and zone
6. **Volumetric Weight**: Always calculate volumetric weight for large/light packages

## Troubleshooting

### Common Issues

**Rate Not Found**
- Check address is valid and complete
- Verify package dimensions are within carrier limits
- Ensure service is available for the route

**Label Generation Failed**
- Verify account credentials
- Check balance/account status with carrier
- Ensure customs info is provided for international

**Tracking Not Updating**
- Some carriers have delays in tracking updates
- Verify tracking number format is correct
- Check carrier's tracking website directly
