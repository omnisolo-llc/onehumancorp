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

    let google_calendar_provider = crate::integrations::google_calendar::provider::GoogleCalendarProvider::new("dummy_token".to_string());
    catalog.push(google_calendar_provider.to_integration_provider());

    let cal_com_provider = crate::integrations::cal_com::provider::CalComProvider::new("dummy_token".to_string());
    catalog.push(cal_com_provider.to_integration_provider());

    let resend_provider = crate::integrations::resend::provider::ResendProvider::new("dummy_token".to_string());
    catalog.push(resend_provider.to_integration_provider());

    let shippo_provider = crate::integrations::shippo::provider::ShippoProvider::new("dummy_token".to_string());
    catalog.push(shippo_provider.to_integration_provider());

    let zoom_provider = crate::integrations::zoom::provider::ZoomProvider::new("dummy_token".to_string());
    catalog.push(zoom_provider.to_integration_provider());

    let mercadopago_provider = crate::integrations::mercadopago::provider::MercadoPagoProvider::new("dummy_token".to_string());
    catalog.push(mercadopago_provider.to_integration_provider());

    let manychat_provider = crate::integrations::manychat::provider::ManychatProvider::new("dummy_token".to_string());
    catalog.push(manychat_provider.to_integration_provider());

    let calendly_provider = crate::integrations::calendly::provider::CalendlyProvider::new("dummy_token".to_string());
    catalog.push(calendly_provider.to_integration_provider());

    let mailchimp_provider = crate::integrations::mailchimp::provider::MailchimpProvider::new("dummy_token".to_string());
    catalog.push(mailchimp_provider.to_integration_provider());

    let ayrshare_provider = crate::integrations::ayrshare::provider::AyrshareProvider::new("dummy_token".to_string());
    catalog.push(ayrshare_provider.to_integration_provider());

    let listmonk_provider = crate::integrations::listmonk::provider::ListmonkProvider::new("dummy_token".to_string());
    catalog.push(listmonk_provider.to_integration_provider());

    let easypost_provider = crate::integrations::easypost::provider::EasyPostProvider::new("dummy_token".to_string());
    catalog.push(easypost_provider.to_integration_provider());

    let jitsi_provider = crate::integrations::jitsi::provider::JitsiProvider::new("dummy_token".to_string());
    catalog.push(jitsi_provider.to_integration_provider());

    catalog
}
