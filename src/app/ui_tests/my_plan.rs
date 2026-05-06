use crate::app;

fn create() -> app::MyPlan { crate::ui_tests::init(); app::MyPlan::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn myplan_xss_tier() {
    let ui = create();
    let xss = "<script>alert('plan')</script>";
    ui.set_tier(xss.into());
    assert_eq!(ui.get_tier(), xss);
}

#[test] fn myplan_injection_actions() {
    let ui = create();
    let inj = "1000'); DROP TABLE actions; --";
    ui.set_total_actions(inj.into());
    assert_eq!(ui.get_total_actions(), inj);
}

#[test] fn myplan_long_date() {
    let ui = create();
    let long = "Date ".repeat(200);
    ui.set_renewal_date(long.clone().into());
    assert_eq!(ui.get_renewal_date(), long);
}

// --- Interaction / Flow Tests ---

#[test] fn myplan_flow_upgrade_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_upgrade(move || { *c.borrow_mut() = true; });
    ui.invoke_upgrade();
    assert!(*called.borrow());
}

#[test] fn myplan_flow_cancel_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_cancel_subscription(move || { *c.borrow_mut() = true; });
    ui.invoke_cancel_subscription();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_tier() {
    let ui = create();
    ui.set_tier("Enterprise".into());
    assert_eq!(ui.get_tier(), "Enterprise");
    ui.set_tier("t11".into());
    assert_eq!(ui.get_tier(), "t11");
    ui.set_tier("t12".into());
    assert_eq!(ui.get_tier(), "t12");
}

#[test]
fn create_verify_plan_status() {
    let ui = create();
    ui.set_plan_status("Past Due".into());
    assert_eq!(ui.get_plan_status(), "Past Due");
    ui.set_plan_status("s71".into());
    assert_eq!(ui.get_plan_status(), "s71");
    ui.set_plan_status("s72".into());
    assert_eq!(ui.get_plan_status(), "s72");
}

#[test]
fn create_verify_estimated_bill() {
    let ui = create();
    ui.set_estimated_bill("$0.00".into());
    assert_eq!(ui.get_estimated_bill(), "$0.00");
    ui.set_estimated_bill("b51".into());
    assert_eq!(ui.get_estimated_bill(), "b51");
    ui.set_estimated_bill("b52".into());
    assert_eq!(ui.get_estimated_bill(), "b52");
}

#[test]
fn create_verify_total_actions() {
    let ui = create();
    ui.set_total_actions("21".into());
    assert_eq!(ui.get_total_actions(), "21");
    ui.set_total_actions("22".into());
    assert_eq!(ui.get_total_actions(), "22");
    ui.set_total_actions("23".into());
    assert_eq!(ui.get_total_actions(), "23");
}

#[test]
fn create_verify_used_storage() {
    let ui = create();
    ui.set_used_storage("u41".into());
    assert_eq!(ui.get_used_storage(), "u41");
    ui.set_used_storage("u42".into());
    assert_eq!(ui.get_used_storage(), "u42");
    ui.set_used_storage("u43".into());
    assert_eq!(ui.get_used_storage(), "u43");
}

#[test]
fn create_verify_renewal_date() {
    let ui = create();
    ui.set_renewal_date("d61".into());
    assert_eq!(ui.get_renewal_date(), "d61");
    ui.set_renewal_date("d62".into());
    assert_eq!(ui.get_renewal_date(), "d62");
    ui.set_renewal_date("d63".into());
    assert_eq!(ui.get_renewal_date(), "d63");
}
