use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::env;

pub struct ChromaDbProvider {
    pub metadata: ProviderMetadata,
    pub is_mock: bool,
    pub base_url: String,
}

impl ChromaDbProvider {
    pub fn new() -> Self {
        let mode = env::var("OHC_EXECUTION_MODE").unwrap_or_else(|_| "standalone".to_string());
        let headless = env::var("OHC_HEADLESS").unwrap_or_else(|_| "false".to_string());

        let is_mock = mode == "cloud" && headless != "true";

        let host = env::var("CHROMADB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("CHROMADB_PORT").unwrap_or_else(|_| "8000".to_string());

        // When in mock/cloud mode without headless, we mock the base URL
        // so we don't accidentally try to hit localhost:8000 in a pure cloud environment.
        let base_url = if is_mock {
            "mock://chromadb".to_string()
        } else {
            format!("http://{}:{}", host, port)
        };

        Self {
            metadata: ProviderMetadata {
                id: "chromadb".to_string(),
                name: "ChromaDB MCP Local Vector Embeddings".to_string(),
                category: "vector_db".to_string(),
                base_url: base_url.clone(),
            },
            is_mock,
            base_url,
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            },
        }
    }
}
