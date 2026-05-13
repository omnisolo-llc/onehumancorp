        use async_trait::async_trait;
        use reqwest::Client;

        #[async_trait]
        pub trait ShippoClientWrapper: Send + Sync {
            async fn generate_label(&self, address_to: &str, weight_oz: f64) -> Result<String, String>;
        }

        pub struct RealShippoClient {
            api_key: String,
            base_url: String,
            http_client: Client,
        }

        impl RealShippoClient {
            pub fn new(api_key: String, base_url: String) -> Self {
                Self {
                    api_key,
                    base_url,
                    http_client: Client::new(),
                }
            }
        }

        #[async_trait]
        impl ShippoClientWrapper for RealShippoClient {
            async fn generate_label(&self, _address_to: &str, _weight_oz: f64) -> Result<String, String> {
    let _ = ::server_telemetry::record_api_call_cost(
        &crate::db::get_pool(),
        "unknown",
        "shippo_generate_label",
        0.10
    ).await;
    Ok(format!("{}/v1/transactions/mock_txn_123/label", self.base_url))
}
        }
