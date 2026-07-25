use reqwest::Client;
use serde_json::json;

pub struct WhatsAppSetupService {
    app_id: String,
    access_token: String,
}

impl WhatsAppSetupService {
    pub fn new(app_id: String, access_token: String) -> Self {
        Self {
            app_id,
            access_token,
        }
    }

    pub async fn register_webhook(&self, callback_url: &str, verify_token: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/subscriptions",
            self.app_id
        );

        let payload = json!({
            "object": "whatsapp_business_account",
            "callback_url": callback_url,
            "verify_token": verify_token,
            "fields": "messages,message_template_status_update"
        });

        let client = Client::new();
        let res = client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    let err_text = response.text().await.unwrap_or_default();
                    Err(format!("Failed to register webhook: {}", err_text))
                }
            }
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}
