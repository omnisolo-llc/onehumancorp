#[cfg(test)]
mod tests {
    use super::super::provider::PowerSyncProvider;

    #[test]
    fn test_powersync_provider_standalone() {
        temp_env::with_vars(
            [
                ("OHC_EXECUTION_MODE", Some("standalone")),
                ("OHC_HEADLESS", Some("false")),
                ("POWERSYNC_HOST", Some("localhost")),
                ("POWERSYNC_PORT", Some("8080")),
            ],
            || {
                let provider = PowerSyncProvider::new();
                assert_eq!(provider.metadata.id, "powersync");
                assert_eq!(provider.metadata.name, "PowerSync Hybrid Data Synchronization");
                assert_eq!(provider.metadata.category, "database_sync");
                assert_eq!(provider.is_mock, false);
                assert_eq!(provider.base_url, "http://localhost:8080");

                let integration_provider = provider.to_integration_provider();
                assert_eq!(integration_provider.metadata.id, "powersync");
            },
        );
    }

    #[test]
    fn test_powersync_provider_cloud_mock() {
        temp_env::with_vars(
            [
                ("OHC_EXECUTION_MODE", Some("cloud")),
                ("OHC_HEADLESS", Some("false")),
            ],
            || {
                let provider = PowerSyncProvider::new();
                assert_eq!(provider.is_mock, true);
                assert_eq!(provider.base_url, "mock://powersync");
            },
        );
    }
}