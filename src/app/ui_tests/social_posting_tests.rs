use crate::app;

fn create() -> app::SocialPosting {
    crate::ui_tests::init();
    app::SocialPosting::new().unwrap()
}

// --- Verified Properties ---

#[test]
fn test_social_posting_is_connected_instagram() {
    let ui = create();
    assert_eq!(ui.get_is_connected_instagram(), false);
    ui.set_is_connected_instagram(true);
    assert_eq!(ui.get_is_connected_instagram(), true);
    ui.set_is_connected_instagram(false);
    assert_eq!(ui.get_is_connected_instagram(), false);
}

#[test]
fn test_social_posting_is_connected_facebook() {
    let ui = create();
    assert_eq!(ui.get_is_connected_facebook(), false);
    ui.set_is_connected_facebook(true);
    assert_eq!(ui.get_is_connected_facebook(), true);
    ui.set_is_connected_facebook(false);
    assert_eq!(ui.get_is_connected_facebook(), false);
}

#[test]
fn test_social_posting_post_content() {
    let ui = create();
    assert_eq!(ui.get_post_content(), "");
    ui.set_post_content("Check out our new sale!".into());
    assert_eq!(ui.get_post_content(), "Check out our new sale!");
    ui.set_post_content("Another post.".into());
    assert_eq!(ui.get_post_content(), "Another post.");
}

// --- Verified Callbacks ---

#[test]
fn test_social_posting_connect_instagram() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));

    let invoked_clone = invoked.clone();
    ui.on_connect_instagram(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_connect_instagram();
    assert!(*invoked.borrow());
}

#[test]
fn test_social_posting_connect_facebook() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));

    let invoked_clone = invoked.clone();
    ui.on_connect_facebook(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_connect_facebook();
    assert!(*invoked.borrow());
}

#[test]
fn test_social_posting_generate_post() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));

    let invoked_clone = invoked.clone();
    ui.on_generate_post(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_generate_post();
    assert!(*invoked.borrow());
}

#[test]
fn test_social_posting_schedule_post() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));

    let invoked_clone = invoked.clone();
    ui.on_schedule_post(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_schedule_post();
    assert!(*invoked.borrow());
}

#[test]
fn test_social_posting_approve_post() {
    let ui = create();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));

    let invoked_clone = invoked.clone();
    ui.on_approve_post(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_approve_post();
    assert!(*invoked.borrow());
}
