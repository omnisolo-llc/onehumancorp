use super::client::MetaClientWrapper;
use std::sync::Arc;

pub struct WhatsAppCloudApi {
    client: Arc<dyn MetaClientWrapper>,
}

impl WhatsAppCloudApi {
    pub fn new(client: Arc<dyn MetaClientWrapper>) -> Self {
        Self { client }
    }

    pub async fn send_reply(&self, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message("whatsapp", None, to, body).await
    }
}
