#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::services::onboarding::env_verifier::verify_environment;

    #[test]
    fn test_telemetry_disabled_in_standalone_by_default() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "standalone".to_string());

        let config = verify_environment(&env).unwrap();
        assert!(!config.telemetry_enabled, "Telemetry must be disabled by default in standalone mode");
    }

    #[test]
    fn test_telemetry_honors_explicit_disabled_in_standalone() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "standalone".to_string());
        env.insert("OHC_TELEMETRY_ENABLED".to_string(), "false".to_string());

        let config = verify_environment(&env).unwrap();
        assert!(!config.telemetry_enabled);
    }
}
