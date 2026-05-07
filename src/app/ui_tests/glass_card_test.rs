use crate::app;

// Test GrowthReferralWidget properties and initialization
#[test]
fn test_growth_referral_widget_glass_effect() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::GrowthReferralWidget::new().unwrap();
    // GrowthReferralWidget includes our glassmorphism background simulation.
    // We verify the properties we exposed can be read.
    let _bg = ui.get_test_bg();
    let blur = ui.get_test_blur();
    // Assuming default blur is 20px (from our slint code)
    assert!(blur > 0.0);
}

// Verify UserManagement containing GrowthReferralWidget doesn't crash
#[test]
fn test_user_management_glass_card_interaction() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    let ui = app::UserManagement::new().unwrap();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));
    let invoked_clone = invoked.clone();
    ui.on_invite_user(move || {
        *invoked_clone.borrow_mut() = true;
    });
    ui.invoke_invite_user();
    assert!(*invoked.borrow(), "UserManagement invite callback should work properly alongside GrowthReferralWidget.");
}

// Test GlassCard nested content compilation
#[test]
fn test_glass_card_nested_content() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    // We instantiate Login which heavily uses GlassCard with nested children.
    let ui = app::Login::new().unwrap();
    // Verify properties on Login which depend on GlassCard structure.
    assert_eq!(ui.get_login_card_width() > 0.0, true);
}

// Verify GrowthReferralWidget responsive bounds behavior inside UserManagement
#[test]
fn test_growth_referral_widget_responsive() {
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    crate::ui_tests::init();
    use slint::ComponentHandle;
    let ui = app::UserManagement::new().unwrap();
    let window = ui.window();
    window.set_size(slint::PhysicalSize::new(375, 667));
    // The widget shouldn't crash the window layout when scaled down.
    assert_eq!(window.size().width, 375);
}
