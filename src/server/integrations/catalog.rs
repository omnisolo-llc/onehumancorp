pub use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};

pub fn get_catalog() -> Vec<IntegrationProvider> {
    vec![
        metadata_provider(
            "taxjar",
            "TaxJar",
            "finance",
            "https://api.taxjar.com/v2".to_string(),
        ),
        metadata_provider(
            "quickbooks",
            "QuickBooks Online",
            "accounting",
            "https://sandbox-quickbooks.api.intuit.com/v3".to_string(),
        ),
        metadata_provider(
            "nats",
            "NATS Event Mesh",
            "event_mesh",
            "nats://localhost:4222".to_string(),
        ),
        metadata_provider(
            "twilio",
            "Twilio SMS",
            "sms",
            "https://api.twilio.com".to_string(),
        ),
        metadata_provider(
            "chromadb",
            "ChromaDB MCP Local Vector Embeddings",
            "vector_db",
            chromadb_base_url(),
        ),
        metadata_provider(
            "meta",
            "Meta Graph API (Facebook, Instagram, WhatsApp)",
            "social",
            "https://graph.facebook.com/v19.0".to_string(),
        ),
        metadata_provider(
            "whatsapp_cloud_api",
            "WhatsApp Cloud API",
            "social",
            "https://graph.facebook.com/v19.0".to_string(),
        ),
        metadata_provider(
            "google_calendar",
            "Google Calendar",
            "calendar",
            "https://www.googleapis.com/calendar/v3".to_string(),
        ),
        metadata_provider(
            "cal_com",
            "Cal.com",
            "calendar",
            "https://api.cal.com/v1".to_string(),
        ),
        metadata_provider(
            "resend",
            "Resend Email Marketing",
            "email_marketing",
            "https://api.resend.com".to_string(),
        ),
        metadata_provider(
            "sendgrid",
            "SendGrid Email",
            "email",
            "https://api.sendgrid.com/v3".to_string(),
        ),
        metadata_provider(
            "shippo",
            "Shippo Logistics",
            "shipping",
            "https://api.goshippo.com".to_string(),
        ),
        metadata_provider(
            "zoom",
            "Zoom Video Conferencing",
            "video",
            "https://api.zoom.us/v2".to_string(),
        ),
        metadata_provider(
            "mercadopago",
            "Mercado Pago",
            "payment",
            "https://api.mercadopago.com".to_string(),
        ),
        metadata_provider(
            "alipay",
            "Alipay",
            "payment",
            "https://openapi.alipay.com".to_string(),
        ),
        metadata_provider(
            "razorpay",
            "Razorpay",
            "payment",
            "https://api.razorpay.com/v1".to_string(),
        ),
        metadata_provider(
            "calendly",
            "Calendly",
            "calendar",
            "https://api.calendly.com".to_string(),
        ),
        metadata_provider(
            "mailchimp",
            "Mailchimp",
            "email_marketing",
            "https://server.api.mailchimp.com/3.0".to_string(),
        ),
        metadata_provider(
            "manychat",
            "Manychat",
            "operations",
            "https://api.manychat.com".to_string(),
        ),
        metadata_provider(
            "ayrshare",
            "Ayrshare",
            "social_media",
            "https://app.ayrshare.com/api".to_string(),
        ),
        metadata_provider(
            "listmonk",
            "Listmonk",
            "email_marketing",
            "http://localhost:9000/api".to_string(),
        ),
        metadata_provider(
            "doordash",
            "DoorDash Drive",
            "delivery",
            "https://openapi.doordash.com".to_string(),
        ),
        metadata_provider(
            "shipday",
            "Shipday Local Delivery",
            "delivery",
            "https://api.shipday.com".to_string(),
        ),
        metadata_provider(
            "easypost",
            "EasyPost",
            "shipping",
            "https://api.easypost.com/v2".to_string(),
        ),
        metadata_provider(
            "jitsi",
            "Jitsi Meet",
            "video_conferencing",
            "https://api.jitsi.net".to_string(),
        ),
        metadata_provider(
            "restic",
            "Restic Local Backup MCP",
            "backup",
            "local://restic".to_string(),
        ),
    ]
}

fn metadata_provider(
    id: &str,
    name: &str,
    category: &str,
    base_url: String,
) -> IntegrationProvider {
    IntegrationProvider {
        metadata: ProviderMetadata {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            base_url,
        },
    }
}

fn chromadb_base_url() -> String {
    let mode = std::env::var("OHC_EXECUTION_MODE").unwrap_or_else(|_| "standalone".to_string());
    let headless = std::env::var("OHC_HEADLESS").unwrap_or_else(|_| "false".to_string());

    if mode == "cloud" && headless != "true" {
        return "mock://chromadb".to_string();
    }

    let host = std::env::var("CHROMADB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("CHROMADB_PORT").unwrap_or_else(|_| "8000".to_string());

    format!("http://{host}:{port}")
}
