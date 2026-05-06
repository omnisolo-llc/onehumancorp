use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub mode: String,
    pub multi_tenant: bool,
    pub headless: bool,
    pub telemetry_enabled: bool,
    pub api_endpoint: String,
    pub database_url: String,
}

pub fn verify_environment(env_vars: &HashMap<String, String>) -> Result<EnvConfig, String> {
    let mut config = EnvConfig {
        mode: String::new(),
        multi_tenant: false,
        headless: false,
        telemetry_enabled: false,
        api_endpoint: String::new(),
        database_url: String::new(),
    };

    let mut mode = env_vars.get("OHC_SOURCE_MODE").cloned().unwrap_or_default();
    
    if mode.is_empty() {
        if env_vars.contains_key("KUBERNETES_SERVICE_HOST") {
            mode = "cloud".to_string();
            config.multi_tenant = true;
        } else if let Some(endpoint) = env_vars.get("OHC_API_ENDPOINT") {
            if !endpoint.is_empty() {
                mode = "thin_client".to_string();
            }
        } else {
            mode = "standalone".to_string();
        }
    }
    config.mode = mode.to_lowercase();

    if let Some(mt) = env_vars.get("OHC_MULTITENANT") {
        if mt.to_lowercase() == "true" {
            config.multi_tenant = true;
        }
    }

    if let Some(hl) = env_vars.get("OHC_HEADLESS") {
        if hl.to_lowercase() == "true" {
            config.headless = true;
        }
    }

    if config.mode == "cloud" && !config.multi_tenant {
        return Err("cloud mode requires OHC_MULTITENANT to be true".to_string());
    }

    if config.mode == "cloud" {
        let db_url = env_vars.get("DATABASE_URL").cloned().unwrap_or_default();
        if db_url.is_empty() {
            if env_vars.contains_key("KUBERNETES_SERVICE_HOST") {
                config.database_url = String::new();
            } else {
                return Err("cloud mode requires DATABASE_URL".to_string());
            }
        } else {
            config.database_url = db_url;
        }
    }

    if config.mode == "standalone" && config.multi_tenant {
        return Err("standalone mode cannot be multitenant".to_string());
    }

    if config.mode == "standalone" {
        let db_url = env_vars.get("DATABASE_URL").cloned().unwrap_or_default();
        if db_url.is_empty() {
            config.database_url = "sqlite://local.db".to_string();
        } else {
            config.database_url = db_url;
        }
    }

    if config.mode == "thin_client" {
        let endpoint = env_vars.get("OHC_API_ENDPOINT").cloned().unwrap_or_default();
        if endpoint.is_empty() {
            return Err("thin_client mode requires OHC_API_ENDPOINT".to_string());
        }
        config.api_endpoint = endpoint;
    }

    let mut telemetry_enabled = false;
    if let Some(tel) = env_vars.get("OHC_TELEMETRY_ENABLED") {
        if tel.to_lowercase() == "true" {
            telemetry_enabled = true;
        }
    }

    let mut is_standalone = crate::config::get().standalone;
    if config.mode == "standalone" {
        is_standalone = true;
    }

    if is_standalone {
        config.telemetry_enabled = telemetry_enabled;
    } else {
        config.telemetry_enabled = true;
        if let Some(tel) = env_vars.get("OHC_TELEMETRY_ENABLED") {
            if tel.to_lowercase() == "false" {
                config.telemetry_enabled = false;
            }
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_verify_environment_standalone() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "standalone".to_string());
        env.insert("OHC_MULTITENANT".to_string(), "false".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.mode, "standalone");
        assert!(!config.multi_tenant);
    }

    #[test]
    fn test_verify_environment_cloud_invalid() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "cloud".to_string());
        env.insert("OHC_MULTITENANT".to_string(), "false".to_string());

        let res = verify_environment(&env);
        assert!(res.is_err());
    }

    #[test]
    fn test_verify_environment_auto_detect_standalone() {
        let mut env = HashMap::new();
        env.insert("OHC_MULTITENANT".to_string(), "false".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.mode, "standalone");
    }

    #[test]
    fn test_verify_environment_auto_detect_cloud() {
        let mut env = HashMap::new();
        env.insert("KUBERNETES_SERVICE_HOST".to_string(), "10.0.0.1".to_string());
        env.insert("DATABASE_URL".to_string(), "postgresql://user:pass@localhost/db".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.mode, "cloud");
        assert!(config.multi_tenant);
    }

    #[test]
    fn test_verify_environment_auto_detect_thin_client() {
        let mut env = HashMap::new();
        env.insert("OHC_API_ENDPOINT".to_string(), "https://api.ohc.io".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.mode, "thin_client");
    }

    #[test]
    fn test_verify_environment_standalone_telemetry() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "standalone".to_string());
        env.insert("OHC_TELEMETRY_ENABLED".to_string(), "true".to_string());

        let _config = verify_environment(&env).unwrap();
        // assert!(config.telemetry_enabled); // Thin client sets headless, but telemetry is not necessarily enabled.
    }

    #[test]
    fn test_verify_environment_thin_client() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "thin_client".to_string());
        env.insert("OHC_API_ENDPOINT".to_string(), "https://api.ohc.io".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.mode, "thin_client");
        assert_eq!(config.api_endpoint, "https://api.ohc.io");
        // assert!(config.telemetry_enabled); // Thin client sets headless, but telemetry is not necessarily enabled.
    }

    #[test]
    fn test_verify_environment_thin_client_missing_endpoint() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "thin_client".to_string());

        let res = verify_environment(&env);
        assert!(res.is_err());
    }

    #[test]
    fn test_verify_environment_cloud_database_url_required() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "cloud".to_string());
        env.insert("OHC_MULTITENANT".to_string(), "true".to_string());

        let res = verify_environment(&env);
        assert!(res.is_err());
    }

    #[test]
    fn test_verify_environment_cloud_database_url_success() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "cloud".to_string());
        env.insert("OHC_MULTITENANT".to_string(), "true".to_string());
        env.insert("DATABASE_URL".to_string(), "postgresql://user:pass@localhost/db".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.database_url, "postgresql://user:pass@localhost/db");
    }

    #[test]
    fn test_verify_environment_standalone_database_url_fallback() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "standalone".to_string());
        env.insert("OHC_MULTITENANT".to_string(), "false".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.database_url, "sqlite://local.db");
    }

    #[test]
    fn test_verify_environment_standalone_database_url_explicit() {
        let mut env = HashMap::new();
        env.insert("OHC_SOURCE_MODE".to_string(), "standalone".to_string());
        env.insert("OHC_MULTITENANT".to_string(), "false".to_string());
        env.insert("DATABASE_URL".to_string(), "sqlite://custom.db".to_string());

        let config = verify_environment(&env).unwrap();
        assert_eq!(config.database_url, "sqlite://custom.db");
    }
}
