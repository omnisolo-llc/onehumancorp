use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait MercadoPagoClientWrapper: Send + Sync {
    async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String>;
    async fn get_oauth_url(&self, redirect_uri: &str) -> String;
    async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<String, String>;
}

pub struct MercadoPagoClient {
    pub api_key: String,
    http_client: Client,
}

impl MercadoPagoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MercadoPagoClientWrapper for MercadoPagoClient {
    async fn create_checkout_preference(&self, _price_id: &str, _tenant_id: &str) -> Result<String, String> {
        let payload = serde_json::json!({
            "items": [
                {
                    "title": "Order",
                    "quantity": 1,
                    "unit_price": 10.0
                }
            ]
        });

        let res = self.http_client.post("https://api.mercadopago.com/checkout/preferences")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok("mp_pref_id_123".to_string()),
            _ => Err("Failed to create MP preference".to_string())
        }
    }

    async fn get_oauth_url(&self, redirect_uri: &str) -> String {
        format!("https://auth.mercadopago.com/authorization?client_id=MOCK&response_type=code&platform_id=mp&redirect_uri={}", redirect_uri)
    }

    async fn exchange_token(&self, _code: &str, _redirect_uri: &str) -> Result<String, String> {
        Ok("mock_mp_token".to_string())
    }
}
