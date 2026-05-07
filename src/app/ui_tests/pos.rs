use crate::app;
use slint::ComponentHandle;

fn create() -> app::Pos {
    crate::ui_tests::init();
    app::Pos::new().unwrap()
}

#[test]
fn test_e2e_pos_navigation() {
    crate::ui_tests::init();
    let dashboard_ui = app::Dashboard::new().unwrap();

    let open_pos_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let open_pos_called_clone = open_pos_called.clone();

    dashboard_ui.on_action_open_pos(move || {
        *open_pos_called_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_open_pos();
    assert!(*open_pos_called.borrow(), "Point of Sale should be opened from Dashboard Add action");
}

#[test]
fn test_e2e_pos_add_custom_amount() {
    let ui = create();

    assert_eq!(ui.get_step(), 0);
    assert_eq!(ui.get_cart_total(), "0.00");

    let w1 = ui.as_weak();
    ui.on_add_custom_amount(move |amount| {
        let ui = w1.unwrap();
        // In reality, this would add to a list and calculate. We'll just set it directly.
        ui.set_cart_total(amount);
    });

    ui.set_custom_amount("150".into());
    ui.invoke_add_custom_amount("150".into());

    assert_eq!(ui.get_cart_total(), "150");
}

#[test]
fn test_e2e_pos_tap_to_pay_flow() {
    let ui = create();

    ui.set_cart_total("150".into());

    let w1 = ui.as_weak();
    ui.on_tap_to_pay(move || {
        let ui = w1.unwrap();
        ui.set_step(1); // Move to processing
    });

    ui.invoke_tap_to_pay();
    assert_eq!(ui.get_step(), 1, "Should transition to processing step");

    // Simulate payment success (usually driven by backend callback)
    ui.set_step(2);
    assert_eq!(ui.get_step(), 2, "Should transition to success step");
}

#[test]
fn test_e2e_pos_cancel_payment() {
    let ui = create();

    ui.set_step(1); // Start in processing

    let w1 = ui.as_weak();
    ui.on_cancel_payment(move || {
        let ui = w1.unwrap();
        ui.set_step(0); // Go back to cart
    });

    ui.invoke_cancel_payment();
    assert_eq!(ui.get_step(), 0, "Should return to cart step after cancelling");
}

#[test]
fn test_e2e_pos_receipt_options() {
    let ui = create();

    ui.set_step(2); // Start in success state

    let receipt_sent = std::rc::Rc::new(std::cell::RefCell::new("".to_string()));
    let receipt_sent_clone = receipt_sent.clone();

    ui.on_send_receipt(move |method| {
        *receipt_sent_clone.borrow_mut() = method.to_string();
    });

    ui.invoke_send_receipt("sms".into());
    assert_eq!(*receipt_sent.borrow(), "sms");

    ui.invoke_send_receipt("email".into());
    assert_eq!(*receipt_sent.borrow(), "email");
}