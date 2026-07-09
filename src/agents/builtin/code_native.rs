use std::any::Any;
use tracing::{info, error, debug};
use std::collections::HashMap;
use std::sync::Arc;
/// SOTA Harness Patterns (2025-2026): 2. Code-native execution -> preserving execution state and rich data structures
/// This module implements the `CodeNativeExecution` mechanic. It provides a `RichExecutionEnvironment`
/// that allows tools to store and retrieve strongly-typed Rust objects (`std::any::Any`) across
/// different context windows or steps, replacing the need to serialize...erything to JSON strings.
pub struct SnapshotNode {
    pub state: HashMap<String, Arc<dyn Any + Send + Sync>>,
    pub parent_id: Option<usize>,
}
#[derive(Default)]
pub struct RichExecutionEnvironment {
    state: HashMap<String, Arc<dyn Any + Send + Sync>>,
    snapshots: HashMap<usize, SnapshotNode>,
    next_snapshot_id: usize,
    current_snapshot_id: Option<usize>,
}
impl RichExecutionEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a snapshot of the current state and returns its ID.
    pub fn snapshot(&mut self) -> usize {
        let id = self.next_snapshot_id;
        self.next_snapshot_id += 1;
        self.snapshots.insert(id, SnapshotNode {
            state: self.state.clone(),
            parent_id: self.current_snapshot_id,
        });
        self.current_snapshot_id = Some(id);
        id
    }
    /// Discards a snapshot without rolling back.
    pub fn commit(&mut self, snapshot_id: usize) {
        self.snapshots.remove(&snapshot_id);
        if self.current_snapshot_id == Some(snapshot_id) {
            self.current_snapshot_id = None;
        }
    }
    /// Rolls back the state to the specified snapshot ID.
    pub fn rollback(&mut self, snapshot_id: usize) -> Result<(), String> {
        if let Some(snapshot) = self.snapshots.remove(&snapshot_id) {
            self.state = snapshot.state;
            self.current_snapshot_id = snapshot.parent_id;
            Ok(())
        } else {
            Err(format!("Snapshot ID {} not found", snapshot_id))
        }
    }
    /// Rolls back the state to the specified ancestor snapshot ID without removing intermediate snapshots.
    pub fn rollback_to_ancestor(&mut self, snapshot_id: usize) -> Result<(), String> {
        if let Some(snapshot) = self.snapshots.get(&snapshot_id) {
            self.state = snapshot.state.clone();
            self.current_snapshot_id = Some(snapshot_id);
            Ok(())
        } else {
            Err(format!("Ancestor Snapshot ID {} not found", snapshot_id))
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
        info!("Starting CodeNativePipeline run_sequence with {} tools", tools.len());
        let start_time = std::time::Instant::now();
        let mut env_lock = self.env.write().await;
        let snapshot_id = env_lock.snapshot();
        debug!("Created atomic snapshot {} for pipeline", snapshot_id);
        let mut results = Vec::new();
        for (i, (tool, args)) in tools.into_iter().enumerate() {
            match tool.execute_native(&mut env_lock, args.clone()).await {
                Ok(res) => {
                    debug!("Tool {} succeeded", i);
                    results.push(res);
                },
                Err(e) => {
                    error!("Tool {} failed with error: {}. Rolling back to snapshot {}", i, e, snapshot_id);
                    let _ = env_lock.rollback(snapshot_id);
                    return Err(format!("Pipeline failed at step {}: {}", results.len(), e));
                }
            }
        }
        env_lock.commit(snapshot_id);
        info!("CodeNativePipeline run_sequence completed successfully in {:?}", start_time.elapsed());
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
    #[test]
    fn test_branching_snapshot_and_rollback_to_ancestor() {
        let mut env = RichExecutionEnvironment::new();
        // Ancestor State
        env.set_variable("root", "initial".to_string());
        let root_snap_id = env.snapshot();
        // Branch A
        env.set_variable("branch_a", "data_a".to_string());
        let branch_a_snap_id = env.snapshot();
        // Check we are in Branch A
        assert!(env.contains_variable("root"));
        assert!(env.contains_variable("branch_a"));
        // Time travel back to Ancestor (without removing it like rollback does)
        env.rollback_to_ancestor(root_snap_id).unwrap();
        // Ensure we are back
        assert!(env.contains_variable("root"));
        assert!(!env.contains_variable("branch_a"));
        // Branch B
        env.set_variable("branch_b", "data_b".to_string());
        let branch_b_snap_id = env.snapshot();
        // Ensure Branch B state
        assert!(env.contains_variable("root"));
        assert!(!env.contains_variable("branch_a"));
        assert!(env.contains_variable("branch_b"));
        // Mutate state after snapshot B
        env.set_variable("branch_b_post_snap", "data".to_string());
        assert!(env.contains_variable("branch_b_post_snap"));
        // Rollback branch B completely, restoring it to exactly `branch_b_snap_id`
        env.rollback(branch_b_snap_id).unwrap();
        assert!(env.contains_variable("root"));
        assert!(env.contains_variable("branch_b"));
        assert!(!env.contains_variable("branch_b_post_snap"));
        // Rollback root completely
        env.rollback(root_snap_id).unwrap();
        assert!(env.contains_variable("root")); // root_snap_id had root
        assert!(!env.contains_variable("branch_b")); // branch_b was not in root_snap_id
        // Use the branch_a_snap_id to avoid unused variable warning and ensure it's still accessible
        env.rollback_to_ancestor(branch_a_snap_id).unwrap();
        assert!(env.contains_variable("branch_a"));
        assert_eq!(env.get_snapshot_count(), 1); // Branch A snapshot is still orphaned in the tree if we didn't remove it
    }
    #[test]
    fn test_commit_parent_and_rollback_child() {
        let mut env = RichExecutionEnvironment::new();
        env.set_variable("root", "1".to_string());
        let root_id = env.snapshot();
        env.set_variable("child", "2".to_string());
        let child_id = env.snapshot();
        // Commit root, removing it from snapshots but leaving child
        env.commit(root_id);
        assert_eq!(env.get_snapshot_count(), 1);
        // Rollback child. It should restore child state, and parent pointer drops.
        env.rollback(child_id).unwrap();
        assert!(env.contains_variable("child"));
        assert!(env.contains_variable("root"));
        assert_eq!(env.current_snapshot_id, Some(root_id)); // Even if the snapshot object is gone from map, the ID pointer is retained
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
