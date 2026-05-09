use serde::{Deserialize, Serialize};
use reqwest::Client;

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

    pub async fn create_checkout_preference(&self, price_id: &str, _tenant_id: &str) -> Result<String, String> {
        let url = "https://api.mercadopago.com/checkout/preferences";
        let req = serde_json::json!({
            "items": [
                {
                    "title": format!("Product {}", price_id),
                    "quantity": 1,
                    "unit_price": 100.0,
                    "currency_id": "BRL"
                }
            ]
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&req)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.map_err(|e| format!("Failed to read response text: {}", e))?;
                    let parsed: MercadoPagoCheckoutSession = serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {}", e))?;
                    Ok(parsed.init_point)
                } else {
                    Err(format!("Mercado Pago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn get_payment(&self, payment_id: &str) -> Result<String, String> {
        let url = format!("https://api.mercadopago.com/v1/payments/{}", payment_id);

        let res = self.http_client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.map_err(|e| format!("Failed to read response text: {}", e))?;
                    Ok(text)
                } else {
                    Err(format!("Mercado Pago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
