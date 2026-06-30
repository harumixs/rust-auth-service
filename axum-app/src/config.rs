use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL", default = "postgresql://postgres:postgres@localhost:5432/rust_auth")]
    pub database_url: String,

    #[envconfig(from = "RABBITMQ_URL", default = "amqp://guest:guest@localhost:5672")]
    pub rabbitmq_url: String,

    #[envconfig(from = "PORT", default = "8000")]
    pub port: u16,
}
