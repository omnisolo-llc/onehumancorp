use async_trait::async_trait;

#[async_trait]
pub trait MessageBirdClientWrapper: Send + Sync {
    async fn send_sms(&self, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealMessageBirdClient {
    pub access_key: String,
}

impl RealMessageBirdClient {
    pub fn new(access_key: String) -> Self {
        Self { access_key }
    }
}

#[async_trait]
impl MessageBirdClientWrapper for RealMessageBirdClient {
    async fn send_sms(&self, _to: &str, _body: &str) -> Result<(), String> {
        // Mock send SMS
        Ok(())
    }
}
