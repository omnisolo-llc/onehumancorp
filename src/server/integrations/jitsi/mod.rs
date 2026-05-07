pub struct JitsiProvider {
    base_url: String,
}

impl JitsiProvider {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url: base_url.is_empty().then(|| "https://meet.jit.si".to_string()).unwrap_or(base_url),
        }
    }

    pub fn generate_meeting_link(&self, room_name: &str) -> String {
        format!("{}/{}", self.base_url, room_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitsi_link_generation() {
        let provider = JitsiProvider::new("".to_string());
        let link = provider.generate_meeting_link("test-room");
        assert_eq!(link, "https://meet.jit.si/test-room");
    }
}
