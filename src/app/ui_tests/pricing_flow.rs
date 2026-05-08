use crate::app;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_pricing_wizard_flow_and_billing_cycles() {
    crate::ui_tests::init();
    let ui = app::Pricing::new().unwrap();

    // Verify initial state
    assert_eq!(ui.get_step(), 0);
    assert_eq!(ui.get_is_annual(), false); // Default property check

    // Set usage manually
    ui.set_current_usage("500 / 1000 Actions".into());
    assert_eq!(ui.get_current_usage(), "500 / 1000 Actions");

    // Set projected cost manually
    ui.set_projected_cost("$15.00".into());
    assert_eq!(ui.get_projected_cost(), "$15.00");

    // Test billing cycle toggle callback
    let billing_toggled = Rc::new(RefCell::new(false));
    let btn_clone = billing_toggled.clone();
    ui.on_toggle_billing_cycle(move || {
        *btn_clone.borrow_mut() = true;
    });

    ui.invoke_toggle_billing_cycle();
    assert!(*billing_toggled.borrow());

    // Test plan selection callback
    let plan_selected = Rc::new(RefCell::new(String::new()));
    let plan_clone = plan_selected.clone();
    ui.on_select_plan(move |plan| {
        *plan_clone.borrow_mut() = plan.to_string();
    });

    ui.invoke_select_plan("Pro".into());
    assert_eq!(*plan_selected.borrow(), "Pro");
}
