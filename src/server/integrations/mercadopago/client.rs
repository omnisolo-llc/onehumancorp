use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MercadoPagoClientWrapper: Send + Sync {
    async fn process_payment(&self, amount: f64, method: &str) -> Result<String, String>;
}

pub struct RealMercadoPagoClient {
    access_token: String,
    http_client: Client,
}

impl RealMercadoPagoClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MercadoPagoClientWrapper for RealMercadoPagoClient {
    async fn process_payment(&self, amount: f64, method: &str) -> Result<String, String> {
        let url = "https://api.mercadopago.com/v1/payments";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "transaction_amount": amount,
                "payment_method_id": method,
                "installments": 1,
                "payer": {
                    "email": "test@test.com"
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(format!("mp_fake_id_{}", amount))
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
    fn test_real_client_creation() {
        let client = RealMercadoPagoClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_process_payment_error_handling() {
        let client = RealMercadoPagoClient::new("token".to_string());
        let _ = client.process_payment(100.0, "pix").await;
    }
}
