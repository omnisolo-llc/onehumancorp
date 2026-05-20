pub struct JitsiClient {
    api_key: String,
}

impl JitsiClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl JitsiClient {
    pub async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        // Mock returning a video conferencing link
        Ok(format!("https://meet.jit.si/{}", meeting_name))
    }
}
