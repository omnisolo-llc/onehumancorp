use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;

pub struct EmailCampaignService {
    registry: Arc<IntegrationsRegistry>,
}

impl EmailCampaignService {
    pub fn new(registry: Arc<IntegrationsRegistry>) -> Self {
        Self { registry }
    }

    pub async fn send_campaign(&self, tenant_id: &str, subject: &str, body: &str, recipient_list: Vec<String>) -> Result<(), String> {
        let instances = self.registry.instances_by_category(tenant_id, "email");
        if let Some(_inst) = instances.iter().find(|i| i.id == "sendgrid" && i.status == "connected") {
            tracing::info!("Sending campaign '{}' via SendGrid for tenant {}", subject, tenant_id);
            // Simulate sending to each recipient
            for recipient in recipient_list {
                tracing::debug!("Email sent to {}", recipient);
            }
            Ok(())
        } else {
            Err("Email provider not connected".to_string())
        }
    }
}
