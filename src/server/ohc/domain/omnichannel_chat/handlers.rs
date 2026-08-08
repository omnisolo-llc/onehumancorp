use super::models::Message;
use super::engine::{CustomerIdentityResolutionEngine, AmbassadorAgent};

pub struct OmnichannelGateway {
    identity_engine: CustomerIdentityResolutionEngine,
    agent: AmbassadorAgent,
}

impl OmnichannelGateway {
    pub fn new(identity_engine: CustomerIdentityResolutionEngine, agent: AmbassadorAgent) -> Self {
        Self {
            identity_engine,
            agent,
        }
    }

    pub fn receive_webhook(&mut self, payload: &serde_json::Value) -> Result<(), String> {
        let channel = payload["channel"].as_str().ok_or("Missing channel")?;
        let handle = payload["handle"].as_str().ok_or("Missing handle")?;
        let content = payload["content"].as_str().ok_or("Missing content")?;
        let tenant_id = payload["tenant_id"].as_str().ok_or("Missing tenant_id")?;

        let customer = self.identity_engine.resolve_identity(channel, handle);
        if let Some(c) = customer {
            let msg = Message {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                customer_id: c.id.clone(),
                channel: channel.to_string(),
                content: content.to_string(),
                direction: "inbound".to_string(),
            };

            let draft = self.agent.process_message(&msg, &c);
            // In a real system, push to ActionRequiredQueue here.
            println!("Draft created: {:?}", draft);
            Ok(())
        } else {
            Err("Customer not found".to_string())
        }
    }
}
