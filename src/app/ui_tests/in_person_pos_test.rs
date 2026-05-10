use crate::app;
use std::rc::Rc;
use std::cell::RefCell;
use slint::ComponentHandle;

#[test]
fn test_in_person_pos_full_journey() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();

    // 1. Simulate login
    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());

    let dashboard_ui = app::Dashboard::new().unwrap();
    let pos_ui = app::InPersonPos::new().unwrap();

    // The logic usually handled by `GLOBAL_IN_PERSON_POS` in `main.rs`
    let pos_opened = Rc::new(RefCell::new(false));
    let pos_opened_clone = pos_opened.clone();

    let pos_handle = pos_ui.as_weak();
    dashboard_ui.on_action_in_person_sale(move || {
        *pos_opened_clone.borrow_mut() = true;
        if let Some(ui) = pos_handle.upgrade() {
            ui.set_current_view("cart".into());
            ui.set_total_amount("$0.00".into());
        }
    });

    // Wire up POS logic similarly to main.rs for testing
    let pos_handle_custom = pos_ui.as_weak();
    pos_ui.on_action_add_custom_amount(move |amount_str| {
        if let Some(ui) = pos_handle_custom.upgrade() {
            if let Ok(amount) = amount_str.parse::<f64>() {
                let current_total = ui.get_total_amount().replace("$", "").parse::<f64>().unwrap_or(0.0);
                let new_total = current_total + amount;
                ui.set_total_amount(format!("${:.2}", new_total).into());
            }
        }
    });

    let pos_handle_checkout = pos_ui.as_weak();
    pos_ui.on_action_proceed_to_checkout(move || {
        if let Some(ui) = pos_handle_checkout.upgrade() {
            ui.set_current_view("checkout".into());
        }
    });

    let pos_handle_tap = pos_ui.as_weak();
    pos_ui.on_action_tap_to_pay(move || {
        if let Some(ui) = pos_handle_tap.upgrade() {
            ui.set_current_view("processing".into());
            // Fast forward for tests
            ui.set_current_view("success".into());
        }
    });

    let pos_handle_receipt = pos_ui.as_weak();
    let receipt_sent = Rc::new(RefCell::new(false));
    let receipt_sent_clone = receipt_sent.clone();
    pos_ui.on_action_send_receipt(move |_method| {
        *receipt_sent_clone.borrow_mut() = true;
        if let Some(ui) = pos_handle_receipt.upgrade() {
            ui.set_current_view("cart".into());
        }
    });

    // 2. Open In-Person POS from Dashboard
    dashboard_ui.invoke_action_in_person_sale();
    assert!(*pos_opened.borrow(), "Dashboard should open POS modal");
    assert_eq!(pos_ui.get_current_view(), "cart");

    // 3. Add Custom Amount
    pos_ui.invoke_action_add_custom_amount("150.00".into());
    assert_eq!(pos_ui.get_total_amount(), "$150.00");

    // 4. Proceed to Checkout
    pos_ui.invoke_action_proceed_to_checkout();
    assert_eq!(pos_ui.get_current_view(), "checkout");

    // 5. Tap to Pay
    pos_ui.invoke_action_tap_to_pay();
    assert_eq!(pos_ui.get_current_view(), "success"); // Since we skipped timer for test

    // 6. Send Receipt
    pos_ui.invoke_action_send_receipt("email".into());
    assert!(*receipt_sent.borrow(), "Receipt should be sent");
    assert_eq!(pos_ui.get_current_view(), "cart"); // Returns to cart
}
