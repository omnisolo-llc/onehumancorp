use uuid::Uuid;

pub struct JitsiClient {
    base_url: String,
}

impl JitsiClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn generate_meeting_link(&self, room_prefix: &str) -> Result<String, String> {
        let room_id = Uuid::new_v4().to_string();
        Ok(format!("{}/{}-{}", self.base_url, room_prefix, room_id))
    }
}
