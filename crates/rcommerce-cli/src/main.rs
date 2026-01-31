use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

use rcommerce_core::{Result, Config};

#[derive(Parser)]
#[command(name = "rcommerce")]
#[command(about = "R Commerce Headless E-Commerce Platform")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, global = true, help = "Configuration file path")]
    config: Option<PathBuf>,
    
    #[arg(short, long, global = true, help = "Set log level")]
    log_level: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the API server
    Server {
        #[arg(short = 'H', long, help = "Bind address", default_value = "0.0.0.0")]
        host: String,
        
        #[arg(short = 'P', long, help = "Port number", default_value = "8080")]
        port: u16,
        
        #[arg(long, help = "Skip automatic database migration on startup")]
        skip_migrate: bool,
    },
    
    /// Database operations
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
    
    /// Product management
    Product {
        #[command(subcommand)]
        command: ProductCommands,
    },
    
    /// Order management
    Order {
        #[command(subcommand)]
        command: OrderCommands,
    },
    
    /// Customer management
    Customer {
        #[command(subcommand)]
        command: CustomerCommands,
    },
    
    /// API Key management
    ApiKey {
        #[command(subcommand)]
        command: ApiKeyCommands,
    },
    
    /// Show configuration
    Config,
}

#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// Run database migrations
    Migrate,
    
    /// Reset database (DANGEROUS - deletes all data)
    Reset {
        #[arg(long, help = "Skip confirmation prompt")]
        force: bool,
    },
    
    /// Seed database with sample data
    Seed,
    
    /// Show database status
    Status,
}

#[derive(Subcommand, Debug)]
pub enum ProductCommands {
    /// List products
    List,
    
    /// Create a product
    Create,
    
    /// Get product details
    Get {
        #[arg(help = "Product ID")]
        id: String,
    },
    
    /// Update a product
    Update {
        #[arg(help = "Product ID")]
        id: String,
    },
    
    /// Delete a product
    Delete {
        #[arg(help = "Product ID")]
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum OrderCommands {
    /// List orders
    List,
    
    /// Get order details
    Get {
        #[arg(help = "Order ID")]
        id: String,
    },
    
    /// Create a test order
    Create,
    
    /// Update order status
    Update {
        #[arg(help = "Order ID")]
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum CustomerCommands {
    /// List customers
    List,
    
    /// Get customer details
    Get {
        #[arg(help = "Customer ID")]
        id: String,
    },
    
    /// Create a customer
    Create,
}

#[derive(Subcommand, Debug)]
pub enum ApiKeyCommands {
    /// List all API keys
    List {
        #[arg(short = 'u', long, help = "Filter by customer ID")]
        customer_id: Option<String>,
    },
    
    /// Create a new API key
    Create {
        #[arg(short = 'u', long, help = "Customer ID (optional for system keys)")]
        customer_id: Option<String>,
        
        #[arg(short = 'n', long, help = "Key name/description")]
        name: Option<String>,
        
        #[arg(short = 's', long, help = "Scopes (comma-separated)", default_value = "read")]
        scopes: String,
        
        #[arg(short = 'e', long, help = "Expiration in days (optional)")]
        expires_days: Option<i64>,
    },
    
    /// Get API key details
    Get {
        #[arg(help = "Key prefix or ID")]
        prefix: String,
    },
    
    /// Revoke an API key
    Revoke {
        #[arg(help = "Key prefix or ID")]
        prefix: String,
        
        #[arg(short, long, help = "Reason for revocation")]
        reason: Option<String>,
    },
    
    /// Delete an API key permanently
    Delete {
        #[arg(help = "Key prefix or ID")]
        prefix: String,
        
        #[arg(long, help = "Skip confirmation")]
        force: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = cli.log_level.as_deref().unwrap_or("info");
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .init();
    
    // Load configuration
    let config = if let Some(ref config_path) = cli.config {
        Config::load(config_path.to_str().unwrap())?
    } else {
        Config::from_env()?
    };
    
    info!("Starting R Commerce v{} with config: {}", rcommerce_core::VERSION, config.server.host);
    
    // Execute command
    match cli.command {
        Commands::Server { host, port, skip_migrate } => {
            // Override config with CLI arguments
            let mut config = config;
            config.server.host = host;
            config.server.port = port;
            
            // Auto-migrate database unless skipped
            if !skip_migrate {
                info!("Running database migrations...");
                match run_migrations(&config).await {
                    Ok(_) => info!("Database migrations completed successfully"),
                    Err(e) => {
                        eprintln!("❌ Database migration failed: {}", e);
                        eprintln!("Use --skip-migrate to start without migration");
                        std::process::exit(1);
                    }
                }
            }
            
            rcommerce_api::run(config).await?;
        }
        
        Commands::Db { command } => {
            use colored::*;
            
            // Create database pool
            let pool = create_pool(&config).await?;
            let migrator = rcommerce_core::Migrator::new(pool);
            
            match command {
                DbCommands::Migrate => {
                    println!("{}", "Running database migrations...".yellow());
                    match migrator.migrate().await {
                        Ok(_) => {
                            println!("{}", "✅ Migrations completed successfully!".green());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Migration failed: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                DbCommands::Reset { force } => {
                    if !force {
                        println!("{}", "⚠️  WARNING: This will DELETE ALL DATA!".red().bold());
                        print!("Type 'yes' to confirm: ");
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                        
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        
                        if input.trim() != "yes" {
                            println!("Aborted.");
                            return Ok(());
                        }
                    }
                    
                    println!("{}", "Resetting database...".red());
                    match migrator.reset().await {
                        Ok(_) => {
                            println!("{}", "✅ Database reset complete!".green());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Reset failed: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                DbCommands::Seed => {
                    println!("{}", "Seeding database with demo data...".green());
                    match migrator.seed().await {
                        Ok(_) => {
                            println!("{}", "✅ Demo data seeded successfully!".green());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Seed failed: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                DbCommands::Status => {
                    match migrator.status().await {
                        Ok(status) => {
                            println!("{}", "Database Status".bold().underline());
                            println!("  Host: {}:{}", config.database.host, config.database.port);
                            println!("  Database: {}", config.database.database);
                            println!("  Applied migrations: {}", status.applied_migrations);
                            println!("  Products: {}", status.product_count);
                            println!("  Customers: {}", status.customer_count);
                            println!("  Orders: {}", status.order_count);
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to get status: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        
        Commands::Product { command } => {
            println!("Product command: {:?}", command);
            println!("Use the API for full CRUD operations");
        }
        
        Commands::Order { command } => {
            println!("Order command: {:?}", command);
            println!("Use the API for full CRUD operations");
        }
        
        Commands::Customer { command } => {
            println!("Customer command: {:?}", command);
            println!("Use the API for full CRUD operations");
        }
        
        Commands::ApiKey { command } => {
            use colored::*;
            
            // Create database pool
            let pool = create_pool(&config).await?;
            
            match command {
                ApiKeyCommands::List { customer_id } => {
                    match list_api_keys(&pool, customer_id).await {
                        Ok(keys) => {
                            if keys.is_empty() {
                                println!("{}", "No API keys found".yellow());
                            } else {
                                println!("{}", "API Keys".bold().underline());
                                println!("{:<12} {:<20} {:<30} {:<10} {:<12}", 
                                    "Prefix", "Name", "Scopes", "Active", "Expires");
                                println!("{}", "-".repeat(90));
                                for key in keys {
                                    let expires = key.expires_at
                                        .map(|d| d.format("%Y-%m-%d").to_string())
                                        .unwrap_or_else(|| "Never".to_string());
                                    println!("{:<12} {:<20} {:<30} {:<10} {:<12}",
                                        key.key_prefix,
                                        key.name,
                                        key.scopes.join(", "),
                                        if key.is_active { "✓".green() } else { "✗".red() },
                                        expires
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to list API keys: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                
                ApiKeyCommands::Create { customer_id, name, scopes, expires_days } => {
                    let auth_service = rcommerce_core::services::AuthService::new(config);
                    
                    match create_api_key(&pool, &auth_service, customer_id, name, scopes, expires_days).await {
                        Ok((key, full_key)) => {
                            println!("{}", "✅ API Key created successfully!".green().bold());
                            println!();
                            println!("{}", "IMPORTANT: Copy this key now - it won't be shown again!".red().bold());
                            println!();
                            println!("  Key: {}", full_key.bright_cyan());
                            println!();
                            println!("  Prefix:      {}", key.key_prefix);
                            println!("  Name:        {}", key.name);
                            println!("  Scopes:      {}", key.scopes.join(", "));
                            println!("  Customer ID: {}", key.customer_id.map(|id: Uuid| id.to_string()).unwrap_or_else(|| "System".to_string()));
                            println!("  Expires:     {}", key.expires_at.map(|d| d.to_string()).unwrap_or_else(|| "Never".to_string()));
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to create API key: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                
                ApiKeyCommands::Get { prefix } => {
                    match get_api_key(&pool, &prefix).await {
                        Ok(Some(key)) => {
                            println!("{}", "API Key Details".bold().underline());
                            println!("  Prefix:       {}", key.key_prefix);
                            println!("  Name:         {}", key.name);
                            println!("  Scopes:       {}", key.scopes.join(", "));
                            println!("  Active:       {}", if key.is_active { "✓ Yes".green() } else { "✗ No".red() });
                            println!("  Customer ID:  {}", key.customer_id.map(|id: Uuid| id.to_string()).unwrap_or_else(|| "System".to_string()));
                            println!("  Created:      {}", key.created_at);
                            println!("  Updated:      {}", key.updated_at);
                            println!("  Expires:      {}", key.expires_at.map(|d| d.to_string()).unwrap_or_else(|| "Never".to_string()));
                            println!("  Last Used:    {}", key.last_used_at.map(|d| d.to_string()).unwrap_or_else(|| "Never".to_string()));
                            if let Some(revoked_at) = key.revoked_at {
                                println!("  Revoked:      {} {}", revoked_at, key.revoked_reason.unwrap_or_default().red());
                            }
                        }
                        Ok(None) => {
                            println!("{}", format!("API key with prefix '{}' not found", prefix).yellow());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to get API key: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                
                ApiKeyCommands::Revoke { prefix, reason } => {
                    match revoke_api_key(&pool, &prefix, reason).await {
                        Ok(true) => {
                            println!("{}", format!("✅ API key '{}' revoked successfully", prefix).green());
                        }
                        Ok(false) => {
                            println!("{}", format!("API key with prefix '{}' not found", prefix).yellow());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to revoke API key: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                
                ApiKeyCommands::Delete { prefix, force } => {
                    if !force {
                        println!("{}", "⚠️  WARNING: This will PERMANENTLY delete the API key!".red().bold());
                        print!("Type 'yes' to confirm: ");
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                        
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        
                        if input.trim() != "yes" {
                            println!("Aborted.");
                            return Ok(());
                        }
                    }
                    
                    match delete_api_key(&pool, &prefix).await {
                        Ok(true) => {
                            println!("{}", format!("✅ API key '{}' deleted permanently", prefix).green());
                        }
                        Ok(false) => {
                            println!("{}", format!("API key with prefix '{}' not found", prefix).yellow());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to delete API key: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        
        Commands::Config => {
            println!("Configuration loaded from: {}", 
                cli.config.map(|p| p.display().to_string()).unwrap_or_else(|| "environment".to_string())
            );
            println!("{:#?}", config);
        }
    }
    
    Ok(())
}

/// Create database pool from config
async fn create_pool(config: &Config) -> Result<sqlx::PgPool> {
    use sqlx::postgres::PgPoolOptions;
    
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.database
    );
    
    let pool = PgPoolOptions::new()
        .max_connections(config.database.pool_size)
        .connect(&database_url)
        .await
        .map_err(|e| rcommerce_core::Error::Database(e))?;
    
    Ok(pool)
}

/// Run database migrations
async fn run_migrations(config: &Config) -> Result<()> {
    let pool = create_pool(config).await?;
    rcommerce_core::auto_migrate(&pool).await?;
    Ok(())
}

// API Key management functions

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// API Key record from database
#[derive(Debug, sqlx::FromRow)]
struct ApiKeyRecord {
    id: Uuid,
    customer_id: Option<Uuid>,
    key_prefix: String,
    key_hash: String,
    name: String,
    scopes: Vec<String>,
    expires_at: Option<DateTime<Utc>>,
    last_used_at: Option<DateTime<Utc>>,
    last_used_ip: Option<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    revoked_at: Option<DateTime<Utc>>,
    revoked_reason: Option<String>,
}

/// List API keys
async fn list_api_keys(pool: &sqlx::PgPool, customer_id: Option<String>) -> Result<Vec<ApiKeyRecord>> {
    let keys = if let Some(cid) = customer_id {
        let cid = Uuid::parse_str(&cid).map_err(|e| rcommerce_core::Error::validation(format!("Invalid customer ID: {}", e)))?;
        sqlx::query_as::<_, ApiKeyRecord>(
            "SELECT * FROM api_keys WHERE customer_id = $1 ORDER BY created_at DESC"
        )
        .bind(cid)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, ApiKeyRecord>(
            "SELECT * FROM api_keys ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?
    };
    
    Ok(keys)
}

/// Create a new API key
async fn create_api_key(
    pool: &sqlx::PgPool,
    auth_service: &rcommerce_core::services::AuthService,
    customer_id: Option<String>,
    name: Option<String>,
    scopes: String,
    expires_days: Option<i64>,
) -> Result<(ApiKeyRecord, String)> {
    // Generate API key
    let api_key = auth_service.generate_api_key();
    let full_key = api_key.full_key.clone().unwrap();
    
    // Parse customer ID if provided
    let customer_uuid = if let Some(cid) = customer_id {
        Some(Uuid::parse_str(&cid).map_err(|e| rcommerce_core::Error::validation(format!("Invalid customer ID: {}", e)))?)
    } else {
        None
    };
    
    // Calculate expiration
    let expires_at = expires_days.map(|days| Utc::now() + chrono::Duration::days(days));
    
    // Parse scopes
    let scopes_vec: Vec<String> = scopes.split(',').map(|s| s.trim().to_string()).collect();
    
    // Insert into database
    let key = sqlx::query_as::<_, ApiKeyRecord>(
        r#"
        INSERT INTO api_keys (customer_id, key_prefix, key_hash, name, scopes, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(customer_uuid)
    .bind(&api_key.prefix)
    .bind(&api_key.hash)
    .bind(name.unwrap_or_else(|| "API Key".to_string()))
    .bind(&scopes_vec)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;
    
    Ok((key, full_key))
}

/// Get API key by prefix
async fn get_api_key(pool: &sqlx::PgPool, prefix: &str) -> Result<Option<ApiKeyRecord>> {
    let key = sqlx::query_as::<_, ApiKeyRecord>(
        "SELECT * FROM api_keys WHERE key_prefix = $1"
    )
    .bind(prefix)
    .fetch_optional(pool)
    .await?;
    
    Ok(key)
}

/// Revoke an API key
async fn revoke_api_key(pool: &sqlx::PgPool, prefix: &str, reason: Option<String>) -> Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE api_keys 
        SET is_active = false, revoked_at = NOW(), revoked_reason = $2
        WHERE key_prefix = $1
        "#
    )
    .bind(prefix)
    .bind(reason)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() > 0)
}

/// Delete an API key permanently
async fn delete_api_key(pool: &sqlx::PgPool, prefix: &str) -> Result<bool> {
    let result = sqlx::query(
        "DELETE FROM api_keys WHERE key_prefix = $1"
    )
    .bind(prefix)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parse() {
        let cli = Cli::parse_from(&["rcommerce", "server"]);
        assert!(matches!(cli.command, Commands::Server { .. }));
    }
}
