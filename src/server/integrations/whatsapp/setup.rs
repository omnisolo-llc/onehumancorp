use reqwest::Client;
use serde_json::json;

pub struct WhatsappWebhookSetupService {
    access_token: String,
    waba_id: String,
    phone_number_id: String,
}

impl WhatsappWebhookSetupService {
    pub fn new(access_token: String, waba_id: String, phone_number_id: String) -> Self {
        Self {
            access_token,
            waba_id,
            phone_number_id,
        }
    }

    pub async fn register_phone_number(&self, pin: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/register",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "pin": pin
        });

        let client = Client::new();
        let res = client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Reqwest error: {}", e))?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            return Err(format!("WhatsApp API error: {}", err_text));
        }

        Ok(())
    }

    pub async fn setup_webhook(&self, _callback_url: &str, _verify_token: &str, fields: Vec<&str>) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/subscribed_apps",
            self.waba_id
        );

        let payload = json!({
            "subscribe_fields": fields.join(",")
        });

        let client = Client::new();
        let res = client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Reqwest error: {}", e))?;

        if !res.status().is_success() {
            let err_text = res.text().await.unwrap_or_default();
            tracing::warn!("Webhook setup response: {}", err_text);
            return Err(format!("WhatsApp Webhook Setup API error: {}", err_text));
        }

        Ok(())
    }
}
