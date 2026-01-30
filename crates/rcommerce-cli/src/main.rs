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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parse() {
        let cli = Cli::parse_from(&["rcommerce", "server"]);
        assert!(matches!(cli.command, Commands::Server { .. }));
    }
}
