
use serde_json::Value;

/// Payload Schema Validator for OHC-SIP
/// This module provides strict runtime schema validation for complex JSON payloads
/// embedded within `agent_missions`. It guarantees type safety across Cloud and Local DB syncs.

pub enum SchemaError {
    MissingField(String),
    TypeMismatch(String, String),
    InvalidFormat(String),
    BusinessRuleViolation(String),
}

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaError::MissingField(fld) => write!(f, "Missing required field: {}", fld),
            SchemaError::TypeMismatch(fld, expected) => write!(f, "Field {} must be of type {}", fld, expected),
            SchemaError::InvalidFormat(fld) => write!(f, "Field {} has invalid format", fld),
            SchemaError::BusinessRuleViolation(msg) => write!(f, "Business rule violation: {}", msg),
        }
    }
}

pub fn validate_string(v: &Value, field: &str) -> Result<(), SchemaError> {
    v.get(field).ok_or_else(|| SchemaError::MissingField(field.to_string()))?
        .as_str().ok_or_else(|| SchemaError::TypeMismatch(field.to_string(), "String".to_string()))?;
    Ok(())
}

pub fn validate_number(v: &Value, field: &str) -> Result<(), SchemaError> {
    v.get(field).ok_or_else(|| SchemaError::MissingField(field.to_string()))?
        .as_f64().ok_or_else(|| SchemaError::TypeMismatch(field.to_string(), "Number".to_string()))?;
    Ok(())
}

pub fn validate_bool(v: &Value, field: &str) -> Result<(), SchemaError> {
    v.get(field).ok_or_else(|| SchemaError::MissingField(field.to_string()))?
        .as_bool().ok_or_else(|| SchemaError::TypeMismatch(field.to_string(), "Boolean".to_string()))?;
    Ok(())
}

pub fn validate_sales_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Sales requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_number(payload, "target_revenue") { errors.push(e); }
    if let Err(e) = validate_string(payload, "crm_sync_id") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_sales_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Sales requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_number(payload, "target_revenue") { errors.push(e); }
    if let Err(e) = validate_string(payload, "crm_sync_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_sales_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Sales requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_number(payload, "target_revenue") { errors.push(e); }
    if let Err(e) = validate_string(payload, "crm_sync_id") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_sales_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Sales requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_number(payload, "target_revenue") { errors.push(e); }
    if let Err(e) = validate_string(payload, "crm_sync_id") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_sales_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Sales requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_number(payload, "target_revenue") { errors.push(e); }
    if let Err(e) = validate_string(payload, "crm_sync_id") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_marketing_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Marketing requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_marketing_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Marketing requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_marketing_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Marketing requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_marketing_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Marketing requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_marketing_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Marketing requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_engineering_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Engineering requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "github_repo") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "requires_code_review") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_engineering_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Engineering requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "github_repo") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "requires_code_review") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_engineering_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Engineering requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "github_repo") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "requires_code_review") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_engineering_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Engineering requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "github_repo") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "requires_code_review") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_engineering_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Engineering requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "github_repo") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "requires_code_review") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_hr_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core HR requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "candidate_email") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "background_check_passed") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_hr_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core HR requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "candidate_email") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "background_check_passed") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_hr_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core HR requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "candidate_email") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "background_check_passed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_hr_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core HR requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "candidate_email") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "background_check_passed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_hr_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core HR requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "candidate_email") { errors.push(e); }
    if let Err(e) = validate_bool(payload, "background_check_passed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_finance_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Finance requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_finance_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Finance requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_finance_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Finance requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_finance_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Finance requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_finance_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Finance requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_legal_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Legal requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_legal_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Legal requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_legal_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Legal requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_legal_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Legal requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_legal_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Legal requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_operations_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Operations requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_operations_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Operations requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_operations_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Operations requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_operations_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Operations requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_operations_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Operations requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_product_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Product requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_product_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Product requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_product_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Product requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_product_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Product requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_product_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Product requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_design_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Design requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_design_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Design requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_design_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Design requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_design_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Design requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_design_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Design requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_support_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Support requirement for Analysis
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_support_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Support requirement for Reporting
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
    if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_support_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Support requirement for Onboarding
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_support_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Support requirement for Sync
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_support_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    // Core Support requirement for Audit
    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_security_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_security_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_security_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_security_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_security_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_compliance_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_compliance_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_compliance_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_compliance_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_compliance_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_it_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_it_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_it_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_it_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_it_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_facilities_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_facilities_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_facilities_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_facilities_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_facilities_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_exec_analysis_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_exec_reporting_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_exec_onboarding_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_exec_sync_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_exec_audit_payload(payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }
    if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

// Unified Router
pub fn validate_dynamic_payload(department: &str, action: &str, payload: &Value) -> Result<(), Vec<SchemaError>> {
    match (department, action) {
        ("Sales", "Analysis") => validate_sales_analysis_payload(payload),
        ("Sales", "Reporting") => validate_sales_reporting_payload(payload),
        ("Sales", "Onboarding") => validate_sales_onboarding_payload(payload),
        ("Sales", "Sync") => validate_sales_sync_payload(payload),
        ("Sales", "Audit") => validate_sales_audit_payload(payload),
        ("Marketing", "Analysis") => validate_marketing_analysis_payload(payload),
        ("Marketing", "Reporting") => validate_marketing_reporting_payload(payload),
        ("Marketing", "Onboarding") => validate_marketing_onboarding_payload(payload),
        ("Marketing", "Sync") => validate_marketing_sync_payload(payload),
        ("Marketing", "Audit") => validate_marketing_audit_payload(payload),
        ("Engineering", "Analysis") => validate_engineering_analysis_payload(payload),
        ("Engineering", "Reporting") => validate_engineering_reporting_payload(payload),
        ("Engineering", "Onboarding") => validate_engineering_onboarding_payload(payload),
        ("Engineering", "Sync") => validate_engineering_sync_payload(payload),
        ("Engineering", "Audit") => validate_engineering_audit_payload(payload),
        ("HR", "Analysis") => validate_hr_analysis_payload(payload),
        ("HR", "Reporting") => validate_hr_reporting_payload(payload),
        ("HR", "Onboarding") => validate_hr_onboarding_payload(payload),
        ("HR", "Sync") => validate_hr_sync_payload(payload),
        ("HR", "Audit") => validate_hr_audit_payload(payload),
        ("Finance", "Analysis") => validate_finance_analysis_payload(payload),
        ("Finance", "Reporting") => validate_finance_reporting_payload(payload),
        ("Finance", "Onboarding") => validate_finance_onboarding_payload(payload),
        ("Finance", "Sync") => validate_finance_sync_payload(payload),
        ("Finance", "Audit") => validate_finance_audit_payload(payload),
        ("Legal", "Analysis") => validate_legal_analysis_payload(payload),
        ("Legal", "Reporting") => validate_legal_reporting_payload(payload),
        ("Legal", "Onboarding") => validate_legal_onboarding_payload(payload),
        ("Legal", "Sync") => validate_legal_sync_payload(payload),
        ("Legal", "Audit") => validate_legal_audit_payload(payload),
        ("Operations", "Analysis") => validate_operations_analysis_payload(payload),
        ("Operations", "Reporting") => validate_operations_reporting_payload(payload),
        ("Operations", "Onboarding") => validate_operations_onboarding_payload(payload),
        ("Operations", "Sync") => validate_operations_sync_payload(payload),
        ("Operations", "Audit") => validate_operations_audit_payload(payload),
        ("Product", "Analysis") => validate_product_analysis_payload(payload),
        ("Product", "Reporting") => validate_product_reporting_payload(payload),
        ("Product", "Onboarding") => validate_product_onboarding_payload(payload),
        ("Product", "Sync") => validate_product_sync_payload(payload),
        ("Product", "Audit") => validate_product_audit_payload(payload),
        ("Design", "Analysis") => validate_design_analysis_payload(payload),
        ("Design", "Reporting") => validate_design_reporting_payload(payload),
        ("Design", "Onboarding") => validate_design_onboarding_payload(payload),
        ("Design", "Sync") => validate_design_sync_payload(payload),
        ("Design", "Audit") => validate_design_audit_payload(payload),
        ("Support", "Analysis") => validate_support_analysis_payload(payload),
        ("Support", "Reporting") => validate_support_reporting_payload(payload),
        ("Support", "Onboarding") => validate_support_onboarding_payload(payload),
        ("Support", "Sync") => validate_support_sync_payload(payload),
        ("Support", "Audit") => validate_support_audit_payload(payload),
        ("Security", "Analysis") => validate_security_analysis_payload(payload),
        ("Security", "Reporting") => validate_security_reporting_payload(payload),
        ("Security", "Onboarding") => validate_security_onboarding_payload(payload),
        ("Security", "Sync") => validate_security_sync_payload(payload),
        ("Security", "Audit") => validate_security_audit_payload(payload),
        ("Compliance", "Analysis") => validate_compliance_analysis_payload(payload),
        ("Compliance", "Reporting") => validate_compliance_reporting_payload(payload),
        ("Compliance", "Onboarding") => validate_compliance_onboarding_payload(payload),
        ("Compliance", "Sync") => validate_compliance_sync_payload(payload),
        ("Compliance", "Audit") => validate_compliance_audit_payload(payload),
        ("IT", "Analysis") => validate_it_analysis_payload(payload),
        ("IT", "Reporting") => validate_it_reporting_payload(payload),
        ("IT", "Onboarding") => validate_it_onboarding_payload(payload),
        ("IT", "Sync") => validate_it_sync_payload(payload),
        ("IT", "Audit") => validate_it_audit_payload(payload),
        ("Facilities", "Analysis") => validate_facilities_analysis_payload(payload),
        ("Facilities", "Reporting") => validate_facilities_reporting_payload(payload),
        ("Facilities", "Onboarding") => validate_facilities_onboarding_payload(payload),
        ("Facilities", "Sync") => validate_facilities_sync_payload(payload),
        ("Facilities", "Audit") => validate_facilities_audit_payload(payload),
        ("Exec", "Analysis") => validate_exec_analysis_payload(payload),
        ("Exec", "Reporting") => validate_exec_reporting_payload(payload),
        ("Exec", "Onboarding") => validate_exec_onboarding_payload(payload),
        ("Exec", "Sync") => validate_exec_sync_payload(payload),
        ("Exec", "Audit") => validate_exec_audit_payload(payload),

        _ => Err(vec![SchemaError::BusinessRuleViolation("Unknown department or action".to_string())]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn complete_payload() -> Value {
        json!({
            "department_id": "dept-1",
            "action_type": "Analysis",
            "priority_score": 0.91,
            "target_revenue": 125000.0,
            "crm_sync_id": "crm-1",
            "data_points_analyzed": 42,
            "report_format": "pdf",
            "internal_tracking_code": "track-1",
            "github_repo": "owner/repo",
            "requires_code_review": true,
            "candidate_email": "candidate@example.com",
            "background_check_passed": true,
            "strict_clearance_level": "confidential"
        })
    }

    #[test]
    fn primitive_validators_report_missing_and_type_errors() {
        let payload = json!({
            "name": "Ada",
            "score": 1.0,
            "active": true,
            "wrong": "not-a-number"
        });

        assert!(validate_string(&payload, "name").is_ok());
        assert!(validate_number(&payload, "score").is_ok());
        assert!(validate_bool(&payload, "active").is_ok());

        let missing = validate_string(&payload, "missing").unwrap_err();
        assert_eq!(missing.to_string(), "Missing required field: missing");

        let wrong_type = validate_number(&payload, "wrong").unwrap_err();
        assert_eq!(wrong_type.to_string(), "Field wrong must be of type Number");

        assert_eq!(
            SchemaError::InvalidFormat("email".to_string()).to_string(),
            "Field email has invalid format"
        );
    }

    #[test]
    fn dynamic_payload_accepts_all_registered_department_actions() {
        let payload = complete_payload();
        let routes = [
            ("Sales", "Analysis"), ("Sales", "Reporting"), ("Sales", "Onboarding"), ("Sales", "Sync"), ("Sales", "Audit"),
            ("Marketing", "Analysis"), ("Marketing", "Reporting"), ("Marketing", "Onboarding"), ("Marketing", "Sync"), ("Marketing", "Audit"),
            ("Engineering", "Analysis"), ("Engineering", "Reporting"), ("Engineering", "Onboarding"), ("Engineering", "Sync"), ("Engineering", "Audit"),
            ("HR", "Analysis"), ("HR", "Reporting"), ("HR", "Onboarding"), ("HR", "Sync"), ("HR", "Audit"),
            ("Finance", "Analysis"), ("Finance", "Reporting"), ("Finance", "Onboarding"), ("Finance", "Sync"), ("Finance", "Audit"),
            ("Legal", "Analysis"), ("Legal", "Reporting"), ("Legal", "Onboarding"), ("Legal", "Sync"), ("Legal", "Audit"),
            ("Operations", "Analysis"), ("Operations", "Reporting"), ("Operations", "Onboarding"), ("Operations", "Sync"), ("Operations", "Audit"),
            ("Product", "Analysis"), ("Product", "Reporting"), ("Product", "Onboarding"), ("Product", "Sync"), ("Product", "Audit"),
            ("Design", "Analysis"), ("Design", "Reporting"), ("Design", "Onboarding"), ("Design", "Sync"), ("Design", "Audit"),
            ("Support", "Analysis"), ("Support", "Reporting"), ("Support", "Onboarding"), ("Support", "Sync"), ("Support", "Audit"),
            ("Security", "Analysis"), ("Security", "Reporting"), ("Security", "Onboarding"), ("Security", "Sync"), ("Security", "Audit"),
            ("Compliance", "Analysis"), ("Compliance", "Reporting"), ("Compliance", "Onboarding"), ("Compliance", "Sync"), ("Compliance", "Audit"),
            ("IT", "Analysis"), ("IT", "Reporting"), ("IT", "Onboarding"), ("IT", "Sync"), ("IT", "Audit"),
            ("Facilities", "Analysis"), ("Facilities", "Reporting"), ("Facilities", "Onboarding"), ("Facilities", "Sync"), ("Facilities", "Audit"),
            ("Exec", "Analysis"), ("Exec", "Reporting"), ("Exec", "Onboarding"), ("Exec", "Sync"), ("Exec", "Audit"),
        ];

        for (department, action) in routes {
            let result = validate_dynamic_payload(department, action, &payload);
            assert!(result.is_ok(), "{} {} should validate", department, action);
        }
    }

    #[test]
    fn dynamic_payload_rejects_missing_type_mismatched_and_unknown_routes() {
        let missing = json!({
            "action_type": "Analysis",
            "priority_score": 1.0,
            "target_revenue": 10.0,
            "crm_sync_id": "crm-1",
            "data_points_analyzed": 1
        });
        let errors = validate_dynamic_payload("Sales", "Analysis", &missing).unwrap_err();
        assert!(errors.iter().any(|error| error.to_string() == "Missing required field: department_id"));

        let mut wrong_type = complete_payload();
        wrong_type["requires_code_review"] = json!("yes");
        let errors = validate_dynamic_payload("Engineering", "Sync", &wrong_type).unwrap_err();
        assert!(errors.iter().any(|error| error.to_string() == "Field requires_code_review must be of type Boolean"));

        let errors = validate_dynamic_payload("Unknown", "Analysis", &complete_payload()).unwrap_err();
        assert_eq!(
            errors[0].to_string(),
            "Business rule violation: Unknown department or action"
        );
    }
}
