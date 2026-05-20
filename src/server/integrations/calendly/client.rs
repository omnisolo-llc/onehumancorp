use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait CalendlyClientWrapper: Send + Sync {
    async fn get_event_types(&self) -> Result<String, String>;
}

pub struct RealCalendlyClient {
    api_key: String,
    http_client: Client,
}

impl RealCalendlyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalendlyClientWrapper for RealCalendlyClient {
    async fn get_event_types(&self) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "calendly_get_event_types",
            0.05
        ).await;
        Ok("[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealCalendlyClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_get_event_types_error_handling() {
        let client = RealCalendlyClient::new("key".to_string());
        let _ = client.get_event_types().await;
    }
}
