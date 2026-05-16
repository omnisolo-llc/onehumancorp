use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DepartmentType {
    OrderManager,
    SocialMediaManager,
    SEOBooster,
    CustomerSupport,
    EmailMarketer,
    Legal,
    BusinessAdvisory,
}

impl FromStr for DepartmentType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "operations" | "ordermanager" | "order_manager" => Ok(DepartmentType::OrderManager),
            "marketing" | "socialmediamanager" | "social_media_manager" => Ok(DepartmentType::SocialMediaManager),
            "sales" | "seobooster" | "seo_booster" => Ok(DepartmentType::SEOBooster),
            "customersuccess" | "customer_success" | "customersupport" | "customer_support" => Ok(DepartmentType::CustomerSupport),
            "finance" | "emailmarketer" | "email_marketer" => Ok(DepartmentType::EmailMarketer),
            "legal" => Ok(DepartmentType::Legal),
            "businessadvisory" | "business_advisory" => Ok(DepartmentType::BusinessAdvisory),
            _ => Err(format!("Unknown department: {}", s)),
        }
    }
}

impl std::fmt::Display for DepartmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DepartmentType::OrderManager => "operations",
            DepartmentType::SocialMediaManager => "marketing",
            DepartmentType::SEOBooster => "sales",
            DepartmentType::CustomerSupport => "customer_success",
            DepartmentType::EmailMarketer => "finance",
            DepartmentType::Legal => "legal",
            DepartmentType::BusinessAdvisory => "business_advisory",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentConfig {
    pub tone_of_voice: String,
    pub auto_approve_limits: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentEvent {
    pub id: String,
    pub tenant_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub tenant_id: String,
    pub department: DepartmentType,
    pub description: String,
    pub status: ApprovalStatus,
    pub action_risk: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}
