use std::collections::HashMap;
use std::sync::Arc;
use crate::orchestration::locks::DistributedLock;

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

pub struct StateMachine {
    repo: Arc<dyn Repository>,
    lock: Arc<dyn DistributedLock>,
    allowed_transitions: HashMap<State, Vec<State>>,
}

impl StateMachine {
    pub fn new(repo: Arc<dyn Repository>, lock: Arc<dyn DistributedLock>) -> Self {
        let mut allowed_transitions = HashMap::new();
        allowed_transitions.insert(State::Pending, vec![State::Ready]);
        allowed_transitions.insert(State::Ready, vec![State::InProgress]);
        allowed_transitions.insert(State::InProgress, vec![State::Completed, State::Blocked, State::Failed]);
        allowed_transitions.insert(State::Blocked, vec![State::InProgress, State::Failed]);

        Self {
            repo,
            lock,
            allowed_transitions,
        }
    }

    pub async fn transition(&self, task_id: &str, new_state: State, agent_id: &str) -> Result<(), String> {
        let _guard = self.lock.acquire(task_id).await?;

        let current_state = self.repo.get_task_state(task_id)?;

        let valid_transitions = self.allowed_transitions.get(&current_state)
            .ok_or_else(|| format!("no valid transitions from state {:?}", current_state))?;

        if !valid_transitions.contains(&new_state) {
            return Err(format!("invalid transition from {:?} to {:?}", current_state, new_state));
        }

        self.repo.update_task_state(task_id, new_state, agent_id)?;

        // Publish to Teammate Mesh here

        Ok(())
    }

    pub async fn transition_to_ready(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Ready, "").await
    }

    pub async fn transition_to_in_progress(&self, task_id: &str, agent_id: &str) -> Result<(), String> {
        self.transition(task_id, State::InProgress, agent_id).await
    }

    pub async fn transition_to_completed(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Completed, "").await
    }

    pub async fn transition_to_blocked(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Blocked, "").await
    }

    pub async fn transition_to_failed(&self, task_id: &str) -> Result<(), String> {
        self.transition(task_id, State::Failed, "").await
    }
}
