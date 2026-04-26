use ohc::orchestration::hub_service_client::HubServiceClient;
use ohc::orchestration::RegisterAgentRequest;
use ohc::orchestration::Agent;

pub mod ohc {
    pub mod orchestration {
        tonic::include_proto!("ohc.orchestration");
    }
}

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
                        provider_type: "Internal".into(),
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

    let ui = FixAssistant::new()?;
    ui.run()?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Running the Slint UI in a headless test environment requires specific setup.
    // Given the constraints and environment, we'll verify the proto setup here as a basic sanity check,
    // since instantiating the UI directly in this environment panics due to missing display/platform backend.
    #[test]
    fn test_proto_struct_initialization() {
        let agent = Agent {
            id: "test_1".into(),
            name: "Test Agent".into(),
            role: "Worker".into(),
            organization_id: "org_1".into(),
            status: "Running".into(),
            provider_type: "Internal".into(),
        };
        assert_eq!(agent.provider_type, "Internal");
        assert_eq!(agent.id, "test_1");
    }
}
