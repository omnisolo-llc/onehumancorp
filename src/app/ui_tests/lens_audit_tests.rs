use crate::app;


#[test]
fn test_business_share_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let share_ui = app::BusinessShare::new().unwrap();

    // Assert visual truth / token truth: test_title exists and matches
    assert_eq!(share_ui.get_test_title(), slint::SharedString::from("Share my business"));

    let copy_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let copy_clone = copy_called.clone();
    share_ui.on_copy_link(move || {
        *copy_clone.borrow_mut() = true;
    });

    share_ui.invoke_copy_link();
    assert!(*copy_called.borrow(), "Copy link callback must be triggered");

    let ig_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ig_clone = ig_called.clone();
    share_ui.on_share_to_instagram(move || {
        *ig_clone.borrow_mut() = true;
    });
    share_ui.invoke_share_to_instagram();
    assert!(*ig_called.borrow(), "Instagram callback must be triggered");

    let x_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let x_clone = x_called.clone();
    share_ui.on_share_to_x(move || {
        *x_clone.borrow_mut() = true;
    });
    share_ui.invoke_share_to_x();
    assert!(*x_called.borrow(), "X callback must be triggered");

    let wa_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let wa_clone = wa_called.clone();
    share_ui.on_share_to_whatsapp(move || {
        *wa_clone.borrow_mut() = true;
    });
    share_ui.invoke_share_to_whatsapp();
    assert!(*wa_called.borrow(), "WhatsApp callback must be triggered");

    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_clone = close_called.clone();
    share_ui.on_close(move || {
        *close_clone.borrow_mut() = true;
    });
    share_ui.invoke_close();
    assert!(*close_called.borrow(), "Close callback must be triggered");
}
