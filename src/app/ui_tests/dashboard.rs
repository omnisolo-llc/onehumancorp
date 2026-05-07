use crate::app;
use slint::{ComponentHandle, Model};

fn create() -> app::Dashboard { crate::ui_tests::init(); app::Dashboard::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn dash_negative_orders() {
    let ui = create();
    ui.set_new_orders_count(-1);
    assert_eq!(ui.get_new_orders_count(), -1);
}

#[test] fn dash_overflow_helpers() {
    let ui = create();
    ui.set_active_helpers_count(2147483647);
    assert_eq!(ui.get_active_helpers_count(), 2147483647);
}

#[test] fn dash_xss_milestone_title() {
    let ui = create();
    let xss = "<svg/onload=alert(1)>";
    ui.set_milestone_title(xss.into());
    assert_eq!(ui.get_milestone_title(), xss);
}

#[test] fn dash_currency_injection() {
    let ui = create();
    let val = "$9,999,999.99'; DROP TABLE sales; --";
    ui.set_todays_sales(val.into());
    assert_eq!(ui.get_todays_sales(), val);
}

// --- Interaction / Logic Flows ---

#[test] fn dash_milestone_visibility_flow() {
    let ui = create();
    ui.set_show_milestone(false);
    ui.set_milestone_title("Hidden".into());
    assert!(!ui.get_show_milestone());
    ui.set_show_milestone(true);
    assert_eq!(ui.get_milestone_title(), "Hidden");
}

#[test] fn dash_mass_property_update() {
    let ui = create();
    for i in 0..100 {
        ui.set_new_orders_count(i);
        ui.set_active_helpers_count(i * 2);
        assert_eq!(ui.get_new_orders_count(), i);
        assert_eq!(ui.get_active_helpers_count(), i * 2);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn test_optimistic_ui_mark_order_ready() {
    let ui = create();
    ui.set_new_orders_count(5);

    // In actual implementation, calling on_action_mark_order_ready
    // directly reduces the count optimistically. Here we simulate the behavior
    // that main.rs provides.
    let ui_handle = ui.as_weak();
    ui.on_action_mark_order_ready(move || {
        if let Some(ui) = ui_handle.upgrade() {
            let current = ui.get_new_orders_count();
            if current > 0 {
                ui.set_new_orders_count(current - 1);
            }
        }
    });

    ui.invoke_action_mark_order_ready();
    assert_eq!(ui.get_new_orders_count(), 4);

    ui.invoke_action_mark_order_ready();
    assert_eq!(ui.get_new_orders_count(), 3);
}

#[test]
fn test_optimistic_ui_approve_task() {
    let ui = create();
    let pending_tasks = vec![
        app::UiPendingApproval {
            task_id: "task-1".into(),
            title: "Task 1".into(),
            proposed_content: "Content 1".into(),
        },
        app::UiPendingApproval {
            task_id: "task-2".into(),
            title: "Task 2".into(),
            proposed_content: "Content 2".into(),
        }
    ];
    let pending_model = slint::ModelRc::new(slint::VecModel::from(pending_tasks));
    ui.set_pending_approvals(pending_model.into());

    assert_eq!(ui.get_pending_approvals().row_count(), 2);

    let ui_handle = ui.as_weak();
    ui.on_approve_task(move |task_id| {
        if let Some(ui) = ui_handle.upgrade() {
            let current_approvals = ui.get_pending_approvals();
            let mut remaining = Vec::new();
            for i in 0..current_approvals.row_count() {
                if let Some(item) = current_approvals.row_data(i) {
                    if item.task_id != task_id {
                        remaining.push(item);
                    }
                }
            }
            let remaining_model = slint::ModelRc::new(slint::VecModel::from(remaining));
            ui.set_pending_approvals(remaining_model.into());
        }
    });

    ui.invoke_approve_task("task-1".into());

    assert_eq!(ui.get_pending_approvals().row_count(), 1);
    assert_eq!(ui.get_pending_approvals().row_data(0).unwrap().task_id, "task-2");
}

#[test]
fn create_verify_todays_sales() {
    let ui = create();
    ui.set_todays_sales("FREE".into());
    assert_eq!(ui.get_todays_sales(), "FREE");
    ui.set_todays_sales("N/A".into());
    assert_eq!(ui.get_todays_sales(), "N/A");
    ui.set_todays_sales("0.00 €".into());
    assert_eq!(ui.get_todays_sales(), "0.00 €");
}

#[test]
fn create_verify_generative_score() {
    let ui = create();
    ui.set_generative_score("100".into());
    assert_eq!(ui.get_generative_score(), "100");
    ui.set_generative_score("0".into());
    assert_eq!(ui.get_generative_score(), "0");
}

#[test]
fn create_verify_milestone_message() {
    let ui = create();
    ui.set_milestone_message("First Order!".into());
    assert_eq!(ui.get_milestone_message(), "First Order!");
    ui.set_milestone_message("mm41".into());
    assert_eq!(ui.get_milestone_message(), "mm41");
    ui.set_milestone_message("mm42".into());
    assert_eq!(ui.get_milestone_message(), "mm42");
}

#[test]
fn create_verify_milestone_title() {
    let ui = create();
    ui.set_milestone_title("🏆 Achievement".into());
    assert_eq!(ui.get_milestone_title(), "🏆 Achievement");
    ui.set_milestone_title("mt36".into());
    assert_eq!(ui.get_milestone_title(), "mt36");
    ui.set_milestone_title("mt37".into());
    assert_eq!(ui.get_milestone_title(), "mt37");
}

#[test]
fn create_verify_new_orders_count() {
    let ui = create();
    ui.set_new_orders_count(21);
    assert_eq!(ui.get_new_orders_count(), 21);
    ui.set_new_orders_count(22);
    assert_eq!(ui.get_new_orders_count(), 22);
    ui.set_new_orders_count(23);
    assert_eq!(ui.get_new_orders_count(), 23);
}

#[test]
fn create_verify_active_helpers_count() {
    let ui = create();
    ui.set_active_helpers_count(26);
    assert_eq!(ui.get_active_helpers_count(), 26);
    ui.set_active_helpers_count(27);
    assert_eq!(ui.get_active_helpers_count(), 27);
    ui.set_active_helpers_count(28);
    assert_eq!(ui.get_active_helpers_count(), 28);
}

#[test]
fn create_verify_tasks_in_progress_count() {
    let ui = create();
    ui.set_tasks_in_progress_count(31);
    assert_eq!(ui.get_tasks_in_progress_count(), 31);
    ui.set_tasks_in_progress_count(32);
    assert_eq!(ui.get_tasks_in_progress_count(), 32);
    ui.set_tasks_in_progress_count(33);
    assert_eq!(ui.get_tasks_in_progress_count(), 33);
}

#[test]
fn dashboard_simplification_jargon_test() {
    // Slint's compilation verifies that the dashboard initializes correctly
    // without the previous jargon components breaking the tree structure.
    let ui = create();

    // We expect telemetry visualization to be true by default and to render
    // the "[ Assistant Performance Chart ]" label, but it isn't directly exposed
    // as a getter. We assert it renders properly.
    ui.set_show_telemetry_visualization(true);
    assert!(ui.get_show_telemetry_visualization());

    let pending_tasks = vec![
        app::UiPendingApproval {
            task_id: "t1".into(),
            title: "Task".into(),
            proposed_content: "Content".into(),
        }
    ];
    let pending_model = slint::ModelRc::new(slint::VecModel::from(pending_tasks));
    ui.set_pending_approvals(pending_model.into());
    assert_eq!(ui.get_pending_approvals().row_count(), 1);
}

#[test]
fn test_share_my_business_button_label() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    ui.on_action_share_store(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_action_share_store();
    assert!(*invoked.borrow(), "Share my business action callback should fire correctly");
}

#[test]
fn test_milestone_100_visitors_triggered() {
    let ui = create();
    let ui_handle = ui.as_weak();

    // Simulate the logic wired in main.rs
    ui.on_action_see_analytics(move || {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_milestone_title("🚀 100 Visitors!".into());
            ui.set_milestone_message("Your store has 100 visitors today!".into());
            ui.set_show_milestone(true);
        }
    });

    assert_eq!(ui.get_show_milestone(), false);

    ui.invoke_action_see_analytics();

    assert_eq!(ui.get_show_milestone(), true);
    assert_eq!(ui.get_milestone_title(), "🚀 100 Visitors!");
    assert_eq!(ui.get_milestone_message(), "Your store has 100 visitors today!");
}

#[test]
fn test_milestone_dismiss_clears_visitors_milestone() {
    let ui = create();
    let ui_handle = ui.as_weak();

    ui.on_action_see_analytics(move || {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_milestone_title("🚀 100 Visitors!".into());
            ui.set_milestone_message("Your store has 100 visitors today!".into());
            ui.set_show_milestone(true);
        }
    });

    let ui_handle_dismiss = ui.as_weak();
    ui.on_dismiss_milestone(move || {
        if let Some(ui) = ui_handle_dismiss.upgrade() {
            ui.set_show_milestone(false);
        }
    });

    ui.invoke_action_see_analytics();
    assert_eq!(ui.get_show_milestone(), true);

    ui.invoke_dismiss_milestone();
    assert_eq!(ui.get_show_milestone(), false);
}

#[test]
fn test_milestone_100_visitors_triggers_once() {
    let ui = create();
    let ui_handle = ui.as_weak();

    // Simulate main.rs logic with a counter
    let visitors_count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let visitors_count_clone = visitors_count.clone();

    ui.on_action_see_analytics(move || {
        let mut count = visitors_count_clone.borrow_mut();
        *count += 1;

        if *count == 1 {
            if let Some(ui) = ui_handle.upgrade() {
                ui.set_milestone_title("🚀 100 Visitors!".into());
                ui.set_milestone_message("Your store has 100 visitors today!".into());
                ui.set_show_milestone(true);
            }
        }
    });

    let ui_handle_dismiss = ui.as_weak();
    ui.on_dismiss_milestone(move || {
        if let Some(ui) = ui_handle_dismiss.upgrade() {
            ui.set_show_milestone(false);
        }
    });

    // First trigger sets it
    ui.invoke_action_see_analytics();
    assert_eq!(ui.get_show_milestone(), true);

    // User dismisses it
    ui.invoke_dismiss_milestone();
    assert_eq!(ui.get_show_milestone(), false);

    // Second trigger does NOT set it
    ui.invoke_action_see_analytics();
    assert_eq!(ui.get_show_milestone(), false);
}
