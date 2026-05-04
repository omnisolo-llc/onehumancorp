#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    app_lib::run_app()
}

#[cfg(target_arch = "wasm32")]
fn main() {}
