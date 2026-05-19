// Stub module - functionality was removed or moved
// This file exists to satisfy module references that weren't cleaned up


pub struct IntegrationProvider {
    pub metadata: ProviderMetadata,
}

#[derive(Clone)]
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
            base_url: std::env::var("NATS_URL").unwrap_or_else(|_| "nats://localhost:4222".to_string()),
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

    let ayrshare_provider = crate::integrations::ayrshare::provider::AyrshareProvider::new(
        std::env::var("AYRSHARE_API_KEY").unwrap_or_default()
    );
    catalog.push(ayrshare_provider.to_integration_provider());

    let calcom_provider = crate::integrations::calcom::provider::CalComProvider::new(
        std::env::var("CALCOM_API_KEY").unwrap_or_default()
    );
    catalog.push(calcom_provider.to_integration_provider());

    let listmonk_provider = crate::integrations::listmonk::provider::ListmonkProvider::new(
        std::env::var("LISTMONK_URL").unwrap_or_default(),
        std::env::var("LISTMONK_API_KEY").unwrap_or_default()
    );
    catalog.push(listmonk_provider.to_integration_provider());

    let easypost_provider = crate::integrations::easypost::provider::EasyPostProvider::new(
        std::env::var("EASYPOST_API_KEY").unwrap_or_default()
    );
    catalog.push(easypost_provider.to_integration_provider());

    let jitsi_provider = crate::integrations::jitsi::provider::JitsiProvider::new(
        std::env::var("JITSI_DOMAIN").unwrap_or_else(|_| "meet.jit.si".to_string())
    );
    catalog.push(jitsi_provider.to_integration_provider());

    catalog
}
