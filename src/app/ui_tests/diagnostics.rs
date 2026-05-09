use crate::app;

fn create() -> app::Diagnostics {
    crate::ui_tests::init();
    app::Diagnostics::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn diag_xss_db_status() {
    let ui = create();
    let xss = "<script>alert('db')</script>";
    ui.set_db_status(xss.into());
    assert_eq!(ui.get_db_status(), xss);
}

#[test]
fn diag_stuck_missions_overflow() {
    let ui = create();
    ui.set_stuck_missions(2147483647);
    assert_eq!(ui.get_stuck_missions(), 2147483647);
}

#[test]
fn diag_stuck_missions_negative() {
    let ui = create();
    ui.set_stuck_missions(-100);
    assert_eq!(ui.get_stuck_missions(), -100);
}

// --- Interaction / Flow Tests ---

#[test]
fn diag_flow_run_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_run_diagnostics(move || {
        *c.borrow_mut() = true;
    });
    ui.invoke_run_diagnostics();
    assert!(*called.borrow());
}

#[test]
fn diag_flow_status_update_loop() {
    let ui = create();
    let statuses = ["Connected", "Disconnected", "Error", "Timeout", "Retrying"];
    for s in statuses {
        ui.set_db_status(s.into());
        ui.set_cache_status(s.into());
        assert_eq!(ui.get_db_status(), s);
        assert_eq!(ui.get_cache_status(), s);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_execution_mode() {
    let ui = create();
    ui.set_execution_mode("CLUSTER".into());
    assert_eq!(ui.get_execution_mode(), "CLUSTER");
    ui.set_execution_mode("LOCAL".into());
    assert_eq!(ui.get_execution_mode(), "LOCAL");
    ui.set_execution_mode("HYBRID".into());
    assert_eq!(ui.get_execution_mode(), "HYBRID");
}

#[test]
fn create_verify_cloud_connectivity() {
    let ui = create();
    ui.set_cloud_connectivity("DISCONNECTED".into());
    assert_eq!(ui.get_cloud_connectivity(), "DISCONNECTED");
    ui.set_cloud_connectivity("s46".into());
    assert_eq!(ui.get_cloud_connectivity(), "s46");
    ui.set_cloud_connectivity("s47".into());
    assert_eq!(ui.get_cloud_connectivity(), "s47");
}

#[test]
fn create_verify_mesh_status() {
    let ui = create();
    ui.set_mesh_status("INACTIVE".into());
    assert_eq!(ui.get_mesh_status(), "INACTIVE");
    ui.set_mesh_status("s36".into());
    assert_eq!(ui.get_mesh_status(), "s36");
    ui.set_mesh_status("s37".into());
    assert_eq!(ui.get_mesh_status(), "s37");
}

#[test]
fn create_verify_db_status() {
    let ui = create();
    ui.set_db_status("Warning: Slow queries detected".into());
    assert_eq!(ui.get_db_status(), "Warning: Slow queries detected");
    ui.set_db_status("s11".into());
    assert_eq!(ui.get_db_status(), "s11");
    ui.set_db_status("s12".into());
    assert_eq!(ui.get_db_status(), "s12");
}

#[test]
fn create_verify_cache_status() {
    let ui = create();
    ui.set_cache_status("Ready".into());
    assert_eq!(ui.get_cache_status(), "Ready");
}

#[test]
fn create_verify_ai_status() {
    let ui = create();
    ui.set_ai_status("Maintenance".into());
    assert_eq!(ui.get_ai_status(), "Maintenance");
    ui.set_ai_status("s31".into());
    assert_eq!(ui.get_ai_status(), "s31");
    ui.set_ai_status("s32".into());
    assert_eq!(ui.get_ai_status(), "s32");
}

#[test]
fn create_verify_stuck_missions() {
    let ui = create();
    ui.set_stuck_missions(21);
    assert_eq!(ui.get_stuck_missions(), 21);
    ui.set_stuck_missions(22);
    assert_eq!(ui.get_stuck_missions(), 22);
    ui.set_stuck_missions(23);
    assert_eq!(ui.get_stuck_missions(), 23);
}
