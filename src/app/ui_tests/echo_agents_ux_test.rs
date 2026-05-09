use crate::app;
use slint::ComponentHandle;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_mission_agents_hint_initial_state() {
    crate::ui_tests::init();
    let ui = app::Agents::new().unwrap();

    // Verify the hint is hidden initially
    assert!(!ui.get_show_agent_hint());
}

#[test]
fn test_mission_agents_hint_toggled_state() {
    crate::ui_tests::init();
    let ui = app::Agents::new().unwrap();

    // Simulate clicking the "?" button for the agent hint
    ui.set_show_agent_hint(true);
    assert!(ui.get_show_agent_hint());
}

#[test]
fn test_mission_agents_hint_tooltip_registry_interaction() {
    crate::ui_tests::init();
    let ui = app::Agents::new().unwrap();
    let tr = ui.global::<app::TooltipRegistry>();

    // Wire up tooltip registry resolver
    tr.on_request_tooltip_text(|id| { crate::get_tooltip_text(id.as_str()) });

    // Request the tooltip for agent_hint
    let text = tr.invoke_request_tooltip_text("agent_hint".into());
    assert_eq!(text, "See what your team members are doing.");
}

#[test]
fn test_mission_agents_flow_toggle_advanced() {
    crate::ui_tests::init();
    let ui = app::Agents::new().unwrap();

    // Advanced toggle initially false
    assert!(!ui.get_is_advanced());

    // Simulate Advanced Toggle
    ui.invoke_toggle_advanced();
    assert!(ui.get_is_advanced());
}

#[test]
fn test_mission_agents_flow_callbacks() {
    crate::ui_tests::init();
    let ui = app::Agents::new().unwrap();

    let fired = Rc::new(RefCell::new(false));
    let fired_clone = fired.clone();

    ui.on_hire_agent(move || {
        *fired_clone.borrow_mut() = true;
    });

    ui.invoke_hire_agent();
    assert!(*fired.borrow());
}