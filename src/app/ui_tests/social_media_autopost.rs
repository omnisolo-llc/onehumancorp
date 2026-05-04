use crate::app;
use slint::ComponentHandle;

#[test]
fn test_social_media_autopost() {
    crate::ui_tests::init();

    let ui = app::SocialMediaAutoPost::new().unwrap();

    let connect_instagram_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let connect_ig_clone = connect_instagram_called.clone();
    ui.on_connect_instagram(move || {
        *connect_ig_clone.borrow_mut() = true;
    });

    let connect_facebook_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let connect_fb_clone = connect_facebook_called.clone();
    ui.on_connect_facebook(move || {
        *connect_fb_clone.borrow_mut() = true;
    });

    let connect_x_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let connect_x_clone = connect_x_called.clone();
    ui.on_connect_x(move || {
        *connect_x_clone.borrow_mut() = true;
    });

    let close_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let close_clone = close_called.clone();
    ui.on_close(move || {
        *close_clone.borrow_mut() = true;
    });

    let approve_post_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let approve_clone = approve_post_called.clone();
    ui.on_approve_post(move |id| {
        assert_eq!(id, "1");
        *approve_clone.borrow_mut() = true;
    });

    assert_eq!(ui.get_instagram_connected(), true);
    ui.set_instagram_connected(false);
    assert_eq!(ui.get_instagram_connected(), false);

    assert_eq!(ui.get_facebook_connected(), false);
    ui.set_facebook_connected(true);
    assert_eq!(ui.get_facebook_connected(), true);

    ui.invoke_connect_instagram();
    assert!(*connect_instagram_called.borrow());

    ui.invoke_connect_facebook();
    assert!(*connect_facebook_called.borrow());

    ui.invoke_connect_x();
    assert!(*connect_x_called.borrow());

    ui.invoke_close();
    assert!(*close_called.borrow());

    ui.invoke_approve_post("1".into());
    assert!(*approve_post_called.borrow());
}
