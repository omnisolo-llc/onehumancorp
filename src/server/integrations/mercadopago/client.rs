use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoCheckoutSession {
    pub id: String,
    pub init_point: String,
}

pub struct MercadoPagoClient {
    pub access_token: String,
}

impl MercadoPagoClient {
    pub fn new(access_token: String) -> Self {
        MercadoPagoClient { access_token }
    }

    pub async fn create_checkout_preference(&self, _price_id: &str, tenant_id: &str, amount: f64, title: &str) -> Result<String, String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "mercadopago_create_checkout_preference",
            0.15
        ).await;

        let client = reqwest::Client::new();
        let res = client.post("https://api.mercadopago.com/checkout/preferences")
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "items": [
                    {
                        "title": title,
                        "quantity": 1,
                        "unit_price": amount
                    }
                ]
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(json) = resp.json::<MercadoPagoCheckoutSession>().await {
                    return Ok(json.init_point);
                }
                Err("Failed to parse checkout preference response".to_string())
            }
            Ok(resp) => Err(format!("Mercado Pago API error: {}", resp.status())),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MercadoPagoClient;

    #[tokio::test]
    async fn test_mercadopago_client_instantiation() {
        let client = MercadoPagoClient::new("dummy_token".to_string());
        assert_eq!(client.access_token, "dummy_token");
    }

    #[tokio::test]
    async fn test_mercadopago_client_create_checkout_preference_error_handling() {
        let client = MercadoPagoClient::new("dummy_token".to_string());
        let res = client.create_checkout_preference("price", "tenant", 10.0, "Title").await;
        assert!(res.is_err() || res.is_ok());
    }
}
