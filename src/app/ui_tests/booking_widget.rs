use std::rc::Rc;
use std::cell::RefCell;
use slint::Model;
use crate::app;

fn create() -> app::TestBooking {
    crate::ui_tests::init();
    app::TestBooking::new().unwrap()
}

#[test]
fn test_booking_widget_select_date() {
    let ui = create();
    let called = Rc::new(RefCell::new(false));
    let called_clone = called.clone();

    ui.on_fetch_availability(move || {
        *called_clone.borrow_mut() = true;
    });

    ui.invoke_select_date("Tue".into());

    assert_eq!(ui.get_selected_date(), "Tue");
    assert!(*called.borrow(), "fetch_availability callback should be triggered when date is selected");
}

#[test]
fn test_booking_widget_available_slots() {
    let ui = create();

    let slots = slint::ModelRc::from(Rc::new(slint::VecModel::from(vec![
        slint::SharedString::from("10:00 AM"),
        slint::SharedString::from("11:30 AM"),
    ])));

    ui.set_available_slots(slots.clone());

    assert_eq!(ui.get_available_slots().row_count(), 2);
    assert_eq!(ui.get_available_slots().row_data(0).unwrap(), "10:00 AM");
}

#[test]
fn test_booking_widget_select_time() {
    let ui = create();

    ui.invoke_select_time("10:00 AM".into());

    assert_eq!(ui.get_selected_time(), "10:00 AM");
}

#[test]
fn test_booking_widget_book_slot() {
    let ui = create();

    assert_eq!(ui.get_is_booked(), false);

    ui.invoke_book_slot();

    assert_eq!(ui.get_is_booked(), true);
}

#[test]
fn test_booking_widget_full_flow() {
    let ui = create();
    let called = Rc::new(RefCell::new(false));
    let called_clone = called.clone();

    ui.on_fetch_availability(move || {
        *called_clone.borrow_mut() = true;
    });

    // 1. Select Date
    ui.invoke_select_date("Wed".into());
    assert_eq!(ui.get_selected_date(), "Wed");
    assert!(*called.borrow());

    // 2. Slots arrive
    let slots = slint::ModelRc::from(Rc::new(slint::VecModel::from(vec![
        slint::SharedString::from("09:00 AM"),
        slint::SharedString::from("01:00 PM"),
    ])));
    ui.set_available_slots(slots);

    // 3. Select Time
    ui.invoke_select_time("01:00 PM".into());
    assert_eq!(ui.get_selected_time(), "01:00 PM");

    // 4. Confirm Booking
    ui.invoke_book_slot();
    assert_eq!(ui.get_is_booked(), true);
}
