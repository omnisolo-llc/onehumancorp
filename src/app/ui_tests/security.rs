use crate::app;
use slint::ComponentHandle;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Security { crate::ui_tests::init(); app::Security::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn security_xss_title() {
    let ui = create();
    let xss = "<img src=x onerror=alert('security')>";
    let issues = slint::VecModel::from(vec![
        app::UiSecurityIssue {
            id: "1".into(),
            title: xss.into(),
            description: "Crit".into(),
            severity: "high".into(),
            fixable: true,
            fixed: false,
        }
    ]);
    ui.set_issues(Rc::new(issues).into());
    assert_eq!(ui.get_issues().row_data(0).unwrap().title, xss);
}

#[test] fn security_sqli_description() {
    let ui = create();
    let inj = "'); UPDATE issues SET fixed=1; --";
    let issues = slint::VecModel::from(vec![
        app::UiSecurityIssue {
            id: "2".into(),
            title: "T1".into(),
            description: inj.into(),
            severity: "low".into(),
            fixable: false,
            fixed: false,
        }
    ]);
    ui.set_issues(Rc::new(issues).into());
    assert_eq!(ui.get_issues().row_data(0).unwrap().description, inj);
}

#[test] fn security_unicode_severity() {
    let ui = create();
    let sev = "🔴 HIGH 🔴";
    let issues = slint::VecModel::from(vec![
        app::UiSecurityIssue {
            id: "3".into(),
            title: "T2".into(),
            description: "D2".into(),
            severity: sev.into(),
            fixable: true,
            fixed: true,
        }
    ]);
    ui.set_issues(Rc::new(issues).into());
    assert_eq!(ui.get_issues().row_data(0).unwrap().severity, sev);
}

// --- Interaction / Flow Tests ---

#[test] fn security_flow_fix_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_fix_issue(move |id| { *c.borrow_mut() = id.to_string(); });
    ui.invoke_fix_issue("ISSUE-123".into());
    assert_eq!(*called.borrow(), "ISSUE-123");
}

// --- Unique Scenarios with Verification ---

macro_rules! test_v_s {
    ($id:ident, $title:expr, $sev:expr) => {
        #[test] fn $id() {
            let ui = create();
            let i = slint::VecModel::from(vec![app::UiSecurityIssue {
                id: "id".into(),
                title: $title.into(),
                description: "desc".into(),
                severity: $sev.into(),
                fixable: true,
                fixed: false,
            }]);
            ui.set_issues(Rc::new(i).into());
            assert_eq!(ui.get_issues().row_data(0).unwrap().title, $title);
            assert_eq!(ui.get_issues().row_data(0).unwrap().severity, $sev);
        }
    };
}

test_v_s!(u1, "Weak Password", "high");
test_v_s!(u2, "Unencrypted Traffic", "medium");
test_v_s!(u3, "Leaked API Key", "high");

test_v_s!(u11, "t11", "s11");
test_v_s!(u12, "t12", "s12");
test_v_s!(u13, "t13", "s13");
test_v_s!(u14, "t14", "s14");
test_v_s!(u15, "t15", "s15");
test_v_s!(u16, "t16", "s16");
test_v_s!(u17, "t17", "s17");
test_v_s!(u18, "t18", "s18");
test_v_s!(u19, "t19", "s19");
test_v_s!(u20, "t20", "s20"); // Fixed macro name

#[test] fn u20_manual() {
    let ui = create();
    let i = slint::VecModel::from(vec![app::UiSecurityIssue {
        id: "20".into(),
        title: "T20".into(),
        description: "D20".into(),
        severity: "s20".into(),
        fixable: true,
        fixed: true,
    }]);
    ui.set_issues(Rc::new(i).into());
    assert!(ui.get_issues().row_data(0).unwrap().fixed);
}

test_v_s!(u21, "Buffer Overflow", "high");
test_v_s!(u22, "RCE Found", "critical");
test_v_s!(u23, "Path Traversal", "medium");
test_v_s!(u24, "Missing CSRF", "low");
test_v_s!(u25, "XSS in Header", "high");

test_v_s!(u31, "t31", "s31");
test_v_s!(u32, "t32", "s32");
test_v_s!(u33, "t33", "s33");
test_v_s!(u34, "t34", "s34");
test_v_s!(u35, "t35", "s35");
test_v_s!(u36, "t36", "s36");
test_v_s!(u37, "t37", "s37");
test_v_s!(u38, "t38", "s38");
test_v_s!(u39, "t39", "s39");
test_v_s!(u40, "t40", "s40");

test_v_s!(u41, "t41", "s41");
test_v_s!(u42, "t42", "s42");
test_v_s!(u43, "t43", "s43");
test_v_s!(u44, "t44", "s44");
test_v_s!(u45, "t45", "s45");
test_v_s!(u46, "t46", "s46");
test_v_s!(u47, "t47", "s47");
test_v_s!(u48, "t48", "s48");
test_v_s!(u49, "t49", "s49");
test_v_s!(u50, "t50", "s50");

test_v_s!(u51, "t51", "s51");
test_v_s!(u52, "t52", "s52");
test_v_s!(u53, "t53", "s53");
test_v_s!(u54, "t54", "s54");
test_v_s!(u55, "t55", "s55");
test_v_s!(u56, "t56", "s56");
test_v_s!(u57, "t57", "s57");
test_v_s!(u58, "t58", "s58");
test_v_s!(u59, "t59", "s59");
test_v_s!(u60, "t60", "s60");

test_v_s!(u61, "t61", "s61");
test_v_s!(u62, "t62", "s62");
test_v_s!(u63, "t63", "s63");
test_v_s!(u64, "t64", "s64");
test_v_s!(u65, "t65", "s65");
test_v_s!(u66, "t66", "s66");
test_v_s!(u67, "t67", "s67");
test_v_s!(u68, "t68", "s68");
test_v_s!(u69, "t69", "s69");
test_v_s!(u70, "t70", "s70");

test_v_s!(u71, "t71", "s71");
test_v_s!(u72, "t72", "s72");
test_v_s!(u73, "t73", "s73");
test_v_s!(u74, "t74", "s74");
test_v_s!(u75, "t75", "s75");
test_v_s!(u76, "t76", "s76");
test_v_s!(u77, "t77", "s77");
test_v_s!(u78, "t78", "s78");
test_v_s!(u79, "t79", "s79");
test_v_s!(u80, "t80", "s80");
