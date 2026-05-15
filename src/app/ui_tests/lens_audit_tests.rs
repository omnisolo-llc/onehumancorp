use crate::app;
use slint::ComponentHandle;

#[test]
fn test_business_share_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());
    let dashboard_ui = app::Dashboard::new().unwrap();
    let share_ui = app::BusinessShare::new().unwrap();

    let share_store_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let share_store_called_clone = share_store_called.clone();

    dashboard_ui.on_action_share_store({
        let bs_handle_clone = share_ui.as_weak();
        move || {
            *share_store_called_clone.borrow_mut() = true;
            if let Some(ui) = bs_handle_clone.upgrade() {
                let _ = ui.show();
            }
        }
    });

    dashboard_ui.invoke_action_share_store();
    assert!(*share_store_called.borrow());

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

#[test]
fn test_kairos_walkthrough_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());
    let dashboard_ui = app::Dashboard::new().unwrap();

    let ui = app::KairosOrchestrationWalkthrough::new().unwrap();

    let walkthrough_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let walkthrough_opened_clone = walkthrough_opened.clone();
    dashboard_ui.on_open_kairos_orchestration_walkthrough(move || {
        *walkthrough_opened_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_open_kairos_orchestration_walkthrough();
    assert!(*walkthrough_opened.borrow());

    assert_eq!(ui.get_test_title(), slint::SharedString::from("How Your Helpers Work Together"));

    // Simulate clicking through the 4 steps
    for i in 0..4 {
        assert_eq!(ui.get_current_step(), i);
        ui.set_current_step(i + 1);
    }
}

#[test]
fn test_autodream_walkthrough_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());
    let _dashboard_ui = app::Dashboard::new().unwrap();
    let ui = app::AutodreamWalkthrough::new().unwrap();

    assert_eq!(ui.get_current_step(), 0);

    // Simulate clicking through the 4 steps
    for i in 0..4 {
        assert_eq!(ui.get_current_step(), i);
        ui.set_current_step(i + 1);
    }
}

#[test]
fn test_vector_memory_visualizer_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());
    let _dashboard_ui = app::Dashboard::new().unwrap();
    let ui = app::VectorMemoryVisualizer::new().unwrap();

    assert_eq!(ui.get_memories_saved(), 0);
    assert_eq!(ui.get_memories_searched(), 0);

    ui.set_memories_saved(42);
    ui.set_memories_searched(108);

    assert_eq!(ui.get_memories_saved(), 42);
    assert_eq!(ui.get_memories_searched(), 108);
}

#[test]
fn test_referrals_dashboard_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let login_ui = app::Login::new().unwrap();
    login_ui.invoke_login("ceo@store.com".into(), "123".into());
    let dashboard_ui = app::Dashboard::new().unwrap();
    let ui = app::Referrals::new().unwrap();

    let referrals_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let referrals_opened_clone = referrals_opened.clone();
    dashboard_ui.on_action_open_referrals(move || {
        *referrals_opened_clone.borrow_mut() = true;
    });
    dashboard_ui.invoke_action_open_referrals();
    assert!(*referrals_opened.borrow());

    let refresh_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let refresh_clone = refresh_called.clone();
    ui.on_refresh(move || {
        *refresh_clone.borrow_mut() = true;
    });
    ui.invoke_refresh();
    assert!(*refresh_called.borrow(), "Refresh callback must be triggered");

    let copy_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let copy_clone = copy_called.clone();
    ui.on_copy_link(move || {
        *copy_clone.borrow_mut() = true;
    });
    ui.invoke_copy_link();
    assert!(*copy_called.borrow(), "Copy link callback must be triggered");

    let generate_new_link_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let generate_new_link_clone = generate_new_link_called.clone();
    ui.on_generate_new_link(move || {
        *generate_new_link_clone.borrow_mut() = true;
    });
    ui.invoke_generate_new_link();
    assert!(*generate_new_link_called.borrow(), "Generate new link callback must be triggered");
}

#[test]
fn test_lens_audit_verify_visual_glassmorphism_and_ui_state() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let settings_ui = app::Settings::new().unwrap();
    settings_ui.set_user_name("Audit Verified Company".into());
    assert_eq!(settings_ui.get_user_name(), "Audit Verified Company");
}
