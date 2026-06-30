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

    tracing::info!("Starting Rust Auth Service");

    // Load .env (safe & idempotent)
    dotenv::dotenv().ok();

    // Initialize database connection (non-fatal failure for demo)
    match Database::new().await {
        Ok(_db) => {
            tracing::info!("Database connection established successfully");
        }
        Err(e) => {
            tracing::error!("Failed to connect to database: {}", e);
            tracing::warn!("Run `cargo run` again after fixing DATABASE_URL in .env");
            // Continue anyway — don't exit with error unless you want strict behavior
        }
    }

    Ok(())
}
