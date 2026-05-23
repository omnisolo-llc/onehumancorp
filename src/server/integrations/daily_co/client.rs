pub struct DailyCoClient {
    _api_key: String,
}

impl DailyCoClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl DailyCoClient {
    pub async fn create_room(&self, room_name: &str) -> Result<String, String> {
        // Mock returning a video conferencing link
        Ok(format!("https://ohc.daily.co/{}", room_name))
    }
}
