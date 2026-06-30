use lapin::{Connection, ConnectionOptions, AMQPResult};

pub async fn init_connection() -> AMQPResult<Connection> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672/".to_string());
    
    let connection = Connection::connect(&rabbitmq_url, ConnectionOptions::default())
        .await?;
    
    println!("RabbitMQ connection established");
    
    Ok(connection)
}
