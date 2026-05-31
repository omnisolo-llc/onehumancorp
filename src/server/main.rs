#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--task" || arg == "--agent" || arg == "agent") {
        if server_lib::is_standalone_runtime() {
            ohc_builtin_agent::run_agent().await
        } else {
            let message =
                "cluster mode uses the separate ohc-builtin-agent binary; run that binary for agent tasks";
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                message,
            )
            .into())
        }
    } else {
        server_lib::run_server().await
    }
}
