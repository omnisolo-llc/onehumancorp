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
    let manychat_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "manychat".to_string(),
            name: "Manychat".to_string(),
            category: "social".to_string(),
            base_url: "https://api.manychat.com".to_string(),
        }
    };
    catalog.push(manychat_provider);

    let cal_com_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "cal_com".to_string(),
            name: "Cal.com".to_string(),
            category: "calendar".to_string(),
            base_url: "https://api.cal.com".to_string(),
        }
    };
    catalog.push(cal_com_provider);

    let resend_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "resend".to_string(),
            name: "Resend".to_string(),
            category: "email".to_string(),
            base_url: "https://api.resend.com".to_string(),
        }
    };
    catalog.push(resend_provider);

    let shippo_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "shippo".to_string(),
            name: "Shippo".to_string(),
            category: "shipping".to_string(),
            base_url: "https://api.goshippo.com".to_string(),
        }
    };
    catalog.push(shippo_provider);

    let zoom_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "zoom".to_string(),
            name: "Zoom".to_string(),
            category: "video".to_string(),
            base_url: "https://api.zoom.us".to_string(),
        }
    };
    catalog.push(zoom_provider);

    let mercadopago_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "mercadopago".to_string(),
            name: "Mercado Pago".to_string(),
            category: "payment".to_string(),
            base_url: "https://api.mercadopago.com".to_string(),
        }
    };
    catalog.push(mercadopago_provider);

    let calendly_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "calendly".to_string(),
            name: "Calendly".to_string(),
            category: "calendar".to_string(),
            base_url: "https://api.calendly.com".to_string(),
        }
    };
    catalog.push(calendly_provider);

    let mailchimp_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "mailchimp".to_string(),
            name: "Mailchimp".to_string(),
            category: "email".to_string(),
            base_url: "https://api.mailchimp.com".to_string(),
        }
    };
    catalog.push(mailchimp_provider);

    catalog.push(meta_provider.to_integration_provider());

    catalog
}
