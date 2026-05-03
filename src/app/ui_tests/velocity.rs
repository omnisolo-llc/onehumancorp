use crate::app;

fn create() -> app::SwarmVelocityWindow { crate::ui_tests::init(); app::SwarmVelocityWindow::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn velocity_flow_metrics_sync() {
    let ui = create();
    ui.set_active_threads("128".into());
    ui.set_avg_latency("45ms".into());
    ui.set_completion_rate("99.9%".into());
    assert_eq!(ui.get_active_threads(), "128");
    assert_eq!(ui.get_avg_latency(), "45ms");
    assert_eq!(ui.get_completion_rate(), "99.9%");
}

#[test] fn velocity_xss_latency() {
    let ui = create();
    let xss = "<script>alert('latency')</script>";
    ui.set_avg_latency(xss.into());
    assert_eq!(ui.get_avg_latency(), xss);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_active_threads() {
    let ui = create();
    ui.set_active_threads("10".into());
    assert_eq!(ui.get_active_threads(), "10");
    ui.set_active_threads("t11".into());
    assert_eq!(ui.get_active_threads(), "t11");
    ui.set_active_threads("t12".into());
    assert_eq!(ui.get_active_threads(), "t12");
}

#[test]
fn create_verify_avg_latency() {
    let ui = create();
    ui.set_avg_latency("100ms".into());
    assert_eq!(ui.get_avg_latency(), "100ms");
    ui.set_avg_latency("l21".into());
    assert_eq!(ui.get_avg_latency(), "l21");
    ui.set_avg_latency("l22".into());
    assert_eq!(ui.get_avg_latency(), "l22");
}

#[test]
fn create_verify_completion_rate() {
    let ui = create();
    ui.set_completion_rate("50%".into());
    assert_eq!(ui.get_completion_rate(), "50%");
    ui.set_completion_rate("r41".into());
    assert_eq!(ui.get_completion_rate(), "r41");
    ui.set_completion_rate("r42".into());
    assert_eq!(ui.get_completion_rate(), "r42");
}
