mod db;
mod rabbit;
mod routes;
mod handlers;
mod config;

use std::env;
use axum::{routing::get, Router};
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize database connection pool
    let db_pool = db::init_pool().await.expect("Failed to initialize database pool");

    // Initialize RabbitMQ connection
    let rabbit_connection = rabbit::init_connection().await.expect("Failed to initialize RabbitMQ connection");

    // Create shared state
    let shared_state = Arc::new(Mutex::new((db_pool, rabbit_connection)));

    // Build application
    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .with_state(shared_state)
        .layer(CorsLayer::permissive());

    // Get port from environment or use default
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("Starting server on {}", addr);
    
    // Run server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
