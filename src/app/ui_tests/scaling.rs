use crate::app;

fn create() -> app::Scaling {
    crate::ui_tests::init();
    app::Scaling::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn scaling_xss_role() {
    let ui = create();
    let xss = "<script>alert('scaling')</script>";
    ui.set_selected_role(xss.into());
    assert_eq!(ui.get_selected_role(), xss);
}

#[test]
fn scaling_count_overflow() {
    let ui = create();
    ui.set_target_count(9999);
    assert_eq!(ui.get_target_count(), 9999);
}

#[test]
fn scaling_count_negative() {
    let ui = create();
    ui.set_target_count(-100);
    assert_eq!(ui.get_target_count(), -100);
}

// --- Interaction / Flow Tests ---

#[test]
fn scaling_flow_callback_trigger() {
    let ui = create();
    let called_role = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let called_count = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c1 = called_role.clone();
    let c2 = called_count.clone();
    ui.on_scale_agents(move |role, count| {
        *c1.borrow_mut() = role.to_string();
        *c2.borrow_mut() = count;
    });

    ui.invoke_scale_agents("DEVOPS".into(), 5);
    assert_eq!(*called_role.borrow(), "DEVOPS");
    assert_eq!(*called_count.borrow(), 5);
}

#[test]
fn scaling_flow_rapid_count_change() {
    let ui = create();
    for i in 1..20 {
        ui.set_target_count(i);
        assert_eq!(ui.get_target_count(), i);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_selected_role() {
    let ui = create();
    ui.set_selected_role("SRE".into());
    assert_eq!(ui.get_selected_role(), "SRE");
    ui.set_selected_role("PRODUCT_OWNER".into());
    assert_eq!(ui.get_selected_role(), "PRODUCT_OWNER");
    ui.set_selected_role("DESIGNER".into());
    assert_eq!(ui.get_selected_role(), "DESIGNER");
}

#[test]
fn create_verify_tarcount() {
    let ui = create();
    ui.set_target_count(21);
    assert_eq!(ui.get_target_count(), 21);
    ui.set_target_count(22);
    assert_eq!(ui.get_target_count(), 22);
    ui.set_target_count(23);
    assert_eq!(ui.get_target_count(), 23);
}
