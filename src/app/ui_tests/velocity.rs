use crate::app;
use slint::ComponentHandle;

fn create() -> app::SwarmVelocityWindow { crate::ui_tests::init(); app::SwarmVelocityWindow::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn velocity_flow_metrics_sync() {
    let ui = create();
    ui.set_active_threads("128".into());
    ui.set_avg_latency("45ms".into());
    ui.set_completion_rate("99.9%".into());
    assert_eq!(ui.get_active_threads(), "128");
    assert_eq!(ui.get_avg_latency(), "45ms");
    assert_eq!(ui.get_completion_rate(), "99.9%");
}

#[test] fn velocity_xss_latency() {
    let ui = create();
    let xss = "<script>alert('latency')</script>";
    ui.set_avg_latency(xss.into());
    assert_eq!(ui.get_avg_latency(), xss);
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v {
    ($id:ident, $get:ident, $set:ident, $val:expr) => {
        #[test] fn $id() { let ui = create(); ui.$set($val.into()); assert_eq!(ui.$get(), $val); }
    };
}

test_v!(u1, get_active_threads, set_active_threads, "10");
test_v!(u2, get_avg_latency, set_avg_latency, "100ms");
test_v!(u3, get_completion_rate, set_completion_rate, "50%");

test_v!(u11, get_active_threads, set_active_threads, "t11");
test_v!(u12, get_active_threads, set_active_threads, "t12");
test_v!(u13, get_active_threads, set_active_threads, "t13");
test_v!(u14, get_active_threads, set_active_threads, "t14");
test_v!(u15, get_active_threads, set_active_threads, "t15");
test_v!(u16, get_active_threads, set_active_threads, "t16");
test_v!(u17, get_active_threads, set_active_threads, "t17");
test_v!(u18, get_active_threads, set_active_threads, "t18");
test_v!(u19, get_active_threads, set_active_threads, "t19");
test_v!(u20, get_active_threads, set_active_threads, "t20");

test_v!(u21, get_avg_latency, set_avg_latency, "l21");
test_v!(u22, get_avg_latency, set_avg_latency, "l22");
test_v!(u23, get_avg_latency, set_avg_latency, "l23");
test_v!(u24, get_avg_latency, set_avg_latency, "l24");
test_v!(u25, get_avg_latency, set_avg_latency, "l25");
test_v!(u26, get_avg_latency, set_avg_latency, "l26");
test_v!(u27, get_avg_latency, set_avg_latency, "l27");
test_v!(u28, get_avg_latency, set_avg_latency, "l28");
test_v!(u29, get_avg_latency, set_avg_latency, "l29");
test_v!(u30, get_avg_latency, set_avg_latency, "l30");

test_v!(u31, get_active_threads, set_active_threads, "Threads with 🧵 Emoji");
test_v!(u32, get_active_threads, set_active_threads, "Threads'Quotes'");
test_v!(u33, get_active_threads, set_active_threads, "Threads;Semi");
test_v!(u34, get_active_threads, set_active_threads, "");
test_v!(u35, get_active_threads, set_active_threads, "Very Long Thread Count ".repeat(5));

test_v!(u41, get_completion_rate, set_completion_rate, "r41");
test_v!(u42, get_completion_rate, set_completion_rate, "r42");
test_v!(u43, get_completion_rate, set_completion_rate, "r43");
test_v!(u44, get_completion_rate, set_completion_rate, "r44");
test_v!(u45, get_completion_rate, set_completion_rate, "r45");
test_v!(u46, get_completion_rate, set_completion_rate, "r46");
test_v!(u47, get_completion_rate, set_completion_rate, "r47");
test_v!(u48, get_completion_rate, set_completion_rate, "r48");
test_v!(u49, get_completion_rate, set_completion_rate, "r49");
test_v!(u50, get_completion_rate, set_completion_rate, "r50");

test_v!(u51, get_active_threads, set_active_threads, "t51");
test_v!(u52, get_active_threads, set_active_threads, "t52");
test_v!(u53, get_active_threads, set_active_threads, "t53");
test_v!(u54, get_active_threads, set_active_threads, "t54");
test_v!(u55, get_active_threads, set_active_threads, "t55");
test_v!(u56, get_active_threads, set_active_threads, "t56");
test_v!(u57, get_active_threads, set_active_threads, "t57");
test_v!(u58, get_active_threads, set_active_threads, "t58");
test_v!(u59, get_active_threads, set_active_threads, "t59");
test_v!(u60, get_active_threads, set_active_threads, "t60");

test_v!(u61, get_avg_latency, set_avg_latency, "l61");
test_v!(u62, get_avg_latency, set_avg_latency, "l62");
test_v!(u63, get_avg_latency, set_avg_latency, "l63");
test_v!(u64, get_avg_latency, set_avg_latency, "l64");
test_v!(u65, get_avg_latency, set_avg_latency, "l65");
test_v!(u66, get_avg_latency, set_avg_latency, "l66");
test_v!(u67, get_avg_latency, set_avg_latency, "l67");
test_v!(u68, get_avg_latency, set_avg_latency, "l68");
test_v!(u69, get_avg_latency, set_avg_latency, "l69");
test_v!(u70, get_avg_latency, set_avg_latency, "l70");

test_v!(u71, get_completion_rate, set_completion_rate, "r71");
test_v!(u72, get_completion_rate, set_completion_rate, "r72");
test_v!(u73, get_completion_rate, set_completion_rate, "r73");
test_v!(u74, get_completion_rate, set_completion_rate, "r74");
test_v!(u75, get_completion_rate, set_completion_rate, "r75");
test_v!(u76, get_completion_rate, set_completion_rate, "r76");
test_v!(u77, get_completion_rate, set_completion_rate, "r77");
test_v!(u78, get_completion_rate, set_completion_rate, "r78");
test_v!(u79, get_completion_rate, set_completion_rate, "r79");
test_v!(u80, get_completion_rate, set_completion_rate, "r80");
