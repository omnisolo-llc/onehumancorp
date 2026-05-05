// Stub module - functionality was removed or moved
// This file exists to satisfy module references that weren't cleaned up


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

    // We instantiate nats as a placeholder, without making actual network connection
    // since this is used in synchronous `new()` of registry
    let nats_provider = crate::integrations::nats::provider::NatsProvider::with_client(
        std::sync::Arc::new(crate::integrations::nats::client::RealNatsClient::dummy()),
        "nats://localhost:4222"
    ).into_integration_provider();
    catalog.push(nats_provider);

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "manychat".to_string(),
            name: "Manychat".to_string(),
            category: "social".to_string(),
            base_url: "https://api.manychat.com".to_string(),
        },
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "calendly".to_string(),
            name: "Calendly".to_string(),
            category: "calendar".to_string(),
            base_url: "https://api.calendly.com".to_string(),
        },
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "mailchimp".to_string(),
            name: "Mailchimp".to_string(),
            category: "marketing".to_string(),
            base_url: "https://usX.api.mailchimp.com".to_string(),
        },
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "mercadopago".to_string(),
            name: "Mercado Pago".to_string(),
            category: "payment".to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
        },
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "shippo".to_string(),
            name: "Shippo".to_string(),
            category: "shipping".to_string(),
            base_url: "https://api.goshippo.com".to_string(),
        },
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "twilio".to_string(),
            name: "Twilio".to_string(),
            category: "sms".to_string(),
            base_url: "https://api.twilio.com".to_string(),
        },
    });

    catalog.push(IntegrationProvider {
        metadata: ProviderMetadata {
            id: "zoom".to_string(),
            name: "Zoom".to_string(),
            category: "video".to_string(),
            base_url: "https://api.zoom.us".to_string(),
        },
    });

    catalog
}
