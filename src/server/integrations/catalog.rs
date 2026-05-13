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

    let meta_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "meta".to_string(),
            name: "Meta Graph API".to_string(),
            category: "messaging".to_string(),
            base_url: "https://graph.facebook.com".to_string(),
        }
    };
    catalog.push(meta_provider);

    let nylas_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "nylas".to_string(),
            name: "Nylas Calendar".to_string(),
            category: "calendar".to_string(),
            base_url: "https://api.nylas.com".to_string(),
        }
    };
    catalog.push(nylas_provider);

    let resend_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "resend".to_string(),
            name: "Resend Email".to_string(),
            category: "marketing".to_string(),
            base_url: "https://api.resend.com".to_string(),
        }
    };
    catalog.push(resend_provider);

    let easypost_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "easypost".to_string(),
            name: "EasyPost Logistics".to_string(),
            category: "logistics".to_string(),
            base_url: "https://api.easypost.com".to_string(),
        }
    };
    catalog.push(easypost_provider);

    let zoom_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "zoom".to_string(),
            name: "Zoom Video".to_string(),
            category: "video".to_string(),
            base_url: "https://api.zoom.us".to_string(),
        }
    };
    catalog.push(zoom_provider);

    let chromadb_provider = crate::integrations::chromadb::provider::ChromaDbProvider::new();
    catalog.push(chromadb_provider.to_integration_provider());

    catalog
}
