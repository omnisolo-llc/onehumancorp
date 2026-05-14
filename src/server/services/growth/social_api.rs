
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleSocialJobRequest {
    pub content: String,
    pub image_url: Option<String>,
    pub platforms: Vec<String>,
    pub scheduled_time_unix: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialJobResponse {
    pub job_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetSocialJobsRequest {
    pub org_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelSocialJobRequest {
    pub job_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialPlatformConfig {
    pub platform: String,
    pub is_connected: bool,
    pub token_expires_at: Option<i64>,
}
