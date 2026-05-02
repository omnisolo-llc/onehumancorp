use crate::app;
use slint::ComponentHandle;

fn create() -> app::WebsiteBuilder { crate::ui_tests::init(); app::WebsiteBuilder::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn builder_invalid_color() {
    let ui = create();
    ui.set_primary_color("not-a-color".into());
    assert_eq!(ui.get_primary_color(), "not-a-color");
}

#[test] fn builder_hex_color_variants() {
    let ui = create();
    ui.set_primary_color("#F00".into());
    assert_eq!(ui.get_primary_color(), "#F00");
    ui.set_primary_color("#FF0000".into());
    assert_eq!(ui.get_primary_color(), "#FF0000");
    ui.set_primary_color("#FF0000FF".into());
    assert_eq!(ui.get_primary_color(), "#FF0000FF");
}

#[test] fn builder_xss_template_name() {
    let ui = create();
    let xss = "'; alert(1); //";
    ui.set_selected_template(xss.into());
    assert_eq!(ui.get_selected_template(), xss);
}

#[test] fn builder_punycode_domain() {
    let ui = create();
    let domain = "xn--bcher-kva.ch";
    ui.set_domain_choice(domain.into());
    assert_eq!(ui.get_domain_choice(), domain);
}

// --- Interaction / Flow Tests ---

#[test] fn builder_flow_rapid_template_switch() {
    let ui = create();
    let templates = ["Modern", "Minimal", "Corporate", "Classic", "Bold"];
    for _ in 0..10 {
        for t in templates {
            ui.set_selected_template(t.into());
            assert_eq!(ui.get_selected_template(), t);
        }
    }
}

#[test] fn builder_flow_price_input_validation() {
    let ui = create();
    ui.set_product_price("10.00".into());
    assert_eq!(ui.get_product_price(), "10.00");
    ui.set_product_price("free".into());
    assert_eq!(ui.get_product_price(), "free");
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_product_name, set_product_name, "My Store");
test_v!(u2, get_product_description, set_product_description, "Selling cool stuff.");
#[test] fn u3() { let ui = create(); ui.set_is_advanced(true); assert!(ui.get_is_advanced()); }
#[test] fn u4() { let ui = create(); ui.set_is_publishing(true); assert!(ui.get_is_publishing()); }
test_v!(u5, get_step, set_step, 1);
test_v!(u6, get_primary_color, set_primary_color, "rgb(255, 0, 0)");
test_v!(u7, get_selected_template, set_selected_template, "Dark Mode");
test_v!(u8, get_domain_choice, set_domain_choice, "shop.com");
test_v!(u9, get_product_price, set_product_price, "99.99");
test_v!(u10, get_product_name, set_product_name, "");

test_v!(u11, get_product_name, set_product_name, "n11");
test_v!(u12, get_product_name, set_product_name, "n12");
test_v!(u13, get_product_name, set_product_name, "n13");
test_v!(u14, get_product_name, set_product_name, "n14");
test_v!(u15, get_product_name, set_product_name, "n15");
test_v!(u16, get_product_name, set_product_name, "n16");
test_v!(u17, get_product_name, set_product_name, "n17");
test_v!(u18, get_product_name, set_product_name, "n18");
test_v!(u19, get_product_name, set_product_name, "n19");
test_v!(u20, get_product_name, set_product_name, "n20");

test_v!(u21, get_product_price, set_product_price, "p21");
test_v!(u22, get_product_price, set_product_price, "p22");
test_v!(u23, get_product_price, set_product_price, "p23");
test_v!(u24, get_product_price, set_product_price, "p24");
test_v!(u25, get_product_price, set_product_price, "p25");
test_v!(u26, get_product_description, set_product_description, "d26");
test_v!(u27, get_product_description, set_product_description, "d27");
test_v!(u28, get_product_description, set_product_description, "d28");
test_v!(u29, get_product_description, set_product_description, "d29");
test_v!(u30, get_product_description, set_product_description, "d30");

test_v!(u31, get_domain_choice, set_domain_choice, "dc31");
test_v!(u32, get_domain_choice, set_domain_choice, "dc32");
test_v!(u33, get_domain_choice, set_domain_choice, "dc33");
test_v!(u34, get_domain_choice, set_domain_choice, "dc34");
test_v!(u35, get_domain_choice, set_domain_choice, "dc35");
test_v!(u36, get_selected_template, set_selected_template, "st36");
test_v!(u37, get_selected_template, set_selected_template, "st37");
test_v!(u38, get_selected_template, set_selected_template, "st38");
test_v!(u39, get_selected_template, set_selected_template, "st39");
test_v!(u40, get_selected_template, set_selected_template, "st40");

test_v!(u41, get_primary_color, set_primary_color, "pc41");
test_v!(u42, get_primary_color, set_primary_color, "pc42");
test_v!(u43, get_primary_color, set_primary_color, "pc43");
test_v!(u44, get_primary_color, set_primary_color, "pc44");
test_v!(u45, get_primary_color, set_primary_color, "pc45");
test_v!(u46, get_step, set_step, 46);
test_v!(u47, get_step, set_step, 47);
test_v!(u48, get_step, set_step, 48);
test_v!(u49, get_step, set_step, 49);
test_v!(u50, get_step, set_step, 50);

#[test] fn u51() { let ui = create(); ui.set_is_advanced(true); ui.set_is_publishing(false); assert!(ui.get_is_advanced()); assert!(!ui.get_is_publishing()); }
#[test] fn u52() { let ui = create(); ui.set_is_advanced(false); ui.set_is_publishing(true); assert!(!ui.get_is_advanced()); assert!(ui.get_is_publishing()); }
test_v!(u53, get_product_name, set_product_name, "Space Inside Name");
test_v!(u54, get_product_name, set_product_name, "Emoji 🏪");
test_v!(u55, get_product_price, set_product_price, "0.00000001");
test_v!(u56, get_domain_choice, set_domain_choice, "test.local");
test_v!(u57, get_selected_template, set_selected_template, "Custom JSON");
test_v!(u58, get_primary_color, set_primary_color, "hsl(0, 100%, 50%)");
test_v!(u59, get_product_description, set_product_description, "Special chars: !@#$%^&*()");
test_v!(u60, get_step, set_step, -5);

test_v!(u61, get_product_name, set_product_name, "n61");
test_v!(u62, get_product_name, set_product_name, "n62");
test_v!(u63, get_product_name, set_product_name, "n63");
test_v!(u64, get_product_name, set_product_name, "n64");
test_v!(u65, get_product_name, set_product_name, "n65");
test_v!(u66, get_product_name, set_product_name, "n66");
test_v!(u67, get_product_name, set_product_name, "n67");
test_v!(u68, get_product_name, set_product_name, "n68");
test_v!(u69, get_product_name, set_product_name, "n69");
test_v!(u70, get_product_name, set_product_name, "n70");

test_v!(u71, get_product_price, set_product_price, "p71");
test_v!(u72, get_product_price, set_product_price, "p72");
test_v!(u73, get_product_price, set_product_price, "p73");
test_v!(u74, get_product_price, set_product_price, "p74");
test_v!(u75, get_product_price, set_product_price, "p75");
test_v!(u76, get_product_description, set_product_description, "d76");
test_v!(u77, get_product_description, set_product_description, "d77");
test_v!(u78, get_product_description, set_product_description, "d78");
test_v!(u79, get_product_description, set_product_description, "d79");
test_v!(u80, get_product_description, set_product_description, "d80");
