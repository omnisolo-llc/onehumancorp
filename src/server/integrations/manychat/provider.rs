use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub fn get_manychat_provider() -> IntegrationProvider {
    IntegrationProvider {
        metadata: ProviderMetadata {
            id: "manychat".to_string(),
            name: "Manychat".to_string(),
            category: "Customer Success".to_string(),
            base_url: "".to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manychat_provider() {
        let provider = get_manychat_provider();
        assert_eq!(provider.metadata.id, "manychat");
        assert_eq!(provider.metadata.name, "Manychat");
        assert_eq!(provider.metadata.category, "Customer Success");
        assert_eq!(provider.metadata.base_url, "");
    }
}
