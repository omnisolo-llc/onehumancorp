use crate::app;
use slint::ComponentHandle;

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

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_my_referral_link, set_my_referral_link, "https://ref.link/xyz");
test_v!(u2, get_total_referrals, set_total_referrals, 5);
test_v!(u3, get_reward_balance, set_reward_balance, "$100");
test_v!(u4, get_bonus_credit, set_bonus_credit, 50);
test_v!(u5, get_waitlist_position, set_waitlist_position, 10);
test_v!(u6, get_download_count, set_download_count, 20);
test_v!(u7, get_viral_coefficient, set_viral_coefficient, 0.5);
test_v!(u8, get_conversion_rate, set_conversion_rate, 12.5);
test_v!(u9, get_my_referral_link, set_my_referral_link, "");
test_v!(u10, get_reward_balance, set_reward_balance, "£10.00");

test_v!(u11, get_my_referral_link, set_my_referral_link, "l11");
test_v!(u12, get_my_referral_link, set_my_referral_link, "l12");
test_v!(u13, get_my_referral_link, set_my_referral_link, "l13");
test_v!(u14, get_my_referral_link, set_my_referral_link, "l14");
test_v!(u15, get_my_referral_link, set_my_referral_link, "l15");
test_v!(u16, get_my_referral_link, set_my_referral_link, "l16");
test_v!(u17, get_my_referral_link, set_my_referral_link, "l17");
test_v!(u18, get_my_referral_link, set_my_referral_link, "l18");
test_v!(u19, get_my_referral_link, set_my_referral_link, "l19");
test_v!(u20, get_my_referral_link, set_my_referral_link, "l20");

test_v!(u21, get_total_referrals, set_total_referrals, 21);
test_v!(u22, get_total_referrals, set_total_referrals, 22);
test_v!(u23, get_total_referrals, set_total_referrals, 23);
test_v!(u24, get_total_referrals, set_total_referrals, 24);
test_v!(u25, get_total_referrals, set_total_referrals, 25);
test_v!(u26, get_click_count, set_click_count, 26);
test_v!(u27, get_click_count, set_click_count, 27);
test_v!(u28, get_click_count, set_click_count, 28);
test_v!(u29, get_click_count, set_click_count, 29);
test_v!(u30, get_click_count, set_click_count, 30);

test_v!(u31, get_conversion_rate, set_conversion_rate, 31.0);
test_v!(u32, get_conversion_rate, set_conversion_rate, 32.0);
test_v!(u33, get_conversion_rate, set_conversion_rate, 33.0);
test_v!(u34, get_conversion_rate, set_conversion_rate, 34.0);
test_v!(u35, get_conversion_rate, set_conversion_rate, 35.0);
test_v!(u36, get_reward_balance, set_reward_balance, "b36");
test_v!(u37, get_reward_balance, set_reward_balance, "b37");
test_v!(u38, get_reward_balance, set_reward_balance, "b38");
test_v!(u39, get_reward_balance, set_reward_balance, "b39");
test_v!(u40, get_reward_balance, set_reward_balance, "b40");

test_v!(u41, get_bonus_credit, set_bonus_credit, 41);
test_v!(u42, get_bonus_credit, set_bonus_credit, 42);
test_v!(u43, get_bonus_credit, set_bonus_credit, 43);
test_v!(u44, get_bonus_credit, set_bonus_credit, 44);
test_v!(u45, get_bonus_credit, set_bonus_credit, 45);
test_v!(u46, get_viral_coefficient, set_viral_coefficient, 46.0);
test_v!(u47, get_viral_coefficient, set_viral_coefficient, 47.0);
test_v!(u48, get_viral_coefficient, set_viral_coefficient, 48.0);
test_v!(u49, get_viral_coefficient, set_viral_coefficient, 49.0);
test_v!(u50, get_viral_coefficient, set_viral_coefficient, 50.0);

test_v!(u51, get_download_count, set_download_count, 51);
test_v!(u52, get_download_count, set_download_count, 52);
test_v!(u53, get_download_count, set_download_count, 53);
test_v!(u54, get_download_count, set_download_count, 54);
test_v!(u55, get_download_count, set_download_count, 55);
test_v!(u56, get_waitlist_position, set_waitlist_position, 56);
test_v!(u57, get_waitlist_position, set_waitlist_position, 57);
test_v!(u58, get_waitlist_position, set_waitlist_position, 58);
test_v!(u59, get_waitlist_position, set_waitlist_position, 59);
test_v!(u60, get_waitlist_position, set_waitlist_position, 60);

test_v!(u61, get_my_referral_link, set_my_referral_link, "l61");
test_v!(u62, get_my_referral_link, set_my_referral_link, "l62");
test_v!(u63, get_my_referral_link, set_my_referral_link, "l63");
test_v!(u64, get_my_referral_link, set_my_referral_link, "l64");
test_v!(u65, get_my_referral_link, set_my_referral_link, "l65");
test_v!(u66, get_my_referral_link, set_my_referral_link, "l66");
test_v!(u67, get_my_referral_link, set_my_referral_link, "l67");
test_v!(u68, get_my_referral_link, set_my_referral_link, "l68");
test_v!(u69, get_my_referral_link, set_my_referral_link, "l69");
test_v!(u70, get_my_referral_link, set_my_referral_link, "l70");

test_v!(u71, get_reward_balance, set_reward_balance, "b71");
test_v!(u72, get_reward_balance, set_reward_balance, "b72");
test_v!(u73, get_reward_balance, set_reward_balance, "b73");
test_v!(u74, get_reward_balance, set_reward_balance, "b74");
test_v!(u75, get_reward_balance, set_reward_balance, "b75");
test_v!(u76, get_total_referrals, set_total_referrals, 76);
test_v!(u77, get_total_referrals, set_total_referrals, 77);
test_v!(u78, get_total_referrals, set_total_referrals, 78);
test_v!(u79, get_total_referrals, set_total_referrals, 79);
test_v!(u80, get_total_referrals, set_total_referrals, 80);
