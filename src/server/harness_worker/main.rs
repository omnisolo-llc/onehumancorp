#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned()))
        .init();
    if std::env::args().nth(1).as_deref() == Some("--local-service-daemon") {
        return omnisolo_harness_worker::local_services::run_daemon().await;
    }
    let config = omnisolo_harness_worker::WorkerConfig::from_env()?;
    tracing::info!(
        worker_id = %config.worker_id,
        harness_id = %config.harness_id,
        pool_id = %config.pool_id,
        grpc_addr = %config.grpc_addr,
        health_addr = %config.health_addr,
        "starting OmniSolo harness worker"
    );
    omnisolo_harness_worker::run(config).await
}
