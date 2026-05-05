use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionRequest {
    pub tool_name: String,
    pub command: String,
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthorizationResponse {
    pub authorized: bool,
    pub reason: Option<String>,
}

#[async_trait]
pub trait BridgeTransport: Send + Sync {
    async fn request_permission(&self, req: PermissionRequest) -> Result<AuthorizationResponse, String>;
}
