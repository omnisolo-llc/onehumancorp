        use async_trait::async_trait;
        use reqwest::Client;

        #[async_trait]
        pub trait DailyCoClientWrapper: Send + Sync {
            async fn create_room(&self) -> Result<String, String>;
        }

        pub struct RealDailyCoClient {
            api_key: String,
            base_url: String,
            http_client: Client,
        }

        impl RealDailyCoClient {
            pub fn new(api_key: String, base_url: String) -> Self {
                Self {
                    api_key,
                    base_url,
                    http_client: Client::new(),
                }
            }
        }

        #[async_trait]
        impl DailyCoClientWrapper for RealDailyCoClient {
            async fn create_room(&self) -> Result<String, String> {
    let _ = ::server_telemetry::record_api_call_cost(
        &crate::db::get_pool(),
        "unknown",
        "dailyco_create_room",
        0.05
    ).await;
    Ok(format!("{}/rooms/mock_room", self.base_url))
}
        }
