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

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str, method: super::routing::MercadoPagoMethod) -> Result<String, String> {
        let url = "https://api.mercadopago.com/checkout/preferences";

        let method_str = match method {
            super::routing::MercadoPagoMethod::Pix => "pix",
            super::routing::MercadoPagoMethod::Boleto => "boleto",
            super::routing::MercadoPagoMethod::CreditCard => "credit_card",
        };

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&serde_json::json!({
                "items": [
                    {
                        "id": price_id,
                        "title": "OneHumanCorp Service",
                        "quantity": 1,
                        "unit_price": 100.0, // Mock price
                    }
                ],
                "payment_methods": {
                    "excluded_payment_methods": [],
                    "excluded_payment_types": [],
                    "installments": 1,
                },
                "external_reference": tenant_id,
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let _ = ::server_telemetry::record_api_call_cost(
                    &crate::db::get_pool(),
                    tenant_id,
                    "mercadopago_create_checkout_preference",
                    0.15
                ).await;
                let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
                Ok(data["init_point"].as_str().unwrap_or_default().to_string())
            }
            Ok(resp) => Err(format!("Mercado Pago API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
