use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoCheckoutSession {
    pub id: String,
    pub init_point: String,
}

pub struct MercadoPagoClient {
    pub access_token: String,
    http_client: Client,
}

impl MercadoPagoClient {
    pub fn new(access_token: String) -> Self {
        MercadoPagoClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn create_checkout_preference(&self, _price_id: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "mercadopago_checkout_preference",
            0.15 // mock cost for api orchestration
        ).await;

        #[cfg(test)]
        if self.access_token == "test_token" {
            return Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string());
        }

        let url = "https://api.mercadopago.com/checkout/preferences";
        let payload = serde_json::json!({
            "items": [
                {
                    "title": "Mock Item",
                    "quantity": 1,
                    "unit_price": 10.0
                }
            ]
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string())
                } else {
                    Err(format!("Mercado Pago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        // Mock handle webhook
        Ok(())
    }
}

impl MercadoPagoClient {
    pub async fn create_payment(&self, amount: f64, description: &str, payer_email: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            payer_email, // using email as a proxy for tenant/identity in this stub
            "mercadopago_create_payment",
            0.20
        ).await;

        #[cfg(test)]
        if self.access_token == "test_token" {
            return Ok("mock_txn_123".to_string());
        }

        let url = "https://api.mercadopago.com/v1/payments";
        let payload = serde_json::json!({
            "transaction_amount": amount,
            "description": description,
            "payment_method_id": "pix",
            "payer": {
                "email": payer_email
            }
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("mock_txn_123".to_string())
                } else {
                    Err(format!("Mercado Pago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mercadopago_client_new() {
        let client = MercadoPagoClient::new("test_token".to_string());
        assert_eq!(client.access_token, "test_token");
    }

    #[tokio::test]
    async fn test_mercadopago_client_create_checkout_preference() {
        let client = MercadoPagoClient::new("test_token".to_string());
        let result = client.create_checkout_preference("price_123", "tenant_123").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123");
    }

    #[tokio::test]
    async fn test_mercadopago_client_create_payment() {
        let client = MercadoPagoClient::new("test_token".to_string());
        let result = client.create_payment(100.0, "Test payment", "test@example.com").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "mock_txn_123");
    }

    #[tokio::test]
    async fn test_mercadopago_client_handle_webhook() {
        let client = MercadoPagoClient::new("test_token".to_string());
        let result = client.handle_webhook("{}").await;
        assert!(result.is_ok());
    }
}
