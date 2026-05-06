use crate::app;

fn create() -> app::Referrals { crate::ui_tests::init(); app::Referrals::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn ref_long_link() {
    let ui = create();
    let long = "https://example.com/".to_string() + &"a".repeat(2000);
    ui.set_my_referral_link(long.clone().into());
    assert_eq!(ui.get_my_referral_link(), long);
}

#[test] fn ref_negative_balance() {
    let ui = create();
    ui.set_reward_balance("-$10.00".into());
    assert_eq!(ui.get_reward_balance(), "-$10.00");
}

#[test] fn ref_max_coefficient() {
    let ui = create();
    ui.set_viral_coefficient(f32::MAX);
    assert_eq!(ui.get_viral_coefficient(), f32::MAX);
}

#[test] fn ref_xss_link() {
    let ui = create();
    let xss = "javascript:alert(1)";
    ui.set_my_referral_link(xss.into());
    assert_eq!(ui.get_my_referral_link(), xss);
}

// --- Interaction / Flow Tests ---

#[test] fn ref_mass_click_update() {
    let ui = create();
    for i in 0..100 {
        ui.set_click_count(i);
        ui.set_conversion_rate(i as f32 * 0.1);
        assert_eq!(ui.get_click_count(), i);
        assert_eq!(ui.get_conversion_rate(), i as f32 * 0.1);
    }
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_my_referral_link() {
    let ui = create();
    ui.set_my_referral_link("https://ref.link/xyz".into());
    assert_eq!(ui.get_my_referral_link(), "https://ref.link/xyz");
    ui.set_my_referral_link("".into());
    assert_eq!(ui.get_my_referral_link(), "");
    ui.set_my_referral_link("l11".into());
    assert_eq!(ui.get_my_referral_link(), "l11");
}

#[test]
fn create_verify_total_referrals() {
    let ui = create();
    ui.set_total_referrals(5);
    assert_eq!(ui.get_total_referrals(), 5);
    ui.set_total_referrals(21);
    assert_eq!(ui.get_total_referrals(), 21);
    ui.set_total_referrals(22);
    assert_eq!(ui.get_total_referrals(), 22);
}

#[test]
fn create_verify_reward_balance() {
    let ui = create();
    ui.set_reward_balance("$100".into());
    assert_eq!(ui.get_reward_balance(), "$100");
    ui.set_reward_balance("£10.00".into());
    assert_eq!(ui.get_reward_balance(), "£10.00");
    ui.set_reward_balance("b36".into());
    assert_eq!(ui.get_reward_balance(), "b36");
}

#[test]
fn create_verify_bonus_credit() {
    let ui = create();
    ui.set_bonus_credit(50);
    assert_eq!(ui.get_bonus_credit(), 50);
    ui.set_bonus_credit(41);
    assert_eq!(ui.get_bonus_credit(), 41);
    ui.set_bonus_credit(42);
    assert_eq!(ui.get_bonus_credit(), 42);
}

#[test]
fn create_verify_waitlist_position() {
    let ui = create();
    ui.set_waitlist_position(10);
    assert_eq!(ui.get_waitlist_position(), 10);
    ui.set_waitlist_position(56);
    assert_eq!(ui.get_waitlist_position(), 56);
    ui.set_waitlist_position(57);
    assert_eq!(ui.get_waitlist_position(), 57);
}

#[test]
fn create_verify_download_count() {
    let ui = create();
    ui.set_download_count(20);
    assert_eq!(ui.get_download_count(), 20);
    ui.set_download_count(51);
    assert_eq!(ui.get_download_count(), 51);
    ui.set_download_count(52);
    assert_eq!(ui.get_download_count(), 52);
}

#[test]
fn create_verify_viral_coefficient() {
    let ui = create();
    ui.set_viral_coefficient(0.5);
    assert_eq!(ui.get_viral_coefficient(), 0.5);
    ui.set_viral_coefficient(46.0);
    assert_eq!(ui.get_viral_coefficient(), 46.0);
    ui.set_viral_coefficient(47.0);
    assert_eq!(ui.get_viral_coefficient(), 47.0);
}

#[test]
fn create_verify_conversion_rate() {
    let ui = create();
    ui.set_conversion_rate(12.5);
    assert_eq!(ui.get_conversion_rate(), 12.5);
    ui.set_conversion_rate(31.0);
    assert_eq!(ui.get_conversion_rate(), 31.0);
    ui.set_conversion_rate(32.0);
    assert_eq!(ui.get_conversion_rate(), 32.0);
}

#[test]
fn create_verify_click_count() {
    let ui = create();
    ui.set_click_count(26);
    assert_eq!(ui.get_click_count(), 26);
    ui.set_click_count(27);
    assert_eq!(ui.get_click_count(), 27);
    ui.set_click_count(28);
    assert_eq!(ui.get_click_count(), 28);
}

#[test]
fn share_flow_send_invite_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();
    ui.on_send_invite_message(move |_| { *c.lock().unwrap() = true; });
    ui.invoke_send_invite_message("test_link".into());
    assert!(*called.lock().unwrap());
}
