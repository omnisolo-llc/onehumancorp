use crate::app;

fn create() -> app::GrowBusiness {
    crate::ui_tests::init();
    app::GrowBusiness::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn grow_xss_strategy() {
    let ui = create();
    let xss = "<script>alert('grow')</script>";
    ui.set_selected_strategy(xss.into());
    assert_eq!(ui.get_selected_strategy(), xss);
}

#[test]
fn grow_step_overflow() {
    let ui = create();
    ui.set_step(999);
    assert_eq!(ui.get_step(), 999);
}

#[test]
fn grow_step_underflow() {
    let ui = create();
    ui.set_step(-999);
    assert_eq!(ui.get_step(), -999);
}

// --- Interaction / Flow Tests ---

#[test]
fn grow_flow_retention_switch() {
    let ui = create();
    ui.set_selected_strategy("A".into());
    ui.set_is_advanced(true);
    ui.set_selected_strategy("B".into());
    assert!(ui.get_is_advanced());
}

#[test]
fn grow_flow_step_loop() {
    let ui = create();
    for i in 0..10 {
        ui.set_step(i);
        assert_eq!(ui.get_step(), i);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_selected_strategy() {
    let ui = create();
    ui.set_selected_strategy("Inbound Marketing".into());
    assert_eq!(ui.get_selected_strategy(), "Inbound Marketing");
    ui.set_selected_strategy("Outbound Sales".into());
    assert_eq!(ui.get_selected_strategy(), "Outbound Sales");
    ui.set_selected_strategy("Content Creation".into());
    assert_eq!(ui.get_selected_strategy(), "Content Creation");
}

#[test]
fn create_verify_step() {
    let ui = create();
    ui.set_step(21);
    assert_eq!(ui.get_step(), 21);
    ui.set_step(22);
    assert_eq!(ui.get_step(), 22);
    ui.set_step(23);
    assert_eq!(ui.get_step(), 23);
}
