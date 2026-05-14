use super::manager::{PermissionManager, PermissionRule, PermissionAction};

// Simple state machine for testing
#[derive(Debug, PartialEq)]
enum State {
    Executing,
    PendingApproval,
    Failed,
}

struct TestMachine {
    tasks: std::collections::HashMap<String, State>,
}

impl TestMachine {
    fn new() -> Self {
        Self { tasks: std::collections::HashMap::new() }
    }

    fn add_task(&mut self, id: String, state: State) {
        self.tasks.insert(id, state);
    }

    fn transition(&mut self, id: &str, state: State) {
        if let Some(s) = self.tasks.get_mut(id) {
            *s = state;
        }
    }

    fn get_state(&self, id: &str) -> Option<&State> {
        self.tasks.get(id)
    }
}

// Dummy gRPC endpoint logic
fn request_permission(
    pm: &PermissionManager,
    sm: &mut TestMachine,
    org_id: &str,
    tool_name: &str,
    args: &str,
    task_id: &str,
) -> State {
    let action = pm.check_permission(org_id, tool_name, args);
    match action {
        PermissionAction::Ask => {
            sm.transition(task_id, State::PendingApproval);
            State::PendingApproval
        }
        PermissionAction::Deny => {
            sm.transition(task_id, State::Failed);
            State::Failed
        }
        PermissionAction::Allow => State::Executing,
    }
}

#[test]
fn test_permission_manager() {
    let mut pm = PermissionManager::new();
    let org_id = "org123".to_string();

    pm.add_rule(org_id.clone(), PermissionRule {
        tool_name: "DeleteDatabase".to_string(),
        pattern: ".*".to_string(),
        action: PermissionAction::Ask,
    });

    let mut sm = TestMachine::new();
    let task_id = "task1".to_string();
    sm.add_task(task_id.clone(), State::Executing);

    // Test a safe tool
    let action = pm.check_permission(&org_id, "SafeTool", "args");
    assert_eq!(action, PermissionAction::Allow);

    // Test a sensitive tool
    let new_state = request_permission(&pm, &mut sm, &org_id, "DeleteDatabase", "args", &task_id);
    assert_eq!(new_state, State::PendingApproval);
    assert_eq!(sm.get_state(&task_id), Some(&State::PendingApproval));

    // Simulate user approval (mocked UI call)
    sm.transition(&task_id, State::Executing);
    assert_eq!(sm.get_state(&task_id), Some(&State::Executing));
}
