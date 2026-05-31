#[cfg(test)]
mod tests {
    use super::super::provider::ResticProvider;

    #[test]
    fn test_restic_provider_standalone_mode() {
        temp_env::with_var("OHC_EXECUTION_MODE", Some("standalone"), || {
            let provider = ResticProvider::new();
            assert_eq!(provider.metadata.id, "restic");
            assert!(provider.is_supported);
        });
    }

    #[test]
    fn test_restic_provider_cloud_mode() {
        temp_env::with_var("OHC_EXECUTION_MODE", Some("cloud"), || {
            let provider = ResticProvider::new();
            assert_eq!(provider.metadata.id, "restic");
            assert!(!provider.is_supported);
        });
    }
}
