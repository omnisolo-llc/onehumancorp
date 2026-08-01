use std::sync::Arc;
use server_integrations_meta::client::MetaClientWrapper;

pub struct WhatsAppProvider {
    client: Arc<dyn MetaClientWrapper>,
}

impl WhatsAppProvider {
    pub fn new(client: Arc<dyn MetaClientWrapper>) -> Self {
        Self { client }
    }

    pub async fn send_reply(&self, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message("whatsapp", None, to, body).await
    }
}
