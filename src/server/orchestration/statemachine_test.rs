use super::statemachine_v2::{StateMachine, State, Repository};
use super::locks::StandaloneLock;
use crate::orchestration::mesh::TeammateMesh;
use ohc_builtin_agent::mesh::transport::Message;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

struct MockRepository {
    states: Mutex<HashMap<String, State>>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            states: Mutex::new(HashMap::new()),
        }
    }
}

impl Repository for MockRepository {
    fn get_task_state(&self, task_id: &str) -> Result<State, String> {
        let states = self.states.lock().unwrap();
        Ok(states.get(task_id).cloned().unwrap_or(State::Pending))
    }

    fn update_task_state(&self, task_id: &str, _from_state: State, to_state: State, _agent_id: &str) -> Result<(), String> {
        let mut states = self.states.lock().unwrap();
        states.insert(task_id.to_string(), to_state);
        Ok(())
    }
}

struct MockMesh {
    published: Mutex<Vec<(String, Vec<u8>)>>,
}

impl MockMesh {
    fn new() -> Self {
        Self {
            published: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait::async_trait]
impl TeammateMesh for MockMesh {
    async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        self.published.lock().unwrap().push((topic.to_string(), payload));
        Ok(())
    }

    async fn publish_with_ack(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        self.published.lock().unwrap().push((topic.to_string(), payload));
        Ok(())
    }

    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }

    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
        Ok(true)
    }

    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
        Ok(())
    }

    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> {
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        Ok(vec![])
    }

    async fn ping(&self) -> Result<(), String> {
        Ok(())
    }

    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }

    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }

    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        Ok(Box::new(|| {}))
    }
}

#[tokio::test]
async fn test_statemachine_valid_transitions() {
    let repo = Arc::new(MockRepository::new());
    let lock = Arc::new(StandaloneLock::new());
    let mesh = Arc::new(MockMesh::new());
    let sm = StateMachine::new(repo.clone(), lock, mesh.clone());

    let task_id = "task1";

    // Pending -> Ready
    sm.transition_to_ready(task_id).await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::Ready);

    // Ready -> InProgress
    sm.transition_to_in_progress(task_id, "agent1").await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::InProgress);

    // InProgress -> Blocked
    sm.transition_to_blocked(task_id).await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::Blocked);

    // Blocked -> InProgress
    sm.transition_to_in_progress(task_id, "agent1").await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::InProgress);

    // InProgress -> Completed
    sm.transition_to_completed(task_id).await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::Completed);

    // Assert messages were published
    let published = mesh.published.lock().unwrap();
    assert_eq!(published.len(), 5);
    for (topic, _payload) in published.iter() {
        assert_eq!(topic, "mesh:tasks");
    }
}

#[tokio::test]
async fn test_statemachine_invalid_transition() {
    let repo = Arc::new(MockRepository::new());
    let lock = Arc::new(StandaloneLock::new());
    let mesh = Arc::new(MockMesh::new());
    let sm = StateMachine::new(repo.clone(), lock, mesh.clone());

    let task_id = "task2";

    // Pending -> InProgress (Invalid)
    let err = sm.transition_to_in_progress(task_id, "agent1").await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_statemachine_concurrent_transitions() {
    let repo = Arc::new(MockRepository::new());
    let lock = Arc::new(StandaloneLock::new());
    let mesh = Arc::new(MockMesh::new());
    let sm = Arc::new(StateMachine::new(repo.clone(), lock, mesh.clone()));

    let task_id = "task3";

    sm.transition_to_ready(task_id).await.unwrap();

    let sm1 = sm.clone();
    let sm2 = sm.clone();

    let t1 = tokio::spawn(async move {
        sm1.transition_to_in_progress("task3", "agent1").await
    });

    let t2 = tokio::spawn(async move {
        sm2.transition_to_in_progress("task3", "agent2").await
    });

    let res1 = t1.await.unwrap();
    let res2 = t2.await.unwrap();

    let mut success_count = 0;
    if res1.is_ok() { success_count += 1; }
    if res2.is_ok() { success_count += 1; }

    assert_eq!(success_count, 1, "Only one transition should succeed");
}
