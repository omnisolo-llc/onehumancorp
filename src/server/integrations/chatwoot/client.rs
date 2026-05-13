        use async_trait::async_trait;
        use reqwest::Client;

        #[async_trait]
        pub trait ChatwootClientWrapper: Send + Sync {
            async fn send_message(&self, inbox_id: &str, contact_id: &str, content: &str) -> Result<(), String>;
        }

        pub struct RealChatwootClient {
            api_key: String,
            base_url: String,
            http_client: Client,
        }

        impl RealChatwootClient {
            pub fn new(api_key: String, base_url: String) -> Self {
                Self {
                    api_key,
                    base_url,
                    http_client: Client::new(),
                }
            }
        }

        #[async_trait]
        impl ChatwootClientWrapper for RealChatwootClient {
            async fn send_message(&self, _inbox_id: &str, _contact_id: &str, _content: &str) -> Result<(), String> {
    let _ = ::server_telemetry::record_api_call_cost(
        &crate::db::get_pool(),
        "unknown",
        "chatwoot_send_message",
        0.01
    ).await;
    Ok(())
}
        }
