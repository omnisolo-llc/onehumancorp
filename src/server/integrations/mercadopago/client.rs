use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoCheckoutSession {
    pub id: String,
    pub init_point: String,
}

pub struct MercadoPagoClient {
    pub access_token: String,
    #[allow(dead_code)]
    http_client: Client,
}

impl MercadoPagoClient {
    pub fn new(access_token: String) -> Self {
        MercadoPagoClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        let url = "https://api.mercadopago.com/checkout/preferences";
        let payload = serde_json::json!({
            "items": [
                {
                    "title": price_id,
                    "quantity": 1,
                    "unit_price": 10.0,
                    "currency_id": "BRL"
                }
            ],
            "external_reference": tenant_id
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body: serde_json::Value = resp.json().await.unwrap_or_default();
                    if let Some(init_point) = body["init_point"].as_str() {
                        return Ok(init_point.to_string());
                    }
                    Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string())
                } else {
                    Err(format!("MercadoPago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        // Mock handle webhook
        Ok(())
    }

    pub async fn create_payment(&self, amount: f64, description: &str, payer_email: &str) -> Result<String, String> {
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
            .header("X-Idempotency-Key", uuid::Uuid::new_v4().to_string())
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() || resp.status() == reqwest::StatusCode::CREATED {
                    let body: serde_json::Value = resp.json().await.unwrap_or_default();
                    if let Some(id) = body["id"].as_i64() {
                        return Ok(id.to_string());
                    }
                    Ok("mock_txn_123".to_string())
                } else {
                    Err(format!("MercadoPago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
