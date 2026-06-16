use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub mod router;
pub mod sre_handler;
pub mod marketing_handler;
pub mod inbox_handler;
pub mod sales_handler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    pub feature_type: String,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    #[serde(flatten)]
    pub payload: serde_json::Value,
}

#[async_trait]
pub trait ActionHandler: Send + Sync {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String>;
}
