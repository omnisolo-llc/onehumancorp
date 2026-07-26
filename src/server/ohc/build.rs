<<<<<<< HEAD
=======
use std::env;

>>>>>>> b1e691c9d (Research Report: Implement Custom Rust Omnichannel Chat System (#35333))
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
        "../../../src/proto/assistant.proto",
        "../../../src/proto/delivery.proto",
        "../../../src/proto/docs.proto",
        "../../../src/proto/invoice.proto",
        "../../../src/proto/ledger.proto",
        "../../../src/proto/model.proto",
        "../../../src/proto/skills.proto",
        "../../../src/proto/supply_chain.proto",
        "../../../src/proto/inventory.proto",
    ];

<<<<<<< HEAD
=======
    if env::var("OUT_DIR").is_err() {
        env::set_var("OUT_DIR", std::env::temp_dir().join("ohc_protos").to_str().unwrap());
    }

>>>>>>> b1e691c9d (Research Report: Implement Custom Rust Omnichannel Chat System (#35333))
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&protos, &["../../.."])?;

    Ok(())
}
