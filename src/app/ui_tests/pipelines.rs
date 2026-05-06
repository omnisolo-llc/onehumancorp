use crate::app;
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
    let called = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let c = called.clone();
    ui.on_promote_pipeline(move |id| { *c.lock().unwrap() = id.to_string(); });
    ui.invoke_promote_pipeline("PIPE-001".into());
    assert_eq!(*called.lock().unwrap(), "PIPE-001");
}

#[test] fn pipelines_flow_refresh_callback() {
    let ui = create();
    let called = std::sync::Arc::new(std::sync::Mutex::new(false));
    let c = called.clone();
    ui.on_refresh(move || { *c.lock().unwrap() = true; });
    ui.invoke_refresh();
    assert!(*called.lock().unwrap());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
