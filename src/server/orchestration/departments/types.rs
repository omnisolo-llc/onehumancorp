use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DepartmentType {
    Operations,
    Marketing,
    Sales,
    CustomerSuccess,
    Finance,
    Legal,
    Advisory,
}

impl fmt::Display for DepartmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for DepartmentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Operations" => Ok(DepartmentType::Operations),
            "Marketing" => Ok(DepartmentType::Marketing),
            "Sales" => Ok(DepartmentType::Sales),
            "CustomerSuccess" => Ok(DepartmentType::CustomerSuccess),
            "Finance" => Ok(DepartmentType::Finance),
            "Legal" => Ok(DepartmentType::Legal),
            "Advisory" => Ok(DepartmentType::Advisory),
            _ => Err(format!("Unknown department type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentConfig {
    pub tone_of_voice: String,
    pub auto_approve_limits: f64,
    #[serde(default)]
    pub auto_execute_enabled: bool,
}

impl Default for DepartmentConfig {
    fn default() -> Self {
        Self {
            tone_of_voice: "helpful and professional".to_string(),
            auto_approve_limits: 0.0,
            auto_execute_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentEvent {
    pub id: String,
    pub tenant_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalStatus {
    PendingApproval,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionRisk {
    AutoExecute,
    DraftForReview,
}

impl fmt::Display for ActionRisk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for ActionRisk {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AutoExecute" => Ok(ActionRisk::AutoExecute),
            "DraftForReview" => Ok(ActionRisk::DraftForReview),
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
