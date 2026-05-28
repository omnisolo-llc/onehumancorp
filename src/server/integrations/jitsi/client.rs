pub struct JitsiClient {
    pub api_key: String,
}

impl JitsiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl JitsiClient {
    pub async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        // Jitsi links are auto-generated from URLs without prior API creation on free tiers
        Ok(format!("https://meet.jit.si/{}", meeting_name))
    }
}
