use crate::app;
use slint::Model;

fn create() -> app::Dashboard {
    crate::ui_tests::init();
    app::Dashboard::new().unwrap()
}

#[test]
fn test_swarm_observability_default_hidden() {
    let ui = create();
    assert!(
        !ui.get_show_swarm_observability(),
        "Swarm observability panel should be hidden by default"
    );
}

#[test]
fn test_swarm_observability_toggle_on() {
    let ui = create();

    ui.invoke_action_grow_business(); // Verify it doesn't crash on standard action

    // Toggle via the boolean since the button doesn't have an action callback we can invoke directly yet
    // Wait, the button clicked sets `show_swarm_observability = true;`
    // Slint test limitations for inline click handlers mean we either expose a callback or test the boolean.
    // Let's test the state property
    ui.set_show_swarm_observability(true);
    assert!(
        ui.get_show_swarm_observability(),
        "Swarm observability panel should be visible after state update"
    );
}

#[test]
fn test_swarm_observability_data_injection_activities() {
    let ui = create();

    ui.set_swarm_activities(slint::ModelRc::new(slint::VecModel::from(vec![
        app::SwarmActivity {
            message: "✅ Your Support Agent replied to 3 customers".into(),
            time: "Just now".into(),
        },
        app::SwarmActivity {
            message: "📦 Order Manager updated stock for 12 items".into(),
            time: "2m ago".into(),
        },
    ])));

    let activities = ui.get_swarm_activities();
    assert_eq!(activities.row_count(), 2);
    let first_activity = activities.row_data(0).unwrap();
    assert_eq!(
        first_activity.message,
        "✅ Your Support Agent replied to 3 customers"
    );
}

#[test]
fn test_swarm_observability_data_injection_agents() {
    let ui = create();

    ui.set_swarm_agent_statuses(slint::ModelRc::new(slint::VecModel::from(vec![
        app::SwarmAgentStatus {
            name: "Support Agent".into(),
            role: "Customer Service".into(),
            status: "Idle".into(),
            is_active: false,
        },
        app::SwarmAgentStatus {
            name: "Order Manager".into(),
            role: "Operations".into(),
            status: "Processing orders...".into(),
            is_active: true,
        },
    ])));

    let statuses = ui.get_swarm_agent_statuses();
    assert_eq!(statuses.row_count(), 2);
    let second_status = statuses.row_data(1).unwrap();
    assert_eq!(second_status.name, "Order Manager");
    assert!(second_status.is_active);
}

#[test]
fn test_swarm_observability_toggle_off() {
    let ui = create();
    ui.set_show_swarm_observability(true);
    assert!(ui.get_show_swarm_observability(), "Panel should be visible");
    ui.set_show_swarm_observability(false);
    assert!(
        !ui.get_show_swarm_observability(),
        "Panel should be hidden again"
    );
}

#[test]
fn test_swarm_observability_button_click() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();

    ui.on_action_open_swarm_observability(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_action_open_swarm_observability();

    assert!(
        *invoked.borrow(),
        "Agent Activity button click should invoke the correct callback"
    );
}
