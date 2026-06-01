use ohc_builtin_agent_core::types::ToolError;
use serde_json::Value;
use std::sync::Arc;
use tokio::time::Duration;
use crate::Tool;
use ohc_builtin_agent_core::code_native::{CodeNativeTool, RichExecutionEnvironment};

pub struct PythonNativeExecutor {
    pub working_dir: Option<std::path::PathBuf>,
    pub runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl CodeNativeTool for PythonNativeExecutor {
    async fn execute_native(&self, env: &mut RichExecutionEnvironment, args: Value) -> Result<String, String> {
        let code = args["code"]
            .as_str()
            .ok_or_else(|| "python_native: code is required".to_string())?
            .to_string();
        let timeout_secs = args["timeout"].as_f64().unwrap_or(120.0);
        let timeout = Duration::from_secs_f64(timeout_secs.max(1.0).min(600.0));

        let wd_ref = self.working_dir.as_deref();

        let session_id = env.get_variable::<String>("python_session_id")
            .map(|s| (*s).clone())
            .unwrap_or_else(|| {
                let new_id = format!("session_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
                env.set_variable("python_session_id", new_id.clone());
                new_id
            });

        // To preserve state, we write the code to a script that loads previous state (if any) via dill/pickle
        // then runs the code, then saves the new state.

        let state_file = wd_ref.map(|p| p.join(format!("{}.pkl", session_id)))
            .unwrap_or_else(|| std::path::PathBuf::from(format!("/tmp/{}.pkl", session_id)));

        let state_file_str = state_file.to_string_lossy().to_string();

        use base64::{Engine as _, engine::general_purpose::STANDARD};
        let b64_code = STANDARD.encode(code.as_bytes());

        let wrapper_code = format!(r#"
import sys
import pickle
import os
import io
import base64

state_file = "{state_file_str}"

# Load previous state
if os.path.exists(state_file):
    with open(state_file, 'rb') as f:
        try:
            previous_globals = pickle.load(f)
            globals().update(previous_globals)
        except Exception as e:
            pass

# Redirect stdout to capture
old_stdout = sys.stdout
sys.stdout = captured_stdout = io.StringIO()

try:
    decoded_code = base64.b64decode("{b64_code}").decode("utf-8")
    exec(decoded_code, globals())
except Exception as e:
    sys.stdout = old_stdout
    import traceback
    traceback.print_exc()
    sys.exit(1)

sys.stdout = old_stdout

# Save state
try:
    # Filter out modules and builtins to only save user variables
    safe_globals = {{k: v for k, v in globals().items() if not k.startswith('__') and type(v).__name__ != 'module' and k not in ['sys', 'pickle', 'os', 'io', 'base64', 'state_file', 'old_stdout', 'captured_stdout', 'decoded_code']}}
    with open(state_file, 'wb') as f:
        pickle.dump(safe_globals, f)
except Exception as e:
    print(f"Warning: Failed to save state: {{e}}", file=sys.stderr)

print(captured_stdout.getvalue(), end="")
"#);

        let output_res = tokio::time::timeout(timeout, self.runner.run("python3", &["-c", &wrapper_code], wd_ref, vec![])).await;

        let output = output_res
            .map_err(|_| format!("python_native: command timed out after {}s", timeout_secs))?
            .map_err(|e| format!("python_native: failed to execute: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let mut result = String::new();
        if !stdout.is_empty() {
            result.push_str(&format!("Stdout:\n{}\n", stdout));
        }
        if !stderr.is_empty() {
            result.push_str(&format!("Stderr:\n{}\n", stderr));
        }

        if !output.status.success() {
            let exit_code = output.status.code().unwrap_or(-1);
            result.push_str(&format!("Exit code: {}\n", exit_code));
            return Err(result);
        }

        if result.is_empty() {
            result = "Success (no output)".to_string();
        }

        // To demonstrate state preservation: increment a counter in the native env
        let count = env.get_variable::<usize>("python_exec_count").map(|c| *c).unwrap_or(0);
        env.set_variable("python_exec_count", count + 1);

        Ok(result)
    }
}

pub fn python_native_tool(working_dir: Option<std::path::PathBuf>, runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
    let tool_executor = Arc::new(PythonNativeExecutor { working_dir, runner });
    Tool {
        name: "PythonNative".to_string(),
        description: "Execute Python code natively, demonstrating SOTA Code-native execution pattern where state and execution details are tracked."
            .to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "The Python code to execute."
                },
                "timeout": {
                    "type": "number",
                    "description": "Timeout in seconds (default 120, max 600)."
                }
            },
            "required": ["code"]
        }),
        execute: Arc::new(ohc_builtin_agent_core::code_native::CodeNativeAdapter {
            env: Arc::new(tokio::sync::RwLock::new(RichExecutionEnvironment::new())),
            tool: tool_executor,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_python_native_success() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(0, "42\n", "")));

        let mut env = RichExecutionEnvironment::new();
        let executor = PythonNativeExecutor { working_dir: None, runner: runner.clone() };

        let result = executor.execute_native(&mut env, json!({
            "code": "print(42)"
        })).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Stdout:\n42\n\n");
    }

    #[tokio::test]
    async fn test_python_native_failure() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        runner.push_response(Ok(crate::runner::mock::mock_output(1, "", "SyntaxError\n")));

        let mut env = RichExecutionEnvironment::new();
        let executor = PythonNativeExecutor { working_dir: None, runner: runner.clone() };

        let result = executor.execute_native(&mut env, json!({
            "code": "invalid python"
        })).await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("Stderr:\nSyntaxError"));
        assert!(err_msg.contains("Exit code: 1"));
    }
}
