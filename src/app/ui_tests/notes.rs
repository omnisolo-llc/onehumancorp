use crate::app;
use slint::ComponentHandle;

fn create() -> app::ReleaseNotes { crate::ui_tests::init(); app::ReleaseNotes::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn notes_flow_version_sync() {
    let ui = create();
    ui.set_current_version("v1.0.0".into());
    assert_eq!(ui.get_current_version(), "v1.0.0");
}

#[test] fn notes_flow_toggle_latest() {
    let ui = create();
    ui.set_show_latest_only(true);
    assert!(ui.get_show_latest_only());
    ui.set_show_latest_only(false);
    assert!(!ui.get_show_latest_only());
}

#[test] fn notes_xss_version() {
    let ui = create();
    let xss = "<script>alert('version')</script>";
    ui.set_current_version(xss.into());
    assert_eq!(ui.get_current_version(), xss);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_current_version, set_current_version, "v2.4.1");
test_v!(u2, get_current_version, set_current_version, "BETA-7");
test_v!(u3, get_current_version, set_current_version, "ALPHA-RC1");

test_v!(u11, get_current_version, set_current_version, "v11");
test_v!(u12, get_current_version, set_current_version, "v12");
test_v!(u13, get_current_version, set_current_version, "v13");
test_v!(u14, get_current_version, set_current_version, "v14");
test_v!(u15, get_current_version, set_current_version, "v15");
test_v!(u16, get_current_version, set_current_version, "v16");
test_v!(u17, get_current_version, set_current_version, "v17");
test_v!(u18, get_current_version, set_current_version, "v18");
test_v!(u19, get_current_version, set_current_version, "v19");
test_v!(u20, get_current_version, set_current_version, "v20");

test_v!(u21, get_current_version, set_current_version, "Version with 🚀 Emoji");
test_v!(u22, get_current_version, set_current_version, "Version'Quotes'");
test_v!(u23, get_current_version, set_current_version, "Version ; Semi");
test_v!(u24, get_current_version, set_current_version, "");
test_v!(u25, get_current_version, set_current_version, "Very Long Version Name ".repeat(5));

test_v!(u31, get_current_version, set_current_version, "v31");
test_v!(u32, get_current_version, set_current_version, "v32");
test_v!(u33, get_current_version, set_current_version, "v33");
test_v!(u34, get_current_version, set_current_version, "v34");
test_v!(u35, get_current_version, set_current_version, "v35");
test_v!(u36, get_current_version, set_current_version, "v36");
test_v!(u37, get_current_version, set_current_version, "v37");
test_v!(u38, get_current_version, set_current_version, "v38");
test_v!(u39, get_current_version, set_current_version, "v39");
test_v!(u40, get_current_version, set_current_version, "v40");

test_v!(u41, get_current_version, set_current_version, "v41");
test_v!(u42, get_current_version, set_current_version, "v42");
test_v!(u43, get_current_version, set_current_version, "v43");
test_v!(u44, get_current_version, set_current_version, "v44");
test_v!(u45, get_current_version, set_current_version, "v45");
test_v!(u46, get_current_version, set_current_version, "v46");
test_v!(u47, get_current_version, set_current_version, "v47");
test_v!(u48, get_current_version, set_current_version, "v48");
test_v!(u49, get_current_version, set_current_version, "v49");
test_v!(u50, get_current_version, set_current_version, "v50");

test_v!(u51, get_show_latest_only, set_show_latest_only, true);
test_v!(u52, get_show_latest_only, set_show_latest_only, false);
test_v!(u53, get_show_latest_only, set_show_latest_only, true);
test_v!(u54, get_show_latest_only, set_show_latest_only, false);
test_v!(u55, get_show_latest_only, set_show_latest_only, true);

test_v!(u61, get_current_version, set_current_version, "v61");
test_v!(u62, get_current_version, set_current_version, "v62");
test_v!(u63, get_current_version, set_current_version, "v63");
test_v!(u64, get_current_version, set_current_version, "v64");
test_v!(u65, get_current_version, set_current_version, "v65");
test_v!(u66, get_current_version, set_current_version, "v66");
test_v!(u67, get_current_version, set_current_version, "v67");
test_v!(u68, get_current_version, set_current_version, "v68");
test_v!(u69, get_current_version, set_current_version, "v69");
test_v!(u70, get_current_version, set_current_version, "v70");

test_v!(u71, get_current_version, set_current_version, "v71");
test_v!(u72, get_current_version, set_current_version, "v72");
test_v!(u73, get_current_version, set_current_version, "v73");
test_v!(u74, get_current_version, set_current_version, "v74");
test_v!(u75, get_current_version, set_current_version, "v75");
test_v!(u76, get_current_version, set_current_version, "v76");
test_v!(u77, get_current_version, set_current_version, "v77");
test_v!(u78, get_current_version, set_current_version, "v78");
test_v!(u79, get_current_version, set_current_version, "v79");
test_v!(u80, get_current_version, set_current_version, "v80");
