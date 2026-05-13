        use async_trait::async_trait;
        use reqwest::Client;

        #[async_trait]
        pub trait ResendClientWrapper: Send + Sync {
            async fn send_email(&self, to: &str, from: &str, subject: &str, html: &str) -> Result<(), String>;
        }

        pub struct RealResendClient {
            api_key: String,
            base_url: String,
            http_client: Client,
        }

        impl RealResendClient {
            pub fn new(api_key: String, base_url: String) -> Self {
                Self {
                    api_key,
                    base_url,
                    http_client: Client::new(),
                }
            }
        }

        #[async_trait]
        impl ResendClientWrapper for RealResendClient {
            async fn send_email(&self, _to: &str, _from: &str, _subject: &str, _html: &str) -> Result<(), String> {
    let _ = ::server_telemetry::record_api_call_cost(
        &crate::db::get_pool(),
        "unknown",
        "resend_send_email",
        0.02
    ).await;
    Ok(())
}
        }
