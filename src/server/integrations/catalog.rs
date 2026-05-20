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

    let meta_provider = crate::integrations::meta::provider::MetaProvider::new("dummy_token".to_string());
    catalog.push(meta_provider.to_integration_provider());

    let tiktok_provider = crate::integrations::tiktok::TikTokProvider::new();
    catalog.push(tiktok_provider.to_integration_provider());

    let outlook_provider = crate::integrations::outlook::OutlookProvider::new();
    catalog.push(outlook_provider.to_integration_provider());

    let shipstation_provider = crate::integrations::shipstation::ShipStationProvider::new();
    catalog.push(shipstation_provider.to_integration_provider());

    let daily_co_provider = crate::integrations::daily_co::DailyCoProvider::new();
    catalog.push(daily_co_provider.to_integration_provider());

    let mailerlite_provider = crate::integrations::mailerlite::MailerLiteProvider::new();
    catalog.push(mailerlite_provider.to_integration_provider());

    let alipay_provider = crate::integrations::alipay::AlipayProvider::new();
    catalog.push(alipay_provider.to_integration_provider());

    let messagebird_provider = crate::integrations::messagebird::MessageBirdProvider::new();
    catalog.push(messagebird_provider.to_integration_provider());


    catalog
}
