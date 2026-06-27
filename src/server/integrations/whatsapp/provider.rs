use crate::client::{RealWhatsAppClient, WhatsAppClientWrapper};
use std::sync::Arc;

pub struct WhatsAppProvider {
    client: Arc<dyn WhatsAppClientWrapper>,
}

impl WhatsAppProvider {
    pub fn new(access_token: String, phone_number_id: String) -> Self {
        Self {
            client: Arc::new(RealWhatsAppClient::new(access_token, phone_number_id)),
        }
    }

    pub async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message(to, body).await
    }
}
