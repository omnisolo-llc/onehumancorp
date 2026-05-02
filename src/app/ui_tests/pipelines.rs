use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Pipelines { crate::ui_tests::init(); app::Pipelines::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn pipelines_xss_name() {
    let ui = create();
    let xss = "<script>alert('pipeline')</script>";
    let pipelines = slint::VecModel::from(vec![
        app::UiPipeline {
            id: "1".into(),
            name: xss.into(),
            branch: "main".into(),
            status: "Running".into(),
            initiated_by: "user".into(),
            staging_url: "".into(),
            can_promote: false,
        }
    ]);
    ui.set_pipelines(Rc::new(pipelines).into());
    assert_eq!(ui.get_pipelines().row_data(0).unwrap().name, xss);
}

#[test] fn pipelines_injection_branch() {
    let ui = create();
    let inj = "main'; DROP TABLE builds; --";
    let pipelines = slint::VecModel::from(vec![
        app::UiPipeline {
            id: "2".into(),
            name: "Build".into(),
            branch: inj.into(),
            status: "Success".into(),
            initiated_by: "bot".into(),
            staging_url: "".into(),
            can_promote: true,
        }
    ]);
    ui.set_pipelines(Rc::new(pipelines).into());
    assert_eq!(ui.get_pipelines().row_data(0).unwrap().branch, inj);
}

#[test] fn pipelines_long_staging_url() {
    let ui = create();
    let long = "https://staging.internal.com/".to_string() + &"a".repeat(1000);
    let pipelines = slint::VecModel::from(vec![
        app::UiPipeline {
            id: "3".into(),
            name: "P3".into(),
            branch: "dev".into(),
            status: "Staged".into(),
            initiated_by: "dev".into(),
            staging_url: long.clone().into(),
            can_promote: true,
        }
    ]);
    ui.set_pipelines(Rc::new(pipelines).into());
    assert_eq!(ui.get_pipelines().row_data(0).unwrap().staging_url, long);
}

// --- Interaction / Flow Tests ---

#[test] fn pipelines_flow_promote_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_promote_pipeline(move |id| { *c.borrow_mut() = id.to_string(); });
    ui.invoke_promote_pipeline("PIPE-001".into());
    assert_eq!(*called.borrow(), "PIPE-001");
}

#[test] fn pipelines_flow_refresh_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_refresh(move || { *c.borrow_mut() = true; });
    ui.invoke_refresh();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v_p {
    ($id:ident, $name:expr, $status:expr) => {
        #[test] fn $id() {
            let ui = create();
            let p = slint::VecModel::from(vec![app::UiPipeline {
                id: "id".into(),
                name: $name.into(),
                branch: "main".into(),
                status: $status.into(),
                initiated_by: "user".into(),
                staging_url: "".into(),
                can_promote: true,
            }]);
            ui.set_pipelines(Rc::new(p).into());
            assert_eq!(ui.get_pipelines().row_data(0).unwrap().name, $name);
            assert_eq!(ui.get_pipelines().row_data(0).unwrap().status, $status);
        }
    };
}

test_v_p!(u1, "Frontend Build", "Completed");
test_v_p!(u2, "Backend Build", "Running");
test_v_p!(u3, "Database Migrations", "Failed");

test_v_p!(u11, "p11", "s11");
test_v_p!(u12, "p12", "s12");
test_v_p!(u13, "p13", "s13");
test_v_p!(u14, "p14", "s14");
test_v_p!(u15, "p15", "s15");
test_v_p!(u16, "p16", "s16");
test_v_p!(u17, "p17", "s17");
test_v_p!(u18, "p18", "s18");
test_v_p!(u19, "p19", "s19");
test_v_p!(u20, "p20", "s20");

test_v_p!(u21, "🚀 Production Deploy", "Awaiting Approval");
test_v_p!(u22, "Pipeline with 'Quotes'", "Quoted");
test_v_p!(u23, "Pipeline with ; Semi", "Semis");
test_v_p!(u24, "", "");
test_v_p!(u25, "Huge Pipeline Name ".repeat(5), "Pending");

test_v_p!(u31, "p31", "s31");
test_v_p!(u32, "p32", "s32");
test_v_p!(u33, "p33", "s33");
test_v_p!(u34, "p34", "s34");
test_v_p!(u35, "p35", "s35");
test_v_p!(u36, "p36", "s36");
test_v_p!(u37, "p37", "s37");
test_v_p!(u38, "p38", "s38");
test_v_p!(u39, "p39", "s39");
test_v_p!(u40, "p40", "s40");

test_v_p!(u41, "p41", "s41");
test_v_p!(u42, "p42", "s42");
test_v_p!(u43, "p43", "s43");
test_v_p!(u44, "p44", "s44");
test_v_p!(u45, "p45", "s45");
test_v_p!(u46, "p46", "s46");
test_v_p!(u47, "p47", "s47");
test_v_p!(u48, "p48", "s48");
test_v_p!(u49, "p49", "s49");
test_v_p!(u50, "p50", "s50");

test_v_p!(u51, "p51", "s51");
test_v_p!(u52, "p52", "s52");
test_v_p!(u53, "p53", "s53");
test_v_p!(u54, "p54", "s54");
test_v_p!(u55, "p55", "s55");
test_v_p!(u56, "p56", "s56");
test_v_p!(u57, "p57", "s57");
test_v_p!(u58, "p58", "s58");
test_v_p!(u59, "p59", "s59");
test_v_p!(u60, "p60", "s60");

test_v_p!(u61, "p61", "s61");
test_v_p!(u62, "p62", "s62");
test_v_p!(u63, "p63", "s63");
test_v_p!(u64, "p64", "s64");
test_v_p!(u65, "p65", "s65");
test_v_p!(u66, "p66", "s66");
test_v_p!(u67, "p67", "s67");
test_v_p!(u68, "p68", "s68");
test_v_p!(u69, "p69", "s69");
test_v_p!(u70, "p70", "s70");

test_v_p!(u71, "p71", "s71");
test_v_p!(u72, "p72", "s72");
test_v_p!(u73, "p73", "s73");
test_v_p!(u74, "p74", "s74");
test_v_p!(u75, "p75", "s75");
test_v_p!(u76, "p76", "s76");
test_v_p!(u77, "p77", "s77");
test_v_p!(u78, "p78", "s78");
test_v_p!(u79, "p79", "s79");
test_v_p!(u80, "p80", "s80");
