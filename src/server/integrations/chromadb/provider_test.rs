#[cfg(test)]
mod tests {
    use crate::integrations::chromadb::provider::ChromaDbProvider;

    #[test]
    fn test_chromadb_provider_standalone_mode() {
        temp_env::with_vars(
            vec![
                ("OHC_EXECUTION_MODE", Some("standalone")),
                ("CHROMADB_HOST", Some("127.0.0.1")),
                ("CHROMADB_PORT", Some("9000")),
            ],
            || {
                let provider = ChromaDbProvider::new();
                assert_eq!(provider.metadata.id, "chromadb");
                assert_eq!(provider.is_mock, false);
                assert_eq!(provider.base_url, "http://127.0.0.1:9000");

                let ip = provider.to_integration_provider();
                assert_eq!(ip.metadata.id, "chromadb");
                assert_eq!(ip.metadata.base_url, "http://127.0.0.1:9000");
            },
        );
    }

    #[test]
    fn test_chromadb_provider_cloud_mode() {
        temp_env::with_vars(
            vec![
                ("OHC_EXECUTION_MODE", Some("cloud")),
                ("OHC_HEADLESS", Some("false")),
            ],
            || {
                let provider = ChromaDbProvider::new();
                assert_eq!(provider.is_mock, true);
                assert_eq!(provider.base_url, "mock://chromadb");

                let ip = provider.to_integration_provider();
                assert_eq!(ip.metadata.base_url, "mock://chromadb");
            },
        );
    }

    #[test]
    fn test_chromadb_provider_cloud_mode_headless() {
        temp_env::with_vars(
            vec![
                ("OHC_EXECUTION_MODE", Some("cloud")),
                ("OHC_HEADLESS", Some("true")),
                ("CHROMADB_HOST", Some("localhost")),
                ("CHROMADB_PORT", Some("8000")),
            ],
            || {
                let provider = ChromaDbProvider::new();
                assert_eq!(provider.is_mock, false);
                assert_eq!(provider.base_url, "http://localhost:8000");
            },
        );
    }
}
