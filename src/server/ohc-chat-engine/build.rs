fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("PROTOC").is_err() {
        if let Ok(path) = protoc_bin_vendored::protoc_bin_path() {
            unsafe { std::env::set_var("PROTOC", path); }
        }
    }
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &[
                "proto/chat_engine.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
