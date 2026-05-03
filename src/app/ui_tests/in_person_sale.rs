use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e_in_person_sale_flow() {
        let _guard = ui_tests::init();
        // 1. Initialize UI component
        let in_person_sale_ui = app::InPersonSale::new().unwrap();

        let close_called = Rc::new(RefCell::new(false));
        let process_payment_called = Rc::new(RefCell::new(false));
        let send_receipt_called = Rc::new(RefCell::new(false));

        let close_called_clone = close_called.clone();
        in_person_sale_ui.on_close(move || {
            *close_called_clone.borrow_mut() = true;
        });

        let process_payment_called_clone = process_payment_called.clone();
        in_person_sale_ui.on_process_payment(move |_| {
            *process_payment_called_clone.borrow_mut() = true;
        });

        let send_receipt_called_clone = send_receipt_called.clone();
        in_person_sale_ui.on_send_receipt(move || {
            *send_receipt_called_clone.borrow_mut() = true;
        });

        // Test Initial State
        assert!(!in_person_sale_ui.get_is_processing());
        assert!(!in_person_sale_ui.get_is_success());
        assert_eq!(in_person_sale_ui.get_amount(), "");

        // User enters an amount
        in_person_sale_ui.set_amount("$150".into());
        assert_eq!(in_person_sale_ui.get_amount(), "$150");

        // User taps "Tap to Pay"
        in_person_sale_ui.invoke_process_payment("$150".into());
        assert!(*process_payment_called.borrow());

        // Simulate processing state
        in_person_sale_ui.set_is_processing(true);
        assert!(in_person_sale_ui.get_is_processing());

        // Simulate successful payment
        in_person_sale_ui.set_is_processing(false);
        in_person_sale_ui.set_is_success(true);
        assert!(!in_person_sale_ui.get_is_processing());
        assert!(in_person_sale_ui.get_is_success());

        // User taps "Email Receipt"
        in_person_sale_ui.invoke_send_receipt();
        assert!(*send_receipt_called.borrow());

        // User taps "Cancel" / "Done"
        in_person_sale_ui.invoke_close();
        assert!(*close_called.borrow());
    }

    #[test]
    fn test_in_person_sale_ui_interaction_from_dashboard() {
        let _guard = ui_tests::init();
        let dashboard_ui = app::Dashboard::new().unwrap();

        let in_person_sale_called = Rc::new(RefCell::new(false));
        let in_person_sale_called_clone = in_person_sale_called.clone();

        dashboard_ui.on_action_in_person_sale(move || {
            *in_person_sale_called_clone.borrow_mut() = true;
        });

        dashboard_ui.invoke_action_in_person_sale();
        assert!(*in_person_sale_called.borrow(), "Dashboard should trigger in person sale action");
    }
}
