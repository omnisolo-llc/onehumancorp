use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait ResendClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, html: &str) -> Result<(), String>;
}

pub struct RealResendClient {
    api_key: String,
}

impl RealResendClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl ResendClientWrapper for RealResendClient {
    async fn send_email(&self, _to: &str, _subject: &str, _html: &str) -> Result<(), String> {
        // Mock implementation
        Ok(())
    }
}
