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


    // Initialize FixAgent so we don't break existing UI behavior if expected
    let ui_fix = FixAgent::new()?;
    // But since the task requires launching the BusinessSetup, we will spawn BusinessSetup instead.
    // However, to keep FixAgent compilable and present in `main.rs`, we instantiate it but don't run it if BusinessSetup is requested.
    // Given that `ui_fix.run()?` takes over the thread, we will just comment out ui_fix.run() and run BusinessSetup.

    let ui = BusinessSetup::new()?;

    let ui_weak = ui.as_weak();
    ui.on_save_state(move |step, btype, name, payment, email| {
        println!("Saving state: step={}, type={}, name={}", step, btype, name);
        tokio::spawn(async move {
            if let Ok(mut client) = HubServiceClient::connect("http://127.0.0.1:18789").await {
                let mut state = HashMap::new();
                state.insert("step".to_string(), step.to_string());
                state.insert("business_type".to_string(), btype.to_string());
                state.insert("company_name".to_string(), name.to_string());
                state.insert("payment_pref".to_string(), payment.to_string());
                state.insert("admin_email".to_string(), email.to_string());

                let req = tonic::Request::new(SaveWizardStateRequest { state });
                let _ = client.save_wizard_state(req).await;
            }
        });
    });

    let ui_weak2 = ui.as_weak();
    ui.on_launch(move |name, btype, email| {
        println!("Launch triggered: {} {} {}", name, btype, email);
        tokio::spawn(async move {
            if let Ok(mut client) = HubServiceClient::connect("http://127.0.0.1:18789").await {
                let mut state = HashMap::new();
                state.insert("company_name".to_string(), name.to_string());
                state.insert("business_type".to_string(), btype.to_string());
                state.insert("admin_email".to_string(), email.to_string());
                state.insert("status".to_string(), "launched".to_string());

                let req = tonic::Request::new(SaveWizardStateRequest { state });
                let _ = client.save_wizard_state(req).await;
            }
        });
    });

    ui.run()?;

    
    Ok(())
}
