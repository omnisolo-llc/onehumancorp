pub struct ShippoClient {
    api_key: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn get_rates(&self) -> Result<String, String> {
        Ok("Mock rates".to_string())
    }
}
