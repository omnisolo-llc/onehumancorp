pub struct IntegrationProvider {
    pub metadata: ProviderMetadata,
}

pub struct ProviderMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_url: String,
}
pub mod inbox_router;

pub use inbox_router::{InboxRouter, AmbassadorAgent, InteractionStream, UnifiedThread};
