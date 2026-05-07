use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MercadoPagoClientWrapper: Send + Sync {
    async fn create_payment(&self, amount: f64, description: &str, payer_email: &str) -> Result<String, String>;
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
    async fn create_payment(&self, _amount: f64, _description: &str, _payer_email: &str) -> Result<String, String> {
        if self.access_token.is_empty() {
            return Err("Access token is required".to_string());
        }
        // Mock Mercado Pago payment creation
        Ok("mp_pay_123456789".to_string())
    }
}
