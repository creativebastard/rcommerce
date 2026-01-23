pub mod config;
pub mod error;
pub mod models;
pub mod traits;
pub mod common;
pub mod repository;
pub mod services;

// Re-export commonly used types
pub use error::{Error, Result};
pub use config::Config;
pub use models::*;
pub use traits::*;
pub use repository::{Database, create_pool};
pub use services::*;

/// Current version of rcommerce
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Error {
    pub fn not_implemented<T: Into<String>>(msg: T) -> Self {
        Error::Other(format!("Not implemented: {}", msg.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version_available() {
        assert!(!VERSION.is_empty());
    }
    
    #[test]
    fn test_error_creation() {
        let err = Error::validation("Test validation error");
        assert_eq!(err.status_code(), 400);
        assert_eq!(err.category(), "validation");
    }
}