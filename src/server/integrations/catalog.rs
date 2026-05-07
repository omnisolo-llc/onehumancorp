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

    let calcom_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "calcom".to_string(),
            name: "Cal.com Scheduling".to_string(),
            category: "scheduling".to_string(),
            base_url: "https://api.cal.com/v1".to_string(),
        }
    };
    catalog.push(calcom_provider);

    let easypost_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "easypost".to_string(),
            name: "EasyPost Shipping".to_string(),
            category: "logistics".to_string(),
            base_url: "https://api.easypost.com/v2".to_string(),
        }
    };
    catalog.push(easypost_provider);

    let mercadopago_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "mercadopago".to_string(),
            name: "Mercado Pago".to_string(),
            category: "payments".to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
        }
    };
    catalog.push(mercadopago_provider);

    let ayrshare_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "ayrshare".to_string(),
            name: "Ayrshare Social".to_string(),
            category: "social".to_string(),
            base_url: "https://api.ayrshare.com/api".to_string(),
        }
    };
    catalog.push(ayrshare_provider);

    let listmonk_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "listmonk".to_string(),
            name: "Listmonk Marketing".to_string(),
            category: "marketing".to_string(),
            base_url: "http://localhost:9000".to_string(),
        }
    };
    catalog.push(listmonk_provider);

    let jitsi_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "jitsi".to_string(),
            name: "Jitsi Video".to_string(),
            category: "conferencing".to_string(),
            base_url: "https://meet.jit.si".to_string(),
        }
    };
    catalog.push(jitsi_provider);

    catalog
}
