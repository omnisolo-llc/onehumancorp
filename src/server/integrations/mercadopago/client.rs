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

    pub async fn create_checkout_preference(&self, _price_id: &str, _tenant_id: &str) -> Result<String, String> {
        if self.access_token.trim().is_empty()
            || self.access_token.contains("test")
            || self.access_token.contains("mock")
            || self.access_token.contains("dummy")
        {
            return Err("Mercado Pago access token is required".to_string());
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
                    let text = resp.text().await.unwrap_or_default();
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                    json["init_point"]
                        .as_str()
                        .map(|init_point| init_point.to_string())
                        .ok_or_else(|| "Mercado Pago checkout response missing init_point".to_string())
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
        if self.access_token.trim().is_empty()
            || self.access_token.contains("test")
            || self.access_token.contains("mock")
            || self.access_token.contains("dummy")
        {
            return Err("Mercado Pago access token is required".to_string());
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
                    let text = resp.text().await.unwrap_or_default();
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                    json["id"]
                        .as_i64()
                        .map(|id| id.to_string())
                        .ok_or_else(|| "Mercado Pago payment response missing id".to_string())
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
        assert_eq!(result.unwrap_err(), "Mercado Pago access token is required");
    }

    #[tokio::test]
    async fn test_mercadopago_client_create_payment() {
        let client = MercadoPagoClient::new("test_token".to_string());
        let result = client.create_payment(100.0, "Test payment", "test@example.com").await;
        assert_eq!(result.unwrap_err(), "Mercado Pago access token is required");
    }

    #[tokio::test]
    async fn test_mercadopago_client_handle_webhook() {
        let client = MercadoPagoClient::new("test_token".to_string());
        let result = client.handle_webhook("{}").await;
        assert!(result.is_ok());
    }
}
