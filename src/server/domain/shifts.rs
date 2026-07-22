use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shift {
    pub id: String,
    pub tenant_id: String,
    pub location_id: Option<String>,
    pub staff_id: String,
    pub clock_in_time: String,
    pub clock_out_time: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
