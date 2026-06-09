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
    snapshots: HashMap<usize, HashMap<String, Arc<dyn Any + Send + Sync>>>,
    next_snapshot_id: usize,
}

impl RichExecutionEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a snapshot of the current state and returns its ID.
    pub fn snapshot(&mut self) -> usize {
        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;
        self.snapshots.insert(id, self.state.clone());
        id
    }

    /// Rolls back the state to the specified snapshot ID.
    /// Discards a snapshot without rolling back.
    pub fn commit(&mut self, snapshot_id: usize) {
        self.snapshots.remove(&snapshot_id);
    }

    pub fn rollback(&mut self, snapshot_id: usize) -> Result<(), String> {
        if let Some(snapshot) = self.snapshots.remove(&snapshot_id) {
            self.state = snapshot;
            Ok(())
        } else {
            Err(format!("Snapshot ID {} not found", snapshot_id))
        }
    }

    /// Stores a typed variable in the execution environment.
    pub fn set_variable<T: Any + Send + Sync>(&mut self, name: &str, value: T) {
        self.state.insert(name.to_string(), Arc::new(value));
    }

    /// Retrieves a typed variable from the execution environment.
    pub fn get_variable<T: Any + Send + Sync>(&self, name: &str) -> Option<Arc<T>> {
        self.state
            .get(name)
            .and_then(|arc_any| arc_any.clone().downcast::<T>().ok())
    }

    /// Checks if a variable exists.
    pub fn contains_variable(&self, name: &str) -> bool {
        self.state.contains_key(name)
    }

    /// Removes a variable from the environment.
    pub fn remove_variable(&mut self, name: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        self.state.remove(name)
    }

    /// Clears all variables from the execution environment.
    pub fn clear(&mut self) {
        self.state.clear();
    }

    pub fn get_snapshot_count(&self) -> usize {
        self.snapshots.len()
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

impl CodeNativeAdapter {
    pub async fn execute_adapter(
        &self,
        args: serde_json::Value,
    ) -> Result<String, crate::types::ToolError> {
        let mut env_lock = self.env.write().await;
        let snapshot_id = env_lock.snapshot();
        match self.tool.execute_native(&mut env_lock, args).await {
            Ok(result) => {
                env_lock.commit(snapshot_id);
                Ok(result)
            }
            Err(e) => {
                let _ = env_lock.rollback(snapshot_id);
                Err(crate::types::ToolError::Fatal(e))
            }
        }
    }
}

#[async_trait::async_trait]
impl crate::tools::ToolExecutor for CodeNativeAdapter {
    async fn execute(&self, args: serde_json::Value) -> Result<String, crate::types::ToolError> {
        self.execute_adapter(args).await
    }
}

/// An integration utility to run multiple `CodeNativeTool`s in sequence
/// within the same isolated environment. This facilitates integration testing.
pub struct CodeNativePipeline {
    env: Arc<tokio::sync::RwLock<RichExecutionEnvironment>>,
}

impl Default for CodeNativePipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeNativePipeline {
    pub fn new() -> Self {
        Self {
            env: Arc::new(tokio::sync::RwLock::new(RichExecutionEnvironment::new())),
        }
    }

    pub fn with_env(env: Arc<tokio::sync::RwLock<RichExecutionEnvironment>>) -> Self {
        Self { env }
    }

    /// Run a sequence of tools. If any fails, the state is rolled back to the
    /// start of the entire pipeline run, ensuring atomicity of the sequence.
    pub async fn run_sequence(
        &self,
        tools: Vec<(Arc<dyn CodeNativeTool>, serde_json::Value)>,
    ) -> Result<Vec<String>, String> {
        let mut env_lock = self.env.write().await;
        let snapshot_id = env_lock.snapshot();

        let mut results = Vec::new();
        for (tool, args) in tools {
            match tool.execute_native(&mut env_lock, args.clone()).await {
                Ok(res) => results.push(res),
                Err(e) => {
                    // Rollback on first failure
                    let _ = env_lock.rollback(snapshot_id);
                    return Err(format!("Pipeline failed at step {}: {}", results.len(), e));
                }
            }
        }

        // All succeeded, commit
        env_lock.commit(snapshot_id);
        Ok(results)
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
            if let Some(mut data) = env
                .get_variable::<ComplexDataStructure>("my_rich_data")
                .map(|arc| (*arc).clone())
            {
                data.records.push(new_record.to_string());
                let record_count = data.records.len();

                // Update it back in the environment
                env.set_variable("my_rich_data", data);

                Ok(format!(
                    "Processed data natively. New record count: {}",
                    record_count
                ))
            } else {
                Err("Data not found in native environment".to_string())
            }
        }
    }

    struct FailingTool;

    #[async_trait::async_trait]
    impl CodeNativeTool for FailingTool {
        async fn execute_native(
            &self,
            env: &mut RichExecutionEnvironment,
            _args: serde_json::Value,
        ) -> Result<String, String> {
            // Corrupt the state before failing
            env.set_variable(
                "my_rich_data",
                ComplexDataStructure {
                    id: "corrupted".to_string(),
                    records: vec!["corrupted".to_string()],
                    computational_cache: HashMap::new(),
                },
            );
            env.set_variable("new_variable", "should_not_exist".to_string());

            Err("Tool failed to execute".to_string())
        }
    }

    #[tokio::test]
    async fn test_rich_state_preservation_and_rollback() {
        let mut env = RichExecutionEnvironment::new();

        let tool1 = GenerateDataTool;
        let tool2 = ProcessDataTool;

        // Step 1: Generate native complex object
        let res1 = tool1.execute_native(&mut env, json!({})).await.unwrap();
        assert!(res1.starts_with("Generated rich data with ID:"));

        // Step 2: Read and modify native complex object
        let res2 = tool2
            .execute_native(&mut env, json!({"record": "step2_data"}))
            .await
            .unwrap();
        assert_eq!(res2, "Processed data natively. New record count: 2");

        // Verify the rich data is exactly as expected
        let final_data = env
            .get_variable::<ComplexDataStructure>("my_rich_data")
            .unwrap();
        assert_eq!(
            final_data.records,
            vec!["init".to_string(), "step2_data".to_string()]
        );
        assert_eq!(
            *final_data.computational_cache.get("metric_a").unwrap(),
            42.0
        );

        // Step 3: Run failing tool via adapter to test rollback
        let env_arc = Arc::new(tokio::sync::RwLock::new(env));
        let adapter = CodeNativeAdapter {
            env: env_arc.clone(),
            tool: Arc::new(FailingTool),
        };

        let res3 = adapter.execute_adapter(json!({})).await;
        assert!(res3.is_err());

        // Verify the state was rolled back and not corrupted
        let env_after_fail = env_arc.read().await;
        let data_after_fail = env_after_fail
            .get_variable::<ComplexDataStructure>("my_rich_data")
            .unwrap();
        assert_eq!(data_after_fail.id, "test_id".to_string());
        assert_eq!(
            data_after_fail.records,
            vec!["init".to_string(), "step2_data".to_string()]
        );

        // Verify new variables added during failed execution are reverted
        assert!(!env_after_fail.contains_variable("new_variable"));
    }

    #[tokio::test]
    async fn test_rollback_invalid_snapshot() {
        let mut env = RichExecutionEnvironment::new();
        let res = env.rollback(999);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Snapshot ID 999 not found");
    }

    #[test]
    fn test_clear_and_snapshot_count() {
        let mut env = RichExecutionEnvironment::new();
        env.set_variable("var1", "value1".to_string());
        env.set_variable("var2", 42);

        assert_eq!(env.get_snapshot_count(), 0);
        let snap_id = env.snapshot();
        assert_eq!(env.get_snapshot_count(), 1);

        env.clear();
        assert!(!env.contains_variable("var1"));
        assert!(!env.contains_variable("var2"));

        env.rollback(snap_id).unwrap();
        assert!(env.contains_variable("var1"));
        assert!(env.contains_variable("var2"));

        env.remove_variable("var1");
        assert!(!env.contains_variable("var1"));
    }

    #[tokio::test]
    async fn test_code_native_pipeline_integration() {
        let pipeline = CodeNativePipeline::new();

        // 1. Test successful sequence
        let tools: Vec<(Arc<dyn CodeNativeTool>, serde_json::Value)> = vec![
            (Arc::new(GenerateDataTool), json!({})),
            (Arc::new(ProcessDataTool), json!({"record": "integrated"})),
        ];

        let res = pipeline.run_sequence(tools).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 2);

        // 2. Test failing sequence with atomicity
        let tools_fail: Vec<(Arc<dyn CodeNativeTool>, serde_json::Value)> = vec![
            (
                Arc::new(ProcessDataTool),
                json!({"record": "will_fail_soon"}),
            ),
            (Arc::new(FailingTool), json!({})),
        ];

        let res_fail = pipeline.run_sequence(tools_fail).await;
        assert!(res_fail.is_err());
    }
}
