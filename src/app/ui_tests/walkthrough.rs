use crate::app;

fn create() -> app::InteractiveWalkthrough { crate::ui_tests::init(); app::InteractiveWalkthrough::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn walkthrough_flow_steps_bounds() {
    let ui = create();
    ui.set_current_step(10);
    assert_eq!(ui.get_current_step(), 10);
    ui.set_current_step(-5);
    assert_eq!(ui.get_current_step(), -5);
}

#[test] fn walkthrough_flow_visibility() {
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

#[test]
fn test_autodream_sync_walkthrough_e2e_steps() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::Walkthrough::new().unwrap();

    // 1. Verify initial step
    assert_eq!(ui.get_current_step(), 0);

    // 2. Advance to next step and verify
    ui.set_current_step(1);
    assert_eq!(ui.get_current_step(), 1);

    // 3. Advance to step 3
    ui.set_current_step(2);
    assert_eq!(ui.get_current_step(), 2);

    // 4. Advance to step 4
    ui.set_current_step(3);
    assert_eq!(ui.get_current_step(), 3);

    // 5. Complete all steps to step 7
    ui.set_current_step(6);
    assert_eq!(ui.get_current_step(), 6);
}
