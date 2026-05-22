use serde_json::Value;

fn validate_schema(args: &serde_json::Value, schema: &serde_json::Value) -> Result<(), String> {
        if let Some(req_array) = schema.get("required").and_then(|v| v.as_array()) {
            if let Some(args_obj) = args.as_object() {
                for req in req_array {
                    if let Some(req_str) = req.as_str() {
                        if !args_obj.contains_key(req_str) {
                            return Err(format!("Missing required field: {}", req_str));
                        }
                    }
                }
            } else if !req_array.is_empty() {
                return Err("arguments must be an object".to_string());
            }
        }

        if let Some(props) = schema.get("properties").and_then(|v| v.as_object()) {
            if let Some(args_obj) = args.as_object() {
                for (k, v) in args_obj {
                    if let Some(prop_schema) = props.get(k) {
                        if let Some(expected_type) = prop_schema.get("type").and_then(|t| t.as_str()) {
                            let type_matches = match expected_type {
                                "string" => v.is_string(),
                                "number" | "integer" => v.is_number(),
                                "boolean" => v.is_boolean(),
                                "object" => v.is_object(),
                                "array" => v.is_array(),
                                _ => true, // Unknown type, skip validation for now
                            };
                            if !type_matches {
                                return Err(format!("Expected {}, got {}", k, expected_type));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

fn main() {
    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "str_param": { "type": "string" },
            "int_param": { "type": "integer" }
        },
        "required": ["str_param"]
    });

    let args = serde_json::json!({ "int_param": 42 });
    println!("{:?}", validate_schema(&args, &schema));
}
