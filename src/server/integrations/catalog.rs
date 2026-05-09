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

    catalog.push(crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "meta".to_string(),
            name: "Meta Unified Inbox".to_string(),
            category: "social_media".to_string(),
            base_url: "https://graph.facebook.com".to_string(),
        }
    });

    catalog.push(crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "cal_com".to_string(),
            name: "Cal.com Scheduling".to_string(),
            category: "calendar".to_string(),
            base_url: "https://api.cal.com/v1".to_string(),
        }
    });

    catalog.push(crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "resend".to_string(),
            name: "Resend Email".to_string(),
            category: "email".to_string(),
            base_url: "https://api.resend.com".to_string(),
        }
    });

    catalog.push(crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "easypost".to_string(),
            name: "EasyPost Shipping".to_string(),
            category: "logistics".to_string(),
            base_url: "https://api.easypost.com/v2".to_string(),
        }
    });

    catalog.push(crate::integrations::catalog::IntegrationProvider {
        metadata: crate::integrations::catalog::ProviderMetadata {
            id: "google_meet".to_string(),
            name: "Google Meet".to_string(),
            category: "video".to_string(),
            base_url: "https://meet.googleapis.com".to_string(),
        }
    });

    catalog
}
