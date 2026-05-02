use crate::app;
use slint::ComponentHandle;

fn create() -> app::ApiDocs { crate::ui_tests::init(); app::ApiDocs::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn docs_flow_config_sync() {
    let ui = create();
    ui.set_api_key("sk_test_123".into());
    ui.set_endpoint_url("http://localhost:8080".into());
    assert_eq!(ui.get_api_key(), "sk_test_123");
    assert_eq!(ui.get_endpoint_url(), "http://localhost:8080");
}

#[test] fn docs_xss_key() {
    let ui = create();
    let xss = "<script>alert('api_key')</script>";
    ui.set_api_key(xss.into());
    assert_eq!(ui.get_api_key(), xss);
}

#[test] fn docs_long_endpoint() {
    let ui = create();
    let long = "https://".to_string() + &"a".repeat(1000) + ".com";
    ui.set_endpoint_url(long.clone().into());
    assert_eq!(ui.get_endpoint_url(), long);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_api_key, set_api_key, "sk_live_555");
test_v!(u2, get_endpoint_url, set_endpoint_url, "https://api.v2.ohc.io");
test_v!(u3, get_api_key, set_api_key, "sk_sandbox_xyz");

test_v!(u11, get_api_key, set_api_key, "k11");
test_v!(u12, get_api_key, set_api_key, "k12");
test_v!(u13, get_api_key, set_api_key, "k13");
test_v!(u14, get_api_key, set_api_key, "k14");
test_v!(u15, get_api_key, set_api_key, "k15");
test_v!(u16, get_api_key, set_api_key, "k16");
test_v!(u17, get_api_key, set_api_key, "k17");
test_v!(u18, get_api_key, set_api_key, "k18");
test_v!(u19, get_api_key, set_api_key, "k19");
test_v!(u20, get_api_key, set_api_key, "k20");

test_v!(u21, get_endpoint_url, set_endpoint_url, "e21");
test_v!(u22, get_endpoint_url, set_endpoint_url, "e22");
test_v!(u23, get_endpoint_url, set_endpoint_url, "e23");
test_v!(u24, get_endpoint_url, set_endpoint_url, "e24");
test_v!(u25, get_endpoint_url, set_endpoint_url, "e25");
test_v!(u26, get_endpoint_url, set_endpoint_url, "e26");
test_v!(u27, get_endpoint_url, set_endpoint_url, "e27");
test_v!(u28, get_endpoint_url, set_endpoint_url, "e28");
test_v!(u29, get_endpoint_url, set_endpoint_url, "e29");
test_v!(u30, get_endpoint_url, set_endpoint_url, "e30");

test_v!(u31, get_api_key, set_api_key, "Key with 🔑 Emoji");
test_v!(u32, get_api_key, set_api_key, "Key'Quotes'");
test_v!(u33, get_api_key, set_api_key, "Key ; Semi");
test_v!(u34, get_api_key, set_api_key, "");
test_v!(u35, get_api_key, set_api_key, "Very Long API Key ".repeat(5));

test_v!(u41, get_endpoint_url, set_endpoint_url, "u41");
test_v!(u42, get_endpoint_url, set_endpoint_url, "u42");
test_v!(u43, get_endpoint_url, set_endpoint_url, "u43");
test_v!(u44, get_endpoint_url, set_endpoint_url, "u44");
test_v!(u45, get_endpoint_url, set_endpoint_url, "u45");
test_v!(u46, get_endpoint_url, set_endpoint_url, "u46");
test_v!(u47, get_endpoint_url, set_endpoint_url, "u47");
test_v!(u48, get_endpoint_url, set_endpoint_url, "u48");
test_v!(u49, get_endpoint_url, set_endpoint_url, "u49");
test_v!(u50, get_endpoint_url, set_endpoint_url, "u50");

test_v!(u51, get_api_key, set_api_key, "k51");
test_v!(u52, get_api_key, set_api_key, "k52");
test_v!(u53, get_api_key, set_api_key, "k53");
test_v!(u54, get_api_key, set_api_key, "k54");
test_v!(u55, get_api_key, set_api_key, "k55");
test_v!(u56, get_api_key, set_api_key, "k56");
test_v!(u57, get_api_key, set_api_key, "k57");
test_v!(u58, get_api_key, set_api_key, "k58");
test_v!(u59, get_api_key, set_api_key, "k59");
test_v!(u60, get_api_key, set_api_key, "k60");

test_v!(u61, get_endpoint_url, set_endpoint_url, "u61");
test_v!(u62, get_endpoint_url, set_endpoint_url, "u62");
test_v!(u63, get_endpoint_url, set_endpoint_url, "u63");
test_v!(u64, get_endpoint_url, set_endpoint_url, "u64");
test_v!(u65, get_endpoint_url, set_endpoint_url, "u65");
test_v!(u66, get_endpoint_url, set_endpoint_url, "u66");
test_v!(u67, get_endpoint_url, set_endpoint_url, "u67");
test_v!(u68, get_endpoint_url, set_endpoint_url, "u68");
test_v!(u69, get_endpoint_url, set_endpoint_url, "u69");
test_v!(u70, get_endpoint_url, set_endpoint_url, "u70");

test_v!(u71, get_api_key, set_api_key, "k71");
test_v!(u72, get_api_key, set_api_key, "k72");
test_v!(u73, get_api_key, set_api_key, "k73");
test_v!(u74, get_api_key, set_api_key, "k74");
test_v!(u75, get_api_key, set_api_key, "k75");
test_v!(u76, get_api_key, set_api_key, "k76");
test_v!(u77, get_api_key, set_api_key, "k77");
test_v!(u78, get_api_key, set_api_key, "k78");
test_v!(u79, get_api_key, set_api_key, "k79");
test_v!(u80, get_api_key, set_api_key, "k80");
