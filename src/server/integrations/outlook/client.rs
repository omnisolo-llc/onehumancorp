use async_trait::async_trait;

#[async_trait]
pub trait OutlookClientWrapper: Send + Sync {
    async fn sync_calendar(&self) -> Result<(), String>;
}

pub struct RealOutlookClient {
    pub access_token: String,
}

impl RealOutlookClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}

#[async_trait]
impl OutlookClientWrapper for RealOutlookClient {
    async fn sync_calendar(&self) -> Result<(), String> {
        // Mock sync
        Ok(())
    }
}
