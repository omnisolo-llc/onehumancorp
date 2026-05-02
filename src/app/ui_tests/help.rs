use crate::app;
use slint::ComponentHandle;

fn create() -> app::HelpCenter { crate::ui_tests::init(); app::HelpCenter::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn help_flow_search_sync() {
    let ui = create();
    ui.set_search_query("billing".into());
    assert_eq!(ui.get_search_query(), "billing");
}

#[test] fn help_xss_query() {
    let ui = create();
    let xss = "<img src=x onerror=alert('help')>";
    ui.set_search_query(xss.into());
    assert_eq!(ui.get_search_query(), xss);
}

#[test] fn help_injection_query() {
    let ui = create();
    let inj = "search'); DROP TABLE articles; --";
    ui.set_search_query(inj.into());
    assert_eq!(ui.get_search_query(), inj);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_search_query, set_search_query, "how to add products");
test_v!(u2, get_search_query, set_search_query, "connecting instagram");
test_v!(u3, get_search_query, set_search_query, "payment methods");

test_v!(u11, get_search_query, set_search_query, "q11");
test_v!(u12, get_search_query, set_search_query, "q12");
test_v!(u13, get_search_query, set_search_query, "q13");
test_v!(u14, get_search_query, set_search_query, "q14");
test_v!(u15, get_search_query, set_search_query, "q15");
test_v!(u16, get_search_query, set_search_query, "q16");
test_v!(u17, get_search_query, set_search_query, "q17");
test_v!(u18, get_search_query, set_search_query, "q18");
test_v!(u19, get_search_query, set_search_query, "q19");
test_v!(u20, get_search_query, set_search_query, "q20");

test_v!(u21, get_search_query, set_search_query, "Query with 🙋 Emoji");
test_v!(u22, get_search_query, set_search_query, "Query'Quotes'");
test_v!(u23, get_search_query, set_search_query, "Query ; Semi");
test_v!(u24, get_search_query, set_search_query, "");
test_v!(u25, get_search_query, set_search_query, "Very Long Search Query ".repeat(5));

test_v!(u31, get_search_query, set_search_query, "q31");
test_v!(u32, get_search_query, set_search_query, "q32");
test_v!(u33, get_search_query, set_search_query, "q33");
test_v!(u34, get_search_query, set_search_query, "q34");
test_v!(u35, get_search_query, set_search_query, "q35");
test_v!(u36, get_search_query, set_search_query, "q36");
test_v!(u37, get_search_query, set_search_query, "q37");
test_v!(u38, get_search_query, set_search_query, "q38");
test_v!(u39, get_search_query, set_search_query, "q39");
test_v!(u40, get_search_query, set_search_query, "q40");

test_v!(u41, get_search_query, set_search_query, "q41");
test_v!(u42, get_search_query, set_search_query, "q42");
test_v!(u43, get_search_query, set_search_query, "q43");
test_v!(u44, get_search_query, set_search_query, "q44");
test_v!(u45, get_search_query, set_search_query, "q45");
test_v!(u46, get_search_query, set_search_query, "q46");
test_v!(u47, get_search_query, set_search_query, "q47");
test_v!(u48, get_search_query, set_search_query, "q48");
test_v!(u49, get_search_query, set_search_query, "q49");
test_v!(u50, get_search_query, set_search_query, "q50");

test_v!(u51, get_search_query, set_search_query, "q51");
test_v!(u52, get_search_query, set_search_query, "q52");
test_v!(u53, get_search_query, set_search_query, "q53");
test_v!(u54, get_search_query, set_search_query, "q54");
test_v!(u55, get_search_query, set_search_query, "q55");
test_v!(u56, get_search_query, set_search_query, "q56");
test_v!(u57, get_search_query, set_search_query, "q57");
test_v!(u58, get_search_query, set_search_query, "q58");
test_v!(u59, get_search_query, set_search_query, "q59");
test_v!(u60, get_search_query, set_search_query, "q60");

test_v!(u61, get_search_query, set_search_query, "q61");
test_v!(u62, get_search_query, set_search_query, "q62");
test_v!(u63, get_search_query, set_search_query, "q63");
test_v!(u64, get_search_query, set_search_query, "q64");
test_v!(u65, get_search_query, set_search_query, "q65");
test_v!(u66, get_search_query, set_search_query, "q66");
test_v!(u67, get_search_query, set_search_query, "q67");
test_v!(u68, get_search_query, set_search_query, "q68");
test_v!(u69, get_search_query, set_search_query, "q69");
test_v!(u70, get_search_query, set_search_query, "q70");

test_v!(u71, get_search_query, set_search_query, "q71");
test_v!(u72, get_search_query, set_search_query, "q72");
test_v!(u73, get_search_query, set_search_query, "q73");
test_v!(u74, get_search_query, set_search_query, "q74");
test_v!(u75, get_search_query, set_search_query, "q75");
test_v!(u76, get_search_query, set_search_query, "q76");
test_v!(u77, get_search_query, set_search_query, "q77");
test_v!(u78, get_search_query, set_search_query, "q78");
test_v!(u79, get_search_query, set_search_query, "q79");
test_v!(u80, get_search_query, set_search_query, "q80");
