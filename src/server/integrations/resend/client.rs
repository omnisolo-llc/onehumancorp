use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait ResendClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, html_body: &str) -> Result<(), String>;
}

pub struct RealResendClient {
    pub api_key: String,
}

impl RealResendClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl ResendClientWrapper for RealResendClient {
    async fn send_email(&self, to: &str, subject: &str, html_body: &str) -> Result<(), String> {
        // Mock Resend email send
        tracing::info!("Sending Resend email to {}: [{}] {}", to, subject, html_body);
        Ok(())
    }
}
