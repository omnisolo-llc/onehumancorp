use crate::app;
use slint::Model;
use std::rc::Rc;

fn create() -> app::TaskList { crate::ui_tests::init(); app::TaskList::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn tasklist_xss_title() {
    let ui = create();
    let xss = "<script>alert('task')</script>";
    let tasks = slint::VecModel::from(vec![
        app::UiTask {
            title: xss.into(),
            status: "Todo".into(),
            helper_id: "a1".into(),
            dependencies: "".into(),
            parent_task_id: "".into(),
            workflow_state: "".into(),
        }
    ]);
    ui.set_tasks(Rc::new(tasks).into());
    assert_eq!(ui.get_tasks().row_data(0).unwrap().title, xss);
}

#[test] fn tasklist_injection_status() {
    let ui = create();
    let inj = "Done'); DROP TABLE tasks; --";
    let tasks = slint::VecModel::from(vec![
        app::UiTask {
            title: "T1".into(),
            status: inj.into(),
            helper_id: "".into(),
            dependencies: "".into(),
            parent_task_id: "".into(),
            workflow_state: "".into(),
        }
    ]);
    ui.set_tasks(Rc::new(tasks).into());
    assert_eq!(ui.get_tasks().row_data(0).unwrap().status, inj);
}

#[test] fn tasklist_massive_list() {
    let ui = create();
    let v: Vec<app::UiTask> = (0..500).map(|i| app::UiTask {
        title: format!("Task {}", i).into(),
        status: "Queued".into(),
        helper_id: "".into(),
        dependencies: "".into(),
        parent_task_id: "".into(),
        workflow_state: "".into(),
    }).collect();
    ui.set_tasks(Rc::new(slint::VecModel::from(v)).into());
    assert_eq!(ui.get_tasks().row_count(), 500);
}

// --- Interaction / Flow Tests ---

#[test] fn tasklist_flow_refresh_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_refresh(move || { *c.borrow_mut() = true; });
    ui.invoke_refresh();
    assert!(*called.borrow());
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---
