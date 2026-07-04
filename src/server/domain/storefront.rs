use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storefront {
    pub tenant_id: String,
    pub domain: String,
    pub active: bool,
    pub html_content: String,
}
