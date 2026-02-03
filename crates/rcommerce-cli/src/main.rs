use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::{Input, Confirm, Select, Password};
use std::path::PathBuf;
use tracing::info;

use rcommerce_core::{Result, Config};
use rcommerce_core::models::{ProductType, Currency};

/// Security checks for CLI operations
mod security {
    use colored::Colorize;
    use std::path::PathBuf;
    
    /// Check if running as root
    pub fn check_not_root() -> Result<(), String> {
        #[cfg(unix)]
        {
            let uid = unsafe { libc::getuid() };
            if uid == 0 {
                return Err(format!("\n{}\n{}\n{}",
                    "❌ ERROR: Running as root is not allowed!".red().bold(),
                    "   The rcommerce CLI should not be run as root for security reasons.",
                    "   Please run as a non-privileged user."
                ));
            }
        }
        Ok(())
    }
    
    /// Check config file permissions
    pub fn check_config_permissions(path: &PathBuf) -> Result<(), String> {
        use std::os::unix::fs::PermissionsExt;
        
        let metadata = std::fs::metadata(path)
            .map_err(|e| format!("Cannot read config file: {}", e))?;
        
        let permissions = metadata.permissions();
        let mode = permissions.mode();
        
        // Check if world-readable (last digit in octal)
        let world_readable = (mode & 0o004) != 0;
        let world_writable = (mode & 0o002) != 0;
        
        if world_writable {
            return Err(format!("\n{}\n{}\n{}",
                "❌ ERROR: Config file is world-writable!".red().bold(),
                format!("   Path: {}", path.display()),
                "   Run: chmod 600 {}".replace("{}", &path.display().to_string())
            ));
        }
        
        if world_readable {
            eprintln!("{}", format!("\n{}\n{}\n{}",
                "⚠️  WARNING: Config file is world-readable".yellow().bold(),
                format!("   Path: {}", path.display()),
                "   Consider running: chmod 600 {}".replace("{}", &path.display().to_string())
            ));
        }
        
        Ok(())
    }
}

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
    
    /// Import data from platforms or files
    Import {
        #[command(subcommand)]
        command: ImportCommands,
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
pub enum ImportCommands {
    /// Import from an e-commerce platform (shopify, woocommerce, magento, medusa)
    Platform {
        /// Platform name
        #[arg(help = "Platform name (shopify, woocommerce, magento, medusa)")]
        platform: String,
        
        /// API base URL
        #[arg(short, long, help = "API base URL")]
        api_url: String,
        
        /// API key or access token
        #[arg(short, long, help = "API key or access token")]
        api_key: String,
        
        /// Additional API secret (for WooCommerce)
        #[arg(long, help = "API secret (for WooCommerce)")]
        api_secret: Option<String>,
        
        /// Entity types to import (products, customers, orders, all)
        #[arg(short, long, help = "Entity types to import", default_value = "all")]
        entities: String,
        
        /// Maximum number of records to import
        #[arg(long, help = "Maximum records to import")]
        limit: Option<usize>,
        
        /// Dry run - validate without importing
        #[arg(long, help = "Dry run without importing")]
        dry_run: bool,
    },
    
    /// Import from a file (csv, json, xml)
    File {
        /// Path to the import file
        #[arg(help = "Path to import file")]
        path: PathBuf,
        
        /// File format (csv, json, xml)
        #[arg(short, long, help = "File format (csv, json, xml)")]
        format: String,
        
        /// Entity type (products, customers, orders)
        #[arg(short, long, help = "Entity type (products, customers, orders)")]
        entity: String,
        
        /// Dry run - validate without importing
        #[arg(long, help = "Dry run without importing")]
        dry_run: bool,
    },
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
        // Check config file permissions
        if let Err(e) = security::check_config_permissions(config_path) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
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
            // Security checks
            if let Err(e) = security::check_not_root() {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            
            let pool = create_pool(&config).await?;
            
            match command {
                ProductCommands::List => {
                    match list_products(&pool).await {
                        Ok(products) => {
                            if products.is_empty() {
                                println!("{}", "No products found".yellow());
                            } else {
                                println!("{}", "Products".bold().underline());
                                println!("{:<36} {:<30} {:<10} {:<8} {:<12}", 
                                    "ID", "Title", "Price", "Currency", "Status");
                                println!("{}", "-".repeat(100));
                                for p in &products {
                                    let status = if p.is_active { "✓ Active".green() } else { "✗ Inactive".red() };
                                    println!("{:<36} {:<30} {:<10.2} {:<8} {:<12}",
                                        p.id.to_string(),
                                        truncate(&p.title, 28),
                                        p.price,
                                        p.currency.to_string(),
                                        status
                                    );
                                }
                                println!("\nTotal: {} products", products.len());
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to list products: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                ProductCommands::Get { id } => {
                    match get_product(&pool, &id).await {
                        Ok(Some(p)) => {
                            println!("{}", "Product Details".bold().underline());
                            println!("  ID:          {}", p.id);
                            println!("  Title:       {}", p.title);
                            println!("  Slug:        {}", p.slug);
                            println!("  Price:       {} {}", p.price, p.currency);
                            println!("  Status:      {}", if p.is_active { "✓ Active".green() } else { "✗ Inactive".red() });
                            println!("  Inventory:   {}", p.inventory_quantity);
                            println!("  Created:     {}", p.created_at);
                            if let Some(desc) = p.description {
                                println!("  Description: {}", truncate(&desc, 100));
                            }
                        }
                        Ok(None) => {
                            println!("{}", format!("Product '{}' not found", id).yellow());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to get product: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                ProductCommands::Create => {
                    match interactive_create_product(&pool).await {
                        Ok(product) => {
                            println!("{}", "\n✅ Product created successfully!".green().bold());
                            println!("  ID:    {}", product.id);
                            println!("  Title: {}", product.title);
                            println!("  Slug:  {}", product.slug);
                            println!("  Price: {} {}", product.price, product.currency);
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to create product: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                ProductCommands::Update { id } => {
                    println!("{}", format!("Product update for '{}' coming soon!", id).yellow());
                }
                ProductCommands::Delete { id } => {
                    println!("{}", "⚠️  Product deletion".red().bold());
                    print!("Type 'yes' to delete product '{}': ", id);
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                    
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    
                    if input.trim() != "yes" {
                        println!("Aborted.");
                        return Ok(());
                    }
                    
                    match delete_product(&pool, &id).await {
                        Ok(true) => println!("{}", format!("✅ Product '{}' deleted", id).green()),
                        Ok(false) => println!("{}", format!("Product '{}' not found", id).yellow()),
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to delete product: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        
        Commands::Order { command } => {
            if let Err(e) = security::check_not_root() {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            
            let pool = create_pool(&config).await?;
            
            match command {
                OrderCommands::List => {
                    match list_orders(&pool).await {
                        Ok(orders) => {
                            if orders.is_empty() {
                                println!("{}", "No orders found".yellow());
                            } else {
                                println!("{}", "Orders".bold().underline());
                                println!("{:<36} {:<20} {:<12} {:<15} {:<12}", 
                                    "ID", "Customer", "Status", "Total", "Created");
                                println!("{}", "-".repeat(100));
                                for o in &orders {
                                    println!("{:<36} {:<20} {:<12} {:<15.2} {:<12}",
                                        o.id.to_string(),
                                        truncate(&o.customer_email, 18),
                                        o.status,
                                        o.total_amount,
                                        o.created_at.format("%Y-%m-%d")
                                    );
                                }
                                println!("\nTotal: {} orders", orders.len());
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to list orders: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                OrderCommands::Get { id } => {
                    println!("{}", format!("Order details for '{}' coming soon!", id).yellow());
                }
                OrderCommands::Create => {
                    println!("{}", "Order creation via CLI coming soon!".yellow());
                }
                OrderCommands::Update { id } => {
                    println!("{}", format!("Order update for '{}' coming soon!", id).yellow());
                }
            }
        }
        
        Commands::Customer { command } => {
            if let Err(e) = security::check_not_root() {
                eprintln!("{}", e);
                std::process::exit(1);
            }
            
            let pool = create_pool(&config).await?;
            
            match command {
                CustomerCommands::List => {
                    match list_customers(&pool).await {
                        Ok(customers) => {
                            if customers.is_empty() {
                                println!("{}", "No customers found".yellow());
                            } else {
                                println!("{}", "Customers".bold().underline());
                                println!("{:<36} {:<30} {:<20} {:<12}", 
                                    "ID", "Email", "Name", "Created");
                                println!("{}", "-".repeat(100));
                                for c in &customers {
                                    let name = format!("{} {}", c.first_name, c.last_name);
                                    println!("{:<36} {:<30} {:<20} {:<12}",
                                        c.id.to_string(),
                                        truncate(&c.email, 28),
                                        truncate(&name, 18),
                                        c.created_at.format("%Y-%m-%d")
                                    );
                                }
                                println!("\nTotal: {} customers", customers.len());
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to list customers: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                CustomerCommands::Get { id } => {
                    println!("{}", format!("Customer details for '{}' coming soon!", id).yellow());
                }
                CustomerCommands::Create => {
                    match interactive_create_customer(&pool).await {
                        Ok(customer) => {
                            println!("{}", "\n✅ Customer created successfully!".green().bold());
                            println!("  ID:    {}", customer.id);
                            println!("  Name:  {} {}", customer.first_name, customer.last_name);
                            println!("  Email: {}", customer.email);
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Failed to create customer: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
            }
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
                            println!("  ID:           {}", key.id);
                            println!("  Prefix:       {}", key.key_prefix);
                            println!("  Name:         {}", key.name);
                            println!("  Scopes:       {}", key.scopes.join(", "));
                            println!("  Active:       {}", if key.is_active { "✓ Yes".green() } else { "✗ No".red() });
                            println!("  Customer ID:  {}", key.customer_id.map(|id: Uuid| id.to_string()).unwrap_or_else(|| "System".to_string()));
                            println!("  Created:      {}", key.created_at);
                            println!("  Updated:      {}", key.updated_at);
                            println!("  Expires:      {}", key.expires_at.map(|d| d.to_string()).unwrap_or_else(|| "Never".to_string()));
                            println!("  Last Used:    {}", key.last_used_at.map(|d| d.to_string()).unwrap_or_else(|| "Never".to_string()));
                            if let Some(last_ip) = key.last_used_ip {
                                println!("  Last IP:      {}", last_ip);
                            }
                            if let Some(revoked_at) = key.revoked_at {
                                println!("  Revoked:      {} {}", revoked_at, key.revoked_reason.unwrap_or_default().red());
                            }
                            println!("  Key Hash:     {}...", &key.key_hash[..16]);
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
        
        Commands::Import { command } => {
            use colored::*;
            use rcommerce_core::import::{
                get_file_importer, get_platform_importer, ImportConfig,
            };
            use rcommerce_core::import::types::{ImportOptions, SourceConfig};
            
            match command {
                ImportCommands::Platform { platform, api_url, api_key, api_secret, entities, limit, dry_run } => {
                    println!("{} {}", "Importing from".bold(), platform.cyan());
                    
                    // Get the platform importer
                    let importer = match get_platform_importer(&platform) {
                        Some(imp) => imp,
                        None => {
                            eprintln!("{}", format!("❌ Unsupported platform: {}", platform).red());
                            eprintln!("Supported platforms: shopify, woocommerce, magento, medusa");
                            std::process::exit(1);
                        }
                    };
                    
                    // Build headers for WooCommerce
                    let mut headers = std::collections::HashMap::new();
                    if let Some(secret) = api_secret {
                        headers.insert("consumer_secret".to_string(), secret);
                    }
                    
                    // Create import config
                    let import_config = ImportConfig {
                        database_url: config.database.url(),
                        source: SourceConfig::Platform {
                            platform: platform.clone(),
                            api_url,
                            api_key,
                            headers,
                        },
                        options: ImportOptions {
                            dry_run,
                            limit: limit.unwrap_or(0),
                            ..Default::default()
                        },
                    };
                    
                    // Progress callback
                    let progress = |p: rcommerce_core::import::ImportProgress| {
                        let pct = p.percentage();
                        print!("\r  [{}] {} - {} ({:.1}%)", 
                            p.stage.bright_blue(),
                            p.message,
                            format!("{}/{}", p.current, p.total).dimmed(),
                            pct
                        );
                        std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    };
                    
                    // Run import
                    let result = match entities.as_str() {
                        "products" => importer.import_products(&import_config, &progress).await,
                        "customers" => importer.import_customers(&import_config, &progress).await,
                        "orders" => importer.import_orders(&import_config, &progress).await,
                        "all" => importer.import_all(&import_config, &progress).await,
                        _ => {
                            eprintln!("{}", format!("❌ Invalid entity type: {}", entities).red());
                            std::process::exit(1);
                        }
                    };
                    
                    println!(); // New line after progress
                    
                    match result {
                        Ok(stats) => {
                            println!("{}", "✅ Import completed successfully!".green().bold());
                            println!("  Created:  {}", stats.created.to_string().green());
                            println!("  Updated:  {}", stats.updated.to_string().yellow());
                            println!("  Skipped:  {}", stats.skipped.to_string().dimmed());
                            println!("  Errors:   {}", stats.errors.to_string().red());
                            println!("  Total:    {}", stats.total);
                            
                            if !stats.error_details.is_empty() {
                                println!("\n{}", "Error Details:".red().bold());
                                for error in stats.error_details.iter().take(10) {
                                    println!("  • {}", error);
                                }
                                if stats.error_details.len() > 10 {
                                    println!("  ... and {} more errors", stats.error_details.len() - 10);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Import failed: {}", e).red());
                            std::process::exit(1);
                        }
                    }
                }
                
                ImportCommands::File { path, format, entity, dry_run } => {
                    println!("{} {} → {}", 
                        "Importing".bold(),
                        path.display().to_string().cyan(),
                        entity.cyan()
                    );
                    
                    // Get the file importer
                    let importer = match get_file_importer(&format) {
                        Some(imp) => imp,
                        None => {
                            eprintln!("{}", format!("❌ Unsupported format: {}", format).red());
                            eprintln!("Supported formats: csv, json, xml");
                            std::process::exit(1);
                        }
                    };
                    
                    // Parse entity type
                    let entity_type = match entity.as_str() {
                        "products" => rcommerce_core::import::EntityType::Products,
                        "customers" => rcommerce_core::import::EntityType::Customers,
                        "orders" => rcommerce_core::import::EntityType::Orders,
                        _ => {
                            eprintln!("{}", format!("❌ Invalid entity type: {}", entity).red());
                            std::process::exit(1);
                        }
                    };
                    
                    // Create import config
                    let import_config = ImportConfig {
                        database_url: config.database.url(),
                        source: SourceConfig::File {
                            path: path.clone(),
                            format: format.clone(),
                        },
                        options: ImportOptions {
                            dry_run,
                            ..Default::default()
                        },
                    };
                    
                    // Progress callback
                    let progress = |p: rcommerce_core::import::ImportProgress| {
                        let pct = p.percentage();
                        print!("\r  {} - {} ({:.1}%)", 
                            p.message,
                            format!("{}/{}", p.current, p.total).dimmed(),
                            pct
                        );
                        std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    };
                    
                    // Run import
                    match importer.import_file(&path, entity_type, &import_config, &progress).await {
                        Ok(stats) => {
                            println!();
                            println!("{}", "✅ Import completed successfully!".green().bold());
                            println!("  Created:  {}", stats.created.to_string().green());
                            println!("  Updated:  {}", stats.updated.to_string().yellow());
                            println!("  Skipped:  {}", stats.skipped.to_string().dimmed());
                            println!("  Errors:   {}", stats.errors.to_string().red());
                            println!("  Total:    {}", stats.total);
                        }
                        Err(e) => {
                            eprintln!("{}", format!("❌ Import failed: {}", e).red());
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

// Product CLI functions

#[derive(Debug, sqlx::FromRow)]
struct ProductRecord {
    id: Uuid,
    title: String,
    slug: String,
    price: rust_decimal::Decimal,
    currency: String,
    description: Option<String>,
    is_active: bool,
    inventory_quantity: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// List all products
async fn list_products(pool: &sqlx::PgPool) -> Result<Vec<ProductRecord>> {
    let products = sqlx::query_as::<_, ProductRecord>(
        "SELECT id, title, slug, price, currency::text, description, is_active, inventory_quantity, created_at 
         FROM products ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(products)
}

/// Get product by ID
async fn get_product(pool: &sqlx::PgPool, id: &str) -> Result<Option<ProductRecord>> {
    let product_id = Uuid::parse_str(id)
        .map_err(|e| rcommerce_core::Error::validation(format!("Invalid product ID: {}", e)))?;
    
    let product = sqlx::query_as::<_, ProductRecord>(
        "SELECT id, title, slug, price, currency::text, description, is_active, inventory_quantity, created_at 
         FROM products WHERE id = $1"
    )
    .bind(product_id)
    .fetch_optional(pool)
    .await?;
    
    Ok(product)
}

/// Delete product by ID
async fn delete_product(pool: &sqlx::PgPool, id: &str) -> Result<bool> {
    let product_id = Uuid::parse_str(id)
        .map_err(|e| rcommerce_core::Error::validation(format!("Invalid product ID: {}", e)))?;
    
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(product_id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

// Order CLI functions

#[derive(Debug, sqlx::FromRow)]
struct OrderRecord {
    id: Uuid,
    customer_email: String,
    status: String,
    total_amount: rust_decimal::Decimal,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// List all orders
async fn list_orders(pool: &sqlx::PgPool) -> Result<Vec<OrderRecord>> {
    let orders = sqlx::query_as::<_, OrderRecord>(
        "SELECT o.id, c.email as customer_email, o.status::text, o.total_amount, o.created_at 
         FROM orders o 
         JOIN customers c ON o.customer_id = c.id 
         ORDER BY o.created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(orders)
}

// Customer CLI functions

#[derive(Debug, sqlx::FromRow)]
struct CustomerRecord {
    id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// List all customers
async fn list_customers(pool: &sqlx::PgPool) -> Result<Vec<CustomerRecord>> {
    let customers = sqlx::query_as::<_, CustomerRecord>(
        "SELECT id, email, first_name, last_name, created_at 
         FROM customers ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(customers)
}

/// Helper function to truncate strings
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

// Interactive creation functions

/// Simple slugify function - converts string to URL-friendly slug
fn slugify(input: &str) -> String {
    input
        .to_lowercase()
        .replace(' ', "-")
        .replace(|c: char| !c.is_alphanumeric() && c != '-', "")
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

/// Interactive product creation using dialoguer
async fn interactive_create_product(pool: &sqlx::PgPool) -> Result<ProductRecord> {
    use colored::Colorize;
    use rust_decimal::Decimal;
    
    println!("{}", "\n📦 Create New Product".bold().underline());
    println!("{}", "Press Ctrl+C to cancel at any time.\n".dimmed());
    
    // Product title
    let title: String = Input::new()
        .with_prompt("Product title")
        .validate_with(|input: &String| {
            if input.trim().is_empty() {
                Err("Title is required")
            } else if input.len() > 255 {
                Err("Title must be less than 255 characters")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // Auto-generate slug from title
    let default_slug = slugify(&title);
    let slug: String = Input::new()
        .with_prompt("URL slug")
        .default(default_slug)
        .validate_with(|input: &String| {
            if input.trim().is_empty() {
                Err("Slug is required")
            } else if input.len() > 255 {
                Err("Slug must be less than 255 characters")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // Product type selection
    let product_types = vec!["Simple", "Variable", "Digital", "Bundle"];
    let type_index = Select::new()
        .with_prompt("Product type")
        .items(&product_types)
        .default(0)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Selection error: {}", e)))?;
    let product_type = match type_index {
        0 => ProductType::Simple,
        1 => ProductType::Variable,
        2 => ProductType::Digital,
        3 => ProductType::Bundle,
        _ => ProductType::Simple,
    };
    
    // Price
    let price_str: String = Input::new()
        .with_prompt("Price")
        .validate_with(|input: &String| {
            match input.parse::<Decimal>() {
                Ok(d) if d >= Decimal::ZERO => Ok(()),
                Ok(_) => Err("Price must be non-negative"),
                Err(_) => Err("Please enter a valid number"),
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    let price: Decimal = price_str.parse().unwrap();
    
    // Currency selection
    let currencies = vec!["USD", "EUR", "GBP", "JPY", "AUD", "CAD", "CNY", "HKD", "SGD"];
    let currency_index = Select::new()
        .with_prompt("Currency")
        .items(&currencies)
        .default(0)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Selection error: {}", e)))?;
    let currency = match currency_index {
        0 => Currency::USD,
        1 => Currency::EUR,
        2 => Currency::GBP,
        3 => Currency::JPY,
        4 => Currency::AUD,
        5 => Currency::CAD,
        6 => Currency::CNY,
        7 => Currency::HKD,
        8 => Currency::SGD,
        _ => Currency::USD,
    };
    
    // SKU (optional) - use String then convert to Option
    let sku_input: String = Input::new()
        .with_prompt("SKU (optional)")
        .allow_empty(true)
        .validate_with(|input: &String| {
            if input.len() > 100 {
                Err("SKU must be less than 100 characters")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    let sku = if sku_input.trim().is_empty() { None } else { Some(sku_input) };
    
    // Inventory quantity
    let inventory_str: String = Input::new()
        .with_prompt("Inventory quantity")
        .default("0".to_string())
        .validate_with(|input: &String| {
            match input.parse::<i32>() {
                Ok(_) => Ok(()),
                Err(_) => Err("Please enter a valid integer"),
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    let inventory_quantity: i32 = inventory_str.parse().unwrap();
    
    // Description (optional) - use String then convert to Option
    let desc_input: String = Input::new()
        .with_prompt("Description (optional)")
        .allow_empty(true)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    let description = if desc_input.trim().is_empty() { None } else { Some(desc_input) };
    
    // Active status
    let is_active = Confirm::new()
        .with_prompt("Make product active?")
        .default(true)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // Featured status
    let is_featured = Confirm::new()
        .with_prompt("Mark as featured?")
        .default(false)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // Summary
    println!("\n{}", "📋 Product Summary".bold().underline());
    println!("  Title:       {}", title);
    println!("  Slug:        {}", slug);
    println!("  Type:        {:?}", product_type);
    println!("  Price:       {} {}", price, currency);
    println!("  SKU:         {}", sku.as_deref().unwrap_or("(none)"));
    println!("  Inventory:   {}", inventory_quantity);
    println!("  Description: {}", description.as_deref().unwrap_or("(none)"));
    println!("  Active:      {}", if is_active { "Yes".green() } else { "No".red() });
    println!("  Featured:    {}", if is_featured { "Yes".green() } else { "No".red() });
    
    // Final confirmation
    let confirmed = Confirm::new()
        .with_prompt("\nCreate this product?")
        .default(true)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    if !confirmed {
        return Err(rcommerce_core::Error::validation("Product creation cancelled"));
    }
    
    // Insert into database
    let product = sqlx::query_as::<_, ProductRecord>(
        r#"
        INSERT INTO products (
            title, slug, description, sku, product_type, price, currency,
            inventory_quantity, inventory_management, is_active, is_featured,
            requires_shipping
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING id, title, slug, price, currency::text, description, is_active, inventory_quantity, created_at
        "#
    )
    .bind(&title)
    .bind(&slug)
    .bind(&description)
    .bind(&sku)
    .bind(product_type)
    .bind(price)
    .bind(currency)
    .bind(inventory_quantity)
    .bind(true) // inventory_management
    .bind(is_active)
    .bind(is_featured)
    .bind(!matches!(product_type, ProductType::Digital)) // requires_shipping (false for digital)
    .fetch_one(pool)
    .await?;
    
    Ok(product)
}

/// Interactive customer creation using dialoguer
async fn interactive_create_customer(pool: &sqlx::PgPool) -> Result<CustomerRecord> {
    use colored::Colorize;
    
    println!("{}", "\n👤 Create New Customer".bold().underline());
    println!("{}", "Press Ctrl+C to cancel at any time.\n".dimmed());
    
    // Email
    let email: String = Input::new()
        .with_prompt("Email address")
        .validate_with(|input: &String| {
            if input.trim().is_empty() {
                Err("Email is required")
            } else if !input.contains('@') || !input.contains('.') {
                Err("Please enter a valid email address")
            } else if input.len() > 255 {
                Err("Email must be less than 255 characters")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // First name
    let first_name: String = Input::new()
        .with_prompt("First name")
        .validate_with(|input: &String| {
            if input.trim().is_empty() {
                Err("First name is required")
            } else if input.len() > 100 {
                Err("First name must be less than 100 characters")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // Last name
    let last_name: String = Input::new()
        .with_prompt("Last name")
        .validate_with(|input: &String| {
            if input.trim().is_empty() {
                Err("Last name is required")
            } else if input.len() > 100 {
                Err("Last name must be less than 100 characters")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // Phone (optional) - use String then convert to Option
    let phone_input: String = Input::new()
        .with_prompt("Phone number (optional)")
        .allow_empty(true)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    let phone = if phone_input.trim().is_empty() { None } else { Some(phone_input) };
    
    // Currency selection
    let currencies = vec!["USD", "EUR", "GBP", "JPY", "AUD", "CAD", "CNY", "HKD", "SGD"];
    let currency_index = Select::new()
        .with_prompt("Preferred currency")
        .items(&currencies)
        .default(0)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Selection error: {}", e)))?;
    let currency = match currency_index {
        0 => Currency::USD,
        1 => Currency::EUR,
        2 => Currency::GBP,
        3 => Currency::JPY,
        4 => Currency::AUD,
        5 => Currency::CAD,
        6 => Currency::CNY,
        7 => Currency::HKD,
        8 => Currency::SGD,
        _ => Currency::USD,
    };
    
    // Marketing consent
    let accepts_marketing = Confirm::new()
        .with_prompt("Accepts marketing emails?")
        .default(false)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    // Password
    let password = Password::new()
        .with_prompt("Password")
        .with_confirmation("Confirm password", "Passwords do not match")
        .validate_with(|input: &String| {
            if input.len() < 8 {
                Err("Password must be at least 8 characters")
            } else if input.len() > 128 {
                Err("Password must be less than 128 characters")
            } else {
                Ok(())
            }
        })
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Password error: {}", e)))?;
    
    // Summary
    println!("\n{}", "📋 Customer Summary".bold().underline());
    println!("  Name:              {} {}", first_name, last_name);
    println!("  Email:             {}", email);
    println!("  Phone:             {}", phone.as_deref().unwrap_or("(none)"));
    println!("  Currency:          {:?}", currency);
    println!("  Accepts Marketing: {}", if accepts_marketing { "Yes".green() } else { "No".red() });
    
    // Final confirmation
    let confirmed = Confirm::new()
        .with_prompt("\nCreate this customer?")
        .default(true)
        .interact()
        .map_err(|e| rcommerce_core::Error::validation(format!("Input error: {}", e)))?;
    
    if !confirmed {
        return Err(rcommerce_core::Error::validation("Customer creation cancelled"));
    }
    
    // Hash password
    let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
        .map_err(|e| rcommerce_core::Error::validation(format!("Failed to hash password: {}", e)))?;
    
    // Insert into database
    let customer = sqlx::query_as::<_, CustomerRecord>(
        r#"
        INSERT INTO customers (
            email, first_name, last_name, phone, accepts_marketing, 
            currency, password_hash, is_verified
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, email, first_name, last_name, created_at
        "#
    )
    .bind(&email)
    .bind(&first_name)
    .bind(&last_name)
    .bind(&phone)
    .bind(accepts_marketing)
    .bind(currency)
    .bind(&password_hash)
    .bind(true) // is_verified
    .fetch_one(pool)
    .await?;
    
    Ok(customer)
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
