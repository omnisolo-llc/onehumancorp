pub struct IntegrationProvider {
    pub metadata: ProviderMetadata,
}

pub struct ProviderMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_url: String,
}

pub fn get_catalog() -> Vec<IntegrationProvider> {
    let mut catalog = vec![];

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "nats".to_string(),
            name: "NATS Event Mesh".to_string(),
            category: "event_mesh".to_string(),
            base_url: "nats://localhost:4222".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "twilio".to_string(),
            name: "Twilio SMS".to_string(),
            category: "sms".to_string(),
            base_url: "https://api.twilio.com".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "meta".to_string(),
            name: "Meta (Instagram & Facebook)".to_string(),
            category: "social_media".to_string(),
            base_url: "https://graph.facebook.com".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "google_calendar".to_string(),
            name: "Google Calendar".to_string(),
            category: "calendar".to_string(),
            base_url: "https://www.googleapis.com/calendar".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "sendgrid".to_string(),
            name: "SendGrid Email".to_string(),
            category: "email".to_string(),
            base_url: "https://api.sendgrid.com".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "shippo".to_string(),
            name: "Shippo Logistics".to_string(),
            category: "shipping".to_string(),
            base_url: "https://api.goshippo.com".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "zoom".to_string(),
            name: "Zoom Conferencing".to_string(),
            category: "video".to_string(),
            base_url: "https://api.zoom.us".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "mercadopago".to_string(),
            name: "Mercado Pago".to_string(),
            category: "payment".to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
        }
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "stripe".to_string(),
            name: "Stripe Payments".to_string(),
            category: "payment".to_string(),
            base_url: "https://api.stripe.com".to_string(),
        }
    });

    let chromadb_provider = crate::integrations::chromadb::provider::ChromaDbProvider::new();
    catalog.push(chromadb_provider.to_integration_provider());

    catalog
}
