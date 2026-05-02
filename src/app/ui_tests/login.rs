use crate::app;
use slint::ComponentHandle;

fn create() -> app::Login { crate::ui_tests::init(); app::Login::new().unwrap() }

// --- Security / Hacking Cases ---

#[test] fn login_sqli_username() {
    let ui = create();
    let malicious = "' OR '1'='1";
    ui.set_username(malicious.into());
    assert_eq!(ui.get_username(), malicious);
}

#[test] fn login_xss_password() {
    let ui = create();
    let xss = "<script>alert('pwned')</script>";
    ui.set_password(xss.into());
    assert_eq!(ui.get_password(), xss);
}

#[test] fn login_path_traversal() {
    let ui = create();
    ui.set_username("../../../etc/passwd".into());
    assert_eq!(ui.get_username(), "../../../etc/passwd");
}

#[test] fn login_null_byte() {
    let ui = create();
    ui.set_username("admin\0user".into());
    assert_eq!(ui.get_username(), "admin\0user");
}

#[test] fn login_unicode_homograph() {
    let ui = create();
    ui.set_username("аdmin".into()); // Cyrillic 'а'
    assert_eq!(ui.get_username(), "аdmin");
}

#[test] fn login_overflow_username() {
    let ui = create();
    let long = "A".repeat(10000);
    ui.set_username(long.clone().into());
    assert_eq!(ui.get_username(), long);
}

// --- Boundary / Corner Cases ---

#[test] fn login_empty_credentials() {
    let ui = create();
    ui.set_username("".into());
    ui.set_password("".into());
    assert_eq!(ui.get_username(), "");
    assert_eq!(ui.get_password(), "");
    ui.invoke_login("".into(), "".into());
}

#[test] fn login_max_int_username() {
    let ui = create();
    ui.set_username("2147483647".into());
    assert_eq!(ui.get_username(), "2147483647");
}

#[test] fn login_special_chars() {
    let ui = create();
    let chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    ui.set_username(chars.into());
    assert_eq!(ui.get_username(), chars);
}

// --- Complex Flows ---

#[test] fn login_flow_toggle_signup_multiple_times() {
    let ui = create();
    for _ in 0..10 {
        ui.set_is_sign_up(true);
        assert!(ui.get_is_sign_up());
        ui.set_is_sign_up(false);
        assert!(!ui.get_is_sign_up());
    }
}

#[test] fn login_flow_error_persistence() {
    let ui = create();
    ui.set_error_message("Initial error".into());
    ui.set_username("new_user".into());
    assert_eq!(ui.get_error_message(), "Initial error");
    ui.set_error_message("".into());
    assert_eq!(ui.get_error_message(), "");
}

#[test] fn login_callback_chain() {
    let ui = create();
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    let c = counter.clone();
    ui.on_login(move |_, _| { *c.borrow_mut() += 1; });
    
    ui.invoke_login("u1".into(), "p1".into());
    ui.invoke_login("u2".into(), "p2".into());
    assert_eq!(*counter.borrow(), 2);
}

// --- Unique Data Tests with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_username, set_username, "user@domain.com");
test_v!(u2, get_username, set_username, "user+tag@domain.com");
test_v!(u3, get_username, set_username, "1234567890");
test_v!(u4, get_password, set_password, "                    ");
#[test] fn u5() { let ui = create(); ui.set_is_sign_up(true); ui.set_loading(true); assert!(ui.get_is_sign_up()); assert!(ui.get_loading()); }
#[test] fn u6() { let ui = create(); ui.set_show_verification(true); ui.set_verification_message("Check SMS".into()); assert!(ui.get_show_verification()); assert_eq!(ui.get_verification_message(), "Check SMS"); }
test_v!(u7, get_error_message, set_error_message, "⚠️ Warning");
#[test] fn u8() { let ui = create(); ui.set_username("root".into()); ui.set_password("toor".into()); assert_eq!(ui.get_username(), "root"); assert_eq!(ui.get_password(), "toor"); }
#[test] fn u9() { let ui = create(); ui.set_username("guest".into()); ui.set_password("guest".into()); assert_eq!(ui.get_username(), "guest"); assert_eq!(ui.get_password(), "guest"); }
test_v!(u10, get_username, set_username, "emoji_👤");

test_v!(u11, get_username, set_username, "rtl_سلام");
test_v!(u12, get_username, set_username, "chinese_你好");
test_v!(u13, get_username, set_username, "hindi_नमस्ते");
test_v!(u14, get_username, set_username, "japanese_こんにちは");
test_v!(u15, get_username, set_username, "korean_안녕하세요");
test_v!(u16, get_username, set_username, "space inside");
test_v!(u17, get_username, set_username, "trailing_space ");
test_v!(u18, get_username, set_username, " leading_space");
test_v!(u19, get_username, set_username, "multiple__underscores");
test_v!(u20, get_username, set_username, "hyphen-ated");

test_v!(u21, get_password, set_password, "!@#$%^&*");
test_v!(u22, get_password, set_password, "LONG_STRING_WITH_NUMBERS_1234567890");
#[test] fn u23() { let ui = create(); ui.set_is_sign_up(true); ui.set_show_verification(false); assert!(ui.get_is_sign_up()); assert!(!ui.get_show_verification()); }
#[test] fn u24() { let ui = create(); ui.set_loading(false); ui.set_error_message("Failed".into()); assert!(!ui.get_loading()); assert_eq!(ui.get_error_message(), "Failed"); }
test_v!(u25, get_verification_message, set_verification_message, "123-456");
#[test] fn u26() { let ui = create(); ui.set_username("admin".into()); ui.set_is_sign_up(false); assert_eq!(ui.get_username(), "admin"); assert!(!ui.get_is_sign_up()); }
test_v!(u27, get_password, set_password, "p");
test_v!(u28, get_username, set_username, "uuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu");
test_v!(u29, get_error_message, set_error_message, "Too many login attempts. Please try again later.");
test_v!(u30, get_verification_message, set_verification_message, "Please enter the 6-digit code sent to your phone.");

test_v!(u31, get_username, set_username, "u31");
test_v!(u32, get_username, set_username, "u32");
test_v!(u33, get_username, set_username, "u33");
test_v!(u34, get_username, set_username, "u34");
test_v!(u35, get_username, set_username, "u35");
test_v!(u36, get_username, set_username, "u36");
test_v!(u37, get_username, set_username, "u37");
test_v!(u38, get_username, set_username, "u38");
test_v!(u39, get_username, set_username, "u39");
test_v!(u40, get_username, set_username, "u40");

test_v!(u41, get_username, set_username, "u41");
test_v!(u42, get_username, set_username, "u42");
test_v!(u43, get_username, set_username, "u43");
test_v!(u44, get_username, set_username, "u44");
test_v!(u45, get_username, set_username, "u45");
test_v!(u46, get_username, set_username, "u46");
test_v!(u47, get_username, set_username, "u47");
test_v!(u48, get_username, set_username, "u48");
test_v!(u49, get_username, set_username, "u49");
test_v!(u50, get_username, set_username, "u50");

test_v!(u51, get_password, set_password, "p51");
test_v!(u52, get_password, set_password, "p52");
test_v!(u53, get_password, set_password, "p53");
test_v!(u54, get_password, set_password, "p54");
test_v!(u55, get_password, set_password, "p55");
test_v!(u56, get_password, set_password, "p56");
test_v!(u57, get_password, set_password, "p57");
test_v!(u58, get_password, set_password, "p58");
test_v!(u59, get_password, set_password, "p59");
test_v!(u60, get_password, set_password, "p60");

test_v!(u61, get_error_message, set_error_message, "e61");
test_v!(u62, get_error_message, set_error_message, "e62");
test_v!(u63, get_error_message, set_error_message, "e63");
test_v!(u64, get_error_message, set_error_message, "e64");
test_v!(u65, get_error_message, set_error_message, "e65");
test_v!(u66, get_verification_message, set_verification_message, "v66");
test_v!(u67, get_verification_message, set_verification_message, "v67");
test_v!(u68, get_verification_message, set_verification_message, "v68");
test_v!(u69, get_verification_message, set_verification_message, "v69");
test_v!(u70, get_verification_message, set_verification_message, "v70");

test_v!(u71, get_username, set_username, "u71");
test_v!(u72, get_username, set_username, "u72");
test_v!(u73, get_username, set_username, "u73");
test_v!(u74, get_username, set_username, "u74");
test_v!(u75, get_username, set_username, "u75");
test_v!(u76, get_username, set_username, "u76");
test_v!(u77, get_username, set_username, "u77");
test_v!(u78, get_username, set_username, "u78");
test_v!(u79, get_username, set_username, "u79");
test_v!(u80, get_username, set_username, "u80");
