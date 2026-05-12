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
