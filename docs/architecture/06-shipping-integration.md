# Shipping Integration Architecture

## Overview

The shipping system provides a unified interface for generating shipping rates, creating shipping labels, and tracking packages across multiple carriers and regional providers. The system is designed to handle both international shipping giants (UPS, FedEx, DHL) and regional providers (including ERP systems like dianxiaomi).

**Key Design Goals:**
- Support multiple carriers with unified API
- Real-time rate calculation
- Automated label generation
- Shipment tracking aggregation
- Multi-location inventory support
- Customs documentation for international shipping

## Supported Providers

### Built-in Providers (Phase 1)
- **ShipStation** (Aggregator - supports 100+ carriers)
- **EasyPost** (Multi-carrier API)
- **Manual/Custom Rates**

### Direct Carrier Integrations (Phase 2)
- **UPS** - Domestic & International
- **FedEx** - Ground & Express
- **DHL** - International shipping
- **USPS** - US Postal Service
- **China Post** - For Chinese market

### ERP/Regional Systems
- **Dianxiaomi** (店小秘) - Chinese cross-border ERP
- **Efulfillment**
- **CJ Dropshipping**
- Custom enterprise ERP integrations

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     API Layer (ShippingController)           │
│  - Rate calculation endpoints                               │
│  - Label generation endpoints                               │
│  - Tracking endpoints                                       │
│  - Zone management                                          │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│              Shipping Orchestrator                          │
│  - Rate aggregation and comparison                          │
│  - Provider selection logic                                 │
│  - Shipment lifecycle management                            │
│  - Multi-package shipment coordination                      │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│              Shipping Zone & Rules Engine                   │
│  - Zone-based rate calculation                              │
│  - Conditional shipping rules                               │
│  - Free shipping thresholds                                 │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│              Provider Factory                               │
│  - Dynamic provider loading                                 │
│  - Provider registry                                        │
│  - Credential management                                    │
└────────────┬────────────────────────┬───────────────────────┘
             │                        │
    ┌────────▼──────────┐    ┌────────▼──────────────┐
    │   ShipStation     │    │      EasyPost         │
    │   Provider        │    │      Provider         │
    │                   │    │                       │
    │ - Rate shopping   │    │ - Multi-carrier API   │
    │ - Label creation  │    │ - Tracking            │
    │ - Tracking        │    │ - Insurance           │
    │ - Order sync      │    │ - Address verification│
    └────────┬──────────┘    └───────────────────────┘
             │
             │  ┌───────────▼────────────┐
             │  │   Dianxiaomi Provider  │
             │  │   (Regional ERP)       │
             │  │                        │
             │  │ - Order sync           │
             │  │ - Inventory management │
             │  │ - Label generation     │
             │  │ - Tracking sync        │
             │  └────────────────────────┘
             │
    ┌────────▼──────────▼─────────┐
    │   Carrier APIs (UPS, FedEx, DHL, USPS)                  │
    └─────────────────────────────────────────────────────────┘
```

## Core Data Models

### Shipment Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub shipment_number: String,
    pub provider: String,  // "shipstation", "easypost", "dianxiaomi"
    pub carrier: Option<String>,  // "ups", "fedex", "dhl"
    pub service_code: String,  // "ground", "2day", "overnight"
    pub service_name: String,
    pub status: ShipmentStatus,
    pub from_address: Address,
    pub to_address: Address,
    pub package: Package,
    pub line_items: Vec<ShipmentLineItem>,
    pub rates: Vec<ShippingRate>,  // All available rates
    pub selected_rate: Option<ShippingRate>,
    pub tracking_number: Option<String>,
    pub tracking_url: Option<String>,
    pub label_url: Option<String>,
    pub customs_info: Option<CustomsInfo>,
    pub insurance_amount: Option<Decimal>,
    pub total_cost: Option<Decimal>,
    pub currency: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub estimated_delivery: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "shipment_status", rename_all = "snake_case")]
pub enum ShipmentStatus {
    Pending,      // Created, awaiting label generation
    LabelCreated, // Shipping label generated
    InTransit,    // Package in transit
    OutForDelivery, // Out for delivery
    Delivered,    // Successfully delivered
    Failed,       // Delivery failed
    Cancelled,    // Shipment cancelled
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub weight: f64,
    pub weight_unit: String,  // "lb", "oz", "kg", "g"
    pub length: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub dimension_unit: Option<String>,  // "in", "cm"
    pub predefined_package: Option<String>, // "small_flat_rate_box"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRate {
    pub service_code: String,
    pub service_name: String,
    pub carrier: String,
    pub rate: Decimal,
    pub currency: String,
    pub delivery_days: Option<i32>,
    pub delivery_date: Option<DateTime<Utc>>,
    pub estimated: bool,
    pub insurance_fee: Option<Decimal>,
    pub confirmation_fee: Option<Decimal>,
    pub other_fees: Option<HashMap<String, Decimal>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsInfo {
    pub contents_type: String,  // "merchandise", "gift", "documents"
    pub contents_description: String,
    pub non_delivery_option: String,  // "return", "abandon"
    pub restriction_type: Option<String>,
    pub restriction_comments: Option<String>,
    pub customs_items: Vec<CustomsItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsItem {
    pub description: String,
    pub quantity: i32,
    pub value: Decimal,
    pub currency: String,
    pub weight: Option<f64>,
    pub weight_unit: Option<String>,
    pub hs_tariff_number: Option<String>,
    pub origin_country: Option<String>,
}
```

### Provider Trait

```rust
#[async_trait]
pub trait ShippingProvider: Send + Sync + 'static {
    /// Provider identifier (e.g., "shipstation", "easypost")
    fn id(&self) -> &'static str;
    
    /// Provider display name
    fn name(&self) -> &'static str;
    
    /// Supported regions/countries
    fn supported_regions(&self) -> Vec<&str>;
    
    /// Get available shipping rates
    async fn get_rates(
        &self,
        from_address: &Address,
        to_address: &Address,
        package: &Package,
        options: &RateOptions,
    ) -> Result<Vec<ShippingRate>>;
    
    /// Create a shipment and generate label
    async fn create_shipment(
        &self,
        order: &Order,
        from_address: &Address,
        to_address: &Address,
        package: &Package,
        service_code: &str,
        customs_info: Option<&CustomsInfo>,
    ) -> Result<Shipment>;
    
    /// Track a shipment
    async fn track_shipment(&self, tracking_number: &str) -> Result<TrackingInfo>;
    
    /// Cancel a shipment (if possible)
    async fn cancel_shipment(&self, shipment_id: &str) -> Result<bool>;
    
    /// Handle webhook/notification from provider
    async fn handle_webhook(
        &self,
        payload: &[u8],
        signature: Option<&str>,
    ) -> Result<WebhookEvent>;
    
    /// Verify webhook signature
    fn verify_webhook_signature(
        &self,
        payload: &[u8],
        signature: &str,
        secret: &str,
    ) -> Result<bool>;
    
    /// Get available carrier services
    async fn get_services(&self) -> Result<Vec<ShippingService>>;
    
    /// Validate shipping address
    async fn validate_address(&self, address: &Address) -> Result<AddressValidation>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateOptions {
    pub carriers: Option<Vec<String>>,  // Filter by carrier
    pub services: Option<Vec<String>>,  // Filter by service level
    pub include_insurance: bool,
    pub signature_confirmation: bool,
    pub saturday_delivery: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingInfo {
    pub tracking_number: String,
    pub status: TrackingStatus,
    pub carrier: String,
    pub events: Vec<TrackingEvent>,
    pub estimated_delivery: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingEvent {
    pub timestamp: DateTime<Utc>,
    pub status: TrackingStatus,
    pub description: String,
    pub location: Option<Address>,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "tracking_status", rename_all = "snake_case")]
pub enum TrackingStatus {
    PreTransit,
    InTransit,
    OutForDelivery,
    Delivered,
    AvailableForPickup,
    ReturnToSender,
    Failure,
    Cancelled,
    Error,
}
```

## ShipStation Provider Implementation

```rust
pub struct ShipStationProvider {
    client: reqwest::Client,
    api_key: String,
    api_secret: String,
    webhook_secret: String,
}

#[async_trait]
impl ShippingProvider for ShipStationProvider {
    fn id(&self) -> &'static str { "shipstation" }
    
    fn name(&self) -> &'static str { "ShipStation" }
    
    async fn get_rates(
        &self,
        from_address: &Address,
        to_address: &Address,
        package: &Package,
        _options: &RateOptions,
    ) -> Result<Vec<ShippingRate>> {
        let body = serde_json::json!({
            "carrierCode": null,  // Get rates from all carriers
            "serviceCode": null,
            "packageCode": package.predefined_package.as_deref(),
            "fromPostalCode": from_address.postal_code,
            "toState": to_address.state,
            "toCountry": to_address.country,
            "toPostalCode": to_address.postal_code,
            "toCity": to_address.city,
            "weight": {
                "value": package.weight,
                "units": package.weight_unit.to_uppercase(),
            },
            "dimensions": package.length.map(|length| serde_json::json!({
                "units": package.dimension_unit.as_deref().unwrap_or("inches").to_uppercase(),
                "length": length,
                "width": package.width,
                "height": package.height,
            })),
            "confirmation": null,
            "residential": to_address.residential,
        });
        
        let response = self.client
            .post("https://ssapi.shipstation.com/shipments/getrates")
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .json(&body)
            .send()
            .await?;
        
        let rates: Vec<ShipStationRate> = response.json().await?;
        
        Ok(rates.into_iter().map(|rate| ShippingRate {
            service_code: rate.service_code,
            service_name: rate.service_name,
            carrier: rate.carrier_code,
            rate: Decimal::from_str(&rate.shipment_cost.to_string()).unwrap(),
            currency: "USD".to_string(),
            delivery_days: rate.delivery_days,
            delivery_date: None,
            estimated: true,
            insurance_fee: rate.insurance_cost.map(|c| Decimal::from_str(&c.to_string()).unwrap()),
            confirmation_fee: None,
            other_fees: None,
        }).collect())
    }
    
    async fn create_shipment(
        &self,
        order: &Order,
        from_address: &Address,
        to_address: &Address,
        package: &Package,
        service_code: &str,
        customs_info: Option<&CustomsInfo>,
    ) -> Result<Shipment> {
        let mut items = order.line_items.iter().map(|item| {
            serde_json::json!({
                "lineItemKey": item.id.to_string(),
                "sku": item.sku.as_deref(),
                "name": item.name,
                "quantity": item.quantity,
                "unitPrice": item.unit_price,
                "weight": {
                    "value": item.weight.unwrap_or(0.0),
                    "units": package.weight_unit,
                },
            })
        }).collect::<Vec<_>>();
        
        let mut body = serde_json::json!({
            "orderId": order.id.to_string(),
            "orderKey": order.order_number,
            "orderDate": order.created_at.to_rfc3339(),
            "orderNumber": order.order_number,
            "customerEmail": order.customer_email,
            "shipTo": self.address_to_shipstation(to_address),
            "shipFrom": self.address_to_shipstation(from_address),
            "packages": [{
                "weight": {
                    "value": package.weight,
                    "units": package.weight_unit.to_uppercase(),
                },
                "dimensions": package.length.map(|length| ({
                    "units": package.dimension_unit.as_deref().unwrap_or("inches"),
                    "length": length,
                    "width": package.width,
                    "height": package.height,
                })),
                "packageCode": package.predefined_package,
            }],
            "items": items,
            "serviceCode": service_code,
            "confirmation": "none",
            "testLabel": cfg!(debug_assertions),
        });
        
        // Add customs info if international
        if let Some(customs) = customs_info {
            let origin_country = from_address.country.clone();
            body["internationalOptions"] = serde_json::json!({
                "contents": customs.contents_type,
                "customsItems": customs.customs_items.iter().map(|item| serde_json::json!({
                    "description": item.description,
                    "quantity": item.quantity,
                    "value": item.value,
                    "harmonizedTariffCode": item.hs_tariff_number,
                    "countryOfOrigin": item.origin_country.as_ref().unwrap_or(&origin_country),
                })).collect::<Vec<_>>(),
            });
        }
        
        let response = self.client
            .post("https://ssapi.shipstation.com/orders/createlabelfororder")
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .json(&body)
            .send()
            .await?;
        
        let label_response: ShipStationLabelResponse = response.json().await?;
        
        // Parse shipment from response
        let tracking_number = label_response.tracking_number.ok_or_else(|| 
            Error::MissingTrackingNumber
        )?;
        
        let label_url = label_response.label_data.map(|data| 
            format!("data:application/pdf;base64,{})", data)
        );
        
        Ok(Shipment {
            id: Uuid::new_v4(),
            order_id: order.id,
            shipment_number: format!("SHIP-{}", order.order_number),
            provider: self.id().to_string(),
            carrier: label_response.carrier_code,
            service_code: service_code.to_string(),
            service_name: label_response.service_code,
            status: ShipmentStatus::LabelCreated,
            from_address: from_address.clone(),
            to_address: to_address.clone(),
            package: package.clone(),
            line_items: order.line_items.iter().map(|item| ShipmentLineItem {
                order_item_id: item.id,
                quantity: item.quantity,
            }).collect(),
            rates: vec![],
            selected_rate: None,
            tracking_number: Some(tracking_number),
            tracking_url: Some(format!("https://tools.usps.com/go/TrackConfirmAction?q={}", tracking_number)),
            label_url,
            customs_info: customs_info.cloned(),
            insurance_amount: None,
            total_cost: label_response.shipment_cost.map(|c| Decimal::from_str(&c.to_string()).unwrap()),
            currency: "USD".to_string(),
            metadata: json!({ "shipstation_order_id": label_response.order_id }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            shipped_at: None,
            estimated_delivery: None,
        })
    }
    
    async fn track_shipment(&self, tracking_number: &str) -> Result<TrackingInfo> {
        // Try to determine carrier from tracking number pattern
        let carrier = self.detect_carrier_from_tracking(tracking_number);
        
        let response = self.client
            .get("https://ssapi.shipstation.com/shipments")
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .query(&[("trackingNumber", tracking_number)])
            .send()
            .await?;
        
        let shipments: Vec<ShipStationShipment> = response.json().await?;
        let shipment = shipments.into_iter().next()
            .ok_or_else(|| Error::TrackingNotFound(tracking_number.to_string()))?;
        
        Ok(TrackingInfo {
            tracking_number: tracking_number.to_string(),
            status: self.parse_tracking_status(&shipment.shipment_status),
            carrier: shipment.carrier_code,
            events: shipment.tracking_events.map(|events| 
                events.into_iter().map(|e| TrackingEvent {
                    timestamp: e.timestamp,
                    status: self.parse_tracking_status(&e.status),
                    description: e.description,
                    location: e.location,
                }).collect()
            ).unwrap_or_default(),
            estimated_delivery: shipment.estimated_delivery_date,
        })
    }
    
    fn detect_carrier_from_tracking(&self, tracking: &str) -> &str {
        // Simple pattern matching for common carriers
        if tracking.len() == 22 && tracking.chars().all(|c| c.is_ascii_alphanumeric()) {
            "ups"
        } else if tracking.len() == 20 || tracking.len() == 22 {
            "fedex"
        } else if tracking.len() == 30 {
            "dhl"
        } else if tracking.starts_with("1Z") {
            "ups"
        } else {
            "usps"
        }
    }
    
    fn address_to_shipstation(&self, addr: &Address) -> serde_json::Value {
        serde_json::json!({
            "name": addr.name,
            "company": addr.company,
            "street1": addr.street1,
            "street2": addr.street2,
            "city": addr.city,
            "state": addr.state,
            "postalCode": addr.postal_code,
            "country": addr.country,
            "phone": addr.phone,
            "residential": addr.residential,
        })
    }
}
```

## Dianxiaomi Provider (Multi-Carrier ERP)

```rust
pub struct DianxiaomiProvider {
    client: reqwest::Client,
    app_key: String,
    app_secret: String,
    access_token: String,
    base_url: String,  // https://erp.dianxiaomi.com
}

#[async_trait]
impl ShippingProvider for DianxiaomiProvider {
    fn id(&self) -> &'static str { "dianxiaomi" }
    
    fn name(&self) -> &'static str { "Dianxiaomi ERP" }
    
    // Dianxiaomi is primarily an ERP that syncs orders and generates labels
    // rather than a rate-shopping API like ShipStation
    
    async fn create_shipment(
        &self,
        order: &Order,
        from_address: &Address,
        to_address: &Address,
        package: &Package,
        service_code: &str,
        customs_info: Option<&CustomsInfo>,
    ) -> Result<Shipment> {
        // First, sync order to Dianxiaomi if not already synced
        let erp_order_id = self.sync_order_to_erp(order, from_address, to_address).await?;
        
        // Get available shipping methods for this order
        let shipping_methods = self.get_shipping_methods(erp_order_id).await?;
        
        // Select the requested method or choose default
        let selected_method = shipping_methods.iter()
            .find(|m| m.service_code == service_code)
            .ok_or_else(|| Error::ShippingMethodNotFound(service_code.to_string()))?;
        
        // Generate shipping label
        let label = self.generate_label(erp_order_id, selected_method.id, package).await?;
        
        // Parse tracking info from label response
        let tracking_info = self.parse_label_tracking(&label)?;
        
        Ok(Shipment {
            id: Uuid::new_v4(),
            order_id: order.id,
            shipment_number: format!("SHIP-CN-{}", order.order_number),
            provider: self.id().to_string(),
            carrier: label.carrier,
            service_code: service_code.to_string(),
            service_name: selected_method.name,
            status: ShipmentStatus::LabelCreated,
            from_address: from_address.clone(),
            to_address: to_address.clone(),
            package: package.clone(),
            line_items: order.line_items.iter().map(|item| ShipmentLineItem {
                order_item_id: item.id,
                quantity: item.quantity,
            }).collect(),
            rates: vec![]. // rates not available before label generation in Dianxiaomi
            selected_rate: None,
            tracking_number: Some(tracking_info.number),
            tracking_url: tracking_info.url,
            label_url: label.file_url.map(|u| u.to_string()),
            customs_info: customs_info.cloned(),
            insurance_amount: label.insurance_amount,
            total_cost: label.total_cost,
            currency: "CNY".to_string(),
            metadata: json!({
                "erp_order_id": erp_order_id,
                "carrier_account": label.carrier_account,
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            shipped_at: None,
            estimated_delivery: label.estimated_delivery,
        })
    }
    
    async fn sync_order_to_erp(
        &self,
        order: &Order,
        from_address: &Address,
        to_address: &Address,
    ) -> Result<i64> {
        let params = self.build_order_params(order, from_address, to_address)?;
        
        let response = self.client
            .post(format!("{}/api/v1/order/refreshOrder", self.base_url))
            .header("X-DXM-AppKey", &self.app_key)
            .header("X-DXM-AccessToken", &self.access_token)
            .form(&params)
            .send()
            .await?;
        
        let result: DianxiaomiResponse<i64> = response.json().await?;
        
        match result {
            DianxiaomiResponse::Success { data, .. } => Ok(data),
            DianxiaomiResponse::Error { code, message } => Err(
                Error::DianxiaomiError { code, message }
            ),
        }
    }
    
    fn build_order_params(
        &self,
        order: &Order,
        from_address: &Address,
        to_address: &Address,
    ) -> Result<HashMap<String, String>> {
        let mut params = HashMap::new();
        params.insert("platform".to_string(), "custom".to_string());
        params.insert("orderNum".to_string(), order.order_number.clone());
        params.insert("expressType".to_string(), "4px".to_string()); // default carrier
        
           //收件人信息 (recipient info)
        params.insert("consignee".to_string(), to_address.name.clone());
        params.insert("countryCode".to_string(), to_address.country.clone());
        params.insert("state".to_string(), to_address.state.clone());
        params.insert("city".to_string(), to_address.city.clone());
        params.insert("postcode".to_string(), to_address.postal_code.clone());
        params.insert("address1".to_string(), to_address.street1.clone());
        params.insert("address2".to_string(), to_address.street2.clone().unwrap_or_default());
        params.insert("phone".to_string(), to_address.phone.clone().unwrap_or_default());
        
           //物品信息 (item info)
        for (idx, item) in order.line_items.iter().enumerate() {
            params.insert(format!("itemName{}", idx + 1), item.name.clone());
            params.insert(format!("itemSku{}", idx + 1), item.sku.clone().unwrap_or_default());
            params.insert(format!("itemNum{}", idx + 1), item.quantity.to_string());
            params.insert(format!("price{}", idx + 1), item.unit_price.to_string());
            params.insert(format!("weight{}", idx + 1), 
                item.weight.map(|w| w.to_string()).unwrap_or_else(|| "0.1".to_string())
            );
        }
        
        Ok(params)
    }
}
```

## Shipping Rate Calculation Engine

```rust
pub struct ShippingRateEngine {
    warehouse_service: Arc<dyn WarehouseService>,
    provider_factory: Arc<ShippingProviderFactory>,
}

impl ShippingRateEngine {
    pub async fn calculate_rates(
        &self,
        order: &Order,
        to_address: &Address,
        options: &RateOptions,
    ) -> Result<Vec<ShippingRate>> {
        // 1. Determine shipping origin(s) based on inventory
        let inventory_locations = self.warehouse_service
            .get_fulfillment_locations(order)
            .await?;
        
        // 2. Calculate rates from each location
        let mut all_rates = Vec::new();
        let provider = self.provider_factory.get_default()?;
        
        for location in &inventory_locations {
            let from_address = &location.address;
            let package = self.calculate_package(order, location)?;
            
            let rates = provider.get_rates(
                from_address,
                to_address,
                &package,
                options,
            ).await?;
            
            // Adjust rates for multi-package if needed
            let adjusted_rates = rates.into_iter()
                .map(|mut rate| {
                    // Add warehouse handling fees
                    rate.rate += location.shipping_fees;
                    rate
                })
                .collect::<Vec<_>>();
            
            all_rates.extend(adjusted_rates);
        }
        
        // 3. Apply sorting and deduplication
        all_rates.sort_by(|a, b| a.rate.cmp(&b.rate));
        
        Ok(all_rates)
    }
    
    fn calculate_package(&self, order: &Order, location: &WarehouseLocation) -> Result<Package> {
        let mut total_weight = 0.0;
        let mut max_length = 0.0;
        let mut max_width = 0.0;
        let mut max_height = 0.0;
        
        for item in &order.line_items {
            total_weight += item.weight.ok_or(Error::MissingWeight) * item.quantity;
            max_length = max_length.max(item.length.unwrap_or(0.0));
            max_width = max_width.max(item.width.unwrap_or(0.0));
            max_height = max_height.max(item.height.unwrap_or(0.0));
        }
        
        Ok(Package {
            weight: total_weight,
            weight_unit: location.weight_unit.clone(),
            length: Some(max_length),
            width: Some(max_width),
            height: Some(max_height),
            dimension_unit: Some(location.dimension_unit.clone()),
            predefined_package: None,
        })
    }
}
```

## Shipping Rules Engine

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRule {
    pub condition: RuleCondition,
    pub action: RuleAction,
    pub priority: i32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    OrderTotal { min: Option<Decimal>, max: Option<Decimal> },
    OrderWeight { min: Option<f64>, max: Option<f64> },
    DestinationCountry { countries: Vec<String> },
    ProductCategory { categories: Vec<String> },
    CustomerGroup { groups: Vec<String> },
    ShippingMethod { methods: Vec<String> },
    CartContains { skus: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    FreeShipping,
    DiscountShipping { percentage: Decimal },
    Surcharge { amount: Decimal },
    HideMethods { methods: Vec<String> },
    RequireSignature,
    RequireInsurance { amount: Decimal },
}

pub struct ShippingRuleEngine {
    rules: Vec<ShippingRule>,
}

impl ShippingRuleEngine {
    pub fn apply_rules(&self, order: &Order, rates: &mut Vec<ShippingRate>) {
        // Sort rules by priority (highest first)
        let mut sorted_rules = self.rules.iter()
            .filter(|r| r.enabled)
            .collect::<Vec<_>>();
        sorted_rules.sort_by_key(|r| -r.priority);
        
        for rule in sorted_rules {
            if self.evaluate_condition(&rule.condition, order) {
                self.apply_action(&rule.action, rates);
            }
        }
    }
    
    fn evaluate_condition(&self, condition: &RuleCondition, order: &Order) -> bool {
        match condition {
            RuleCondition::OrderTotal { min, max } => {
                let total = order.total;
                min.map(|m| total >= m).unwrap_or(true) && 
                max.map(|m| total <= m).unwrap_or(true)
            }
            RuleCondition::DestinationCountry { countries } => {
                countries.contains(&order.shipping_address.country)
            }
            // ... other conditions
        }
    }
    
    fn apply_action(&self, action: &RuleAction, rates: &mut Vec<ShippingRate>) {
        match action {
            RuleAction::FreeShipping => {
                for rate in rates {
                    rate.rate = Decimal::ZERO;
                }
            }
            RuleAction::DiscountShipping { percentage } => {
                let multiplier = (Decimal::ONE - percentage);
                for rate in rates {
                    rate.rate *= multiplier;
                }
            }
            RuleAction::HideMethods { methods } => {
                rates.retain(|r| !methods.contains(&r.service_code));
            }
        }
    }
}
```

## Webhook Handling for Tracking Updates

```rust
pub struct TrackingWebhookHandler {
    provider_factory: Arc<ShippingProviderFactory>,
    shipment_service: Arc<dyn ShipmentService>,
    event_dispatcher: Arc<dyn EventDispatcher>,
}

impl TrackingWebhookHandler {
    pub async fn handle_webhook(
        &self,
        provider_id: &str,
        payload: &[u8],
        signature: Option<&str>,
    ) -> Result<()> {
        let provider = self.provider_factory.get(provider_id)?;
        
        // Verify webhook signature
        if let Some(sig) = signature {
            let secret = self.get_webhook_secret(provider_id)?;
            if !provider.verify_webhook_signature(payload, sig, &secret)? {
                return Err(Error::InvalidWebhookSignature);
            }
        }
        
        // Process webhook event
        let event = provider.handle_webhook(payload, signature).await?;
        self.process_tracking_event(event).await?;
        
        Ok(())
    }
    
    async fn process_tracking_event(&self, event: WebhookEvent) -> Result<()> {
        match event {
            WebhookEvent::TrackingUpdated {
                tracking_number,
                status,
                events,
                estimated_delivery,
            } => {
                // Find shipment by tracking number
                let shipment = self.shipment_service
                    .find_by_tracking_number(&tracking_number)
                    .await?
                    .ok_or_else(|| Error::ShipmentNotFoundByTracking(tracking_number.clone()))?;
                
                // Update shipment status
                let mut shipment = shipment;
                shipment.status = status;
                shipment.updated_at = Utc::now();
                
                if let Some(ed) = estimated_delivery {
                    shipment.estimated_delivery = Some(ed);
                }
                
                self.shipment_service.update(shipment.clone()).await?;
                
                // Dispatch events based on status
                match status {
                    ShipmentStatus::InTransit => {
                        self.event_dispatcher.dispatch(
                            Event::ShipmentInTransit {
                                order_id: shipment.order_id,
                                shipment_id: shipment.id,
                                tracking_number: tracking_number.clone(),
                            }
                        ).await?;
                    }
                    ShipmentStatus::OutForDelivery => {
                        self.event_dispatcher.dispatch(
                            Event::ShipmentOutForDelivery {
                                order_id: shipment.order_id,
                                shipment_id: shipment.id,
                                tracking_number: tracking_number.clone(),
                            }
                        ).await?;
                    }
                    ShipmentStatus::Delivered => {
                        self.event_dispatcher.dispatch(
                            Event::ShipmentDelivered {
                                order_id: shipment.order_id,
                                shipment_id: shipment.id,
                                tracking_number: tracking_number.clone(),
                            }
                        ).await?;
                        
                        // Update order status
                        self.order_service.update_shipment_status(
                            shipment.order_id,
                            ShipmentStatus::Delivered,
                        ).await?;
                    }
                    ShipmentStatus::Failed => {
                        self.event_dispatcher.dispatch(
                            Event::ShipmentFailed {
                                order_id: shipment.order_id,
                                shipment_id: shipment.id,
                                tracking_number: tracking_number.clone(),
                            }
                        ).await?;
                    }
                    _ => {}
                }
            }
        }
        
        Ok(())
    }
}
```

---

Next: [07-order-management.md](07-order-management.md) - Order management system details
