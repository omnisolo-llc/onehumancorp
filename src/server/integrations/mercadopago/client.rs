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

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        Ok(format!("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id={}_{}", tenant_id, price_id))
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        Ok(())
    }
}

impl MercadoPagoClient {
    pub async fn create_payment(&self, amount: f64, description: &str, payer_email: &str) -> Result<String, String> {
        Ok(format!("mock_txn_{}_{}_{}", amount, description, payer_email).replace(" ", "_"))
    }
}
