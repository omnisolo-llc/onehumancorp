pub struct JitsiClient {
    _api_key: String,
}

impl JitsiClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl JitsiClient {
    pub async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        // Mock returning a video conferencing link
        Ok(format!("https://meet.jit.si/{}", meeting_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitsi_client_new() {
        let client = JitsiClient::new("test_token".to_string());
        assert_eq!(client._api_key, "test_token");
    }

    #[tokio::test]
    async fn test_jitsi_client_create_meeting() {
        let client = JitsiClient::new("test_token".to_string());
        let result = client.create_meeting("my-meeting").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://meet.jit.si/my-meeting");
    }
}
