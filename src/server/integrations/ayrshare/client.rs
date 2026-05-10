pub struct AyrshareClient {
    api_key: String,
}

impl AyrshareClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn link_social_account(&self, platform: &str, _oauth_token: &str) -> Result<String, String> {
        Ok(format!("Linked {} account with Ayrshare", platform))
    }

    pub async fn fetch_messages(&self) -> Result<Vec<String>, String> {
        Ok(vec!["New Instagram DM from Maya", "New Facebook Comment from Carlos"].into_iter().map(String::from).collect())
    }

    pub async fn send_reply(&self, platform: &str, user_id: &str, message: &str) -> Result<(), String> {
        println!("Sending reply to {} on {}: {}", user_id, platform, message);
        Ok(())
    }

    pub async fn schedule_post(&self, content: &str, platforms: Vec<&str>) -> Result<String, String> {
        println!("Scheduling post on {:?}: {}", platforms, content);
        Ok("fake-post-id".to_string())
    }
}
