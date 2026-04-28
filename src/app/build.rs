fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint_build::compile("src/app.slint").unwrap();
    
    tonic_build::configure()
        .compile_protos(
            &["../proto/hub.proto"],
            &["../proto"],
        )?;
    Ok(())
}
