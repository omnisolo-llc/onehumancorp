
use slint::ModelRc;
use slint::VecModel;
use std::rc::Rc;
use std::cell::RefCell;

use crate::app;

fn create() -> app::BusinessManager {
    crate::ui_tests::init();
    app::BusinessManager::new().unwrap()
}

#[test]
fn test_business_manager_ux_list_view() {
    let app = create();

    // Add dummy products to test touch targets
    let products = Rc::new(VecModel::from(vec![
        app::UiProduct {
            id: "1".into(),
            name: "Test Item".into(),
            type_label: "PHYSICAL".into(),
            price: "10.00".into(),
            inventory_count: 5,
            is_out_of_stock: false,
        }
    ]));
    app.set_products(ModelRc::from(products));

    let edit_clicked = Rc::new(RefCell::new(false));
    let edit_clicked_clone = edit_clicked.clone();
    app.on_action_edit(move |_id| {
        *edit_clicked_clone.borrow_mut() = true;
    });

    let archive_clicked = Rc::new(RefCell::new(false));
    let archive_clicked_clone = archive_clicked.clone();
    app.on_action_archive(move |_id| {
        *archive_clicked_clone.borrow_mut() = true;
    });

    app.invoke_action_edit("1".into());
    assert!(*edit_clicked.borrow(), "Edit action should be triggered");

    app.invoke_action_archive("1".into());
    assert!(*archive_clicked.borrow(), "Archive action should be triggered");
}

#[test]
fn test_business_manager_ux_add_new() {
    let app = create();

    app.invoke_action_add_new();
    // Verify properties change internally instead of catching via on_...
    assert_eq!(app.get_current_view(), "add");
    assert_eq!(app.get_step(), 0);
}

#[test]
fn test_business_manager_ux_select_type() {
    let app = create();
    app.set_current_view("add".into());
    app.set_step(0);

    app.invoke_select_type("PHYSICAL".into());
    assert_eq!(app.get_selected_type(), "PHYSICAL");
}

#[test]
fn test_business_manager_ux_next_step() {
    let app = create();
    app.set_current_view("add".into());
    app.set_step(0);
    app.set_selected_type("PHYSICAL".into());

    app.invoke_next_step();
    assert_eq!(app.get_step(), 1, "Should advance to step 1");
}

#[test]
fn test_business_manager_ux_submit() {
    let app = create();
    app.set_current_view("add".into());
    app.set_step(1);
    app.set_selected_type("PHYSICAL".into());
    app.set_product_name("Test Product".into());
    app.set_product_description("Test Description".into());
    app.set_product_price("99.99".into());

    let submitted = Rc::new(RefCell::new(false));
    let submitted_clone = submitted.clone();
    app.on_submit(move |_type, _name, _desc, _price, _duration, _schedule| {
        *submitted_clone.borrow_mut() = true;
    });

    app.invoke_submit(
        app.get_selected_type(),
        app.get_product_name(),
        app.get_product_description(),
        app.get_product_price(),
        app.get_service_duration(),
        app.get_service_schedule(),
    );

    assert!(*submitted.borrow(), "Submit action should be triggered");
}
