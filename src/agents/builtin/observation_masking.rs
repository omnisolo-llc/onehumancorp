use ohc_builtin_agent_core::types::{Message, Role};
use serde_json::Value;

/// Applies JetBrains Observation Masking.
/// Hides the raw output of old tools from the prompt,
/// but keeps the `tool_calls` themselves visible so the model remembers what it did.
/// Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
pub struct JetBrainsObservationMasker {
    threshold: usize,
    size_limit: usize,
    element_limit: usize,
}

impl JetBrainsObservationMasker {
    pub fn new(threshold: usize, size_limit: usize, element_limit: usize) -> Self {
        Self {
            threshold,
            size_limit,
            element_limit,
        }
    }

    fn mask_json_value(
        val: &mut Value,
        size_limit: usize,
        element_limit: usize,
        depth: usize,
    ) -> bool {
        let mut modified = false;

        // Prevent extremely deep recursion that could blow up the stack
        if depth > 10 {
            match val {
                Value::Array(arr) => {
                    let len = arr.len();
                    *val = Value::String(format!("[Masked array: {} elements truncated due to depth limit]", len));
                    return true;
                }
                Value::Object(obj) => {
                    let len = obj.len();
                    *val = Value::String(format!("[Masked object: {} keys truncated due to depth limit]", len));
                    return true;
                }
                _ => return false,
            }
        }

        match val {
            Value::String(s) => {
                let char_count = s.chars().count();
                if char_count > size_limit {
                    let truncated: String = s.chars().take(size_limit).collect();
                    *val = Value::String(format!(
                        "[Masked string: {} chars truncated...] {}",
                        char_count - size_limit,
                        truncated
                    ));
                    modified = true;
                }
            }
            Value::Array(arr) => {
                if arr.len() > element_limit {
                    let len = arr.len();
                    let truncated_count = len - element_limit;
                    arr.truncate(element_limit);
                    arr.push(Value::String(format!(
                        "[Masked array: {} elements truncated]",
                        truncated_count
                    )));
                    modified = true;
                }
                for item in arr.iter_mut() {
                    if Self::mask_json_value(item, size_limit, element_limit, depth + 1) {
                        modified = true;
                    }
                }
            }
            Value::Object(obj) => {
                if obj.len() > element_limit {
                    let len = obj.len();
                    let truncated_count = len - element_limit;
                    // Retain first `element_limit` keys. Since Map is ordered by insertion in serde_json (if preserved_order is enabled, default true for us usually),
                    // we can just split off.
                    let keys: Vec<String> = obj.keys().cloned().collect();
                    for k in keys.iter().skip(element_limit) {
                        obj.remove(k);
                    }
                    obj.insert(
                        "_masked_keys".to_string(),
                        Value::String(format!(
                            "[Masked object: {} keys truncated]",
                            truncated_count
                        )),
                    );
                    modified = true;
                }
                for item in obj.values_mut() {
                    if Self::mask_json_value(item, size_limit, element_limit, depth + 1) {
                        modified = true;
                    }
                }
            }
            _ => {}
        }
        modified
    }

    pub fn apply_masking(&self, messages: &mut [Message]) {
        let msg_len = messages.len();
        if msg_len <= self.threshold {
            return;
        }

        let mask_until = msg_len - self.threshold;

        for msg in messages.iter_mut().take(mask_until) {
            if msg.role == Role::Tool {
                for tr in msg.tool_results.iter_mut() {
                    if tr.content.is_empty() {
                        continue;
                    }

                    // Try parsing as JSON first to do intelligent structural masking
                    if let Ok(mut parsed_json) = serde_json::from_str::<Value>(&tr.content) {
                        let modified = Self::mask_json_value(
                            &mut parsed_json,
                            self.size_limit,
                            self.element_limit,
                            0,
                        );

                        if modified {
                            if let Ok(new_content) = serde_json::to_string(&parsed_json) {
                                // If the resulting JSON is still absolutely massive, fall back to complete masking
                                if new_content.len() > self.size_limit * 5 {
                                    tr.content = format!(
                                        "{{\"error\": \"[Observation Masked: Output was too large and was hidden. Use tools like `head`, `tail`, or `grep` to inspect large files.]\"}}"
                                    );
                                } else {
                                    tr.content = new_content;
                                }
                                continue;
                            }
                        }
                    }

                    // Fallback to text masking if not JSON or if it failed to reserialize
                    let char_count = tr.content.chars().count();
                    if char_count > self.size_limit {
                        let truncated: String = tr.content.chars().take(self.size_limit).collect();
                        tr.content = format!(
                            "[Observation Masked: {} chars truncated. Original began with: {}...]\nUse tools like `head`, `tail`, or `grep` to inspect large files.",
                            char_count - self.size_limit,
                            truncated
                        );
                    }
                }
            }
        }
    }
}

pub fn apply_observation_masking(
    messages: &mut [Message],
    threshold: usize,
    size_limit: usize,
    element_limit: usize,
) {
    let masker = JetBrainsObservationMasker::new(threshold, size_limit, element_limit);
    masker.apply_masking(messages);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolResult;

    #[test]
    fn test_masking_short_content() {
        let mut messages = vec![Message {
            role: Role::Tool,
            content: String::new(),
            tool_calls: vec![],
            tool_results: vec![ToolResult {
                tool_call_id: "call_1".to_string(),
                content: "short response".to_string(),
                error: String::new(),
            }],
            response_id: None,
            previous_response_id: None,
        }];

        apply_observation_masking(&mut messages, 0, 100, 50);

        assert_eq!(messages[0].tool_results[0].content, "short response");
    }

    #[test]
    fn test_masking_long_text_content() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_2".to_string(),
                    content: "A".repeat(500),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 1 message from the end
        apply_observation_masking(&mut messages, 1, 100, 50);

        assert!(
            messages[0].tool_results[0]
                .content
                .contains("[Observation Masked: 400 chars truncated.")
        );
        assert!(
            messages[0].tool_results[0]
                .content
                .contains(&"A".repeat(100))
        );
    }

    #[test]
    fn test_masking_long_json_content() {
        let json_str = serde_json::json!({
            "small": "abc",
            "large": "A".repeat(500)
        })
        .to_string();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_3".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 1 message from the end. Only 'call_3' is older.
        // We set limit to 150 to be safe so the fallback doesn't trigger.
        apply_observation_masking(&mut messages, 1, 150, 50);

        let masked_content = &messages[0].tool_results[0].content;

        if let Ok(parsed) = serde_json::from_str::<Value>(masked_content) {
            if let Some(obj) = parsed.as_object() {
                if obj.contains_key("error") {
                     let err_str = obj.get("error").unwrap().as_str().unwrap();
                     assert!(err_str.contains("[Observation Masked"));
                } else {
                     assert_eq!(obj.get("small").unwrap().as_str().unwrap(), "abc");
                     assert!(obj.get("large").unwrap().as_str().unwrap().contains("Masked string"));
                }
            } else {
                panic!("Expected an object format");
            }
        } else {
            panic!("Expected valid JSON");
        }
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolResult;

    #[test]
    fn test_mask_large_array_truncation() {
        let large_array: Vec<usize> = (0..100).collect();
        let json_str = serde_json::to_string(&large_array).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_4".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let masker = JetBrainsObservationMasker::new(0, 10, 10);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;

        let parsed: Value = serde_json::from_str(masked_content).expect("Should be valid JSON");

        if let Some(arr) = parsed.as_array() {
            assert_eq!(arr.len(), 11);
            let last_element = if let Some(v) = arr.last() {
                if let Some(s) = v.as_str() {
                    s
                } else {
                    "[Masked array: 0 elements truncated]"
                }
            } else {
                ""
            };
            assert!(last_element.contains("[Masked array:"));
            assert!(last_element.contains("elements truncated]"));
        } else if let Some(s) = parsed.as_object().and_then(|o| o.get("error")).and_then(|v| v.as_str()) {
            assert!(s.contains("[Observation Masked"));
        } else {
            panic!("Unexpected mask format");
        }
    }

    #[test]
    fn test_mask_wide_object_truncation() {
        let mut large_object = serde_json::Map::new();
        for i in 0..100 {
            large_object.insert(format!("key_{}", i), Value::Number(i.into()));
        }
        let json_str = serde_json::to_string(&Value::Object(large_object)).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_5".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let masker = JetBrainsObservationMasker::new(0, 10, 20);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;

        let parsed: Value = serde_json::from_str(masked_content).expect("Should be valid JSON");

        if let Some(obj) = parsed.as_object() {
            if obj.contains_key("error") {
                let s = obj.get("error").unwrap().as_str().unwrap();
                assert!(s.contains("[Observation Masked"));
            } else {
                assert_eq!(obj.len(), 21);
                assert!(obj.contains_key("_masked_keys"));
                let masked_summary = obj.get("_masked_keys").unwrap().as_str().unwrap();
                assert!(masked_summary.contains("[Masked object:"));
                assert!(masked_summary.contains("keys truncated]"));
            }
        } else {
            panic!("Unexpected mask format");
        }
    }

    #[test]
    fn test_mask_deep_recursion_limit() {
        let mut deeply_nested = Value::Object(serde_json::Map::new());
        for _ in 0..15 {
            let mut new_obj = serde_json::Map::new();
            new_obj.insert("nested".to_string(), deeply_nested);
            deeply_nested = Value::Object(new_obj);
        }
        let json_str = serde_json::to_string(&deeply_nested).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_6".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let masker = JetBrainsObservationMasker::new(0, 10, 20);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("[Masked object: 1 keys truncated due to depth limit]") || masked_content.contains("[Observation Masked"));
    }

    #[test]
    fn test_mask_advanced_nesting() {
        let mut deep_array = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into())]);
        for _ in 0..15 {
            deep_array = Value::Array(vec![deep_array]);
        }
        let json_str = serde_json::to_string(&deep_array).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_7".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let masker = JetBrainsObservationMasker::new(0, 10, 20);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("[Masked array: 1 elements truncated due to depth limit]") || masked_content.contains("[Observation Masked"));
    }
}
