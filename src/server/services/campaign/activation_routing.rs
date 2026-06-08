#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CampaignChannel {
    SendGrid,
    Twilio,
    Meta,
}

impl CampaignChannel {
    pub fn from_asset_type(asset_type: &str) -> Option<Self> {
        match asset_type.trim().to_ascii_lowercase().as_str() {
            "email" | "sendgrid" => Some(Self::SendGrid),
            "sms" | "text" | "twilio" => Some(Self::Twilio),
            "meta" | "social" | "facebook" | "instagram" | "whatsapp" => Some(Self::Meta),
            _ => None,
        }
    }
}
