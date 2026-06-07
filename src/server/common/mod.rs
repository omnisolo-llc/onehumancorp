pub use ::server_config as config;
use serde::{Deserialize, Serialize};

pub mod auth_utils;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub organization_id: Option<String>,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub jti: String,
}

#[cfg(test)]
mod common_security_tests {
    use super::auth_utils::*;

    #[tokio::test]
    async fn test_get_default_tenant_logic() {
        // We can't easily mock server_config::get() because it's a static OnceLock
        // but we can verify the function returns consistent results.
        let def = get_default_tenant();
        if ::server_config::get().multitenant {
            assert_eq!(def, "");
        } else {
            assert_eq!(def, "system");
        }
    }
}
