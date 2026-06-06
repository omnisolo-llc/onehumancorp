use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadGenCampaign {
    pub id: String,
    pub tenant_id: String,
    pub budget: f64,
    pub radius_miles: f64,
    pub zip_code: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadGenCampaignRequest {
    pub budget: f64,
    pub radius_miles: f64,
    pub zip_code: String,
}
