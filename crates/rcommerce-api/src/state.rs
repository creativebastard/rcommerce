use std::sync::Arc;

use rcommerce_core::cache::RedisPool;
use rcommerce_core::payment::agnostic::PaymentService;
use rcommerce_core::repository::{Database, PostgresApiKeyRepository, PostgresSubscriptionRepository};
use rcommerce_core::services::{AuthService, CustomerService, ProductService, SubscriptionService, CouponService};
use rcommerce_core::{DigitalProductService, BundleService, FileUploadService};

use crate::middleware::AuthRateLimiter;

/// Parameters for creating AppState
pub struct AppStateParams {
    pub product_service: ProductService,
    pub customer_service: CustomerService,
    pub auth_service: AuthService,
    pub db: Database,
    pub redis: Option<RedisPool>,
    pub api_key_repository: PostgresApiKeyRepository,
    pub subscription_repository: PostgresSubscriptionRepository,
    pub coupon_service: CouponService,
    pub payment_service: PaymentService,
    pub file_upload_service: FileUploadService,
}

impl AppStateParams {
    /// Create a new AppStateParams with all required fields
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        product_service: ProductService,
        customer_service: CustomerService,
        auth_service: AuthService,
        db: Database,
        redis: Option<RedisPool>,
        api_key_repository: PostgresApiKeyRepository,
        subscription_repository: PostgresSubscriptionRepository,
        coupon_service: CouponService,
        payment_service: PaymentService,
        file_upload_service: FileUploadService,
    ) -> Self {
        Self {
            product_service,
            customer_service,
            auth_service,
            db,
            redis,
            api_key_repository,
            subscription_repository,
            coupon_service,
            payment_service,
            file_upload_service,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub product_service: ProductService,
    pub customer_service: CustomerService,
    pub auth_service: AuthService,
    pub subscription_service: SubscriptionService<PostgresSubscriptionRepository>,
    pub subscription_repository: Arc<PostgresSubscriptionRepository>,
    pub coupon_service: CouponService,
    pub payment_service: Arc<PaymentService>,
    pub digital_product_service: Arc<DigitalProductService>,
    pub bundle_service: Arc<BundleService>,
    pub file_upload_service: Arc<FileUploadService>,
    pub db: Database,
    pub redis: Option<RedisPool>,
    pub auth_rate_limiter: AuthRateLimiter,
    pub api_key_repository: Arc<PostgresApiKeyRepository>,
}

impl AppState {
    pub fn new(params: AppStateParams) -> Self {
        // Create auth rate limiter: 5 attempts per minute per IP
        let auth_rate_limiter = AuthRateLimiter::new(5, 60);
        
        // Create subscription service
        let subscription_service = SubscriptionService::new(params.subscription_repository.clone());
        
        // Create digital product service
        let file_upload_service = Arc::new(params.file_upload_service);
        let digital_product_service = Arc::new(DigitalProductService::new(
            params.db.clone(),
            file_upload_service.clone(),
        ));
        
        // Create bundle service
        let bundle_service = Arc::new(BundleService::new(params.db.clone()));
        
        Self {
            product_service: params.product_service,
            customer_service: params.customer_service,
            auth_service: params.auth_service,
            subscription_service,
            subscription_repository: Arc::new(params.subscription_repository),
            coupon_service: params.coupon_service,
            payment_service: Arc::new(params.payment_service),
            digital_product_service,
            bundle_service,
            file_upload_service,
            db: params.db,
            redis: params.redis,
            auth_rate_limiter,
            api_key_repository: Arc::new(params.api_key_repository),
        }
    }
}
