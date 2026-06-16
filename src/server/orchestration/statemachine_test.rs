use super::statemachine_v2::{StateMachine, State, Repository};
use super::locks::StandaloneLock;
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

    fn update_task_state(&self, task_id: &str, new_state: State, _agent_id: &str) -> Result<(), String> {
        let mut states = self.states.lock().unwrap();
        states.insert(task_id.to_string(), new_state);
        Ok(())
    }

    fn get_dependent_tasks(&self, _task_id: &str) -> Result<Vec<String>, String> {
        Ok(vec![])
    }

    fn are_all_dependencies_met(&self, _task_id: &str) -> Result<bool, String> {
        Ok(true)
    }
}

#[tokio::test]
async fn test_statemachine_valid_transitions() {
    let repo = Arc::new(MockRepository::new());
    let lock = Arc::new(StandaloneLock::new());
    let sm = StateMachine::new(repo.clone(), lock, None);

    let task_id = "task1";
    let tenant_id = "tenant1";

    // Pending -> Ready
    sm.transition_to_ready(tenant_id, task_id).await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::Ready);

    // Ready -> InProgress
    sm.transition_to_in_progress(tenant_id, task_id, "agent1").await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::InProgress);

    // InProgress -> Blocked
    sm.transition_to_blocked(tenant_id, task_id).await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::Blocked);

    // Blocked -> InProgress
    sm.transition_to_in_progress(tenant_id, task_id, "agent1").await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::InProgress);

    // InProgress -> Completed
    sm.transition_to_completed(tenant_id, task_id).await.unwrap();
    assert_eq!(repo.get_task_state(task_id).unwrap(), State::Completed);
}

#[tokio::test]
async fn test_statemachine_invalid_transition() {
    let repo = Arc::new(MockRepository::new());
    let lock = Arc::new(StandaloneLock::new());
    let sm = StateMachine::new(repo.clone(), lock, None);

    let task_id = "task2";
    let tenant_id = "tenant1";

    // Pending -> InProgress (Invalid)
    let err = sm.transition_to_in_progress(tenant_id, task_id, "agent1").await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_statemachine_concurrent_transitions() {
    let repo = Arc::new(MockRepository::new());
    let lock = Arc::new(StandaloneLock::new());
    let sm = Arc::new(StateMachine::new(repo.clone(), lock, None));

    let task_id = "task3";
    let tenant_id = "tenant1";

    sm.transition_to_ready(tenant_id, task_id).await.unwrap();

    let sm1 = sm.clone();
    let sm2 = sm.clone();

    let t1 = tokio::spawn(async move {
        sm1.transition_to_in_progress("tenant1", "task3", "agent1").await
    });

    let t2 = tokio::spawn(async move {
        sm2.transition_to_in_progress("tenant1", "task3", "agent2").await
    });

    let res1 = t1.await.unwrap();
    let res2 = t2.await.unwrap();

    let mut success_count = 0;
    if res1.is_ok() { success_count += 1; }
    if res2.is_ok() { success_count += 1; }

    assert_eq!(success_count, 1, "Only one transition should succeed");
}
