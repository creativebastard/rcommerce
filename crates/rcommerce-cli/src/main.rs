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
    
    /// Reset database (DANGEROUS)
    Reset,
    
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
        Commands::Server { host, port } => {
            // Override config with CLI arguments
            let mut config = config;
            config.server.host = host;
            config.server.port = port;
            
            rcommerce_api::run(config).await?;
        }
        
        Commands::Db { command } => {
            use colored::*;
            match command {
                DbCommands::Migrate => {
                    println!("{}", "Running migrations...".yellow());
                    println!("Use: psql < migrations/001_initial_schema.sql");
                }
                DbCommands::Reset => {
                    println!("{}", "Resetting database...".red().bold());
                    println!("DANGER: This will delete all data!");
                }
                DbCommands::Seed => {
                    println!("{}", "Seeding sample data...".green());
                }
                DbCommands::Status => {
                    println!("Database: {}", config.database.database);
                    println!("Host: {}:{}", config.database.host, config.database.port);
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_parse() {
        let cli = Cli::parse_from(&["rcommerce", "server"]);
        assert!(matches!(cli.command, Commands::Server { .. }));
    }
}