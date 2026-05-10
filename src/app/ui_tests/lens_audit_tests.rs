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

#[test]
fn test_kairos_walkthrough_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    let ui = app::KairosOrchestrationWalkthrough::new().unwrap();
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

    let ui = app::Referrals::new().unwrap();

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
fn test_padding_layout_fixes_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

    // 1. Business Manager
    let business_manager_ui = app::BusinessManager::new().unwrap();
    assert_eq!(business_manager_ui.get_test_title(), slint::SharedString::from("One Human Corp - Business Manager"));

    // 2. Task List
    let task_list_ui = app::TaskList::new().unwrap();
    // Test instantiation without crashing

    // 3. Unified Inbox
    let unified_inbox_ui = app::UnifiedInbox::new().unwrap();
    // Test instantiation without crashing

    // 4. Website Builder
    let website_builder_ui = app::WebsiteBuilder::new().unwrap();
    // Test instantiation without crashing
}
