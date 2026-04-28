// Stub for catalog integration

pub fn get_catalog() -> Vec<ProviderStub> {
    Vec::new()
}

pub struct ProviderStub {
    pub metadata: ProviderMetadata,
}

pub struct ProviderMetadata {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_url: String,
}
