pub struct ShippoClient {
    pub api_token: String,
}

impl ShippoClient {
    pub fn new(api_token: String) -> Self {
        ShippoClient { api_token }
    }

    pub async fn create_shipment(&self, tenant_id: &str) -> Result<String, String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "shippo_create_shipment",
            0.15
        ).await;
        Ok("mock_shipment_123".to_string())
    }
}
