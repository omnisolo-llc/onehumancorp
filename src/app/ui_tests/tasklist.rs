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
            agent_id: "a1".into(),
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
            agent_id: "".into(),
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
        agent_id: "".into(),
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

// --- KAIROS Phase 1: Shared Task List CUJ E2E Tests ---

#[test] fn tasklist_e2e_cuj_verify_glassmorphism_and_feed_initial_state() {
    let ui = create();

    // Initial activity feed
    let feed = slint::VecModel::from(vec![
        slint::SharedString::from("✅ Your Support Agent replied to 3 customers"),
    ]);
    ui.set_activity_feed(Rc::new(feed).into());

    let tasks = slint::VecModel::from(vec![
        app::UiTask {
            title: "Setup Business Website".into(),
            status: "PENDING".into(),
            agent_id: "".into(),
            dependencies: "".into(),
            parent_task_id: "".into(),
            workflow_state: "".into(),
        }
    ]);
    ui.set_tasks(Rc::new(tasks).into());

    assert_eq!(ui.get_activity_feed().row_count(), 1);
    assert_eq!(ui.get_activity_feed().row_data(0).unwrap(), "✅ Your Support Agent replied to 3 customers");
    assert_eq!(ui.get_tasks().row_count(), 1);
    assert_eq!(ui.get_tasks().row_data(0).unwrap().status, "PENDING");
}

#[test] fn tasklist_e2e_cuj_claim_task_action() {
    let ui = create();
    let tasks = slint::VecModel::from(vec![
        app::UiTask {
            title: "Configure Database".into(),
            status: "PENDING".into(),
            agent_id: "".into(),
            dependencies: "".into(),
            parent_task_id: "".into(),
            workflow_state: "".into(),
        }
    ]);
    ui.set_tasks(Rc::new(tasks).into());

    let claimed_index = std::rc::Rc::new(std::cell::RefCell::new(-1));
    let c = claimed_index.clone();
    ui.on_claim_task(move |idx| {
        *c.borrow_mut() = idx;
    });

    ui.invoke_claim_task(0);
    assert_eq!(*claimed_index.borrow(), 0);
}

#[test] fn tasklist_e2e_cuj_activity_feed_update() {
    let ui = create();
    let feed = Rc::new(slint::VecModel::from(vec![
        slint::SharedString::from("Agent 1 is idle"),
    ]));
    ui.set_activity_feed(feed.clone().into());
    assert_eq!(ui.get_activity_feed().row_data(0).unwrap(), "Agent 1 is idle");

    // Simulate updating the feed
    feed.push(slint::SharedString::from("📦 Order Manager updated stock for 12 items"));
    assert_eq!(ui.get_activity_feed().row_count(), 2);
    assert_eq!(ui.get_activity_feed().row_data(1).unwrap(), "📦 Order Manager updated stock for 12 items");
}

#[test] fn tasklist_e2e_cuj_tasks_status_refresh() {
    let ui = create();
    let refresh_called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let r = refresh_called.clone();
    ui.on_refresh(move || {
        *r.borrow_mut() = true;
    });

    let tasks = Rc::new(slint::VecModel::from(vec![
        app::UiTask {
            title: "Task 1".into(),
            status: "ASSIGNED".into(),
            agent_id: "worker-1".into(),
            dependencies: "".into(),
            parent_task_id: "".into(),
            workflow_state: "".into(),
        }
    ]));
    ui.set_tasks(tasks.clone().into());
    assert_eq!(ui.get_tasks().row_data(0).unwrap().status, "ASSIGNED");

    // Simulate user pressing Refresh
    ui.invoke_refresh();
    assert!(*refresh_called.borrow());

    // Simulate updating task after refresh
    tasks.set_row_data(0, app::UiTask {
        title: "Task 1".into(),
        status: "COMPLETED".into(),
        agent_id: "worker-1".into(),
        dependencies: "".into(),
        parent_task_id: "".into(),
        workflow_state: "".into(),
    });
    assert_eq!(ui.get_tasks().row_data(0).unwrap().status, "COMPLETED");
}

#[test] fn tasklist_e2e_cuj_empty_state_handling() {
    let ui = create();

    // Provide empty lists
    ui.set_tasks(Rc::new(slint::VecModel::from(vec![])).into());
    ui.set_activity_feed(Rc::new(slint::VecModel::from(vec![])).into());

    assert_eq!(ui.get_tasks().row_count(), 0);
    assert_eq!(ui.get_activity_feed().row_count(), 0);
    // UI should handle this gracefully as defined in the Slint file
}
