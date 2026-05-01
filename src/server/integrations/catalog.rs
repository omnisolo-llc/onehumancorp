// Stub module - functionality was removed or moved
// This file exists to satisfy module references that weren't cleaned up

#[allow(unused_imports)]
use crate::ohc::orchestration::*;

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
    // Return empty catalog since functionality was removed
    vec![]
}
