use serde_json::Value;

/// MinifyJSONString takes a string, checks if it is valid JSON, and returns
/// the minified version of it (whitespace removed). If it's not valid JSON,
/// it returns the original string.
pub fn minify_json_string(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return input.to_string();
    }

    // Quick check to see if it even looks like JSON
    if !(trimmed.starts_with('{') && trimmed.ends_with('}')) &&
       !(trimmed.starts_with('[') && trimmed.ends_with(']')) {
        return input.to_string();
    }

    match serde_json::from_str::<Value>(trimmed) {
        Ok(v) => serde_json::to_string(&v).unwrap_or_else(|_| input.to_string()),
        Err(_) => input.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify_json_string() {
        let tests = vec![
            ("Empty string", "", ""),
            ("Whitespace only", "   \n  \t ", "   \n  \t "),
            ("Not JSON", "Just a normal string", "Just a normal string"),
            ("Not JSON with whitespace", "  Just a normal string  \n", "  Just a normal string  \n"),
            (
                "Valid JSON Object",
                r#"{
                    "key1": "value1",
                    "key2": 2
                }"#,
                r#"{"key1":"value1","key2":2}"#,
            ),
            (
                "Valid JSON Object with surrounding whitespace",
                r#"  {
                    "key1": "value1",
                    "key2": 2
                }  "#,
                r#"{"key1":"value1","key2":2}"#,
            ),
            (
                "Valid JSON Array",
                r#"[
                    "item1",
                    "item2"
                ]"#,
                r#"["item1","item2"]"#,
            ),
            (
                "Invalid JSON that looks like JSON",
                r#"{
                    "key1": "value1",
                    "key2": 2,
                }"#,
                r#"{
                    "key1": "value1",
                    "key2": 2,
                }"#,
            ),
            (
                "Invalid JSON that looks like JSON with surrounding whitespace",
                r#"  {
                    "key1": "value1",
                    "key2": 2,
                }  "#,
                r#"  {
                    "key1": "value1",
                    "key2": 2,
                }  "#,
            ),
        ];

        for (name, input, expected) in tests {
            let result = minify_json_string(input);
            assert_eq!(result, expected, "Failed on test case: {}", name);
        }
    }
}
