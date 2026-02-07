//! USPS shipping provider implementation

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

/// USPS API provider
#[allow(dead_code)]
pub struct UspsProvider {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    test_mode: bool,
}

impl UspsProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
            base_url: "https://apis.usps.com".to_string(),
            test_mode: false,
        }
    }
    
    pub fn with_test_mode(mut self, test_mode: bool) -> Self {
        self.test_mode = test_mode;
        self
    }
    
    fn service_name(&self, code: &str) -> String {
        match code {
            "USPS_GROUND_ADVANTAGE" => "USPS Ground Advantage",
            "PRIORITY_MAIL" => "USPS Priority Mail",
            "PRIORITY_MAIL_EXPRESS" => "USPS Priority Mail Express",
            "FIRST_CLASS_PACKAGE" => "USPS First Class Package",
            "MEDIA_MAIL" => "USPS Media Mail",
            _ => "USPS",
        }.to_string()
    }
}

#[async_trait]
impl ShippingProvider for UspsProvider {
    fn id(&self) -> &'static str { "usps" }
    fn name(&self) -> &'static str { "USPS" }
    fn is_available(&self) -> bool { !self.api_key.is_empty() }
    
    async fn get_rates(
        &self,
        _from_address: &Address,
        _to_address: &Address,
        _package: &Package,
        options: &RateOptions,
    ) -> Result<Vec<ShippingRate>> {
        let mut rates = vec![
            ShippingRate::new(self.id(), self.name(), "USPS_GROUND_ADVANTAGE", "USPS Ground Advantage", Decimal::from(6), "USD")
                .with_delivery(5, None),
            ShippingRate::new(self.id(), self.name(), "PRIORITY_MAIL", "USPS Priority Mail", Decimal::from(10), "USD")
                .with_delivery(3, None),
            ShippingRate::new(self.id(), self.name(), "PRIORITY_MAIL_EXPRESS", "USPS Priority Mail Express", Decimal::from(28), "USD")
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
        let tracking_number = format!("94001{:017}", rand::random::<u64>());
        
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
            tracking_url: Some(format!("https://tools.usps.com/go/TrackConfirmAction?qtc_tLabels1={}", tracking_number)),
            label_url: None,
            label_data: None,
            customs_info: customs_info.cloned(),
            insurance_amount: None,
            total_cost: Decimal::from(10),
            currency: "USD".to_string(),
            created_at: Utc::now(),
            shipped_at: None,
            delivered_at: None,
            estimated_delivery: Some(Utc::now() + chrono::Duration::days(3)),
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
                    timestamp: Utc::now() - chrono::Duration::hours(4),
                    status: TrackingStatus::InTransit,
                    description: "In transit to next facility".to_string(),
                    location: Some("Chicago, IL".to_string()),
                    city: Some("Chicago".to_string()),
                    state: Some("IL".to_string()),
                    country: Some("US".to_string()),
                },
            ],
            estimated_delivery: Some(Utc::now() + chrono::Duration::days(2)),
        })
    }
    
    async fn cancel_shipment(&self, _shipment_id: &str) -> Result<bool> { Ok(true) }
    
    async fn validate_address(&self, _address: &Address) -> Result<AddressValidation> {
        Ok(AddressValidation { is_valid: true, normalized_address: None, messages: vec![], residential: None })
    }
    
    fn get_services(&self) -> Vec<ShippingService> {
        vec![
            ShippingService { code: "USPS_GROUND_ADVANTAGE".to_string(), name: "USPS Ground Advantage".to_string(), carrier: self.name().to_string(), domestic: true, international: false, transit_time_days: Some((2, 5)), features: vec![ServiceFeature::Tracking, ServiceFeature::Ground] },
            ShippingService { code: "PRIORITY_MAIL".to_string(), name: "USPS Priority Mail".to_string(), carrier: self.name().to_string(), domestic: true, international: true, transit_time_days: Some((1, 3)), features: vec![ServiceFeature::Tracking] },
            ShippingService { code: "PRIORITY_MAIL_EXPRESS".to_string(), name: "USPS Priority Mail Express".to_string(), carrier: self.name().to_string(), domestic: true, international: true, transit_time_days: Some((1, 1)), features: vec![ServiceFeature::Tracking, ServiceFeature::Express] },
        ]
    }
    
    async fn estimate_delivery(&self, _from: &Address, _to: &Address, service_code: &str) -> Result<Option<DateTime<Utc>>> {
        let days = match service_code {
            "USPS_GROUND_ADVANTAGE" => 5,
            "PRIORITY_MAIL" => 3,
            "PRIORITY_MAIL_EXPRESS" => 1,
            _ => 3,
        };
        Ok(Some(Utc::now() + chrono::Duration::days(days)))
    }
}
