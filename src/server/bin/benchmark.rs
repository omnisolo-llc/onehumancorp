#[tokio::main]
async fn main() {
    // Initialize tracing to see the output from tracing::info!
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting OmniSolo Server Benchmarks (Telemetry & Costs)");
    ::server_lib::benchmarks::benchmark_telemetry::run_telemetry_benchmarks().await;
    tracing::info!("All Benchmarks Completed.");
}
