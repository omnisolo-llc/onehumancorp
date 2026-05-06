use crate::app;

fn create() -> app::SwarmMemory {
    crate::ui_tests::init();
    app::SwarmMemory::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn memory_xss_activity() {
    let ui = create();
    let xss = "<script>alert('mesh')</script>";
    ui.set_mesh_activity(xss.into());
    assert_eq!(ui.get_mesh_activity(), xss);
}

#[test]
fn memory_velocity_overflow() {
    let ui = create();
    ui.set_velocity_score(2147483647);
    assert_eq!(ui.get_velocity_score(), 2147483647);
}

#[test]
fn memory_velocity_negative() {
    let ui = create();
    ui.set_velocity_score(-999);
    assert_eq!(ui.get_velocity_score(), -999);
}

// --- Interaction / Flow Tests ---

#[test]
fn memory_flow_walkthrough_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_view_walkthrough(move || {
        *c.borrow_mut() = true;
    });
    ui.invoke_view_walkthrough();
    assert!(*called.borrow());
}

#[test]
fn memory_flow_sync_loop() {
    let ui = create();
    for i in 0..50 {
        let act = format!("Activity {}", i);
        ui.set_mesh_activity(act.clone().into());
        ui.set_velocity_score(i);
        assert_eq!(ui.get_mesh_activity(), act);
        assert_eq!(ui.get_velocity_score(), i);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_durable_memory() {
    let ui = create();
    ui.set_durable_memory("Cached State".into());
    assert_eq!(ui.get_durable_memory(), "Cached State");
    ui.set_durable_memory("Cloud Sync Active".into());
    assert_eq!(ui.get_durable_memory(), "Cloud Sync Active");
    ui.set_durable_memory("Offline Mode".into());
    assert_eq!(ui.get_durable_memory(), "Offline Mode");
}

#[test]
fn create_verify_mesh_activity() {
    let ui = create();
    ui.set_mesh_activity("a11".into());
    assert_eq!(ui.get_mesh_activity(), "a11");
    ui.set_mesh_activity("a12".into());
    assert_eq!(ui.get_mesh_activity(), "a12");
    ui.set_mesh_activity("a13".into());
    assert_eq!(ui.get_mesh_activity(), "a13");
}

#[test]
fn create_verify_velocity_score() {
    let ui = create();
    ui.set_velocity_score(21);
    assert_eq!(ui.get_velocity_score(), 21);
    ui.set_velocity_score(22);
    assert_eq!(ui.get_velocity_score(), 22);
    ui.set_velocity_score(23);
    assert_eq!(ui.get_velocity_score(), 23);
}
