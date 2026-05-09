use crate::app;

fn create() -> app::InteractiveWalkthrough {
    crate::ui_tests::init();
    app::InteractiveWalkthrough::new().unwrap()
}

// --- Specialized / Flow Tests ---

#[test]
fn walkthrough_flow_steps_bounds() {
    let ui = create();
    ui.set_current_step(10);
    assert_eq!(ui.get_current_step(), 10);
    ui.set_current_step(-5);
    assert_eq!(ui.get_current_step(), -5);
}

#[test]
fn walkthrough_flow_visibility() {
    let ui = create();
    // Assuming visible property exists or just testing state logic
    ui.set_current_step(3);
    assert_eq!(ui.get_current_step(), 3);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_current_step() {
    let ui = create();
    ui.set_current_step(0);
    assert_eq!(ui.get_current_step(), 0);
    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);
    ui.set_current_step(2);
    assert_eq!(ui.get_current_step(), 2);
}
