use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing::info;

/// Database connection pool
pub struct Database {
    pool: sqlx::PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        info!("Connecting to database at {}", database_url);
        
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;
        
        // Test connection with a simple query
        sqlx::query("SELECT 1").fetch_one(&pool).await?;
        
        info!("Database connection successful");
        
        Ok(Self { pool })
    }
    
    /// Get the connection pool
    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }
}

impl Default for Database {
    fn default() -> Self {
        panic!("Database must be created with Database::new().await")
    }
}
