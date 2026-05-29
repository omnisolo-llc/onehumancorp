
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MercadoPagoMethod {
    Pix,
    Boleto,
    CreditCard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoWebhookPayload {
    pub action: String,
    pub api_version: String,
    pub data: MercadoPagoWebhookData,
    pub date_created: String,
    pub id: i64,
    pub live_mode: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MercadoPagoWebhookData {
    pub id: String,
}

pub async fn handle_mercadopago_webhook(payload_str: &str) -> Result<(), String> {
    let payload: Result<MercadoPagoWebhookPayload, _> = serde_json::from_str(payload_str);
    match payload {
        Ok(parsed) => {
            tracing::info!("Received valid Mercado Pago webhook for payment ID: {}", parsed.data.id);
            // In a real implementation this would update the DB order status
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to parse Mercado Pago webhook: {}", e);
            Err("Invalid webhook payload".to_string())
        }
    }
}
