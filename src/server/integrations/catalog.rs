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

    let ayrshare_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "ayrshare".to_string(),
            name: "Ayrshare".to_string(),
            category: "social_media".to_string(),
            base_url: "https://app.ayrshare.com/api".to_string(),
        }
    };
    catalog.push(ayrshare_provider);

    let cal_com_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "cal_com".to_string(),
            name: "Cal.com".to_string(),
            category: "calendar_scheduling".to_string(),
            base_url: "https://api.cal.com/v1".to_string(),
        }
    };
    catalog.push(cal_com_provider);

    let listmonk_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "listmonk".to_string(),
            name: "Listmonk".to_string(),
            category: "email_marketing".to_string(),
            base_url: "http://localhost:9000".to_string(),
        }
    };
    catalog.push(listmonk_provider);

    let mercado_pago_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "mercadopago".to_string(),
            name: "Mercado Pago".to_string(),
            category: "payment".to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
        }
    };
    catalog.push(mercado_pago_provider);

    let easypost_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "easypost".to_string(),
            name: "EasyPost".to_string(),
            category: "shipping".to_string(),
            base_url: "https://api.easypost.com/v2".to_string(),
        }
    };
    catalog.push(easypost_provider);

    let jitsi_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "jitsi".to_string(),
            name: "Jitsi Meet".to_string(),
            category: "video".to_string(),
            base_url: "https://meet.jit.si".to_string(),
        }
    };
    catalog.push(jitsi_provider);

    catalog
}
