pub struct IntegrationProvider {
    pub metadata: ProviderMetadata,
}

pub struct ProviderMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_url: String,
}

pub mod models;
pub mod repository;
pub mod service;

pub use models::*;
pub use repository::*;
pub use service::*;
pub mod tests;
