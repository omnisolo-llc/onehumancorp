use crate::integrations::calendly::client::CalendlyClient;

pub struct CalendlyProvider {
    #[allow(dead_code)]
    client: CalendlyClient,
}

impl CalendlyProvider {
    pub fn new(api_token: String) -> Self {
        Self {
            client: CalendlyClient::new(api_token),
        }
    }
    pub async fn list_event_types(&self, tenant_id: &str) -> Result<Vec<crate::integrations::calendly::client::CalendlyEventType>, String> {
        self.client.list_event_types(tenant_id).await
    }
}
