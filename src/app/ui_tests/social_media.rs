use crate::app;

#[test]
fn test_social_media_creation() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let _ = app::SocialMedia::new();
}

#[test]
fn test_social_media_callbacks() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::SocialMedia::new().unwrap();

    let connect_ig_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let connect_ig_clone = connect_ig_called.clone();
    ui.on_connect_instagram(move || *connect_ig_clone.borrow_mut() = true);

    let connect_fb_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let connect_fb_clone = connect_fb_called.clone();
    ui.on_connect_facebook(move || *connect_fb_clone.borrow_mut() = true);

    let connect_x_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let connect_x_clone = connect_x_called.clone();
    ui.on_connect_x(move || *connect_x_clone.borrow_mut() = true);

    let generate_posts_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let generate_posts_clone = generate_posts_called.clone();
    ui.on_generate_posts(move || *generate_posts_clone.borrow_mut() = true);

    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_clone = close_called.clone();
    ui.on_close(move || *close_clone.borrow_mut() = true);

    ui.invoke_connect_instagram();
    assert!(*connect_ig_called.borrow());

    ui.invoke_connect_facebook();
    assert!(*connect_fb_called.borrow());

    ui.invoke_connect_x();
    assert!(*connect_x_called.borrow());

    ui.invoke_generate_posts();
    assert!(*generate_posts_called.borrow());

    ui.invoke_close();
    assert!(*close_called.borrow());
}

#[test]
fn test_social_media_properties() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::SocialMedia::new().unwrap();

    ui.set_is_instagram_connected(true);
    assert!(ui.get_is_instagram_connected());

    ui.set_is_facebook_connected(true);
    assert!(ui.get_is_facebook_connected());

    ui.set_is_x_connected(true);
    assert!(ui.get_is_x_connected());

    ui.set_status_message("Generated!".into());
    assert_eq!(ui.get_status_message(), "Generated!");
}
