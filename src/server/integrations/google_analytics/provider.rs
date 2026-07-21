use super::client::GoogleAnalyticsClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleAnalyticsProvider {
    client: Arc<GoogleAnalyticsClient>,
    metadata: ProviderMetadata,
}

impl GoogleAnalyticsProvider {
    pub fn new(access_token: String, property_id: String) -> Self {
        let client = GoogleAnalyticsClient::new(access_token, property_id);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "google_analytics".to_string(),
                name: "Google Analytics".to_string(),
                category: "analytics".to_string(),
                base_url: "https://analyticsdata.googleapis.com/v1beta".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<GoogleAnalyticsClient>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "google_analytics".to_string(),
                name: "Google Analytics".to_string(),
                category: "analytics".to_string(),
                base_url: "https://analyticsdata.googleapis.com/v1beta".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            },
        }
    }

    pub async fn get_realtime_report(
        &self,
        metrics: &[String],
        dimensions: &[String],
    ) -> Result<super::client::GAReport, String> {
        self.client.get_realtime_report(metrics, dimensions).await
    }

    pub async fn get_report(
        &self,
        date_range_start: &str,
        date_range_end: &str,
        metrics: &[String],
        dimensions: &[String],
    ) -> Result<super::client::GAReport, String> {
        self.client
            .get_report(date_range_start, date_range_end, metrics, dimensions)
            .await
    }

    pub async fn get_visitors(&self, days: u32) -> Result<(u64, u64), String> {
        self.client.get_visitors(days).await
    }

    pub async fn get_top_pages(&self, limit: u32) -> Result<Vec<(String, u64)>, String> {
        self.client.get_top_pages(limit).await
    }

    pub async fn get_traffic_sources(&self, days: u32) -> Result<Vec<(String, u64)>, String> {
        self.client.get_traffic_sources(days).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_analytics_provider_new() {
        let provider =
            GoogleAnalyticsProvider::new("test-token".to_string(), "properties/123".to_string());
        assert_eq!(provider.metadata.id, "google_analytics");
        assert_eq!(provider.metadata.category, "analytics");
    }

    #[test]
    fn test_google_analytics_provider_to_integration_provider() {
        let provider =
            GoogleAnalyticsProvider::new("test-token".to_string(), "properties/123".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "google_analytics");
    }

    #[test]
    fn test_google_analytics_provider_with_client() {
        let client = Arc::new(GoogleAnalyticsClient::new(
            "test-token".to_string(),
            "properties/123".to_string(),
        ));
        let provider = GoogleAnalyticsProvider::with_client(client);
        assert_eq!(provider.metadata.id, "google_analytics");
    }
}
