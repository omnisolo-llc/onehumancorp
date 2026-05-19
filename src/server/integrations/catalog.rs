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
    let nats_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "nats".to_string(),
            name: "NATS Event Mesh".to_string(),
            category: "event_mesh".to_string(),
            base_url: "nats://localhost:4222".to_string(),
        }
    };
    catalog.push(nats_provider);

    // We avoid initializing a real TwilioProvider client here just for metadata
    // to prevent unwanted HTTP client instantiation during registry initialization
    let twilio_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "twilio".to_string(),
            name: "Twilio SMS".to_string(),
            category: "sms".to_string(),
            base_url: "https://api.twilio.com".to_string(),
        }
    };
    catalog.push(twilio_provider);
    let chromadb_provider = crate::integrations::chromadb::provider::ChromaDbProvider::new();
    catalog.push(chromadb_provider.to_integration_provider());

    let meta_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "meta".to_string(),
            name: "Meta Graph API".to_string(),
            category: "social_media".to_string(),
            base_url: "https://graph.facebook.com".to_string(),
        }
    };
    catalog.push(meta_provider);

    let google_calendar_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "google_calendar".to_string(),
            name: "Google Calendar API".to_string(),
            category: "calendar".to_string(),
            base_url: "https://www.googleapis.com/calendar/v3".to_string(),
        }
    };
    catalog.push(google_calendar_provider);

    let mercado_pago_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "mercadopago".to_string(),
            name: "Mercado Pago API".to_string(),
            category: "payments".to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
        }
    };
    catalog.push(mercado_pago_provider);

    let sendgrid_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "sendgrid".to_string(),
            name: "SendGrid API".to_string(),
            category: "email_marketing".to_string(),
            base_url: "https://api.sendgrid.com".to_string(),
        }
    };
    catalog.push(sendgrid_provider);

    let shippo_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "shippo".to_string(),
            name: "Shippo API".to_string(),
            category: "shipping".to_string(),
            base_url: "https://api.goshippo.com".to_string(),
        }
    };
    catalog.push(shippo_provider);

    let zoom_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "zoom".to_string(),
            name: "Zoom API".to_string(),
            category: "video".to_string(),
            base_url: "https://api.zoom.us/v2".to_string(),
        }
    };
    catalog.push(zoom_provider);

    catalog
}
