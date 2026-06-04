use crate::client::{TaxJarClient, TaxRequest, TaxResponse};

pub struct TaxJarProvider {
    client: TaxJarClient,
}

impl TaxJarProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: TaxJarClient::new(api_key),
        }
    }

    pub async fn get_tax_for_order(&self, amount: f64, shipping: f64, dest_zip: String) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        let req = TaxRequest {
            from_country: "US".to_string(),
            from_zip: "92093".to_string(), // Stub origin
            from_state: "CA".to_string(),
            to_country: "US".to_string(),
            to_zip: dest_zip,
            to_state: "CA".to_string(),
            amount,
            shipping,
        };

        let response = self.client.calculate_tax(&req).await?;
        Ok(response.tax.amount_to_collect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_stub() {
        let provider = TaxJarProvider::new("dummy_key".to_string());
        let tax = provider.get_tax_for_order(100.0, 10.0, "90210".to_string()).await.unwrap();
        assert_eq!(tax, 8.25);
    }
}
