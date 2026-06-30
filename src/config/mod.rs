use std::env;

/// Database configuration
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    /// Load database configuration from environment
    pub fn from_env() -> Self {
        let url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        Self { url }
    }
}
