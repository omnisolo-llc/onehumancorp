use slint::ComponentHandle;
use crate::app;

fn create() -> app::SocialPosting {
    crate::ui_tests::init();
    app::SocialPosting::new().unwrap()
}

#[test]
fn test_default_values() {
    let ui = create();
    assert_eq!(ui.get_post_content(), "");
    assert_eq!(ui.get_is_connected_instagram(), false);
    assert_eq!(ui.get_is_connected_facebook(), false);
}

#[test]
fn test_connect_instagram_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_connect_instagram(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_connect_instagram();
    assert!(*called.borrow());
}

#[test]
fn test_connect_facebook_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_connect_facebook(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_connect_facebook();
    assert!(*called.borrow());
}

#[test]
fn test_generate_post_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_generate_post(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_generate_post();
    assert!(*called.borrow());
}

#[test]
fn test_schedule_post_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_schedule_post(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_schedule_post();
    assert!(*called.borrow());
}

#[test]
fn test_approve_post_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_approve_post(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_approve_post();
    assert!(*called.borrow());
}

#[test]
fn test_post_content_update() {
    let ui = create();
    ui.set_post_content("Check out our new products!".into());
    assert_eq!(ui.get_post_content(), "Check out our new products!");
}
