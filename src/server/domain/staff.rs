use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StaffMember {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub role: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimecardEvent {
    pub id: String,
    pub tenant_id: String,
    pub staff_member_id: String,
    pub event_type: String, // 'CLOCK_IN', 'CLOCK_OUT'
    pub client_timestamp: chrono::DateTime<chrono::Utc>,
    pub server_timestamp: chrono::DateTime<chrono::Utc>,
    pub sync_id: Option<String>,
}
