#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ohc_mono::run_server().await
}
