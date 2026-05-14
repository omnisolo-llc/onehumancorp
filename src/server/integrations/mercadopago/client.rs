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

        let url = "https://api.mercadopago.com/checkout/preferences";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "items": [
                    {
                        "title": format!("OHC Plan: {}", price_id),
                        "quantity": 1,
                        "currency_id": "BRL",
                        "unit_price": 100.0
                    }
                ],
                "external_reference": tenant_id
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
                    let init_point = body["init_point"].as_str().ok_or("Missing init_point in response")?;
                    Ok(init_point.to_string())
                } else {
                    let status = resp.status();
                    let error_body = resp.text().await.unwrap_or_default();
                    Err(format!("Mercado Pago API error ({}): {}", status, error_body))
                }
            }
            Err(e) => Err(format!("Network error while contacting Mercado Pago: {}", e)),
        }
    }
}
