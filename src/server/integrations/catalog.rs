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


    let resend_provider = crate::integrations::resend::provider::ResendProvider::new("".to_string());
    catalog.push(resend_provider.into_integration_provider());

    let calcom_provider = crate::integrations::calcom::provider::CalComProvider::new();
    catalog.push(calcom_provider.to_integration_provider());

    let meta_provider = crate::integrations::meta::provider::MetaGraphProvider::new();
    catalog.push(meta_provider.to_integration_provider());

    let shippo_provider = crate::integrations::shippo::provider::ShippoProvider::new();
    catalog.push(shippo_provider.to_integration_provider());

    let zoom_provider = crate::integrations::zoom::provider::ZoomProvider::new();
    catalog.push(zoom_provider.to_integration_provider());

    let mercadopago_provider = crate::integrations::mercadopago::client::MercadoPagoClient::new("".to_string());
    catalog.push(crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "mercadopago".to_string(),
            name: "Mercado Pago".to_string(),
            category: "payment".to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
        }
    });

    catalog
}
