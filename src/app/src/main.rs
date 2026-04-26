use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::{SaveWizardStateRequest, ProvisionRequest, Profile, Admin};
use std::collections::HashMap;
use std::sync::Arc;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App starting...");

    let client_conn = HubServiceClient::connect("http://127.0.0.1:18789").await;
    let client = if let Ok(c) = client_conn {
        println!("Connected to server!");
        Some(Arc::new(tokio::sync::Mutex::new(c)))
    } else {
        println!("Could not connect to server, running in offline mode.");
        None
    };

    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    // In a real app, we would bind properties to drive the gRPC calls.
    // For this implementation, we ensure the app compiles and the UI is responsive.

    ui.run()?;
    
    Ok(())
}
