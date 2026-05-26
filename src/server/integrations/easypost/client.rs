pub struct EasyPostClient {
    _api_key: String,
}

impl EasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl EasyPostClient {
    pub async fn create_shipment(&self, _to_address: &str, _from_address: &str, _parcel_details: &str) -> Result<String, String> {
        // Mock returning a shipping label url
        Ok("https://easypost.com/labels/mock_label_123.pdf".to_string())
    }
}
