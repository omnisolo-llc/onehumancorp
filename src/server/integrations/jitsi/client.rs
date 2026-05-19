use uuid::Uuid;

pub struct JitsiClient {
    pub domain: String,
}

impl JitsiClient {
    pub fn new(domain: String) -> Self {
        Self { domain }
    }

    pub fn generate_meeting_url(&self, room_name: &str) -> String {
        format!("https://{}/{}", self.domain, room_name)
    }

    pub fn generate_unique_meeting_url(&self) -> String {
        let room_name = Uuid::new_v4().to_string();
        self.generate_meeting_url(&room_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitsi_url_gen() {
        let client = JitsiClient::new("meet.jit.si".to_string());
        let url = client.generate_meeting_url("test-room");
        assert_eq!(url, "https://meet.jit.si/test-room");
    }

    #[test]
    fn test_jitsi_unique_url_gen() {
        let client = JitsiClient::new("meet.jit.si".to_string());
        let url = client.generate_unique_meeting_url();
        assert!(url.starts_with("https://meet.jit.si/"));
        assert!(url.len() > "https://meet.jit.si/".len());
    }
}
