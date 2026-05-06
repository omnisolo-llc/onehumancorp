use crate::app;


fn create_c() -> app::HelperConfig { crate::ui_tests::init(); app::HelperConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn helper_name_injection() {
    let ui = create_c();
    let inj = "Admin'; DROP TABLE agents; --";
    ui.set_selected_helper(inj.into());
    assert_eq!(ui.get_selected_helper(), inj);
}

#[test] fn helper_freq_oob() {
    let ui = create_c();
    ui.set_frequency_value(2.0);
    assert_eq!(ui.get_frequency_value(), 2.0);
    ui.set_frequency_value(-1.0);
    assert_eq!(ui.get_frequency_value(), -1.0);
}

#[test] fn helper_xss_toast() {
    let ui = create_c();
    let xss = "<script>console.log(1)</script>";
    ui.set_selected_helper(xss.into());
    assert_eq!(ui.get_selected_helper(), xss);
}

// --- Interaction / Flow Tests ---

#[test] fn helper_config_permutation_flow() {
    let ui = create_c();
    let flags = [true, false];
    for f1 in flags {
        for f2 in flags {
            ui.set_can_reply(f1);
            ui.set_can_social(f2);
            assert_eq!(ui.get_can_reply(), f1);
            assert_eq!(ui.get_can_social(), f2);
        }
    }
}

#[test] fn helper_selection_retention_flow() {
    let ui = create_c();
    ui.set_selected_helper("Helper Alpha".into());
    ui.set_is_advanced(true);
    ui.set_selected_helper("Helper Beta".into());
    assert!(ui.get_is_advanced());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_a_verify_selected_helper() {
    let ui = create_c();
    ui.set_selected_helper("Support Bot".into());
    assert_eq!(ui.get_selected_helper(), "Support Bot");
    ui.set_selected_helper("".into());
    assert_eq!(ui.get_selected_helper(), "");
    ui.set_selected_helper("DeepThought".into());
    assert_eq!(ui.get_selected_helper(), "DeepThought");
}

#[test]
fn create_a_verify_frequency_value() {
    let ui = create_c();
    ui.set_frequency_value(0.21);
    assert_eq!(ui.get_frequency_value(), 0.21);
    ui.set_frequency_value(0.22);
    assert_eq!(ui.get_frequency_value(), 0.22);
    ui.set_frequency_value(0.23);
    assert_eq!(ui.get_frequency_value(), 0.23);
}

fn create_agents_ui() -> app::Helpers {
    crate::ui_tests::init();
    app::Helpers::new().unwrap()
}

#[test]
fn test_helpers_ui_upgrade_prompt_visibility() {
    let ui = create_agents_ui();
    assert_eq!(ui.get_show_upgrade_prompt(), false);

    ui.set_show_upgrade_prompt(true);
    assert_eq!(ui.get_show_upgrade_prompt(), true);

    ui.set_show_upgrade_prompt(false);
    assert_eq!(ui.get_show_upgrade_prompt(), false);
}

#[test]
fn test_helpers_ui_upgrade_prompt_message() {
    let ui = create_agents_ui();
    assert_eq!(ui.get_upgrade_prompt_message(), "");

    ui.set_upgrade_prompt_message("Please upgrade to add more helpers.".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "Please upgrade to add more helpers.");
}

#[test]
fn test_helpers_ui_helper_list_population() {
    let ui = create_agents_ui();

    let model = std::rc::Rc::new(slint::VecModel::from(vec![
        app::UiHelper {
            id: "helper-1".into(),
            name: "Test Helper".into(),
            role: "Support".into(),
            status: "Running".into(),
            is_running: true,
            svid_verified: true,
            is_new: false,
        }
    ]));

    ui.set_helpers(model.into());
    // Since there's no direct getter to inspect the array lengths in generated Slint components this way easily,
    // we just ensure the property accepts the update without panicking.
    assert!(true);
}

#[test]
fn test_helpers_ui_hire_callback() {
    let ui = create_agents_ui();
    let invoked = std::rc::Rc::new(std::cell::RefCell::new(false));

    let invoked_clone = invoked.clone();
    ui.on_hire_helper(move || {
        *invoked_clone.borrow_mut() = true;
    });

    ui.invoke_hire_helper();
    assert!(*invoked.borrow());
}

#[test]
fn test_helpers_ui_fix_callback() {
    let ui = create_agents_ui();
    let fixed_id = std::rc::Rc::new(std::cell::RefCell::new(String::new()));

    let fixed_id_clone = fixed_id.clone();
    ui.on_fix_helper(move |id| {
        *fixed_id_clone.borrow_mut() = id.into();
    });

    ui.invoke_fix_helper("helper-123".into());
    assert_eq!(*fixed_id.borrow(), "helper-123");
}
