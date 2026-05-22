pub struct JitsiClient {
    _api_key: String,
}

impl JitsiClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }

    pub async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        let safe_name = meeting_name.replace(" ", "-").to_lowercase();
        Ok(format!("https://meet.jit.si/{}", safe_name))
    }
}
