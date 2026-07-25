use super::client::WhatsAppClient;

pub struct WebhookSetupService {
    client: WhatsAppClient,
}

impl WebhookSetupService {
    pub fn new(client: WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn setup_webhook_and_register(&self, pin: &str) -> Result<(), String> {
        // Step 1: Register the phone number with Meta
        tracing::info!("Registering phone number with Meta API...");
        self.client.register_phone_number(pin).await?;
        tracing::info!("Phone number successfully registered.");

        // Additional setup like subscribing to webhooks could go here if managed dynamically.
        // For Meta Cloud API, webhooks are typically configured in the App Dashboard,
        // but this orchestrates the local side of the setup.

        Ok(())
    }
}
