
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


pub fn validate_department_payload(department: &str, action: &str, payload: &Value) -> Result<(), Vec<SchemaError>> {
    let mut errors = Vec::new();

    if let Err(e) = validate_string(payload, "department_id") { errors.push(e); }
    if let Err(e) = validate_string(payload, "action_type") { errors.push(e); }
    if let Err(e) = validate_number(payload, "priority_score") { errors.push(e); }

    match action {
        "Analysis" => {
            if let Err(e) = validate_number(payload, "data_points_analyzed") { errors.push(e); }
        }
        "Reporting" => {
            if let Err(e) = validate_string(payload, "report_format") { errors.push(e); }
        }
        "Onboarding" | "Sync" | "Audit" => {}
        _ => {}
    }

    match department {
        "Sales" => {
            if let Err(e) = validate_number(payload, "target_revenue") { errors.push(e); }
            if let Err(e) = validate_string(payload, "crm_sync_id") { errors.push(e); }
        }
        "Marketing" | "Legal" | "Operations" | "Finance" | "Design" | "Product" | "Support" | "Compliance" | "IT" | "Facilities" | "Exec" => {
            if let Err(e) = validate_string(payload, "internal_tracking_code") { errors.push(e); }
        }
        "Engineering" => {
            if let Err(e) = validate_string(payload, "github_repo") { errors.push(e); }
            if let Err(e) = validate_bool(payload, "requires_code_review") { errors.push(e); }
        }
        "HR" => {
            if let Err(e) = validate_string(payload, "candidate_email") { errors.push(e); }
            if let Err(e) = validate_bool(payload, "background_check_passed") { errors.push(e); }
        }
        "Security" => {
            if let Err(e) = validate_string(payload, "strict_clearance_level") { errors.push(e); }
        }
        _ => {}
    }

    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

pub fn validate_dynamic_payload(department: &str, action: &str, payload: &Value) -> Result<(), Vec<SchemaError>> {
    let valid_departments = [
        "Sales", "Marketing", "Engineering", "HR", "Finance", "Legal", "Operations",
        "Product", "Design", "Support", "Security", "Compliance", "IT", "Facilities", "Exec"
    ];
    let valid_actions = ["Analysis", "Reporting", "Onboarding", "Sync", "Audit"];

    if !valid_departments.contains(&department) || !valid_actions.contains(&action) {
        return Err(vec![SchemaError::BusinessRuleViolation("Unknown department or action".to_string())]);
    }

    validate_department_payload(department, action, payload)
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
