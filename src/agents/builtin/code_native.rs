use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// SOTA Harness Patterns (2025-2026): 2. Code-native execution -> preserving execution state and rich data structures
/// This module implements the `CodeNativeExecution` mechanic. It provides a `RichExecutionEnvironment`
/// that allows tools to store and retrieve strongly-typed Rust objects (`std::any::Any`) across
/// different context windows or steps, replacing the need to serialize everything to JSON strings.

#[derive(Default)]
pub struct RichExecutionEnvironment {
    state: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl RichExecutionEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stores a typed variable in the execution environment.
    pub fn set_variable<T: Any + Send + Sync>(&mut self, name: &str, value: T) {
        self.state.insert(name.to_string(), Arc::new(value));
    }

    /// Retrieves a typed variable from the execution environment.
    pub fn get_variable<T: Any + Send + Sync>(&self, name: &str) -> Option<Arc<T>> {
        self.state.get(name).and_then(|arc_any| arc_any.clone().downcast::<T>().ok())
    }

    /// Checks if a variable exists.
    pub fn contains_variable(&self, name: &str) -> bool {
        self.state.contains_key(name)
    }

    /// Removes a variable from the environment.
    pub fn remove_variable(&mut self, name: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        self.state.remove(name)
    }
}

/// A trait for tools that can operate natively on the `RichExecutionEnvironment`.
#[async_trait::async_trait]
pub trait CodeNativeTool: Send + Sync {
    /// Executes the tool natively, potentially modifying the rich execution environment.
    async fn execute_native(
        &self,
        env: &mut RichExecutionEnvironment,
        args: serde_json::Value,
    ) -> Result<String, String>;
}

/// An adapter that allows a `CodeNativeTool` to be used as a standard `ToolExecutor`
pub struct CodeNativeAdapter {
    pub env: Arc<tokio::sync::RwLock<RichExecutionEnvironment>>,
    pub tool: Arc<dyn CodeNativeTool>,
}

// Since ToolExecutor is defined in `ohc_builtin_agent_tools` which depends on `core`,
// we will implement the adapter within `ohc_builtin_agent_tools` (or wherever ToolExecutor is available).
// Wait, `ohc_builtin_agent_tools` depends on `core`. So we can't implement `ToolExecutor` here if it's not in `core`.
// We will just provide the struct here and implement `ToolExecutor` where it's defined, or we can just implement the execute logic here.
impl CodeNativeAdapter {
    pub async fn execute_adapter(&self, args: serde_json::Value) -> Result<String, crate::types::ToolError> {
        let mut env_lock = self.env.write().await;
        self.tool.execute_native(&mut env_lock, args).await.map_err(|e| crate::types::ToolError::Fatal(e))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // A rich, complex data structure that we want to preserve across steps
    #[derive(Debug, PartialEq, Clone)]
    struct ComplexDataStructure {
        id: String,
        records: Vec<String>,
        computational_cache: HashMap<String, f64>,
    }

    impl ComplexDataStructure {
        fn new() -> Self {
            let mut cache = HashMap::new();
            cache.insert("metric_a".to_string(), 42.0);
            Self {
                id: "test_id".to_string(),
                records: vec!["init".to_string()],
                computational_cache: cache,
            }
        }
    }

    struct GenerateDataTool;

    #[async_trait::async_trait]
    impl CodeNativeTool for GenerateDataTool {
        async fn execute_native(
            &self,
            env: &mut RichExecutionEnvironment,
            _args: serde_json::Value,
        ) -> Result<String, String> {
            let data = ComplexDataStructure::new();
            let id = data.id.clone();
            env.set_variable("my_rich_data", data);
            Ok(format!("Generated rich data with ID: {}", id))
        }
    }

    struct ProcessDataTool;

    #[async_trait::async_trait]
    impl CodeNativeTool for ProcessDataTool {
        async fn execute_native(
            &self,
            env: &mut RichExecutionEnvironment,
            args: serde_json::Value,
        ) -> Result<String, String> {
            let new_record = args["record"].as_str().unwrap_or("default");

            // Retrieve the native object without JSON parsing/serialization
            if let Some(mut data) = env.get_variable::<ComplexDataStructure>("my_rich_data").map(|arc| (*arc).clone()) {
                data.records.push(new_record.to_string());
                let record_count = data.records.len();

                // Update it back in the environment
                env.set_variable("my_rich_data", data);

                Ok(format!("Processed data natively. New record count: {}", record_count))
            } else {
                Err("Data not found in native environment".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_rich_state_preservation() {
        let mut env = RichExecutionEnvironment::new();

        let tool1 = GenerateDataTool;
        let tool2 = ProcessDataTool;

        // Step 1: Generate native complex object
        let res1 = tool1.execute_native(&mut env, json!({})).await.unwrap();
        assert!(res1.starts_with("Generated rich data with ID:"));

        // Step 2: Read and modify native complex object
        let res2 = tool2.execute_native(&mut env, json!({"record": "step2_data"})).await.unwrap();
        assert_eq!(res2, "Processed data natively. New record count: 2");

        // Verify the rich data is exactly as expected
        let final_data = env.get_variable::<ComplexDataStructure>("my_rich_data").unwrap();
        assert_eq!(final_data.records, vec!["init".to_string(), "step2_data".to_string()]);
        assert_eq!(*final_data.computational_cache.get("metric_a").unwrap(), 42.0);
    }
}
