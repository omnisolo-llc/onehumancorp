pub struct ShippoClient {
    pub api_key: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient { api_key }
    }

    pub async fn create_shipping_label(&self, _address_from: &str, _address_to: &str, _parcel_details: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "shippo_create_shipping_label",
            0.05
        ).await;
        Ok("mock_label_url".to_string())
    }
}
