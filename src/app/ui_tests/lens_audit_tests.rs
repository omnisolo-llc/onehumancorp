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
fn test_agent_config_cuj_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::AgentConfig::new().unwrap();
    assert_eq!(ui.get_step(), 0);
    assert_eq!(ui.get_is_advanced(), false);
    let next_step_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let next_step_clone = next_step_called.clone();
    ui.on_next_step(move || {
        *next_step_clone.borrow_mut() = true;
    });
    ui.invoke_next_step();
    assert!(*next_step_called.borrow(), "Next step callback must be triggered");
    let save_state_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let save_state_clone = save_state_called.clone();
    ui.on_save_state(move || {
        *save_state_clone.borrow_mut() = true;
    });
    ui.invoke_save_state();
    assert!(*save_state_called.borrow(), "Save state callback must be triggered");
    ui.set_is_advanced(true);
    assert_eq!(ui.get_is_advanced(), true);
    ui.set_api_scope_override("test_scope".into());
    assert_eq!(ui.get_api_scope_override(), slint::SharedString::from("test_scope"));
    ui.set_cron_override("0 0 * * *".into());
    assert_eq!(ui.get_cron_override(), slint::SharedString::from("0 0 * * *"));
}

#[test]
fn test_agent_config_toggles_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::AgentConfig::new().unwrap();

    // Test initial states
    assert_eq!(ui.get_can_reply(), false);
    assert_eq!(ui.get_can_social(), false);
    assert_eq!(ui.get_can_write_descriptions(), false);
    assert_eq!(ui.get_can_send_updates(), false);

    // Toggle and verify
    ui.set_can_reply(true);
    assert_eq!(ui.get_can_reply(), true);
    ui.set_can_social(true);
    assert_eq!(ui.get_can_social(), true);
    ui.set_can_write_descriptions(true);
    assert_eq!(ui.get_can_write_descriptions(), true);
    ui.set_can_send_updates(true);
    assert_eq!(ui.get_can_send_updates(), true);
}

#[test]
fn test_agent_config_frequency_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::AgentConfig::new().unwrap();

    // Test initial frequency
    assert_eq!(ui.get_frequency_value(), 2.0);

    // Change frequency and verify
    ui.set_frequency_value(0.0);
    assert_eq!(ui.get_frequency_value(), 0.0);

    ui.set_frequency_value(3.0);
    assert_eq!(ui.get_frequency_value(), 3.0);
}

#[test]
fn test_agent_config_selection_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::AgentConfig::new().unwrap();

    // Test initial selection
    assert_eq!(ui.get_selected_agent(), slint::SharedString::from(""));
    assert_eq!(ui.get_selected_agent_display(), slint::SharedString::from(""));

    // Select an agent and verify
    ui.set_selected_agent("Customer Support".into());
    assert_eq!(ui.get_selected_agent(), slint::SharedString::from("Customer Support"));
    ui.set_selected_agent_display("Customer Support".into());
    assert_eq!(ui.get_selected_agent_display(), slint::SharedString::from("Customer Support"));
}

#[test]
fn test_agent_config_activate_lens_audit() {
    crate::ui_tests::init();
    if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
    let ui = app::AgentConfig::new().unwrap();

    let activate_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let activate_clone = activate_called.clone();
    ui.on_activate_agent(move |_, _, _, _, _, _, _, _, _| {
        *activate_clone.borrow_mut() = true;
    });

    // Simulate clicking activate agent (which triggers the callback with all parameters)
    ui.invoke_activate_agent("".into(), false, false, false, false, "".into(), "".into(), "".into(), "".into());
    assert!(*activate_called.borrow(), "Activate agent callback must be triggered");
}
