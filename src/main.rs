mod config;
mod db;

use anyhow::Result;
use db::Database;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    info!("Starting Rust Auth Service");
    
    // Initialize database connection
    let _db = Database::new().await?;
    
    info!("Database connection established successfully");
    
    // For now, just exit after successful connection
    Ok(())
}
