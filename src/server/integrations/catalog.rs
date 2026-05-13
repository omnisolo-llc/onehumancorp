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


    let chatwoot_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "chatwoot".to_string(),
            name: "Chatwoot Unified Inbox".to_string(),
            category: "social_media".to_string(),
            base_url: "https://app.chatwoot.com".to_string(),
        }
    };
    catalog.push(chatwoot_provider);

    let calcom_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "calcom".to_string(),
            name: "Cal.com Booking".to_string(),
            category: "calendar".to_string(),
            base_url: "https://api.cal.com".to_string(),
        }
    };
    catalog.push(calcom_provider);

    let resend_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "resend".to_string(),
            name: "Resend Email Marketing".to_string(),
            category: "email_marketing".to_string(),
            base_url: "https://api.resend.com".to_string(),
        }
    };
    catalog.push(resend_provider);

    let shippo_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "shippo".to_string(),
            name: "Shippo Automated Labels".to_string(),
            category: "shipping".to_string(),
            base_url: "https://api.goshippo.com".to_string(),
        }
    };
    catalog.push(shippo_provider);

    let dailyco_provider = crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "dailyco".to_string(),
            name: "Daily.co Video Rooms".to_string(),
            category: "video".to_string(),
            base_url: "https://api.daily.co".to_string(),
        }
    };
    catalog.push(dailyco_provider);

    catalog
}
