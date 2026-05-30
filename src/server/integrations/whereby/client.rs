use async_trait::async_trait;

#[async_trait]
pub trait WherebyClientWrapper: Send + Sync {
    async fn create_meeting(&self, meeting_name: &str) -> Result<String, String>;
}

pub struct RealWherebyClient {
    pub api_key: String,
}

impl RealWherebyClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl WherebyClientWrapper for RealWherebyClient {
    async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        // Mock create meeting
        Ok(format!("https://whereby.com/{}", meeting_name))
    }
}
