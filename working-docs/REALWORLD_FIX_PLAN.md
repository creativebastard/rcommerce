# R Commerce Real-World Testing Fix Plan

**Objective:** Make the system ready for real-world testing  
**Timeline:** 1-2 weeks  
**Priority:** Critical → High → Medium

---

## Phase 1: Critical Fixes (Week 1) - Blocking All Testing

### Task 1.1: Implement Real Cart API (3-4 days)

**Files to Modify:**
- `crates/rcommerce-api/src/routes/cart.rs`
- `crates/rcommerce-api/src/state.rs` (ensure CartService is available)

**Implementation Steps:**

1. **Add CartService to AppState**
```rust
// In state.rs
pub struct AppState {
    // ... existing fields
    pub cart_service: Arc<CartService>,  // Add this
}
```

2. **Rewrite `create_guest_cart`**
```rust
pub async fn create_guest_cart(
    State(state): State<AppState>,
) -> Result<Json<Cart>, Error> {
    let session_token = format!("sess_{}", Uuid::new_v4().to_string().replace("-", ""));
    
    let cart = state.cart_service.create_guest_cart(&session_token).await?;
    Ok(Json(cart))
}
```

3. **Rewrite `add_item_to_cart`**
```rust
pub async fn add_item_to_cart(
    State(state): State<AppState>,
    Path(cart_id): Path<Uuid>,
    Json(request): Json<AddItemRequest>,
) -> Result<Json<CartItem>, Error> {
    let input = AddToCartInput {
        product_id: request.product_id,
        variant_id: request.variant_id,
        quantity: request.quantity,
        custom_attributes: None,
    };
    
    let item = state.cart_service.add_item(cart_id, input).await?;
    Ok(Json(item))
}
```

4. **Update all other cart endpoints** to use `cart_service`

**Testing:**
```bash
curl -X POST http://localhost:8080/api/v1/carts/guest
curl -X POST http://localhost:8080/api/v1/carts/{cart_id}/items \
  -H "Content-Type: application/json" \
  -d '{"product_id": "...", "quantity": 2}'
curl http://localhost:8080/api/v1/carts/{cart_id}
```

---

### Task 1.2: Fix Notification Service Database Connection (1 day)

**Files to Modify:**
- `crates/rcommerce-core/src/notification/service.rs`
- `crates/rcommerce-core/src/notification/mod.rs`

**Implementation Steps:**

1. **Add PgPool to NotificationService**
```rust
pub struct NotificationService {
    email_channel: EmailChannel,
    sms_channel: SmsChannel,
    webhook_channel: WebhookChannel,
    db: PgPool,  // Add this
}

impl NotificationService {
    pub fn new(
        email_channel: EmailChannel,
        sms_channel: SmsChannel,
        webhook_channel: WebhookChannel,
        db: PgPool,  // Add parameter
    ) -> Self {
        Self {
            email_channel,
            sms_channel,
            webhook_channel,
            db,
        }
    }
    
    fn db(&self) -> &PgPool {
        &self.db  // Return actual pool
    }
}
```

2. **Update service initialization** in `server.rs` to pass the pool

**Verification:** Verify `queue()`, `cancel_queued()`, `get_history()` no longer panic

---

### Task 1.3: Fix Customer List/Get APIs (1 day)

**Files to Modify:**
- `crates/rcommerce-api/src/routes/customer.rs`

**Implementation Steps:**

1. **Rewrite `list_customers`**
```rust
pub async fn list_customers(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, Error> {
    let customers = state.customer_service.list_customers().await?;
    
    Ok(Json(serde_json::json!({
        "customers": customers,
        "meta": {
            "total": customers.len(),
            "page": 1,
            "per_page": 20,
        }
    })))
}
```

2. **Rewrite `get_customer`**
```rust
pub async fn get_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, Error> {
    let customer = state.customer_service.get_customer(id).await?;
    
    match customer {
        Some(c) => Ok(Json(serde_json::json!({"customer": c}))),
        None => Err(Error::not_found("Customer not found")),
    }
}
```

---

## Phase 2: High Priority Fixes (Week 1-2) - Data Integrity

### Task 2.1: Integrate Tax & Shipping into Order Creation (2 days)

**Files to Modify:**
- `crates/rcommerce-api/src/routes/order.rs`
- Potentially create `crates/rcommerce-api/src/routes/checkout.rs` for proper checkout flow

**Implementation Steps:**

1. **Create a CheckoutService in AppState** (if not already present)

2. **Create new checkout endpoint** that orchestrates properly:
```rust
pub async fn initiate_checkout(
    State(state): State<AppState>,
    Extension(auth): Extension<JwtAuth>,
    Json(request): Json<InitiateCheckoutRequest>,
) -> Result<Json<CheckoutSummary>, Error> {
    let summary = state.checkout_service.initiate_checkout(
        InitiateCheckoutRequest {
            cart_id: request.cart_id,
            shipping_address: request.shipping_address,
            billing_address: request.billing_address,
            vat_id: request.vat_id,
            customer_id: Some(auth.customer_id),
            currency: request.currency,
        }
    ).await?;
    
    Ok(Json(summary))
}
```

3. **Update order creation** to use real calculations or delegate to CheckoutService

---

### Task 2.2: Standardize Authentication Handling (1 day)

**Files to Modify:**
- `crates/rcommerce-api/src/routes/customer.rs`
- `crates/rcommerce-api/src/middleware/mod.rs` (ensure claims are properly extracted)

**Implementation Steps:**

1. **Update `get_current_customer`** to use Extension:
```rust
pub async fn get_current_customer(
    Extension(auth): Extension<JwtAuth>,  // Use middleware-provided auth
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, Error> {
    let customer = state.customer_service.get_customer(auth.customer_id).await?;
    // ... rest of handler
}
```

2. **Apply `auth_middleware` to customer routes** in route configuration

3. **Review all handlers** for manual auth extraction and convert to Extension pattern

---

## Phase 3: Medium Priority - Security Hardening (Week 2)

### Task 3.1: Restrict CORS Configuration (0.5 day)

**Files to Modify:**
- `crates/rcommerce-api/src/server.rs`
- `crates/rcommerce-core/src/config.rs`

**Implementation Steps:**

1. **Add CORS config to Config:**
```rust
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
    pub allow_credentials: bool,
}
```

2. **Update CORS layer:**
```rust
let cors = if config.cors.allowed_origins.contains(&"*".to_string()) {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
} else {
    let origins: Vec<_> = config.cors.allowed_origins
        .iter()
        .map(|o| o.as_str().parse::<HeaderValue>().unwrap())
        .collect();
    
    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
};
```

---

### Task 3.2: Add Security Headers (0.5 day)

**Files to Modify:**
- `crates/rcommerce-api/src/tls/mod.rs` (or create security middleware)

**Implementation Steps:**

1. **Create security headers middleware:**
```rust
pub async fn security_headers_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    
    response
}
```

2. **Apply middleware to all routes** in `build_router()`

---

### Task 3.3: Remove Sensitive Data from Responses (0.5 day)

**Files to Modify:**
- `crates/rcommerce-api/src/routes/auth.rs`

**Changes:**
- Remove `token` field from `PasswordResetResponse` in production
- Only return token in development mode

```rust
pub async fn request_password_reset(...) -> Result<Json<PasswordResetResponse>, Error> {
    // ... generate token
    
    let token = if cfg!(debug_assertions) {
        Some(reset_token)  // Only in dev
    } else {
        None  // Production
    };
    
    Ok(Json(PasswordResetResponse {
        message: "Password reset instructions sent".to_string(),
        token,  // None in production
    }))
}
```

---

## Phase 4: Testing & Validation (Week 2)

### Task 4.1: Create Integration Test Suite (2 days)

**Create File:** `crates/rcommerce-api/tests/integration_tests.rs`

**Test Scenarios:**

1. **Complete Purchase Flow:**
```rust
#[tokio::test]
async fn test_complete_purchase_flow() {
    // 1. Register customer
    // 2. Login
    // 3. Create cart
    // 4. Add items to cart
    // 5. Initiate checkout (get tax/shipping)
    // 6. Complete checkout
    // 7. Verify order created
    // 8. Verify inventory deducted
    // 9. Verify notification sent
}
```

2. **Cart Persistence:**
```rust
#[tokio::test]
async fn test_cart_persistence() {
    // 1. Create guest cart
    // 2. Add items
    // 3. Fetch cart by ID
    // 4. Verify items present
    // 5. Login and merge carts
    // 6. Verify merged cart
}
```

3. **Authentication Flows:**
```rust
#[tokio::test]
async fn test_auth_flows() {
    // 1. Register
    // 2. Login with credentials
    // 3. Access protected endpoint
    // 4. Refresh token
    // 5. Access with new token
    // 6. Request password reset
    // 7. Reset password
    // 8. Login with new password
}
```

---

### Task 4.2: Update Test Scripts (1 day)

**Files to Modify:**
- `scripts/test_api.sh`
- `scripts/test_complete_system.sh`

**Add Tests For:**
- Cart operations with verification
- Checkout flow
- Order creation with tax/shipping verification
- Notification delivery

---

## Summary: Files to Modify

### Critical Priority
| File | Lines of Change | Description |
|------|-----------------|-------------|
| `crates/rcommerce-api/src/routes/cart.rs` | ~200 | Replace mock data with real service calls |
| `crates/rcommerce-core/src/notification/service.rs` | ~20 | Add PgPool, fix db() method |
| `crates/rcommerce-api/src/routes/customer.rs` | ~50 | Replace mock data with real queries |
| `crates/rcommerce-api/src/state.rs` | ~10 | Ensure CartService is available |

### High Priority
| File | Lines of Change | Description |
|------|-----------------|-------------|
| `crates/rcommerce-api/src/routes/order.rs` | ~100 | Integrate tax/shipping services |
| `crates/rcommerce-api/src/routes/checkout.rs` | ~150 | New checkout endpoint (if needed) |
| `crates/rcommerce-api/src/middleware/mod.rs` | ~30 | Ensure auth context is properly passed |

### Medium Priority
| File | Lines of Change | Description |
|------|-----------------|-------------|
| `crates/rcommerce-api/src/server.rs` | ~30 | CORS configuration |
| `crates/rcommerce-core/src/config.rs` | ~20 | Add CORS config struct |
| `crates/rcommerce-api/src/tls/mod.rs` | ~30 | Security headers middleware |
| `crates/rcommerce-api/src/routes/auth.rs` | ~10 | Remove token from production response |

---

## Success Criteria

The system is ready for real-world testing when:

1. ✅ A customer can register, login, and access their profile
2. ✅ A customer can create a cart, add items, and retrieve the cart
3. ✅ Cart persists across requests (can be retrieved by ID)
4. ✅ Checkout calculates real tax and shipping costs
5. ✅ Orders are created with accurate totals
6. ✅ Notifications can be queued without panicking
7. ✅ Inventory is properly deducted on order
8. ✅ All API endpoints return real data (no mocks)
9. ✅ Security headers are present on all responses
10. ✅ CORS is restricted to known origins

---

## Development Order Recommendation

**Week 1:**
- Day 1-2: Task 1.1 (Cart API)
- Day 3: Task 1.2 (Notification DB connection)
- Day 4: Task 1.3 (Customer API)
- Day 5: Task 2.1 (Tax/Shipping integration)

**Week 2:**
- Day 1: Task 2.2 (Auth standardization)
- Day 2: Task 3.1 & 3.2 (Security hardening)
- Day 3: Task 3.3 (Remove sensitive data)
- Day 4-5: Task 4.1 & 4.2 (Testing)

---

*This plan prioritizes getting the system functional first, then secure, then fully tested.*
