use crate::app;

fn create() -> app::Dashboard { crate::ui_tests::init(); app::Dashboard::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn dash_negative_orders() {
    let ui = create();
    ui.set_new_orders_count(-1);
    assert_eq!(ui.get_new_orders_count(), -1);
}

#[test] fn dash_overflow_helpers() {
    let ui = create();
    ui.set_active_helpers_count(2147483647);
    assert_eq!(ui.get_active_helpers_count(), 2147483647);
}

#[test] fn dash_xss_milestone_title() {
    let ui = create();
    let xss = "<svg/onload=alert(1)>";
    ui.set_milestone_title(xss.into());
    assert_eq!(ui.get_milestone_title(), xss);
}

#[test] fn dash_currency_injection() {
    let ui = create();
    let val = "$9,999,999.99'; DROP TABLE sales; --";
    ui.set_todays_sales(val.into());
    assert_eq!(ui.get_todays_sales(), val);
}

// --- Interaction / Logic Flows ---

#[test] fn dash_milestone_visibility_flow() {
    let ui = create();
    ui.set_show_milestone(false);
    ui.set_milestone_title("Hidden".into());
    assert!(!ui.get_show_milestone());
    ui.set_show_milestone(true);
    assert_eq!(ui.get_milestone_title(), "Hidden");
}

#[test] fn dash_mass_property_update() {
    let ui = create();
    for i in 0..100 {
        ui.set_new_orders_count(i);
        ui.set_active_helpers_count(i * 2);
        assert_eq!(ui.get_new_orders_count(), i);
        assert_eq!(ui.get_active_helpers_count(), i * 2);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_todays_sales() {
    let ui = create();
    ui.set_todays_sales("FREE".into());
    assert_eq!(ui.get_todays_sales(), "FREE");
    ui.set_todays_sales("N/A".into());
    assert_eq!(ui.get_todays_sales(), "N/A");
    ui.set_todays_sales("0.00 €".into());
    assert_eq!(ui.get_todays_sales(), "0.00 €");
}

#[test]
fn create_verify_milestone_message() {
    let ui = create();
    ui.set_milestone_message("First Order!".into());
    assert_eq!(ui.get_milestone_message(), "First Order!");
    ui.set_milestone_message("mm41".into());
    assert_eq!(ui.get_milestone_message(), "mm41");
    ui.set_milestone_message("mm42".into());
    assert_eq!(ui.get_milestone_message(), "mm42");
}

#[test]
fn create_verify_milestone_title() {
    let ui = create();
    ui.set_milestone_title("🏆 Achievement".into());
    assert_eq!(ui.get_milestone_title(), "🏆 Achievement");
    ui.set_milestone_title("mt36".into());
    assert_eq!(ui.get_milestone_title(), "mt36");
    ui.set_milestone_title("mt37".into());
    assert_eq!(ui.get_milestone_title(), "mt37");
}

#[test]
fn create_verify_new_orders_count() {
    let ui = create();
    ui.set_new_orders_count(21);
    assert_eq!(ui.get_new_orders_count(), 21);
    ui.set_new_orders_count(22);
    assert_eq!(ui.get_new_orders_count(), 22);
    ui.set_new_orders_count(23);
    assert_eq!(ui.get_new_orders_count(), 23);
}

#[test]
fn create_verify_active_helpers_count() {
    let ui = create();
    ui.set_active_helpers_count(26);
    assert_eq!(ui.get_active_helpers_count(), 26);
    ui.set_active_helpers_count(27);
    assert_eq!(ui.get_active_helpers_count(), 27);
    ui.set_active_helpers_count(28);
    assert_eq!(ui.get_active_helpers_count(), 28);
}

#[test]
fn create_verify_tasks_in_progress_count() {
    let ui = create();
    ui.set_tasks_in_progress_count(31);
    assert_eq!(ui.get_tasks_in_progress_count(), 31);
    ui.set_tasks_in_progress_count(32);
    assert_eq!(ui.get_tasks_in_progress_count(), 32);
    ui.set_tasks_in_progress_count(33);
    assert_eq!(ui.get_tasks_in_progress_count(), 33);
}

#[test]
fn dash_open_billing_flow() {
    let ui = create();
    let billing_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let billing_opened_clone = billing_opened.clone();
    ui.on_open_billing(move || {
        *billing_opened_clone.borrow_mut() = true;
    });
    ui.invoke_open_billing();
    assert!(*billing_opened.borrow(), "Billing should be opened from Dashboard");
}
