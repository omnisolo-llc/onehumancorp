use crate::app;

fn create_p() -> app::Pricing { crate::ui_tests::init(); app::Pricing::new().unwrap() }
fn create_m() -> app::MyPlan { crate::ui_tests::init(); app::MyPlan::new().unwrap() }
fn create_c() -> app::CostDashboard { crate::ui_tests::init(); app::CostDashboard::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn price_annual_rapid_toggle() {
    let ui = create_p();
    for _ in 0..50 {
        ui.set_is_annual(true);
        assert!(ui.get_is_annual());
        ui.set_is_annual(false);
        assert!(!ui.get_is_annual());
    }
}

#[test] fn price_tier_xss() {
    let ui = create_m();
    let xss = "<iframe src=javascript:alert(1)>";
    ui.set_tier(xss.into());
    assert_eq!(ui.get_tier(), xss);
}

#[test] fn price_spend_injection() {
    let ui = create_c();
    let inj = "100.00'); UPDATE spend SET amount=0; --";
    ui.set_total_spend(inj.into());
    assert_eq!(ui.get_total_spend(), inj);
}

#[test] fn price_tier_unicode() {
    let ui = create_m();
    let tier = "💎 VIP Elite 💎";
    ui.set_tier(tier.into());
    assert_eq!(ui.get_tier(), tier);
}

// --- Interaction / Flow Tests ---

#[test] fn price_dashboard_spend_update_flow() {
    let ui = create_c();
    let amounts = ["$10.00", "€20.50", "£5.00", "¥1000", "0.00"];
    for a in amounts {
        ui.set_total_spend(a.into());
        assert_eq!(ui.get_total_spend(), a);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_p_verify_is_annual() {
    let ui = create_p();
    ui.set_is_annual(true);
    assert_eq!(ui.get_is_annual(), true);
    ui.set_is_annual(false);
    assert_eq!(ui.get_is_annual(), false);
}

#[test]
fn create_m_verify_tier() {
    let ui = create_m();
    ui.set_tier("Pro".into());
    assert_eq!(ui.get_tier(), "Pro");
    ui.set_tier("Free".into());
    assert_eq!(ui.get_tier(), "Free");
    ui.set_tier("Enterprise".into());
    assert_eq!(ui.get_tier(), "Enterprise");
}

#[test]
fn create_c_verify_total_spend() {
    let ui = create_c();
    ui.set_total_spend("$99.00".into());
    assert_eq!(ui.get_total_spend(), "$99.00");
    ui.set_total_spend("Free".into());
    assert_eq!(ui.get_total_spend(), "Free");
    ui.set_total_spend("N/A".into());
    assert_eq!(ui.get_total_spend(), "N/A");
}

#[test]
fn test_upgrade_prompt_visibility_when_empty() {
    let ui = create_m();
    ui.set_upgrade_prompt_message("".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "");
}

#[test]
fn test_upgrade_prompt_visibility_storage_limit() {
    let ui = create_m();
    ui.set_upgrade_prompt_message("Storage limit exceeded (500MB). Upgrade to Starter.".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "Storage limit exceeded (500MB). Upgrade to Starter.");
}

#[test]
fn test_upgrade_prompt_visibility_agent_limit() {
    let ui = create_m();
    ui.set_upgrade_prompt_message("Agent limit exceeded (1 max). Upgrade to Pro.".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "Agent limit exceeded (1 max). Upgrade to Pro.");
}

#[test]
fn test_upgrade_prompt_visibility_action_limit() {
    let ui = create_m();
    ui.set_upgrade_prompt_message("Monthly action limit reached (100). Upgrade to Starter.".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "Monthly action limit reached (100). Upgrade to Starter.");
}

#[test]
fn test_upgrade_prompt_visibility_clears() {
    let ui = create_m();
    ui.set_upgrade_prompt_message("Some error".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "Some error");
    ui.set_upgrade_prompt_message("".into());
    assert_eq!(ui.get_upgrade_prompt_message(), "");
}
