pub struct EasyPostClient {
    api_key: String,
}

impl EasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl EasyPostClient {
    pub async fn create_shipment(&self, to_address: &str, from_address: &str, parcel_details: &str) -> Result<String, String> {
        // Mock returning a shipping label url
        Ok("https://easypost.com/labels/mock_label_123.pdf".to_string())
    }
}
