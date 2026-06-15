use std::collections::HashMap;
use std::sync::Arc;
use super::locks::DistributedLock;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    Pending,
    Ready,
    InProgress,
    Blocked,
    Completed,
    Failed,
}

impl State {
    pub fn as_str(&self) -> &'static str {
        match self {
            State::Pending => "PENDING",
            State::Ready => "READY",
            State::InProgress => "IN_PROGRESS",
            State::Blocked => "BLOCKED",
            State::Completed => "COMPLETED",
            State::Failed => "FAILED",
        }
    }
}

pub trait Repository: Send + Sync {
    fn get_task_state(&self, task_id: &str) -> Result<State, String>;
    fn update_task_state(&self, task_id: &str, new_state: State, agent_id: &str) -> Result<(), String>;
}

use crate::orchestration::mesh::TeammateMesh;
use serde_json::json;

pub struct StateMachine {
    repo: Arc<dyn Repository>,
    lock: Arc<dyn DistributedLock>,
    mesh: Option<Arc<dyn TeammateMesh>>,
    allowed_transitions: HashMap<State, Vec<State>>,
}

impl StateMachine {
    pub fn new(repo: Arc<dyn Repository>, lock: Arc<dyn DistributedLock>, mesh: Option<Arc<dyn TeammateMesh>>) -> Self {
        let mut allowed_transitions = HashMap::new();
        allowed_transitions.insert(State::Pending, vec![State::Ready]);
        allowed_transitions.insert(State::Ready, vec![State::InProgress]);
        allowed_transitions.insert(State::InProgress, vec![State::Completed, State::Blocked, State::Failed]);
        allowed_transitions.insert(State::Blocked, vec![State::InProgress, State::Failed]);

        Self {
            repo,
            lock,
            mesh,
            allowed_transitions,
        }
    }

    pub async fn transition(&self, tenant_id: &str, task_id: &str, new_state: State, agent_id: &str) -> Result<(), String> {
        let _guard = self.lock.acquire(task_id).await?;

        let current_state = self.repo.get_task_state(task_id)?;

        let valid_transitions = self.allowed_transitions.get(&current_state)
            .ok_or_else(|| format!("no valid transitions from state {:?}", current_state))?;

        if !valid_transitions.contains(&new_state) {
            return Err(format!("invalid transition from {:?} to {:?}", current_state, new_state));
        }

        self.repo.update_task_state(task_id, new_state.clone(), agent_id)?;

        // Publish to Teammate Mesh here
        if let Some(mesh) = &self.mesh {
            let topic = format!("{}:mesh:tasks", tenant_id);
            let payload = json!({
                "tenant_id": tenant_id,
                "task_id": task_id,
                "state": new_state.as_str(),
                "agent_id": agent_id
            }).to_string().into_bytes();
            let _ = mesh.publish(&topic, payload).await;
        }

        Ok(())
    }

    pub async fn transition_to_ready(&self, tenant_id: &str, task_id: &str) -> Result<(), String> {
        self.transition(tenant_id, task_id, State::Ready, "").await
    }

    pub async fn transition_to_in_progress(&self, tenant_id: &str, task_id: &str, agent_id: &str) -> Result<(), String> {
        self.transition(tenant_id, task_id, State::InProgress, agent_id).await
    }

    pub async fn transition_to_completed(&self, tenant_id: &str, task_id: &str) -> Result<(), String> {
        self.transition(tenant_id, task_id, State::Completed, "").await
    }

    pub async fn transition_to_blocked(&self, tenant_id: &str, task_id: &str) -> Result<(), String> {
        self.transition(tenant_id, task_id, State::Blocked, "").await
    }

    pub async fn transition_to_failed(&self, tenant_id: &str, task_id: &str) -> Result<(), String> {
        self.transition(tenant_id, task_id, State::Failed, "").await
    }

    pub async fn start_mesh_listener(self: Arc<Self>, mesh: Arc<dyn TeammateMesh>) -> Result<(), String> {
        let pattern = "*:mesh:tasks".to_string();
        mesh.subscribe_pattern(&pattern, Box::new(move |msg| {
            let sm = self.clone();
            tokio::spawn(async move {
                if let Ok(payload) = String::from_utf8(msg.payload.clone()) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&payload) {
                        if let (Some(tenant), Some(task_id), Some(action)) = (
                            json.get("tenant_id").and_then(|v| v.as_str()),
                            json.get("task_id").and_then(|v| v.as_str()),
                            json.get("action").and_then(|v| v.as_str())
                        ) {
                            let agent_id = msg.agent_id.as_str();
                            let _ = match action {
                                "ready" => sm.transition_to_ready(tenant, task_id).await,
                                "in_progress" => sm.transition_to_in_progress(tenant, task_id, agent_id).await,
                                "completed" => sm.transition_to_completed(tenant, task_id).await,
                                "blocked" => sm.transition_to_blocked(tenant, task_id).await,
                                "failed" => sm.transition_to_failed(tenant, task_id).await,
                                _ => Ok(()),
                            };
                        }
                    }
                }
            });
        })).await?;
        Ok(())
    }
}
