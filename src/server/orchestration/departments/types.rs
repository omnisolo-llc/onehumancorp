use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DepartmentType {
    Operations,
    Marketing,
    Sales,
    CustomerSuccess,
    Finance,
    Legal,
    BusinessAdvisory,
}

impl FromStr for DepartmentType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "operations" => Ok(DepartmentType::Operations),
            "marketing" => Ok(DepartmentType::Marketing),
            "sales" => Ok(DepartmentType::Sales),
            "customersuccess" | "customer_success" => Ok(DepartmentType::CustomerSuccess),
            "finance" => Ok(DepartmentType::Finance),
            "legal" => Ok(DepartmentType::Legal),
            "businessadvisory" | "business_advisory" => Ok(DepartmentType::BusinessAdvisory),
            _ => Err(format!("Unknown department: {}", s)),
        }
    }
}

impl std::fmt::Display for DepartmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DepartmentType::Operations => "operations",
            DepartmentType::Marketing => "marketing",
            DepartmentType::Sales => "sales",
            DepartmentType::CustomerSuccess => "customer_success",
            DepartmentType::Finance => "finance",
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionRisk {
    AutoExecute,
    DraftForReview,
}

impl std::fmt::Display for ActionRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ActionRisk::AutoExecute => "LOW",
            ActionRisk::DraftForReview => "HIGH",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ActionRisk {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "LOW" | "AUTO_EXECUTE" => Ok(ActionRisk::AutoExecute),
            "HIGH" | "DRAFT_FOR_REVIEW" => Ok(ActionRisk::DraftForReview),
            _ => Err(format!("Unknown action risk: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub tenant_id: String,
    pub department: DepartmentType,
    pub description: String,
    pub status: ApprovalStatus,
    pub action_risk: ActionRisk,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}
