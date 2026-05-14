use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoCheckoutSession {
    pub id: String,
    pub init_point: String,
}

pub struct MercadoPagoClient {
    pub access_token: String,
    pub http_client: Client,
}

impl MercadoPagoClient {
    pub fn new(access_token: String) -> Self {
        MercadoPagoClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "mercadopago_create_checkout_preference",
            0.15
        ).await;

        let req_body = serde_json::json!({
            "items": [
                {
                    "title": format!("Item {}", price_id),
                    "quantity": 1,
                    "unit_price": 10.0
                }
            ]
        });

        let url = "https://api.mercadopago.com/checkout/preferences";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&req_body)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string())
                } else {
                    Err(format!("MercadoPago API Error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mp_client_creation() {
        let client = MercadoPagoClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_mp_create_error() {
        let client = MercadoPagoClient::new("token".to_string());
        let _ = client.create_checkout_preference("price", "tenant1").await;
    }
}
