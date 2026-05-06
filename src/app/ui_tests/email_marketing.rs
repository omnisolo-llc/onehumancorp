use slint::ComponentHandle;
use crate::app;

fn create() -> app::EmailMarketing {
    crate::ui_tests::init();
    app::EmailMarketing::new().unwrap()
}

#[test]
fn test_default_values() {
    let ui = create();
    assert_eq!(ui.get_total_subscribers(), 150);
    assert_eq!(ui.get_selected_template(), "");
    assert_eq!(ui.get_preview_text(), "");
    assert_eq!(ui.get_emails_sent(), 0);
    assert_eq!(ui.get_open_rate(), "0%");
    assert_eq!(ui.get_status_message(), "");
}

#[test]
fn test_generate_template_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    let ui_handle = ui.as_weak();

    ui.on_generate_template(move |template| {
        *c.borrow_mut() = template.to_string();
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_preview_text(format!("Generated for: {}", template).into());
        }
    });

    ui.set_selected_template("Flash sale".into());
    ui.invoke_generate_template("Flash sale".into());

    assert_eq!(*called.borrow(), "Flash sale");
    assert_eq!(ui.get_preview_text(), "Generated for: Flash sale");
}

#[test]
fn test_send_campaign_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    let ui_handle = ui.as_weak();

    ui.on_send_campaign(move || {
        *c.borrow_mut() = true;
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_emails_sent(150);
            ui.set_open_rate("32%".into());
            ui.set_status_message("Sent!".into());
        }
    });

    ui.invoke_send_campaign();

    assert!(*called.borrow());
    assert_eq!(ui.get_emails_sent(), 150);
    assert_eq!(ui.get_open_rate(), "32%");
    assert_eq!(ui.get_status_message(), "Sent!");
}

#[test]
fn test_close_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_close(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_close();

    assert!(*called.borrow());
}

#[test]
fn test_default_values_extra() {
    let ui = create();
    assert_eq!(ui.get_total_subscribers(), 150);
}
