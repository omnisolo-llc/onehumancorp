#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server_lib::run_server().await
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod main_tests;
