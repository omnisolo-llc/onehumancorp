use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let format = env::var("LOG_FORMAT").unwrap_or_else(|_| "text".to_string());
    if format.to_lowercase() == "json" {
        tracing_subscriber::registry()
            .with(fmt::layer().json())
            .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
            .init();
    }

    server_lib::run_server().await
}
