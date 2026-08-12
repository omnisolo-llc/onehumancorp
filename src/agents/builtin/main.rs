#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    omnisolo_builtin_agent::run_agent().await
}
