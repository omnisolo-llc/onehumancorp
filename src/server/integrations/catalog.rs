pub struct IntegrationProviderMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_url: String,
}

pub struct IntegrationProvider {
    pub metadata: IntegrationProviderMetadata,
}

pub fn get_catalog() -> Vec<IntegrationProvider> {
    let mut catalog = vec![];
    catalog.push(IntegrationProvider {
        metadata: IntegrationProviderMetadata {
            id: "nats".to_string(),
            name: "NATS Event Mesh".to_string(),
            category: "event_mesh".to_string(),
            base_url: "nats://localhost:4222".to_string(),
        }
    });
    catalog
}
