use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoCheckoutSession {
    pub id: String,
    pub init_point: String,
}

pub struct MercadoPagoClient {
    pub access_token: String,
    pub pool: Option<sqlx::PgPool>,
}

impl MercadoPagoClient {
    pub fn new(access_token: String, pool: Option<sqlx::PgPool>) -> Self {
        MercadoPagoClient { access_token, pool }
    }

    pub async fn create_checkout_preference(&self, _price_id: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            if let Some(p) = &self.pool { p } else { return Ok("mock".to_string()) },
            tenant_id,
            "mercadopago_create_checkout_preference",
            0.15
        ).await;
        // Return a mock checkout URL for Mercado Pago
        Ok("https://www.mercadopago.com.br/checkout/v1/redirect?pref_id=mock_pref_123".to_string())
    }
}
