use crate::app;
use slint::ComponentHandle;

fn create() -> app::Skills { crate::ui_tests::init(); app::Skills::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn skills_xss_category() {
    let ui = create();
    let xss = "<textarea onload=alert(1)>";
    ui.set_selected_category(xss.into());
    assert_eq!(ui.get_selected_category(), xss);
}

#[test] fn skills_injection_category() {
    let ui = create();
    let inj = "'; DELETE FROM skills; --";
    ui.set_selected_category(inj.into());
    assert_eq!(ui.get_selected_category(), inj);
}

#[test] fn skills_empty_category() {
    let ui = create();
    ui.set_selected_category("".into());
    assert_eq!(ui.get_selected_category(), "");
}

// --- Interaction / Flow Tests ---

#[test] fn skills_flow_rapid_category_switch() {
    let ui = create();
    let cats = ["Coding", "Design", "Writing", "Marketing", "Sales"];
    for _ in 0..20 {
        for c in cats {
            ui.set_selected_category(c.into());
            assert_eq!(ui.get_selected_category(), c);
        }
    }
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_selected_category, set_selected_category, "Frontend");
test_v!(u2, get_selected_category, set_selected_category, "Backend");
test_v!(u3, get_selected_category, set_selected_category, "Fullstack");
test_v!(u4, get_selected_category, set_selected_category, "Mobile");
test_v!(u5, get_selected_category, set_selected_category, "DevOps");
test_v!(u6, get_selected_category, set_selected_category, "Cloud");
test_v!(u7, get_selected_category, set_selected_category, "Data Science");
test_v!(u8, get_selected_category, set_selected_category, "AI/ML");
test_v!(u9, get_selected_category, set_selected_category, "Security");
test_v!(u10, get_selected_category, set_selected_category, "Blockchain");

test_v!(u11, get_selected_category, set_selected_category, "c11");
test_v!(u12, get_selected_category, set_selected_category, "c12");
test_v!(u13, get_selected_category, set_selected_category, "c13");
test_v!(u14, get_selected_category, set_selected_category, "c14");
test_v!(u15, get_selected_category, set_selected_category, "c15");
test_v!(u16, get_selected_category, set_selected_category, "c16");
test_v!(u17, get_selected_category, set_selected_category, "c17");
test_v!(u18, get_selected_category, set_selected_category, "c18");
test_v!(u19, get_selected_category, set_selected_category, "c19");
test_v!(u20, get_selected_category, set_selected_category, "c20");

test_v!(u21, get_selected_category, set_selected_category, "Category with Space");
test_v!(u22, get_selected_category, set_selected_category, "Category_with_Underscore");
test_v!(u23, get_selected_category, set_selected_category, "Category-with-Hyphen");
test_v!(u24, get_selected_category, set_selected_category, "Category & Ampersand");
test_v!(u25, get_selected_category, set_selected_category, "Category / Slash");
test_v!(u26, get_selected_category, set_selected_category, "Category (Parentheses)");
test_v!(u27, get_selected_category, set_selected_category, "Category [Brackets]");
test_v!(u28, get_selected_category, set_selected_category, "Category {Braces}");
test_v!(u29, get_selected_category, set_selected_category, "Category !@#$");
test_v!(u30, get_selected_category, set_selected_category, "Long Category Name Long Category Name ");

test_v!(u31, get_selected_category, set_selected_category, "c31");
test_v!(u32, get_selected_category, set_selected_category, "c32");
test_v!(u33, get_selected_category, set_selected_category, "c33");
test_v!(u34, get_selected_category, set_selected_category, "c34");
test_v!(u35, get_selected_category, set_selected_category, "c35");
test_v!(u36, get_selected_category, set_selected_category, "c36");
test_v!(u37, get_selected_category, set_selected_category, "c37");
test_v!(u38, get_selected_category, set_selected_category, "c38");
test_v!(u39, get_selected_category, set_selected_category, "c39");
test_v!(u40, get_selected_category, set_selected_category, "c40");

test_v!(u41, get_selected_category, set_selected_category, "c41");
test_v!(u42, get_selected_category, set_selected_category, "c42");
test_v!(u43, get_selected_category, set_selected_category, "c43");
test_v!(u44, get_selected_category, set_selected_category, "c44");
test_v!(u45, get_selected_category, set_selected_category, "c45");
test_v!(u46, get_selected_category, set_selected_category, "c46");
test_v!(u47, get_selected_category, set_selected_category, "c47");
test_v!(u48, get_selected_category, set_selected_category, "c48");
test_v!(u49, get_selected_category, set_selected_category, "c49");
test_v!(u50, get_selected_category, set_selected_category, "c50");

test_v!(u51, get_selected_category, set_selected_category, "c51");
test_v!(u52, get_selected_category, set_selected_category, "c52");
test_v!(u53, get_selected_category, set_selected_category, "c53");
test_v!(u54, get_selected_category, set_selected_category, "c54");
test_v!(u55, get_selected_category, set_selected_category, "c55");
test_v!(u56, get_selected_category, set_selected_category, "c56");
test_v!(u57, get_selected_category, set_selected_category, "c57");
test_v!(u58, get_selected_category, set_selected_category, "c58");
test_v!(u59, get_selected_category, set_selected_category, "c59");
test_v!(u60, get_selected_category, set_selected_category, "c60");

test_v!(u61, get_selected_category, set_selected_category, "c61");
test_v!(u62, get_selected_category, set_selected_category, "c62");
test_v!(u63, get_selected_category, set_selected_category, "c63");
test_v!(u64, get_selected_category, set_selected_category, "c64");
test_v!(u65, get_selected_category, set_selected_category, "c65");
test_v!(u66, get_selected_category, set_selected_category, "c66");
test_v!(u67, get_selected_category, set_selected_category, "c67");
test_v!(u68, get_selected_category, set_selected_category, "c68");
test_v!(u69, get_selected_category, set_selected_category, "c69");
test_v!(u70, get_selected_category, set_selected_category, "c70");

test_v!(u71, get_selected_category, set_selected_category, "c71");
test_v!(u72, get_selected_category, set_selected_category, "c72");
test_v!(u73, get_selected_category, set_selected_category, "c73");
test_v!(u74, get_selected_category, set_selected_category, "c74");
test_v!(u75, get_selected_category, set_selected_category, "c75");
test_v!(u76, get_selected_category, set_selected_category, "c76");
test_v!(u77, get_selected_category, set_selected_category, "c77");
test_v!(u78, get_selected_category, set_selected_category, "c78");
test_v!(u79, get_selected_category, set_selected_category, "c79");
test_v!(u80, get_selected_category, set_selected_category, "c80");
