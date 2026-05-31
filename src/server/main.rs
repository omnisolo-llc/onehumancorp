#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--task" || arg == "--agent" || arg == "agent") {
        ohc_builtin_agent::run_agent().await
    } else {
        server_lib::run_server().await
    }
}
