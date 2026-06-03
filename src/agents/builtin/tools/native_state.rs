use ohc_builtin_agent_core::code_native::{CodeNativeTool, RichExecutionEnvironment};

/// NativeMemoryStash tool
/// Allows the agent to natively stash and retrieve complex context or large textual data
/// into the RAM (RichExecutionEnvironment) so it doesn't pollute the JSON LLM context window.
pub struct NativeMemoryStashTool;

#[async_trait::async_trait]
impl CodeNativeTool for NativeMemoryStashTool {
    async fn execute_native(&self, env: &mut RichExecutionEnvironment, args: serde_json::Value) -> Result<String, String> {
        let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("");

        match action {
            "set" => {
                let key = args.get("key").and_then(|v| v.as_str()).unwrap_or("");
                if key.is_empty() {
                    return Err("Missing 'key' for set action".to_string());
                }

                let value = args.get("value").cloned().ok_or("Missing 'value' for set action")?;

                // Store natively as a serde_json::Value (preserving rich data structure)
                env.set_variable(key, value);
                Ok(format!("Successfully stored native memory under key '{}'.", key))
            },
            "get" => {
                let key = args.get("key").and_then(|v| v.as_str()).unwrap_or("");
                if key.is_empty() {
                    return Err("Missing 'key' for get action".to_string());
                }

                if let Some(val) = env.get_variable::<serde_json::Value>(key) {
                    // For getting it back out
                    Ok(val.to_string())
                } else {
                    Err(format!("Key '{}' not found in native memory stash.", key))
                }
            },
            "merge" => {
                let key = args.get("key").and_then(|v| v.as_str()).unwrap_or("");
                if key.is_empty() {
                    return Err("Missing 'key' for merge action".to_string());
                }

                let patch = args.get("value").cloned().ok_or("Missing 'value' (patch) for merge action")?;

                if let Some(arc_val) = env.get_variable::<serde_json::Value>(key) {
                    let mut obj = (*arc_val).clone();
                    if let (Some(target_obj), Some(patch_obj)) = (obj.as_object_mut(), patch.as_object()) {
                        for (k, v) in patch_obj {
                            target_obj.insert(k.clone(), v.clone());
                        }
                        env.set_variable(key, obj);
                        Ok(format!("Successfully merged native memory under key '{}'.", key))
                    } else {
                        Err(format!("Cannot merge key '{}' because it is not an object or patch is not an object.", key))
                    }
                } else {
                    Err(format!("Key '{}' not found in native memory stash.", key))
                }
            },
            "remove" => {
                let key = args.get("key").and_then(|v| v.as_str()).unwrap_or("");
                if key.is_empty() {
                    return Err("Missing 'key' for remove action".to_string());
                }

                if env.remove_variable(key).is_some() {
                    Ok(format!("Successfully removed key '{}' from native memory stash.", key))
                } else {
                    Err(format!("Key '{}' not found in native memory stash.", key))
                }
            },
            _ => Err("Invalid action. Must be 'set', 'get', 'merge', or 'remove'.".to_string()),
        }
    }
}

pub fn native_memory_stash_tool(env: std::sync::Arc<tokio::sync::RwLock<RichExecutionEnvironment>>) -> super::Tool {
    super::Tool {
        name: "NativeMemoryStash".to_string(),
        description: "Stores and retrieves arbitrary large text or JSON natively in RAM, bypassing the LLM context window limits. \
            Use this to stash intermediate results, large file contents, or working memory between steps. \
            Actions: 'set' (requires 'key' and 'value'), 'get' (requires 'key'), 'merge' (requires 'key' and 'value' patch), 'remove' (requires 'key').".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["set", "get", "merge", "remove"],
                    "description": "The action to perform."
                },
                "key": {
                    "type": "string",
                    "description": "The key to store or retrieve data."
                },
                "value": {
                    "description": "The value to store (only required for 'set' and 'merge'). Can be a string or object."
                }
            },
            "required": ["action", "key"]
        }),
        execute: std::sync::Arc::new(ohc_builtin_agent_core::code_native::CodeNativeAdapter {
            env,
            tool: std::sync::Arc::new(NativeMemoryStashTool),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_native_memory_stash() {
        let env = std::sync::Arc::new(tokio::sync::RwLock::new(RichExecutionEnvironment::new()));
        let tool = native_memory_stash_tool(env);

        // Test Set Rich JSON
        let set_args = json!({
            "action": "set",
            "key": "test_key",
            "value": { "nested": "data", "num": 42 }
        });
        let set_res = tool.execute.execute(set_args).await.unwrap();
        assert_eq!(set_res, "Successfully stored native memory under key 'test_key'.");

        // Test Get
        let get_args = json!({
            "action": "get",
            "key": "test_key"
        });
        let get_res = tool.execute.execute(get_args).await.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&get_res).unwrap();
        assert_eq!(parsed["num"], 42);

        // Test Merge
        let merge_args = json!({
            "action": "merge",
            "key": "test_key",
            "value": { "new_field": "hello" }
        });
        let merge_res = tool.execute.execute(merge_args).await.unwrap();
        assert_eq!(merge_res, "Successfully merged native memory under key 'test_key'.");

        let get_args2 = json!({
            "action": "get",
            "key": "test_key"
        });
        let get_res2 = tool.execute.execute(get_args2).await.unwrap();
        let parsed2: serde_json::Value = serde_json::from_str(&get_res2).unwrap();
        assert_eq!(parsed2["new_field"], "hello");
        assert_eq!(parsed2["num"], 42);

        // Test Remove
        let rm_args = json!({
            "action": "remove",
            "key": "test_key"
        });
        let rm_res = tool.execute.execute(rm_args).await.unwrap();
        assert_eq!(rm_res, "Successfully removed key 'test_key' from native memory stash.");
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_native_memory_stash_missing() {
        let env = std::sync::Arc::new(tokio::sync::RwLock::new(RichExecutionEnvironment::new()));
        let tool = native_memory_stash_tool(env);

        // Test Get Missing
        let get_missing_args = json!({
            "action": "get",
            "key": "test_key_missing"
        });
        let get_missing_res = tool.execute.execute(get_missing_args).await;
        assert!(get_missing_res.is_err());
    }
}
