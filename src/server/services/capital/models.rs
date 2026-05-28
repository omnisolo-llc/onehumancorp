use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapitalOffer {
    pub id: String,
    pub tenant_id: String,
    pub advance_amount: f64,
    pub flat_fee: f64,
    pub repayment_percentage: f64,
    pub status: CapitalContractStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CapitalContractStatus {
    Offered,
    Active,
    Repaid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapitalContract {
    pub id: String,
    pub tenant_id: String,
    pub advance_amount: f64,
    pub flat_fee: f64,
    pub repayment_percentage: f64,
    pub repaid_amount: f64,
    pub status: CapitalContractStatus,
}
