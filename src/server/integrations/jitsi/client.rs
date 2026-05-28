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
        // Jitsi Meet uses URL-safe room names. We sanitize the meeting name by
        // replacing spaces with hyphens and removing non-alphanumeric characters.
        let sanitized_name: String = meeting_name
            .chars()
            .map(|c| if c.is_whitespace() { '-' } else { c })
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();

        let sanitized_name = sanitized_name.trim_matches('-').to_string();

        if sanitized_name.is_empty() {
            return Err("Invalid meeting name".to_string());
        }

        Ok(format!("https://meet.jit.si/{}", sanitized_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitsi_client_new() {
        let client = JitsiClient::new("test_token".to_string());
        assert_eq!(client.api_key, "test_token");
    }

    #[tokio::test]
    async fn test_jitsi_client_create_meeting() {
        let client = JitsiClient::new("test_token".to_string());

        // Test basic name
        let result = client.create_meeting("WeeklySync").await;
        assert_eq!(result.unwrap(), "https://meet.jit.si/WeeklySync");

        // Test name with spaces and special characters
        let result = client.create_meeting("Weekly Sync & Review!").await;
        assert_eq!(result.unwrap(), "https://meet.jit.si/Weekly-Sync--Review");

        // Test invalid name
        let result = client.create_meeting("  !!!  ").await;
        assert!(result.is_err());
    }
}
