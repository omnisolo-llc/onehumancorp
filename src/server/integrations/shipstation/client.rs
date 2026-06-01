pub struct ShipStationClient {
    _api_key: String,
}

impl ShipStationClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl ShipStationClient {
    pub async fn mock_method(&self) -> Result<(), String> {
        Ok(())
    }
}
