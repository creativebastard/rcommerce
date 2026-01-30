pub mod product;
pub mod customer;
pub mod order;
pub mod auth;
pub mod cart;
pub mod coupon;
pub mod payment;
pub mod payment_v2;

pub use product::router as product_router;
pub use customer::router as customer_router;
pub use order::router as order_router;
pub use auth::router as auth_router;
pub use cart::router as cart_router;
pub use coupon::router as coupon_router;
pub use payment::router as payment_router;
pub use payment_v2::router as payment_v2_router;
