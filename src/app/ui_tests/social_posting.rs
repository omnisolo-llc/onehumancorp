use crate::app;

fn create() -> app::SocialPosting { crate::ui_tests::init(); app::SocialPosting::new().unwrap() }

#[test]
fn test_social_posting_content_update() {
    let ui = create();
    ui.set_post_content("New product launch!".into());
    assert_eq!(ui.get_post_content(), "New product launch!");
}

#[test]
fn test_social_posting_instagram_connection() {
    let ui = create();
    ui.set_is_connected_instagram(true);
    assert!(ui.get_is_connected_instagram());
}

#[test]
fn test_social_posting_facebook_connection() {
    let ui = create();
    ui.set_is_connected_facebook(true);
    assert!(ui.get_is_connected_facebook());
}

#[test]
fn test_social_posting_generate_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_generate_post(move || { *c.borrow_mut() = true; });
    ui.invoke_generate_post();
    assert!(*called.borrow());
}

#[test]
fn test_social_posting_schedule_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_schedule_post(move || { *c.borrow_mut() = true; });
    ui.invoke_schedule_post();
    assert!(*called.borrow());
}
