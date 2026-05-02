use crate::app;
use slint::ComponentHandle;

fn create() -> app::VideoTutorials { crate::ui_tests::init(); app::VideoTutorials::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn tutorials_flow_playback() {
    let ui = create();
    ui.set_selected_video_title("How to Scale".into());
    ui.set_is_playing(true);
    assert_eq!(ui.get_selected_video_title(), "How to Scale");
    assert!(ui.get_is_playing());
    ui.set_is_playing(false);
    assert!(!ui.get_is_playing());
}

#[test] fn tutorials_xss_title() {
    let ui = create();
    let xss = "<iframe src=javascript:alert('tutorial')>";
    ui.set_selected_video_title(xss.into());
    assert_eq!(ui.get_selected_video_title(), xss);
}

#[test] fn tutorials_injection_title() {
    let ui = create();
    let inj = "Intro'); DROP TABLE tutorials; --";
    ui.set_selected_video_title(inj.into());
    assert_eq!(ui.get_selected_video_title(), inj);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_selected_video_title, set_selected_video_title, "Basic Setup");
test_v!(u2, get_selected_video_title, set_selected_video_title, "Advanced Agents");
test_v!(u3, get_selected_video_title, set_selected_video_title, "Billing Help");

test_v!(u11, get_selected_video_title, set_selected_video_title, "t11");
test_v!(u12, get_selected_video_title, set_selected_video_title, "t12");
test_v!(u13, get_selected_video_title, set_selected_video_title, "t13");
test_v!(u14, get_selected_video_title, set_selected_video_title, "t14");
test_v!(u15, get_selected_video_title, set_selected_video_title, "t15");
test_v!(u16, get_selected_video_title, set_selected_video_title, "t16");
test_v!(u17, get_selected_video_title, set_selected_video_title, "t17");
test_v!(u18, get_selected_video_title, set_selected_video_title, "t18");
test_v!(u19, get_selected_video_title, set_selected_video_title, "t19");
test_v!(u20, get_selected_video_title, set_selected_video_title, "t20");

test_v!(u21, get_selected_video_title, set_selected_video_title, "Title with 🎬 Emoji");
test_v!(u22, get_selected_video_title, set_selected_video_title, "Title'Quotes'");
test_v!(u23, get_selected_video_title, set_selected_video_title, "Title ; Semi");
test_v!(u24, get_selected_video_title, set_selected_video_title, "");
test_v!(u25, get_selected_video_title, set_selected_video_title, "Very Long Tutorial Title ".repeat(5));

test_v!(u31, get_selected_video_title, set_selected_video_title, "t31");
test_v!(u32, get_selected_video_title, set_selected_video_title, "t32");
test_v!(u33, get_selected_video_title, set_selected_video_title, "t33");
test_v!(u34, get_selected_video_title, set_selected_video_title, "t34");
test_v!(u35, get_selected_video_title, set_selected_video_title, "t35");
test_v!(u36, get_selected_video_title, set_selected_video_title, "t36");
test_v!(u37, get_selected_video_title, set_selected_video_title, "t37");
test_v!(u38, get_selected_video_title, set_selected_video_title, "t38");
test_v!(u39, get_selected_video_title, set_selected_video_title, "t39");
test_v!(u40, get_selected_video_title, set_selected_video_title, "t40");

test_v!(u41, get_selected_video_title, set_selected_video_title, "t41");
test_v!(u42, get_selected_video_title, set_selected_video_title, "t42");
test_v!(u43, get_selected_video_title, set_selected_video_title, "t43");
test_v!(u44, get_selected_video_title, set_selected_video_title, "t44");
test_v!(u45, get_selected_video_title, set_selected_video_title, "t45");
test_v!(u46, get_selected_video_title, set_selected_video_title, "t46");
test_v!(u47, get_selected_video_title, set_selected_video_title, "t47");
test_v!(u48, get_selected_video_title, set_selected_video_title, "t48");
test_v!(u49, get_selected_video_title, set_selected_video_title, "t49");
test_v!(u50, get_selected_video_title, set_selected_video_title, "t50");

test_v!(u51, get_is_playing, set_is_playing, true);
test_v!(u52, get_is_playing, set_is_playing, false);
test_v!(u53, get_is_playing, set_is_playing, true);
test_v!(u54, get_is_playing, set_is_playing, false);
test_v!(u55, get_is_playing, set_is_playing, true);

test_v!(u61, get_selected_video_title, set_selected_video_title, "t61");
test_v!(u62, get_selected_video_title, set_selected_video_title, "t62");
test_v!(u63, get_selected_video_title, set_selected_video_title, "t63");
test_v!(u64, get_selected_video_title, set_selected_video_title, "t64");
test_v!(u65, get_selected_video_title, set_selected_video_title, "t65");
test_v!(u66, get_selected_video_title, set_selected_video_title, "t66");
test_v!(u67, get_selected_video_title, set_selected_video_title, "t67");
test_v!(u68, get_selected_video_title, set_selected_video_title, "t68");
test_v!(u69, get_selected_video_title, set_selected_video_title, "t69");
test_v!(u70, get_selected_video_title, set_selected_video_title, "t70");

test_v!(u71, get_selected_video_title, set_selected_video_title, "t71");
test_v!(u72, get_selected_video_title, set_selected_video_title, "t72");
test_v!(u73, get_selected_video_title, set_selected_video_title, "t73");
test_v!(u74, get_selected_video_title, set_selected_video_title, "t74");
test_v!(u75, get_selected_video_title, set_selected_video_title, "t75");
test_v!(u76, get_selected_video_title, set_selected_video_title, "t76");
test_v!(u77, get_selected_video_title, set_selected_video_title, "t77");
test_v!(u78, get_selected_video_title, set_selected_video_title, "t78");
test_v!(u79, get_selected_video_title, set_selected_video_title, "t79");
test_v!(u80, get_selected_video_title, set_selected_video_title, "t80");
