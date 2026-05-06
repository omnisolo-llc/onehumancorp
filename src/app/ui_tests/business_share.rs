use crate::app;

fn create() -> app::BusinessShare {
    crate::ui_tests::init();
    app::BusinessShare::new().unwrap()
}

#[test]
fn test_default_values() {
    let ui = create();
    assert_eq!(ui.get_business_name(), "My Awesome Store");
    assert_eq!(ui.get_business_tagline(), "The best place to buy things");
    assert_eq!(ui.get_share_link(), "ohc://share?b=123");
}

#[test]
fn test_set_business_name() {
    let ui = create();
    ui.set_business_name("New Store Name".into());
    assert_eq!(ui.get_business_name(), "New Store Name");
}

#[test]
fn test_set_business_tagline() {
    let ui = create();
    ui.set_business_tagline("New Tagline".into());
    assert_eq!(ui.get_business_tagline(), "New Tagline");
}

#[test]
fn test_set_share_link() {
    let ui = create();
    ui.set_share_link("ohc://share?b=abc".into());
    assert_eq!(ui.get_share_link(), "ohc://share?b=abc");
}

#[test]
fn test_copy_link_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_copy_link(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_copy_link();
    assert!(*called.borrow());
}

#[test]
fn test_share_to_instagram_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();

    ui.on_share_to_instagram(move || {
        *c.borrow_mut() = true;
    });

    ui.invoke_share_to_instagram();
    assert!(*called.borrow());
}
