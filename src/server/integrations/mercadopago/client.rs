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

    pub async fn create_checkout_preference(&self, _price_id: &str, _tenant_id: &str) -> Result<String, String> {
        // Return a mock checkout URL for Mercado Pago
        Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string())
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        // Mock handle webhook
        Ok(())
    }
}

impl MercadoPagoClient {
    pub async fn create_payment(&self, _amount: f64, _description: &str, _payer_email: &str) -> Result<String, String> {
        // Mock returning a transaction ID
        Ok("mock_txn_123".to_string())
    }
}
