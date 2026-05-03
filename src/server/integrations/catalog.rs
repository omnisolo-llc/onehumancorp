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
    let nats_provider = crate::integrations::nats::provider::NatsProvider::with_client(
        std::sync::Arc::new(crate::integrations::nats::client::RealNatsClient::dummy()),
        "nats://localhost:4222"
    ).into_integration_provider();
    catalog.push(nats_provider);

    catalog
}
