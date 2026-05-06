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
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();
    let ui_handle = ui.as_weak();

    ui.on_connect_instagram(move || {
        *c.lock().unwrap() = true;
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_is_connected_instagram(true);
        }
    });

    ui.invoke_connect_instagram();
    assert!(*called.lock().unwrap());
    assert_eq!(ui.get_is_connected_instagram(), true);
}

#[test]
fn test_connect_facebook_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();
    let ui_handle = ui.as_weak();

    ui.on_connect_facebook(move || {
        *c.lock().unwrap() = true;
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_is_connected_facebook(true);
        }
    });

    ui.invoke_connect_facebook();
    assert!(*called.lock().unwrap());
    assert_eq!(ui.get_is_connected_facebook(), true);
}

#[test]
fn test_generate_post_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();
    let ui_handle = ui.as_weak();

    ui.on_generate_post(move || {
        *c.lock().unwrap() = true;
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_post_content("Check out our new products!".into());
        }
    });

    ui.invoke_generate_post();
    assert!(*called.lock().unwrap());
    assert_eq!(ui.get_post_content(), "Check out our new products!");
}

#[test]
fn test_schedule_post_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();

    ui.on_schedule_post(move || {
        *c.lock().unwrap() = true;
    });

    ui.invoke_schedule_post();
    assert!(*called.lock().unwrap());
}

#[test]
fn test_approve_post_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();

    ui.on_approve_post(move || {
        *c.lock().unwrap() = true;
    });

    ui.invoke_approve_post();
    assert!(*called.lock().unwrap());
}
