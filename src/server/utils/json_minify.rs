use serde_json::Value;

/// MinifyJSONString takes a string, checks if it is valid JSON, and returns
/// the minified version of it (whitespace removed). If it's not valid JSON,
/// it returns the original string.

/// Minifies any embedded JSON structures found within a larger text payload.
/// Uses a simple bracket matching algorithm to find potential JSON blocks and
/// attempts to parse them. If valid, replaces the block with its minified version.
pub fn minify_embedded_json(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' || c == '[' {
            // Find the matching closing bracket
            let closing_bracket = if c == '{' { '}' } else { ']' };
            let mut depth = 1;
            let mut potential_json = String::new();
            potential_json.push(c);

            let mut in_string = false;
            let mut escape_next = false;

            let mut valid_match = false;

            while let Some(&next_c) = chars.peek() {
                chars.next();
                potential_json.push(next_c);

                if escape_next {
                    escape_next = false;
                    continue;
                }

                if next_c == '\\' {
                    escape_next = true;
                    continue;
                }

                if next_c == '"' {
                    in_string = !in_string;
                    continue;
                }

                if !in_string {
                    if next_c == c {
                        depth += 1;
                    } else if next_c == closing_bracket {
                        depth -= 1;
                        if depth == 0 {
                            valid_match = true;
                            break;
                        }
                    }
                }
            }

            if valid_match {
                // Try parsing as JSON
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&potential_json) {
                    if let Ok(minified) = serde_json::to_string(&v) {
                        result.push_str(&minified);
                    } else {
                        result.push_str(&potential_json);
                    }
                } else {
                    result.push_str(&potential_json);
                }
            } else {
                result.push_str(&potential_json);
            }
        } else {
            result.push(c);
        }
    }

    result
}

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
    fn test_minify_embedded_json() {
        let tests = vec![
            (
                "No JSON",
                "Just plain text",
                "Just plain text"
            ),
            (
                "Simple embedded JSON",
                r#"Here is the payload: {
                    "key": "value"
                }"#,
                r#"Here is the payload: {"key":"value"}"#
            ),
            (
                "Multiple embedded JSONs",
                r#"First: [
                    1, 2, 3
                ]. Second: {
                    "a": "b"
                } End."#,
                r#"First: [1,2,3]. Second: {"a":"b"} End."#
            ),
            (
                "JSON with strings containing brackets",
                r#"Text {
                    "key": "value { inside } [ brackets ]"
                }"#,
                r#"Text {"key":"value { inside } [ brackets ]"}"#
            ),
            (
                "Invalid JSON embedded (should be left alone)",
                r#"Text {
                    "key": "value",
                }"#,
                r#"Text {
                    "key": "value",
                }"#
            ),
        ];

        for (name, input, expected) in tests {
            let result = minify_embedded_json(input);
            assert_eq!(result, expected, "Failed on test case: {}", name);
        }
    }

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
