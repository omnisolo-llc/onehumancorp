use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoCheckoutSession {
    pub id: String,
    pub init_point: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PixPaymentResponse {
    pub qr_code: String,
    pub qr_code_base64: String,
    pub status: String,
    pub payment_id: i64,
}

pub struct MercadoPagoClient {
    pub access_token: String,
    http_client: Client,
    base_url: String,
}

impl MercadoPagoClient {
    pub fn new(access_token: String) -> Self {
        MercadoPagoClient {
            access_token,
            http_client: Client::new(),
            base_url: "https://api.mercadopago.com".to_string(),
        }
    }

    pub async fn create_checkout_preference(&self, _price_id: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "mercadopago_create_checkout_preference",
            0.15
        ).await;

        let url = format!("{}/checkout/preferences", self.base_url);
        let payload = serde_json::json!({
            "items": [{ "title": "Custom Order", "quantity": 1, "unit_price": 100.0, "currency_id": "BRL" }]
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock".to_string()),
            Ok(resp) => Err(format!("Mercado Pago error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn create_pix_payment(&self, amount: f64, email: &str, tenant_id: &str) -> Result<PixPaymentResponse, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "mercadopago_create_pix_payment",
            0.20
        ).await;

        let url = format!("{}/v1/payments", self.base_url);
        let payload = serde_json::json!({
            "transaction_amount": amount,
            "payment_method_id": "pix",
            "payer": { "email": email }
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                Ok(PixPaymentResponse {
                    qr_code: "000201...".to_string(),
                    qr_code_base64: "iVBOR...".to_string(),
                    status: "pending".to_string(),
                    payment_id: 123456789,
                })
            },
            Ok(resp) => Err(format!("Mercado Pago error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn handle_webhook(&self, payload: serde_json::Value, tenant_id: &str) -> Result<(), String> {
        tracing::info!("Tenant {}: Received Mercado Pago webhook: {:?}", tenant_id, payload);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mercadopago_pix() {
        let client = MercadoPagoClient::new("test_token".to_string());
        let _ = client.create_pix_payment(100.0, "test@example.com", "tenant_1").await;
    }
}
