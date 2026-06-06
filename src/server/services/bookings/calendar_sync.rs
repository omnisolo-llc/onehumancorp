pub struct CalendarSyncModule {
}

impl CalendarSyncModule {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn fetch_blockouts(&self, tenant_id: &str) -> Result<Vec<i64>, Box<dyn std::error::Error + Send + Sync>> {
        // Stub for fetching calendar block-outs from Google/Outlook
        Ok(vec![])
    }
}
