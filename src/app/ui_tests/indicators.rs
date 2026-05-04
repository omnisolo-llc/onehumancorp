use crate::app;

fn create() -> app::HelperStatusIndicatorWindow { crate::ui_tests::init(); app::HelperStatusIndicatorWindow::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn indicators_flow_active_toggle() {
    let ui = create();
    ui.set_is_active(true);
    assert!(ui.get_is_active());
    ui.set_is_active(false);
    assert!(!ui.get_is_active());
}

#[test] fn indicators_flow_status_logic() {
    let ui = create();
    ui.set_status_text("Error".into());
    ui.set_status_color("red".into());
    assert_eq!(ui.get_status_text(), "Error");
    assert_eq!(ui.get_status_color(), "red");
}

#[test] fn indicators_xss_text() {
    let ui = create();
    let xss = "<script>alert('indicator')</script>";
    ui.set_status_text(xss.into());
    assert_eq!(ui.get_status_text(), xss);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_status_text() {
    let ui = create();
    ui.set_status_text("Working...".into());
    assert_eq!(ui.get_status_text(), "Working...");
    ui.set_status_text("🚀 Deploying".into());
    assert_eq!(ui.get_status_text(), "🚀 Deploying");
    ui.set_status_text("s11".into());
    assert_eq!(ui.get_status_text(), "s11");
}

#[test]
fn create_verify_status_color() {
    let ui = create();
    ui.set_status_color("#00ff00".into());
    assert_eq!(ui.get_status_color(), "#00ff00");
    ui.set_status_color("c21".into());
    assert_eq!(ui.get_status_color(), "c21");
    ui.set_status_color("c22".into());
    assert_eq!(ui.get_status_color(), "c22");
}
