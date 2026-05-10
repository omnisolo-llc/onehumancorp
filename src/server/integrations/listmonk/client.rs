pub struct ListmonkClient {
    base_url: String,
    username: String,
    password: Option<String>,
}

impl ListmonkClient {
    pub fn new(base_url: String, username: String, password: Option<String>) -> Self {
        Self { base_url, username, password }
    }

    pub async fn create_campaign(&self, _list_ids: Vec<i32>, name: &str, _subject: &str, _body: &str) -> Result<i32, String> {
        println!("Creating Listmonk campaign: {}", name);
        Ok(123)
    }

    pub async fn send_campaign(&self, campaign_id: i32) -> Result<(), String> {
        println!("Sending Listmonk campaign: {}", campaign_id);
        Ok(())
    }
}
