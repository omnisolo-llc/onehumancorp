use serde_json::Value;

/// Robust JSON Schema validator for tool arguments.
pub struct JsonSchemaValidator;

impl JsonSchemaValidator {
    pub fn validate(args: &Value, schema: &Value) -> Result<(), String> {
        let mut errors = Vec::new();
        Self::validate_recursive(args, schema, "", &mut errors);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    fn validate_recursive(args: &Value, schema: &Value, path: &str, errors: &mut Vec<String>) {
        let prefix = if path.is_empty() {
            String::new()
        } else {
            format!("{}.", path)
        };

        // 1. Required fields check
        if let Some(req_array) = schema.get("required").and_then(|v| v.as_array()) {
            if let Some(args_obj) = args.as_object() {
                for req in req_array {
                    if let Some(req_str) = req.as_str() {
                        if !args_obj.contains_key(req_str) {
                            errors.push(format!("missing required parameter: '{}{}'", prefix, req_str));
                        }
                    }
                }
            } else if !req_array.is_empty() {
                let p = if path.is_empty() {
                    "arguments".to_string()
                } else {
                    format!("parameter '{}'", path)
                };
                errors.push(format!("{} must be an object", p));
            }
        }

        // 2. Properties validation
        if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
            if let Some(args_obj) = args.as_object() {
                for (k, v) in args_obj {
                    if let Some(prop_schema) = props.get(k) {
                        let current_path = format!("{}{}", prefix, k);

                        // Validate type
                        if let Some(expected_type) = prop_schema.get("type").and_then(|t| t.as_str()) {
                            let type_matches = match expected_type {
                                "string" => v.is_string(),
                                "number" => v.is_number(),
                                "integer" => v.is_i64() || (v.is_number() && v.as_f64().map(|f| f.fract() == 0.0).unwrap_or(false)),
                                "boolean" => v.is_boolean(),
                                "object" => v.is_object(),
                                "array" => v.is_array(),
                                "null" => v.is_null(),
                                _ => true,
                            };
                            if !type_matches {
                                errors.push(format!(
                                    "parameter '{}' has invalid type: expected {}, got {}",
                                    current_path,
                                    expected_type,
                                    Self::get_type_name(v)
                                ));
                            }
                        }

                        // Validate enum
                        if let Some(enum_vals) = prop_schema.get("enum").and_then(|e| e.as_array()) {
                            if !enum_vals.contains(v) {
                                let allowed: Vec<String> = enum_vals.iter().map(|ev| ev.to_string()).collect();
                                errors.push(format!(
                                    "parameter '{}' has invalid value: expected one of [{}], got {}",
                                    current_path,
                                    allowed.join(", "),
                                    v
                                ));
                            }
                        }

                        // Recurse into objects
                        if v.is_object() {
                            Self::validate_recursive(v, prop_schema, &current_path, errors);
                        }

                        // Recurse into arrays
                        if let (Some(arr), Some(items_schema)) = (v.as_array(), prop_schema.get("items")) {
                            for (i, item) in arr.iter().enumerate() {
                                let item_path = format!("{}[{}]", current_path, i);

                                if let Some(expected_type) = items_schema.get("type").and_then(|t| t.as_str()) {
                                    let type_matches = match expected_type {
                                        "string" => item.is_string(),
                                        "number" => item.is_number(),
                                        "integer" => item.is_i64() || (item.is_number() && item.as_f64().map(|f| f.fract() == 0.0).unwrap_or(false)),
                                        "boolean" => item.is_boolean(),
                                        "object" => item.is_object(),
                                        "array" => item.is_array(),
                                        "null" => item.is_null(),
                                        _ => true,
                                    };
                                    if !type_matches {
                                        errors.push(format!(
                                            "item at '{}' has invalid type: expected {}, got {}",
                                            item_path,
                                            expected_type,
                                            Self::get_type_name(item)
                                        ));
                                    }
                                }

                                if item.is_object() {
                                    Self::validate_recursive(item, items_schema, &item_path, errors);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_type_name(v: &Value) -> &'static str {
        if v.is_string() {
            "string"
        } else if v.is_i64() {
            "integer"
        } else if v.is_number() {
            "number"
        } else if v.is_boolean() {
            "boolean"
        } else if v.is_object() {
            "object"
        } else if v.is_array() {
            "array"
        } else if v.is_null() {
            "null"
        } else {
            "unknown"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_nested_object() {
        let schema = json!({
            "type": "object",
            "properties": {
                "user": {
                    "type": "object",
                    "properties": {
                        "id": { "type": "integer" },
                        "name": { "type": "string" }
                    },
                    "required": ["id"]
                }
            },
            "required": ["user"]
        });

        // Valid
        let args = json!({ "user": { "id": 1, "name": "Alice" } });
        assert!(JsonSchemaValidator::validate(&args, &schema).is_ok());

        // Missing nested required field
        let args = json!({ "user": { "name": "Alice" } });
        let err = JsonSchemaValidator::validate(&args, &schema).unwrap_err();
        assert!(err.contains("missing required parameter: 'user.id'"));

        // Invalid nested type
        let args = json!({ "user": { "id": "not an int" } });
        let err = JsonSchemaValidator::validate(&args, &schema).unwrap_err();
        assert!(err.contains("parameter 'user.id' has invalid type: expected integer, got string"));
    }

    #[test]
    fn test_validate_array_of_objects() {
        let schema = json!({
            "type": "object",
            "properties": {
                "items": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "sku": { "type": "string" },
                            "qty": { "type": "integer" }
                        },
                        "required": ["sku"]
                    }
                }
            }
        });

        // Valid
        let args = json!({
            "items": [
                { "sku": "A1", "qty": 5 },
                { "sku": "B2" }
            ]
        });
        assert!(JsonSchemaValidator::validate(&args, &schema).is_ok());

        // Invalid item in array
        let args = json!({
            "items": [
                { "sku": "A1" },
                { "qty": 10 } // Missing sku
            ]
        });
        let err = JsonSchemaValidator::validate(&args, &schema).unwrap_err();
        assert!(err.contains("missing required parameter: 'items[1].sku'"));
    }

    #[test]
    fn test_validate_enum() {
        let schema = json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["active", "inactive", "pending"]
                }
            }
        });

        // Valid
        let args = json!({ "status": "active" });
        assert!(JsonSchemaValidator::validate(&args, &schema).is_ok());

        // Invalid enum value
        let args = json!({ "status": "deleted" });
        let err = JsonSchemaValidator::validate(&args, &schema).unwrap_err();
        assert!(err.contains("parameter 'status' has invalid value: expected one of [\"active\", \"inactive\", \"pending\"], got \"deleted\""));
    }

    #[test]
    fn test_validate_integer_float() {
        let schema = json!({
            "type": "object",
            "properties": {
                "count": { "type": "integer" }
            }
        });

        // Valid integer
        let args = json!({ "count": 10 });
        assert!(JsonSchemaValidator::validate(&args, &schema).is_ok());

        // Valid float representing integer
        let args = json!({ "count": 10.0 });
        assert!(JsonSchemaValidator::validate(&args, &schema).is_ok());

        // Invalid float
        let args = json!({ "count": 10.5 });
        let err = JsonSchemaValidator::validate(&args, &schema).unwrap_err();
        assert!(err.contains("parameter 'count' has invalid type: expected integer, got number"));
    }
}
