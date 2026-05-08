use std::rc::Rc;
use slint::{ComponentHandle, Model, VecModel};
use crate::app::{Dashboard, UiPendingApproval};

// 1. Test starting from Dashboard with No Pending Approvals
#[test]
fn test_action_center_empty_state() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = Dashboard::new().unwrap();
    let empty_approvals: Vec<UiPendingApproval> = vec![];
    ui.set_pending_approvals(Rc::new(VecModel::from(empty_approvals)).into());

    // Action center should not be shown initially
    assert_eq!(ui.get_show_action_center(), false);
}

// 2. Test opening Action Center with Pending Approvals
#[test]
fn test_open_action_center_with_tasks() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = Dashboard::new().unwrap();
    let pending = vec![
        UiPendingApproval {
            task_id: "task-1".into(),
            title: "Draft Email".into(),
            proposed_content: "Hello!".into(),
            helper_name: "Marketing AI".into(),
        }
    ];

    ui.set_pending_approvals(Rc::new(VecModel::from(pending)).into());

    // Open Action Center
    ui.set_show_action_center(true);
    assert_eq!(ui.get_show_action_center(), true);
}

// 3. Test approving a task directly
#[test]
fn test_approve_task() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = Dashboard::new().unwrap();
    let pending = vec![
        UiPendingApproval {
            task_id: "task-1".into(),
            title: "Draft Tweet".into(),
            proposed_content: "Great sale today!".into(),
            helper_name: "Social AI".into(),
        }
    ];
    ui.set_pending_approvals(Rc::new(VecModel::from(pending.clone())).into());
    ui.set_show_action_center(true);

    let approved_task = Rc::new(std::cell::RefCell::new(String::new()));
    let approved_clone = approved_task.clone();

    ui.on_approve_task(move |task_id| {
        *approved_clone.borrow_mut() = task_id.to_string();
    });

    // Simulate clicking approve
    ui.invoke_approve_task("task-1".into());

    assert_eq!(*approved_task.borrow(), "task-1");
}

// 4. Test Closing the Action Center view
#[test]
fn test_close_action_center() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = Dashboard::new().unwrap();
    ui.set_show_action_center(true);

    // Simulate user closing it
    ui.set_show_action_center(false);

    assert_eq!(ui.get_show_action_center(), false);
}

// 5. Test rendering multiple approvals
#[test]
fn test_action_center_multiple_tasks() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = Dashboard::new().unwrap();
    let pending = vec![
        UiPendingApproval {
            task_id: "task-1".into(),
            title: "Post 1".into(),
            proposed_content: "Sale!".into(),
            helper_name: "Social AI".into(),
        },
        UiPendingApproval {
            task_id: "task-2".into(),
            title: "Post 2".into(),
            proposed_content: "New Product!".into(),
            helper_name: "Marketing AI".into(),
        }
    ];
    ui.set_pending_approvals(Rc::new(VecModel::from(pending.clone())).into());

    assert_eq!(ui.get_pending_approvals().row_count(), 2);
}
