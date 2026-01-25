pub mod product;
pub mod customer;
pub mod order;
pub mod auth;

pub use product::router as product_router;
pub use customer::router as customer_router;
pub use order::router as order_router;
pub use auth::router as auth_router;
