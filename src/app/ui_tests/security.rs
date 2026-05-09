use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::Security {
    crate::ui_tests::init();
    app::Security::new().unwrap()
}

// --- Hacking / Corner Cases ---

#[test]
fn security_xss_title() {
    let ui = create();
    let xss = "<img src=x onerror=alert('security')>";
    let issues = slint::VecModel::from(vec![app::UiSecurityIssue {
        id: "1".into(),
        title: xss.into(),
        description: "Crit".into(),
        severity: "high".into(),
        fixable: true,
        fixed: false,
    }]);
    ui.set_issues(Rc::new(issues).into());
    assert_eq!(ui.get_issues().row_data(0).unwrap().title, xss);
}

#[test]
fn security_sqli_description() {
    let ui = create();
    let inj = "'); UPDATE issues SET fixed=1; --";
    let issues = slint::VecModel::from(vec![app::UiSecurityIssue {
        id: "2".into(),
        title: "T1".into(),
        description: inj.into(),
        severity: "low".into(),
        fixable: false,
        fixed: false,
    }]);
    ui.set_issues(Rc::new(issues).into());
    assert_eq!(ui.get_issues().row_data(0).unwrap().description, inj);
}

#[test]
fn security_unicode_severity() {
    let ui = create();
    let sev = "🔴 HIGH 🔴";
    let issues = slint::VecModel::from(vec![app::UiSecurityIssue {
        id: "3".into(),
        title: "T2".into(),
        description: "D2".into(),
        severity: sev.into(),
        fixable: true,
        fixed: true,
    }]);
    ui.set_issues(Rc::new(issues).into());
    assert_eq!(ui.get_issues().row_data(0).unwrap().severity, sev);
}

// --- Interaction / Flow Tests ---

#[test]
fn security_flow_fix_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called.clone();
    ui.on_fix_issue(move |id| {
        *c.borrow_mut() = id.to_string();
    });
    ui.invoke_fix_issue("ISSUE-123".into());
    assert_eq!(*called.borrow(), "ISSUE-123");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
