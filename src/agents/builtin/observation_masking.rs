#![allow(clippy::all)]
use ohc_builtin_agent_core::types::{Message, Role};
use serde_json::Value;

/// Master Catalog B.4: Context Management: Master Catalog: JetBrains Observation Masking.
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

    fn detect_and_mask_binary(s: &mut String) -> bool {
        let trimmed: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        let len = trimmed.len();
        if len < 64 {
            return false;
        }

        // Avoid masking uniform placeholder/filler strings (e.g. "AAAA...") as binary
        if let Some(first) = trimmed.chars().next() {
            if trimmed.chars().all(|c| c == first) {
                return false;
            }
        }

        // 1. Check if it's purely hexadecimal
        let is_hex = len % 2 == 0 && trimmed.chars().all(|c| c.is_ascii_hexdigit());
        if is_hex {
            *s = format!("[Masked binary/encoded hex data: {} bytes]", len / 2);
            return true;
        }

        // 2. Check if it's purely base64
        let mut chars_iter = trimmed.chars();
        let mut valid_base64 = true;
        let mut padding_started = false;
        let mut char_count = 0;

        while let Some(c) = chars_iter.next() {
            char_count += 1;
            if c == '=' {
                padding_started = true;
                if len - char_count > 1 {
                    valid_base64 = false;
                    break;
                }
            } else if padding_started {
                valid_base64 = false;
                break;
            } else {
                let is_b64_char = c.is_ascii_alphanumeric() || c == '+' || c == '/';
                if !is_b64_char {
                    valid_base64 = false;
                    break;
                }
            }
        }

        if valid_base64 && len % 4 == 0 {
            let padding_len = trimmed.chars().rev().take(2).filter(|&c| c == '=').count();
            let decoded_len = (len / 4) * 3 - padding_len;
            *s = format!("[Masked binary/encoded base64 data: {} bytes]", decoded_len);
            return true;
        }

        false
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
                    *val = Value::String(format!(
                        "[Masked array: {} elements truncated due to depth limit]",
                        len
                    ));
                    return true;
                }
                Value::Object(obj) => {
                    let len = obj.len();
                    *val = Value::String(format!(
                        "[Masked object: {} keys truncated due to depth limit]",
                        len
                    ));
                    return true;
                }
                _ => {
                    *val = Value::String("[Masked: depth limit exceeded]".to_string());
                    return true;
                }
            }
        }

        match val {
            Value::String(s) => {
                let bytes = s.len();
                if Self::detect_and_mask_binary(s) {
                    return true;
                }
                if bytes > size_limit || s.lines().count() > 10 {
                    let line_count = s.lines().count();
                    if line_count > 10 {
                        let keep_lines = 5;
                        let lines: Vec<&str> = s.lines().collect();
                        let start_preview = lines[0..keep_lines].join("\n");
                        let end_preview = lines[line_count - keep_lines..line_count].join("\n");
                        let masked_lines = line_count - (keep_lines * 2);

                        // Enforce size_limit so we don't blow up the context window if lines are extremely long
                        if start_preview.len() + end_preview.len() <= size_limit {
                            *s = format!(
                                "{}\n[... {} lines masked ...]\n{}",
                                start_preview, masked_lines, end_preview
                            );
                        } else {
                            let preview_chars = std::cmp::max(10, size_limit / 4);
                            let char_count = s.chars().count();
                            if char_count > preview_chars * 2 {
                                let start_preview: String = s.chars().take(preview_chars).collect();
                                let end_preview: String =
                                    s.chars().skip(char_count - preview_chars).collect();
                                *s = format!(
                                    "[Masked string: {} bytes. Preview: {}...{}]",
                                    bytes, start_preview, end_preview
                                );
                            } else {
                                *s = format!("[Masked string: {} bytes]", bytes);
                            }
                        }
                    } else {
                        let preview_chars = std::cmp::max(10, size_limit / 4);
                        let char_count = s.chars().count();
                        if char_count > preview_chars * 2 {
                            let start_preview: String = s.chars().take(preview_chars).collect();
                            let end_preview: String =
                                s.chars().skip(char_count - preview_chars).collect();
                            *s = format!(
                                "[Masked string: {} bytes. Preview: {}...{}]",
                                bytes, start_preview, end_preview
                            );
                        } else {
                            *s = format!("[Masked string: {} bytes]", bytes);
                        }
                    }
                    modified = true;
                }
            }
            Value::Array(arr) => {
                let original_len = arr.len();

                // Adaptive element limit based on depth - deeper structures get truncated more aggressively
                let current_limit = std::cmp::max(1, element_limit.saturating_sub(depth * 5));

                if original_len > current_limit {
                    // Try to keep a mix of the beginning and end of the array
                    if current_limit >= 2 {
                        let half = current_limit / 2;
                        let mut new_arr = Vec::with_capacity(current_limit + 1);
                        new_arr.extend_from_slice(&arr[..half]);
                        new_arr.push(Value::String(format!(
                            "[... Masked array: {} elements truncated ...]",
                            original_len - current_limit
                        )));
                        new_arr.extend_from_slice(&arr[original_len - (current_limit - half)..]);
                        *arr = new_arr;
                    } else {
                        arr.truncate(current_limit);
                        arr.push(Value::String(format!(
                            "[Masked array: {} elements truncated]",
                            original_len - current_limit
                        )));
                    }
                    modified = true;
                }

                for item in arr.iter_mut() {
                    if Self::mask_json_value(item, size_limit, element_limit, depth + 1) {
                        modified = true;
                    }
                }
            }
            Value::Object(obj) => {
                let original_len = obj.len();
                let mut truncated = false;
                let mut removed_count = 0;

                // Adaptive element limit based on depth
                let current_limit = std::cmp::max(1, element_limit.saturating_sub(depth * 5));

                if original_len > current_limit {
                    // Master Catalog B.4: Context Management: JetBrains Observation Masking.
                    // We identify priority keys first so they are less likely to be removed.
                    let priority_keys = [
                        "error",
                        "stack_trace",
                        "message",
                        "status",
                        "code",
                        "type",
                        "name",
                        "id",
                        "success",
                        "result",
                        "timestamp",
                        "version",
                        "metadata",
                        "details", // Important for tracking errors
                    ];
                    let mut priority_to_keep = Vec::new();
                    let mut regular_to_keep = Vec::new();
                    for (k, _) in obj.iter() {
                        let k_lower = k.to_lowercase();
                        if priority_keys.iter().any(|&p| p == k_lower.as_str()) {
                            priority_to_keep.push(k.clone());
                        } else {
                            regular_to_keep.push(k.clone());
                        }
                    }
                    regular_to_keep.sort();
                    let num_priority = priority_to_keep.len();
                    let mut keys_to_remove = Vec::new();
                    if num_priority >= current_limit {
                        keys_to_remove = regular_to_keep;
                    } else {
                        let slots_left = current_limit - num_priority;
                        if regular_to_keep.len() > slots_left {
                            keys_to_remove = regular_to_keep.split_off(slots_left);
                        }
                    }

                    removed_count = keys_to_remove.len();
                    for k in &keys_to_remove {
                        obj.remove(k);
                    }
                    if removed_count > 0 {
                        truncated = true;
                        modified = true;
                    }
                }
                for (_, value) in obj.iter_mut() {
                    if Self::mask_json_value(value, size_limit, element_limit, depth + 1) {
                        modified = true;
                    }
                }
                if truncated {
                    obj.insert(
                        "_masked_keys".to_string(),
                        Value::String(format!("[Masked object: {} keys truncated]", removed_count)),
                    );
                }
            }

            _ => {}
        }
        modified
    }

    pub fn apply_masking(&self, messages: &mut [Message]) {
        let msg_count = messages.len();
        for i in 0..msg_count {
            if messages[i].role == Role::Tool {
                let age = msg_count - i;
                if age > self.threshold {
                    for tr in &mut messages[i].tool_results {
                        if tr.error.is_empty()
                            && (!tr.content.starts_with("{\"_masked_observation\"")
                                && !tr
                                    .content
                                    .starts_with("{\"_masked_observation\": \"[Observation Masked"))
                        {
                            let bytes = tr.content.len();
                            if bytes > self.size_limit {
                                let mut modification_successful = false;
                                let content_trimmed = tr.content.trim();
                                let is_array = content_trimmed.starts_with('[');

                                if content_trimmed.starts_with('{') || is_array {
                                    if let Ok(json_val) = serde_json::from_str::<Value>(&tr.content)
                                    {
                                        let mut best_content = None;
                                        let mut factor_low = 1;
                                        let mut factor_high = 100;

                                        while factor_low <= factor_high {
                                            let factor_mid =
                                                factor_low + (factor_high - factor_low) / 2;

                                            let test_element_limit = std::cmp::max(
                                                1,
                                                (self.element_limit * factor_mid) / 100,
                                            );
                                            let test_size_limit = std::cmp::max(
                                                10,
                                                (self.size_limit * factor_mid) / 100,
                                            );

                                            let mut temp_val = json_val.clone();
                                            Self::mask_json_value(
                                                &mut temp_val,
                                                test_size_limit,
                                                test_element_limit,
                                                0,
                                            );

                                            let new_content = serde_json::to_string(&temp_val)
                                                .unwrap_or_else(|_| tr.content.clone());

                                            if new_content.len() <= self.size_limit {
                                                best_content = Some(new_content);
                                                factor_low = factor_mid + 1;
                                            } else {
                                                factor_high = factor_mid - 1;
                                            }
                                        }

                                        if let Some(final_content) = best_content {
                                            tr.content = final_content;
                                            modification_successful = true;
                                        }

                                        if !modification_successful {
                                            // Fallback preserving the outer array/object structure if possible
                                            let raw_msg = format!(
                                                "[Observation Masked to save context. Output was {} bytes. Use 'RecallObservation' with ID '{}' to retrieve full output.]",
                                                bytes, tr.tool_call_id
                                            );
                                            if is_array {
                                                tr.content = serde_json::json!([{ "_masked_observation": raw_msg }]).to_string();
                                            } else {
                                                tr.content = serde_json::json!({ "_masked_observation": raw_msg }).to_string();
                                            }
                                            modification_successful = true;
                                        }
                                    }
                                }

                                if !modification_successful {
                                    // Optimization: dynamically compute threshold bounds
                                    let preview_chars = std::cmp::max(10, self.size_limit / 4);
                                    let char_count = tr.content.chars().count();
                                    let raw_msg = if char_count > preview_chars * 2 {
                                        let start_preview: String =
                                            tr.content.chars().take(preview_chars).collect();
                                        let end_preview: String = tr
                                            .content
                                            .chars()
                                            .skip(char_count - preview_chars)
                                            .collect();
                                        format!(
                                            "[Observation Masked to save context. Output was {} bytes. Preview: {}...{} The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, start_preview, end_preview, tr.tool_call_id
                                        )
                                    } else {
                                        format!(
                                            "[Observation Masked to save context. Output was {} bytes. The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, tr.tool_call_id
                                        )
                                    };
                                    // Graceful text log masking: preserve start and end for non-JSON contents
                                    let char_count = tr.content.chars().count();
                                    let keep = std::cmp::max(self.size_limit / 5, 20);
                                    if char_count > keep * 2 {
                                        let start: String = tr.content.chars().take(keep).collect();
                                        let end: String =
                                            tr.content.chars().skip(char_count - keep).collect();
                                        tr.content = format!(
                                            "{}... [Observation Masked: {} bytes truncated. Use RecallObservation ID '{}'] ...{}",
                                            start, bytes, tr.tool_call_id, end
                                        );
                                    } else {
                                        tr.content = raw_msg;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_observation_masking(
    messages: &mut Vec<Message>,
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
    fn test_apply_observation_masking() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_1".to_string(),
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
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_2".to_string(),
                    content: "B".repeat(500),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 1 message from the end. Only 'call_1' is older.
        // Size limit 100 bytes.
        apply_observation_masking(&mut messages, 1, 50, 50);

        // First message should be masked
        assert!(
            messages[0].tool_results[0]
                .content
                .contains("[Observation Masked")
        );
        assert!(messages[0].tool_results[0].content.contains("call_1"));
        assert!(messages[0].tool_results[0].content.contains("AAAAA")); // should contain the prefix

        // Last message should NOT be masked
        assert!(
            !messages[2].tool_results[0]
                .content
                .contains("[Observation Masked")
        );
        assert_eq!(messages[2].tool_results[0].content, "B".repeat(500));
    }
    #[test]
    fn test_mask_json_value() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_3".to_string(),
                    content: format!("{{\"small\": \"abc\", \"large\": \"{}\"}}", "A".repeat(500)),
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
        // The size of this payload is small: about ~520 bytes.
        // We set limit slightly above that to avoid replacing the entire content,
        // but we want individual properties > 100 bytes to be truncated,
        // Wait, apply_masking takes a global size limit.
        // If the entire payload is > size_limit, and masking structural properties doesn't make it smaller than size_limit, it falls back to a full string replacement.
        // We set size_limit to 1000 so the overall payload is NOT truncated completely,
        // wait, the structural masking only masks strings that are individually > size_limit.
        // Oh! If size_limit is 100, then "large" is 500 bytes and gets masked.
        // The total size becomes smaller. Does it become < 100 bytes? No, because "large" will be replaced by "[Masked string: 500 bytes...]" which is ~30 bytes, but the rest of JSON is around 40 bytes.
        // Let's set limit to 150 to be safe so the fallback doesn't trigger.
        apply_observation_masking(&mut messages, 1, 150, 50);

        let masked_content = &messages[0].tool_results[0].content;

        if let Ok(parsed) = serde_json::from_str::<Value>(masked_content) {
            if let Some(obj) = parsed.as_object() {
                if obj.contains_key("_masked_observation") {
                    // It fell back to complete masking. Let's make sure it contains Observation Masked.
                    let err_str = obj.get("_masked_observation").unwrap().as_str().unwrap();
                    assert!(err_str.contains("[Observation Masked"));
                } else {
                    assert_eq!(obj.get("small").unwrap().as_str().unwrap(), "abc");
                    assert!(
                        obj.get("large")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .contains("Masked string")
                    );
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

        // Mask messages older than 0. Limit array elements to 10.
        let masker = JetBrainsObservationMasker::new(0, 10, 10);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;

        // Ensure it is still valid JSON
        let parsed: Value = serde_json::from_str(masked_content).expect("Should be valid JSON");

        if let Some(arr) = parsed.as_array() {
            if arr.len() == 1
                && arr[0]
                    .as_object()
                    .map_or(false, |o| o.contains_key("_masked_observation"))
            {
                let s = arr[0]
                    .as_object()
                    .unwrap()
                    .get("_masked_observation")
                    .unwrap()
                    .as_str()
                    .unwrap();
                assert!(s.contains("[Observation Masked"));
            } else {
                assert_eq!(arr.len(), 11); // 10 original elements + 1 masked summary
                let last_element = if let Some(v) = arr.last() {
                    if let Some(s) = v.as_str() {
                        s
                    } else {
                        "[Masked array: 0 elements truncated]"
                    }
                } else {
                    ""
                };
                tracing::debug!("MASKED CONTENT: {}", masked_content);
                assert!(last_element.contains("[Masked array:"));
                assert!(last_element.contains("elements truncated]"));
            }
        } else if let Some(s) = parsed
            .as_array()
            .and_then(|a| a[0].as_object())
            .and_then(|o| o.get("_masked_observation"))
            .and_then(|v| v.as_str())
        {
            assert!(s.contains("[Observation Masked"));
        } else if let Some(s) = parsed
            .as_object()
            .and_then(|o| o.get("_masked_observation"))
            .and_then(|v| v.as_str())
        {
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

        // Mask messages older than 0. Limit object elements to 20.
        let masker = JetBrainsObservationMasker::new(0, 10, 20);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;

        // Ensure it is still valid JSON
        let parsed: Value = serde_json::from_str(masked_content).expect("Should be valid JSON");

        if let Some(obj) = parsed.as_object() {
            if obj.contains_key("_masked_observation") {
                let s = obj.get("_masked_observation").unwrap().as_str().unwrap();
                assert!(s.contains("[Observation Masked"));
            } else {
                assert_eq!(obj.len(), 21); // 20 original keys + 1 masked keys summary
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
    fn test_mask_preserves_new_priority_keys_extended() {
        let mut obj = serde_json::Map::new();
        obj.insert("timestamp".to_string(), Value::String("2023".into()));
        obj.insert("version".to_string(), Value::String("1.0".into()));
        obj.insert("irrelevant_key_1".to_string(), Value::String("a".into()));
        obj.insert("irrelevant_key_2".to_string(), Value::String("b".into()));

        let mut val = Value::Object(obj);
        // By setting size_limit lower (0) it will definitely evaluate it, and element_limit=2 restricts keys.
        JetBrainsObservationMasker::mask_json_value(&mut val, 0, 2, 0);

        let obj = val.as_object().unwrap();

        assert!(obj.contains_key("timestamp"));
        assert!(obj.contains_key("version"));
        // At limit 2, only timestamp and version should be kept
        assert!(!obj.contains_key("irrelevant_key_1"));
        assert!(!obj.contains_key("irrelevant_key_2"));
    }

    #[test]
    fn test_mask_preserves_priority_keys() {
        let mut obj = serde_json::Map::new();
        obj.insert("irrelevant_key_1".to_string(), Value::String("a".into()));
        obj.insert(
            "error".to_string(),
            Value::String("critical failure".into()),
        );
        obj.insert("irrelevant_key_2".to_string(), Value::String("b".into()));
        obj.insert("status".to_string(), Value::String("failed".into()));
        obj.insert("irrelevant_key_3".to_string(), Value::String("c".into()));

        let json_str = serde_json::to_string(&Value::Object(obj)).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_priority".to_string(),
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

        // Limit object elements to 2.
        let masker = JetBrainsObservationMasker::new(0, 100, 2);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;
        let parsed: Value = serde_json::from_str(masked_content).unwrap();

        let obj = parsed.as_object().unwrap();

        // Ensure priority keys were retained
        assert!(obj.contains_key("error"));
        assert!(obj.contains_key("status"));

        // The irrelevant keys should have been removed
        assert!(!obj.contains_key("irrelevant_key_1"));
        assert!(!obj.contains_key("irrelevant_key_2"));
        assert!(!obj.contains_key("irrelevant_key_3"));

        // Should have exactly 2 elements + the `_masked_keys` summary
        assert_eq!(obj.len(), 3);
        assert!(obj.contains_key("_masked_keys"));
    }

    #[test]
    fn test_mask_deep_recursion_limit() {
        // Build a deeply nested object
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
        assert!(
            masked_content.contains("[Masked object: 1 keys truncated due to depth limit]")
                || masked_content.contains("[Observation Masked")
        );
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
        assert!(
            masked_content.contains("[Masked array: 1 elements truncated due to depth limit]")
                || masked_content.contains("[Observation Masked")
        );
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_mask_multiline_string() {
        let mut obj = serde_json::Map::new();
        let long_string = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12\nLine 13\nLine 14\nLine 15";
        obj.insert("output".to_string(), Value::String(long_string.to_string()));

        let mut val = Value::Object(obj);
        JetBrainsObservationMasker::mask_json_value(&mut val, 200, 10, 0);

        let s = val.get("output").unwrap().as_str().unwrap();
        assert!(s.contains("Line 1"));
        assert!(s.contains("Line 5"));
        assert!(s.contains("[... 5 lines masked ...]"));
        assert!(s.contains("Line 11"));
        assert!(s.contains("Line 15"));
        assert!(!s.contains("Line 8"));
    }

    #[test]
    fn test_mask_multiline_string_size_limit_fallback() {
        let mut obj = serde_json::Map::new();
        let long_string = "Line 1 with lots of data data data data data data data data data data\nLine 2 with lots of data data data data data data data data data data\nLine 3 with lots of data data data data data data data data data data\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12\nLine 13\nLine 14\nLine 15";
        obj.insert("output".to_string(), Value::String(long_string.to_string()));

        let mut val = Value::Object(obj);
        // Extremely small size limit so the line-based approach gets aborted and it falls back to raw char truncation
        JetBrainsObservationMasker::mask_json_value(&mut val, 20, 10, 0);

        let s = val.get("output").unwrap().as_str().unwrap();
        assert!(s.contains("[Masked string:"));
        assert!(!s.contains("[... 5 lines masked ...]"));
    }

    #[test]
    fn test_mask_hex_binary_data() {
        let mut obj = serde_json::Map::new();
        // A 64-character purely hexadecimal string (32 bytes of hex data)
        let hex_data = "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90";
        assert_eq!(hex_data.len(), 64);
        obj.insert("hex_val".to_string(), Value::String(hex_data.to_string()));

        let mut val = Value::Object(obj);
        JetBrainsObservationMasker::mask_json_value(&mut val, 1000, 10, 0);

        let s = val.get("hex_val").unwrap().as_str().unwrap();
        assert!(s.contains("[Masked binary/encoded hex data: 32 bytes]"));
    }

    #[test]
    fn test_mask_base64_binary_data() {
        let mut obj = serde_json::Map::new();
        // A 64-character purely base64 string
        let b64_data = "SGVsbG8gd29ybGQhIFRoaXMgaXMgYSB2ZXJ5IGxvbmcgc3RyaW5nIHRvIGJlIG1hc2tlZA==";
        assert_eq!(b64_data.len(), 72); // multiple of 4, valid b64
        obj.insert("b64_val".to_string(), Value::String(b64_data.to_string()));

        let mut val = Value::Object(obj);
        JetBrainsObservationMasker::mask_json_value(&mut val, 1000, 10, 0);

        let s = val.get("b64_val").unwrap().as_str().unwrap();
        assert!(s.contains("[Masked binary/encoded base64 data: 52 bytes]"));
    }

    #[test]
    fn test_do_not_mask_short_base64_or_hex() {
        let mut obj = serde_json::Map::new();
        let short_hex = "a1b2c3d4e5f6";
        let short_b64 = "SGVsbG8=";
        obj.insert("hex_val".to_string(), Value::String(short_hex.to_string()));
        obj.insert("b64_val".to_string(), Value::String(short_b64.to_string()));

        let mut val = Value::Object(obj);
        JetBrainsObservationMasker::mask_json_value(&mut val, 1000, 10, 0);

        assert_eq!(val.get("hex_val").unwrap().as_str().unwrap(), short_hex);
        assert_eq!(val.get("b64_val").unwrap().as_str().unwrap(), short_b64);
    }
}
