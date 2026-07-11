use serde::{Deserialize, Serialize};
/// SOTA Harness Patterns (2025-2026): 2. Code-native execution -> preserving execution state and rich data structures
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepStatus {
    Pending,
    Running,
    Completed(String),
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    InProgress,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub workflow_id: String,
    pub steps: HashMap<String, StepStatus>,
    pub context: HashMap<String, String>,
}

impl WorkflowState {
    pub fn new(workflow_id: &str) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
            steps: HashMap::new(),
            context: HashMap::new(),
        }
    }

    pub fn set_step_status(&mut self, step_id: &str, status: StepStatus) {
        self.steps.insert(step_id.to_string(), status);
    }

    pub fn get_step_status(&self, step_id: &str) -> Option<&StepStatus> {
        self.steps.get(step_id)
    }
}

pub struct DurableExecutionEngine {
    state_store: Arc<Mutex<HashMap<String, WorkflowState>>>,
}

impl Default for DurableExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DurableExecutionEngine {
    pub fn new() -> Self {
        Self {
            state_store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn start_or_resume_workflow(&self, workflow_id: &str) -> WorkflowState {
        let mut store = self.state_store.lock().await;
        if let Some(state) = store.get(workflow_id) {
            state.clone()
        } else {
            let new_state = WorkflowState::new(workflow_id);
            store.insert(workflow_id.to_string(), new_state.clone());
            new_state
        }
    }

    pub async fn update_step(
        &self,
        workflow_id: &str,
        step_id: &str,
        status: StepStatus,
    ) -> Result<(), String> {
        let mut store = self.state_store.lock().await;
        if let Some(state) = store.get_mut(workflow_id) {
            state.set_step_status(step_id, status);
            Ok(())
        } else {
            Err(format!("Workflow {} not found", workflow_id))
        }
    }

    pub async fn determine_workflow_status(&self, workflow_id: &str) -> Option<WorkflowStatus> {
        let store = self.state_store.lock().await;
        if let Some(state) = store.get(workflow_id) {
            if state.steps.is_empty() {
                return Some(WorkflowStatus::InProgress);
            }

            let mut all_completed = true;
            for status in state.steps.values() {
                match status {
                    StepStatus::Failed(msg) => return Some(WorkflowStatus::Failed(msg.clone())),
                    StepStatus::Completed(_) => continue,
                    _ => all_completed = false,
                }
            }

            if all_completed {
                Some(WorkflowStatus::Completed)
            } else {
                Some(WorkflowStatus::InProgress)
            }
        } else {
            None
        }
    }

    pub async fn get_workflow_state(&self, workflow_id: &str) -> Option<WorkflowState> {
        let store = self.state_store.lock().await;
        store.get(workflow_id).cloned()
    }

    // New code-native capabilities to store arbitrary structured data across boundaries
    pub async fn set_context_var(
        &self,
        workflow_id: &str,
        key: &str,
        value: &str,
    ) -> Result<(), String> {
        let mut store = self.state_store.lock().await;
        if let Some(state) = store.get_mut(workflow_id) {
            state.context.insert(key.to_string(), value.to_string());
            Ok(())
        } else {
            Err(format!("Workflow {} not found", workflow_id))
        }
    }

    pub async fn get_context_var(&self, workflow_id: &str, key: &str) -> Option<String> {
        let store = self.state_store.lock().await;
        if let Some(state) = store.get(workflow_id) {
            state.context.get(key).cloned()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_start_new_workflow() {
        let engine = DurableExecutionEngine::new();
        let state = engine.start_or_resume_workflow("wf-1").await;
        assert_eq!(state.workflow_id, "wf-1");
        assert!(state.steps.is_empty());
    }

    #[tokio::test]
    async fn test_update_step_status() {
        let engine = DurableExecutionEngine::new();
        engine.start_or_resume_workflow("wf-2").await;

        let res = engine
            .update_step(
                "wf-2",
                "step-1",
                StepStatus::Completed("Success".to_string()),
            )
            .await;
        assert!(res.is_ok());

        let state = engine.get_workflow_state("wf-2").await.unwrap();
        assert_eq!(
            state.get_step_status("step-1"),
            Some(&StepStatus::Completed("Success".to_string()))
        );
    }

    #[tokio::test]
    async fn test_resume_existing_workflow() {
        let engine = DurableExecutionEngine::new();
        engine.start_or_resume_workflow("wf-3").await;
        engine
            .update_step("wf-3", "step-1", StepStatus::Completed("Done".to_string()))
            .await
            .unwrap();

        // Resume should return existing state
        let state = engine.start_or_resume_workflow("wf-3").await;
        assert_eq!(
            state.get_step_status("step-1"),
            Some(&StepStatus::Completed("Done".to_string()))
        );
    }

    #[tokio::test]
    async fn test_update_nonexistent_workflow() {
        let engine = DurableExecutionEngine::new();
        let res = engine
            .update_step("invalid-wf", "step-1", StepStatus::Running)
            .await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_workflow_status() {
        let engine = DurableExecutionEngine::new();
        engine.start_or_resume_workflow("wf-4").await;

        assert_eq!(
            engine.determine_workflow_status("wf-4").await,
            Some(WorkflowStatus::InProgress)
        );

        engine
            .update_step("wf-4", "step-1", StepStatus::Completed("Done".to_string()))
            .await
            .unwrap();
        assert_eq!(
            engine.determine_workflow_status("wf-4").await,
            Some(WorkflowStatus::Completed)
        );

        engine
            .update_step("wf-4", "step-2", StepStatus::Pending)
            .await
            .unwrap();
        assert_eq!(
            engine.determine_workflow_status("wf-4").await,
            Some(WorkflowStatus::InProgress)
        );

        engine
            .update_step("wf-4", "step-2", StepStatus::Failed("Err".to_string()))
            .await
            .unwrap();
        assert_eq!(
            engine.determine_workflow_status("wf-4").await,
            Some(WorkflowStatus::Failed("Err".to_string()))
        );
    }

    #[tokio::test]
    async fn test_workflow_context_vars() {
        let engine = DurableExecutionEngine::new();
        engine.start_or_resume_workflow("wf-5").await;

        let res = engine.set_context_var("wf-5", "key1", "val1").await;
        assert!(res.is_ok());

        let val = engine.get_context_var("wf-5", "key1").await;
        assert_eq!(val, Some("val1".to_string()));

        let missing = engine.get_context_var("wf-5", "missing").await;
        assert_eq!(missing, None);
    }
}
