fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = [
        "../../../src/proto/agent.proto",
        "../../../src/proto/agent_service.proto",
        "../../../src/proto/app.proto",
        "../../../src/proto/billing.proto",
        "../../../src/proto/campaign.proto",
        "../../../src/proto/collective.proto",
        "../../../src/proto/common.proto",
        "../../../src/proto/hub.proto",
        "../../../src/proto/interop.proto",
        "../../../src/proto/mcp_proxy.proto",
        "../../../src/proto/organization.proto",
        "../../../src/proto/invoice.proto",
    ];

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&protos, &["../../.."])?;

    Ok(())
}
