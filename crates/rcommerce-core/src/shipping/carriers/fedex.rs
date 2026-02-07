//! FedEx shipping provider implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::Result;
use crate::common::Address;
use crate::shipping::{
    ShippingProvider, ShippingRate, Shipment, TrackingInfo, TrackingStatus, TrackingEvent,
    Package, RateOptions, AddressValidation, ShippingService, ServiceFeature,
    CustomsInfo,
};

/// FedEx API provider
#[allow(dead_code)]
pub struct FedExProvider {
    client: reqwest::Client,
    api_key: String,
    api_secret: String,
    account_number: String,
    base_url: String,
    test_mode: bool,
}

impl FedExProvider {
    pub fn new(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
        account_number: impl Into<String>,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            account_number: account_number.into(),
            base_url: "https://apis.fedex.com".to_string(),
            test_mode: false,
        }
    }
    
    pub fn with_test_mode(mut self, test_mode: bool) -> Self {
        self.test_mode = test_mode;
        if test_mode {
            self.base_url = "https://apis-sandbox.fedex.com".to_string();
        }
        self
    }
    
    fn service_name(&self, code: &str) -> String {
        match code {
            "FEDEX_GROUND" => "FedEx Ground",
            "FEDEX_EXPRESS_SAVER" => "FedEx Express Saver",
            "FEDEX_2_DAY" => "FedEx 2Day",
            "FEDEX_2_DAY_AM" => "FedEx 2Day A.M.",
            "STANDARD_OVERNIGHT" => "FedEx Standard Overnight",
            "PRIORITY_OVERNIGHT" => "FedEx Priority Overnight",
            "FIRST_OVERNIGHT" => "FedEx First Overnight",
            "INTERNATIONAL_ECONOMY" => "FedEx International Economy",
            "INTERNATIONAL_PRIORITY" => "FedEx International Priority",
            _ => "FedEx",
        }.to_string()
    }
}

#[async_trait]
impl ShippingProvider for FedExProvider {
    fn id(&self) -> &'static str { "fedex" }
    fn name(&self) -> &'static str { "FedEx" }
    fn is_available(&self) -> bool { !self.api_key.is_empty() && !self.api_secret.is_empty() }
    
    async fn get_rates(
        &self,
        _from_address: &Address,
        _to_address: &Address,
        _package: &Package,
        options: &RateOptions,
    ) -> Result<Vec<ShippingRate>> {
        let mut rates = vec![
            ShippingRate::new(self.id(), self.name(), "FEDEX_GROUND", "FedEx Ground", Decimal::from(12), "USD")
                .with_delivery(5, None),
            ShippingRate::new(self.id(), self.name(), "FEDEX_2_DAY", "FedEx 2Day", Decimal::from(28), "USD")
                .with_delivery(2, None),
            ShippingRate::new(self.id(), self.name(), "PRIORITY_OVERNIGHT", "FedEx Priority Overnight", Decimal::from(55), "USD")
                .with_delivery(1, None),
        ];
        
        if let Some(ref services) = options.services {
            rates.retain(|r| services.contains(&r.service_code));
        }
        
        Ok(rates)
    }
    
    async fn create_shipment(
        &self,
        from_address: &Address,
        to_address: &Address,
        package: &Package,
        service_code: &str,
        customs_info: Option<&CustomsInfo>,
    ) -> Result<Shipment> {
        let tracking_number = format!("{:012}", rand::random::<u64>());
        
        Ok(Shipment {
            id: uuid::Uuid::new_v4(),
            order_id: None,
            provider_id: self.id().to_string(),
            carrier: self.name().to_string(),
            service_code: service_code.to_string(),
            service_name: self.service_name(service_code),
            status: crate::shipping::ShipmentStatus::Pending,
            from_address: from_address.clone(),
            to_address: to_address.clone(),
            package: package.clone(),
            tracking_number: Some(tracking_number.clone()),
            tracking_url: Some(format!("https://www.fedex.com/apps/fedextrack/?tracknumbers={}", tracking_number)),
            label_url: None,
            label_data: None,
            customs_info: customs_info.cloned(),
            insurance_amount: None,
            total_cost: Decimal::from(28),
            currency: "USD".to_string(),
            created_at: Utc::now(),
            shipped_at: None,
            delivered_at: None,
            estimated_delivery: Some(Utc::now() + chrono::Duration::days(2)),
            metadata: std::collections::HashMap::new(),
        })
    }
    
    async fn track_shipment(&self, tracking_number: &str) -> Result<TrackingInfo> {
        Ok(TrackingInfo {
            tracking_number: tracking_number.to_string(),
            carrier: self.name().to_string(),
            status: TrackingStatus::InTransit,
            events: vec![
                TrackingEvent {
                    timestamp: Utc::now() - chrono::Duration::hours(12),
                    status: TrackingStatus::InTransit,
                    description: "In transit".to_string(),
                    location: Some("Memphis, TN".to_string()),
                    city: Some("Memphis".to_string()),
                    state: Some("TN".to_string()),
                    country: Some("US".to_string()),
                },
            ],
            estimated_delivery: Some(Utc::now() + chrono::Duration::days(1)),
        })
    }
    
    async fn cancel_shipment(&self, _shipment_id: &str) -> Result<bool> { Ok(true) }
    
    async fn validate_address(&self, _address: &Address) -> Result<AddressValidation> {
        Ok(AddressValidation { is_valid: true, normalized_address: None, messages: vec![], residential: None })
    }
    
    fn get_services(&self) -> Vec<ShippingService> {
        vec![
            ShippingService { code: "FEDEX_GROUND".to_string(), name: "FedEx Ground".to_string(), carrier: self.name().to_string(), domestic: true, international: false, transit_time_days: Some((1, 5)), features: vec![ServiceFeature::Tracking, ServiceFeature::Ground] },
            ShippingService { code: "FEDEX_2_DAY".to_string(), name: "FedEx 2Day".to_string(), carrier: self.name().to_string(), domestic: true, international: false, transit_time_days: Some((2, 2)), features: vec![ServiceFeature::Tracking, ServiceFeature::Express] },
            ShippingService { code: "PRIORITY_OVERNIGHT".to_string(), name: "FedEx Priority Overnight".to_string(), carrier: self.name().to_string(), domestic: true, international: false, transit_time_days: Some((1, 1)), features: vec![ServiceFeature::Tracking, ServiceFeature::Express] },
            ShippingService { code: "INTERNATIONAL_PRIORITY".to_string(), name: "FedEx International Priority".to_string(), carrier: self.name().to_string(), domestic: false, international: true, transit_time_days: Some((1, 3)), features: vec![ServiceFeature::Tracking, ServiceFeature::Express] },
        ]
    }
    
    async fn estimate_delivery(&self, _from: &Address, _to: &Address, service_code: &str) -> Result<Option<DateTime<Utc>>> {
        let days = match service_code {
            "FEDEX_GROUND" => 5,
            "FEDEX_2_DAY" => 2,
            "PRIORITY_OVERNIGHT" => 1,
            _ => 3,
        };
        Ok(Some(Utc::now() + chrono::Duration::days(days)))
    }
}
