use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// Subagent event types — mirrors Go SubagentEventType.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubagentEventType {
    Unspecified = 0,
    Spawned = 1,
    Heartbeat = 2,
    Completed = 3,
    Failed = 4,
    Killed = 5,
}

/// Task notification sent when a sub-agent task completes.
#[derive(Debug, Clone)]
pub struct TaskNotification {
    pub task_id: String,
    pub tool_use_id: String,
    pub output_file: String,
    pub status: String,
    pub summary: String,
    pub result: String,
    pub token_count: i64,
    pub tool_uses: i64,
    pub duration_ms: i64,
}

/// Subagent lifecycle event — mirrors Go SubagentLifecycleEvent.
#[derive(Debug, Clone)]
pub struct SubagentLifecycleEvent {
    pub event_type: SubagentEventType,
    pub task_id: String,
    pub parent_task_id: String,
    pub timestamp_ms: i64,
    pub notification: Option<TaskNotification>,
}

/// An in-process pub/sub bus for subagent lifecycle events.
/// Mirrors Go SubagentBus.
pub struct SubagentBus {
    sender: broadcast::Sender<SubagentLifecycleEvent>,
}

impl SubagentBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(256);
        Self { sender }
    }

    /// Publish an event to all subscribers.
    pub fn publish(&self, mut evt: SubagentLifecycleEvent) {
        if evt.timestamp_ms == 0 {
            evt.timestamp_ms = Utc::now().timestamp_millis();
        }
        // Errors only if no receivers — that's fine.
        let _ = self.sender.send(evt);
    }

    /// Subscribe to all lifecycle events.
    pub fn subscribe(&self) -> broadcast::Receiver<SubagentLifecycleEvent> {
        self.sender.subscribe()
    }
}

impl Default for SubagentBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry of spawned subagents — mirrors Go SubagentRegistry.
#[derive(Debug, Default)]
pub struct SubagentState {
    pub task_id: String,
    pub description: String,
    pub status: String, // "running" | "completed" | "failed" | "killed"
    pub result: String,
    pub error: String,
    pub started_at: i64,
    pub ended_at: i64,
    pub tool_use_id: String,
    pub output_file: String,
    pub token_count: i64,
    pub tool_uses: i64,
}

#[derive(Clone, Default)]
pub struct SubagentRegistry {
    tasks: Arc<RwLock<HashMap<String, SubagentState>>>,
}

impl SubagentRegistry {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register(&self, state: SubagentState) {
        let mut tasks = self.tasks.write().expect("lock failed");
        tasks.insert(state.task_id.clone(), state);
    }

    pub fn get(&self, task_id: &str) -> Option<SubagentState> {
        let tasks = self.tasks.read().expect("lock failed");
        tasks.get(task_id).map(|s| SubagentState {
            task_id: s.task_id.clone(),
            description: s.description.clone(),
            status: s.status.clone(),
            result: s.result.clone(),
            error: s.error.clone(),
            started_at: s.started_at,
            ended_at: s.ended_at,
            tool_use_id: s.tool_use_id.clone(),
            output_file: s.output_file.clone(),
            token_count: s.token_count,
            tool_uses: s.tool_uses,
        })
    }

    pub fn kill(&self, task_id: &str) -> bool {
        let mut tasks = self.tasks.write().expect("lock failed");
        if let Some(s) = tasks.get_mut(task_id)
            && s.status == "running" {
                s.status = "killed".to_string();
                s.ended_at = chrono::Utc::now().timestamp_millis();
                return true;
            }
        false
    }

    pub fn all(&self) -> Vec<SubagentState> {
        let tasks = self.tasks.read().expect("lock failed");
        tasks
            .values()
            .map(|s| SubagentState {
                task_id: s.task_id.clone(),
                description: s.description.clone(),
                status: s.status.clone(),
                result: s.result.clone(),
                error: s.error.clone(),
                started_at: s.started_at,
                ended_at: s.ended_at,
                tool_use_id: s.tool_use_id.clone(),
                output_file: s.output_file.clone(),
                token_count: s.token_count,
                tool_uses: s.tool_uses,
            })
            .collect()
    }
}

/// Build a task notification message.
#[allow(clippy::too_many_arguments)]
pub fn build_task_notification(
    task_id: &str,
    tool_use_id: &str,
    output_file: &str,
    status: &str,
    summary: &str,
    result: &str,
    token_count: i64,
    tool_uses: i64,
    duration_ms: i64,
) -> TaskNotification {
    TaskNotification {
        task_id: task_id.to_string(),
        tool_use_id: tool_use_id.to_string(),
        output_file: output_file.to_string(),
        status: status.to_string(),
        summary: truncate_notif(summary, 1000),
        result: truncate_notif(result, 2000),
        token_count,
        tool_uses,
        duration_ms,
    }
}

fn truncate_notif(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_register_get() {
        let reg = SubagentRegistry::new();
        reg.register(SubagentState {
            task_id: "t1".to_string(),
            status: "running".to_string(),
            ..Default::default()
        });
        let s = reg.get("t1").expect("lock failed");
        assert_eq!(s.status, "running");
    }

    #[test]
    fn test_registry_kill() {
        let reg = SubagentRegistry::new();
        reg.register(SubagentState {
            task_id: "t2".to_string(),
            status: "running".to_string(),
            ..Default::default()
        });
        assert!(reg.kill("t2"));
        let s = reg.get("t2").expect("lock failed");
        assert_eq!(s.status, "killed");
        // Second kill should return false
        assert!(!reg.kill("t2"));
    }

    #[test]
    fn test_bus_pubsub() {
        let bus = SubagentBus::new();
        let mut rx = bus.subscribe();
        bus.publish(SubagentLifecycleEvent {
            event_type: SubagentEventType::Spawned,
            task_id: "t3".to_string(),
            parent_task_id: String::new(),
            timestamp_ms: 0,
            notification: None,
        });
        let evt = rx.try_recv().expect("lock failed");
        assert_eq!(evt.task_id, "t3");
        assert_eq!(evt.event_type, SubagentEventType::Spawned);
    }
}
