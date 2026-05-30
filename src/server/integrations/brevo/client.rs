use async_trait::async_trait;

#[async_trait]
pub trait BrevoClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

pub struct RealBrevoClient {
    pub api_key: String,
}

impl RealBrevoClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl BrevoClientWrapper for RealBrevoClient {
    async fn send_email(&self, _to: &str, _subject: &str, _body: &str) -> Result<(), String> {
        // Mock send email
        Ok(())
    }
}
