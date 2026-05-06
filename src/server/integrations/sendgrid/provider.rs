use crate::integrations::sendgrid::client::{SendgridClientWrapper, RealSendgridClient};
use async_trait::async_trait;

pub struct SendgridProvider {
    client: Box<dyn SendgridClientWrapper>,
}

impl SendgridProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Box::new(RealSendgridClient::new(api_key)),
        }
    }

    pub async fn send_email(&self, to: &str, from: &str, subject: &str, body: &str) -> Result<(), String> {
        self.client.send_email(to, from, subject, body).await
    }
}
