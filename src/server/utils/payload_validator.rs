
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


macro_rules! define_validator {
    ($name:ident, $($validator:ident($field:expr)),*) => {
        pub fn $name(payload: &Value) -> Result<(), Vec<SchemaError>> {
            let mut errors = Vec::new();
            $(
                if let Err(e) = $validator(payload, $field) {
                    errors.push(e);
                }
            )*
            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }
    };
}

define_validator!(validate_sales_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_number("target_revenue"), validate_string("crm_sync_id"), validate_number("data_points_analyzed"));
define_validator!(validate_sales_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_number("target_revenue"), validate_string("crm_sync_id"), validate_string("report_format"));
define_validator!(validate_sales_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_number("target_revenue"), validate_string("crm_sync_id"));
define_validator!(validate_sales_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_number("target_revenue"), validate_string("crm_sync_id"));
define_validator!(validate_sales_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_number("target_revenue"), validate_string("crm_sync_id"));
define_validator!(validate_marketing_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_number("data_points_analyzed"));
define_validator!(validate_marketing_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_string("report_format"));
define_validator!(validate_marketing_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_marketing_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_marketing_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_engineering_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("github_repo"), validate_bool("requires_code_review"), validate_number("data_points_analyzed"));
define_validator!(validate_engineering_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("github_repo"), validate_bool("requires_code_review"), validate_string("report_format"));
define_validator!(validate_engineering_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("github_repo"), validate_bool("requires_code_review"));
define_validator!(validate_engineering_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("github_repo"), validate_bool("requires_code_review"));
define_validator!(validate_engineering_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("github_repo"), validate_bool("requires_code_review"));
define_validator!(validate_hr_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("candidate_email"), validate_bool("background_check_passed"), validate_number("data_points_analyzed"));
define_validator!(validate_hr_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("candidate_email"), validate_bool("background_check_passed"), validate_string("report_format"));
define_validator!(validate_hr_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("candidate_email"), validate_bool("background_check_passed"));
define_validator!(validate_hr_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("candidate_email"), validate_bool("background_check_passed"));
define_validator!(validate_hr_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("candidate_email"), validate_bool("background_check_passed"));
define_validator!(validate_finance_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_number("data_points_analyzed"));
define_validator!(validate_finance_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_string("report_format"));
define_validator!(validate_finance_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_finance_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_finance_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_legal_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_number("data_points_analyzed"));
define_validator!(validate_legal_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_string("report_format"));
define_validator!(validate_legal_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_legal_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_legal_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_operations_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_number("data_points_analyzed"));
define_validator!(validate_operations_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_string("report_format"));
define_validator!(validate_operations_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_operations_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_operations_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_product_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_number("data_points_analyzed"));
define_validator!(validate_product_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_string("report_format"));
define_validator!(validate_product_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_product_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_product_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_design_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_number("data_points_analyzed"));
define_validator!(validate_design_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_string("report_format"));
define_validator!(validate_design_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_design_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_design_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_support_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_number("data_points_analyzed"));
define_validator!(validate_support_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"), validate_string("report_format"));
define_validator!(validate_support_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_support_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_support_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("internal_tracking_code"));
define_validator!(validate_security_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_security_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_security_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_security_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_security_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_compliance_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_compliance_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_compliance_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_compliance_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_compliance_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_it_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_it_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_it_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_it_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_it_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_facilities_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_facilities_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_facilities_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_facilities_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_facilities_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_exec_analysis_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_exec_reporting_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_exec_onboarding_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_exec_sync_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));
define_validator!(validate_exec_audit_payload, validate_string("department_id"), validate_string("action_type"), validate_number("priority_score"), validate_string("strict_clearance_level"));

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
