use crate::integrations::manychat::client::{RealManychatClient, ManychatClientWrapper};

pub struct ManychatProvider {
    #[allow(dead_code)]
    client: Box<dyn ManychatClientWrapper>,
}

impl ManychatProvider {
    pub fn new(api_token: String) -> Self {
        Self {
            client: Box::new(RealManychatClient::new(api_token)),
        }
    }

    pub async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message(to, body).await
    }
}
