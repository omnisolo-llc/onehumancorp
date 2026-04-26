use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

use ohc::orchestration::SaveWizardStateRequest;
use std::collections::HashMap;
slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("App starting...");

    tokio::spawn(async move {
        match HubServiceClient::connect("http://127.0.0.1:18789").await {
            Ok(mut client) => {
                println!("Connected to server!");
                let request = tonic::Request::new(RegisterAgentRequest {
                    agent: Some(Agent {
                        id: "agent_1".into(),
                        name: "Rust Agent".into(),
                        role: "Worker".into(),
                        organization_id: "org_1".into(),
                        status: "Running".into(),
                        provider_type: "Mock".into(),
                    }),
                });
                match client.register_agent(request).await {
                    Ok(response) => println!("RESPONSE={:?}", response),
                    Err(e) => println!("ERR={:?}", e),
                }
            }
            Err(e) => {
                println!("Could not connect to server: {:?}", e);
            }
        }
    });

    let ui = BusinessSetup::new()?;
    let ui_handle = ui.as_weak();

    ui.on_launch(move |name: slint::SharedString, btype: slint::SharedString, email: slint::SharedString| {
        println!("Launch triggered: {} {} {}", name, btype, email);
        tokio::spawn(async move {
            if let Ok(mut client) = HubServiceClient::connect("http://127.0.0.1:18789").await {
                let mut state = HashMap::new();
                state.insert("company_name".to_string(), name.to_string());
                state.insert("business_type".to_string(), btype.to_string());
                state.insert("admin_email".to_string(), email.to_string());

                let req = tonic::Request::new(SaveWizardStateRequest { state });
                let _ = client.save_wizard_state(req).await;
            }
        });
    });

    ui.run()?;
    
    Ok(())
}
