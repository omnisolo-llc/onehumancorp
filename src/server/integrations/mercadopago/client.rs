use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoCheckoutSession {
    pub id: String,
    pub init_point: String,
}

use reqwest::Client;

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

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        let url = "https://api.mercadopago.com/checkout/preferences";

        let payload = serde_json::json!({
            "items": [
                {
                    "id": price_id,
                    "title": "OHC Service",
                    "quantity": 1,
                    "currency_id": "BRL",
                    "unit_price": 100.0 // Mock static price for implementation
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
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        tenant_id,
                        "mercadopago_checkout_preference",
                        0.15 // mock cost for api orchestration
                    ).await;
                    // Mock returning a checkout URL for Mercado Pago. A real impl parses the response.
                    Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string())
                } else {
                    Err(format!("MercadoPago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        // Mock handle webhook logic (typically just parsing the JSON and verifying signatures)
        Ok(())
    }
}

impl MercadoPagoClient {
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
            .header("X-Idempotency-Key", format!("{}_{}", payer_email, amount))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        payer_email, // using email as a proxy for tenant/identity in this stub
                        "mercadopago_create_payment",
                        0.20
                    ).await;
                    // Mock returning a transaction ID. A real impl parses the response.
                    Ok("mock_txn_123".to_string())
                } else {
                    Err(format!("MercadoPago API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
