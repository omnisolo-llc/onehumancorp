use crate::app;


fn create_c() -> app::AgentConfig { crate::ui_tests::init(); app::AgentConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn agent_name_injection() {
    let ui = create_c();
    let inj = "Admin'; DROP TABLE agents; --";
    ui.set_selected_helper(inj.into());
    assert_eq!(ui.get_selected_helper(), inj);
}

#[test] fn agent_freq_oob() {
    let ui = create_c();
    ui.set_frequency_value(2.0);
    assert_eq!(ui.get_frequency_value(), 2.0);
    ui.set_frequency_value(-1.0);
    assert_eq!(ui.get_frequency_value(), -1.0);
}

#[test] fn agent_xss_toast() {
    let ui = create_c();
    let xss = "<script>console.log(1)</script>";
    ui.set_selected_helper(xss.into());
    assert_eq!(ui.get_selected_helper(), xss);
}

// --- Interaction / Flow Tests ---

#[test] fn agent_config_permutation_flow() {
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

#[test] fn agent_selection_retention_flow() {
    let ui = create_c();
    ui.set_selected_helper("Agent Alpha".into());
    ui.set_is_advanced(true);
    ui.set_selected_helper("Agent Beta".into());
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
