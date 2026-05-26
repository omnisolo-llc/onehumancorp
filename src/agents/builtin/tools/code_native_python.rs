use ohc_builtin_agent_core::code_native::{CodeNativeAdapter, CodeNativeTool, RichExecutionEnvironment};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::Tool;

#[derive(Debug, PartialEq, Clone)]
pub struct PythonReplState {
    pub variables: HashMap<String, String>,
}

impl Default for PythonReplState {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

pub struct NativePythonReplTool;

#[async_trait::async_trait]
impl CodeNativeTool for NativePythonReplTool {
    async fn execute_native(
        &self,
        env: &mut RichExecutionEnvironment,
        args: serde_json::Value,
    ) -> Result<String, String> {
        let code = args.get("code").and_then(|v| v.as_str()).unwrap_or("");

        let mut state = env
            .get_variable::<PythonReplState>("python_repl_state")
            .map(|arc| (*arc).clone())
            .unwrap_or_default();

        let mut output = String::new();

        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("print(") && line.ends_with(")") {
                let var_name = &line[6..line.len() - 1].trim();
                if let Some(val) = state.variables.get(*var_name) {
                    output.push_str(&format!("{}\n", val));
                } else {
                    output.push_str(&format!("NameError: name '{}' is not defined\n", var_name));
                }
            } else if let Some(eq_idx) = line.find('=') {
                let var_name = line[..eq_idx].trim().to_string();
                let var_value = line[eq_idx + 1..].trim().to_string();

                let mut evaluated_val = var_value.clone();
                for (k, v) in &state.variables {
                    if evaluated_val == *k {
                        evaluated_val = v.clone();
                    }
                }

                state.variables.insert(var_name, evaluated_val);
            } else {
                output.push_str(&format!("SyntaxError: unsupported syntax: {}\n", line));
            }
        }

        env.set_variable("python_repl_state", state);

        if output.is_empty() {
            Ok("Code executed successfully (no output)".to_string())
        } else {
            Ok(output.trim().to_string())
        }
    }
}

pub fn native_python_repl_tool(env: Arc<tokio::sync::RwLock<RichExecutionEnvironment>>) -> Tool {
    Tool {
        name: "native_python_repl".to_string(),
        description: "Executes pseudo-Python code. Persists global variables natively across agent steps without JSON serialization. Supported syntax: 'var = value', 'var2 = var1', 'print(var)'.".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "The Python code to execute."
                }
            },
            "required": ["code"]
        }),
        execute: Arc::new(CodeNativeAdapter {
            env,
            tool: Arc::new(NativePythonReplTool),
        }),
    }
}
