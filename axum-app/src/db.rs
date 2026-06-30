use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Test connection
    pool.ping().await?;
    
    Ok(pool)
}
