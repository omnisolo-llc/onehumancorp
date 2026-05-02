use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub business_name: String,
    pub business_type: String, // e.g. "Bakery", "Handyman"
    pub flags: TenantFlags,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantFlags {
    pub enable_booking: bool,
    pub enable_pos: bool,
    pub enable_menu: bool,
    pub enable_ecommerce: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAgentAssignment {
    pub tenant_id: String,
    pub agent_id: String, // The ID of the assigned AI Department (e.g. "Operations", "Salesperson")
    pub role: String,
}
