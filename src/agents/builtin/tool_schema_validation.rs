use serde_json::Value;

pub fn validate_schema(args: &Value, schema: &Value) -> Result<(), String> {
    let schema_obj = match schema.as_object() {
        Some(obj) => obj,
        None => return Ok(()),
    };

    if let Some(req_array) = schema_obj.get("required").and_then(|r: &Value| r.as_array()) {
        for req in req_array {
            if let Some(req_str) = req.as_str() {
                if !args.as_object().map_or(false, |obj: &serde_json::Map<String, Value>| obj.contains_key(req_str)) {
                    return Err(format!("Validation error: Missing required property '{}'", req_str));
                }
            }
        }
    }

    if let Some(properties) = schema_obj.get("properties").and_then(|p: &Value| p.as_object()) {
        for (prop_name, prop_schema) in properties {
            if let Some(arg_val) = args.as_object().and_then(|obj: &serde_json::Map<String, Value>| obj.get(prop_name)) {
                if let Some(expected_type) = prop_schema.get("type").and_then(|t: &Value| t.as_str()) {
                    let is_valid = match expected_type {
                        "string" => arg_val.is_string(),
                        "number" | "integer" => arg_val.is_number(),
                        "boolean" => arg_val.is_boolean(),
                        "array" => arg_val.is_array(),
                        "object" => arg_val.is_object(),
                        _ => true,
                    };
                    if !is_valid {
                         return Err(format!("Validation error: Property '{}' expected type '{}', but got different type", prop_name, expected_type));
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_schema_success() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "number"}
            },
            "required": ["name"]
        });
        let args = json!({"name": "Alice", "age": 30});
        assert!(validate_schema(&args, &schema).is_ok());
    }

    #[test]
    fn test_validate_schema_missing_required() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });
        let args = json!({});
        let result = validate_schema(&args, &schema);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Validation error: Missing required property 'name'");
    }

    #[test]
    fn test_validate_schema_wrong_type() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {"type": "number"}
            }
        });
        let args = json!({"age": "30"});
        let result = validate_schema(&args, &schema);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Validation error: Property 'age' expected type 'number', but got different type");
    }
}
