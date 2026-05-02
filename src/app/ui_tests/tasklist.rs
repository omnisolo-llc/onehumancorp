use crate::app;
use slint::ComponentHandle;
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

macro_rules! test_v_t {
    ($id:ident, $title:expr, $status:expr) => {
        #[test] fn $id() {
            let ui = create();
            let t = slint::VecModel::from(vec![app::UiTask {
                title: $title.into(),
                status: $status.into(),
                agent_id: "".into(),
                dependencies: "".into(),
                parent_task_id: "".into(),
                workflow_state: "".into(),
            }]);
            ui.set_tasks(Rc::new(t).into());
            assert_eq!(ui.get_tasks().row_data(0).unwrap().title, $title);
            assert_eq!(ui.get_tasks().row_data(0).unwrap().status, $status);
        }
    };
}

test_v_t!(u1, "Task One", "In Progress");
test_v_t!(u2, "Task Two", "Completed");
test_v_t!(u3, "Task Three", "Blocked");

test_v_t!(u11, "t11", "s11");
test_v_t!(u12, "t12", "s12");
test_v_t!(u13, "t13", "s13");
test_v_t!(u14, "t14", "s14");
test_v_t!(u15, "t15", "s15");
test_v_t!(u16, "t16", "s16");
test_v_t!(u17, "t17", "s17");
test_v_t!(u18, "t18", "s18");
test_v_t!(u19, "t19", "s19");
test_v_t!(u20, "t20", "s20");

test_v_t!(u21, "🚀 Launch Rocket", "Ready");
test_v_t!(u22, "Task with 'Quotes'", "Quoted");
test_v_t!(u23, "Task with ; Semicolon", "Semis");
test_v_t!(u24, "", "");
test_v_t!(u25, "Very Long Task Title ".repeat(10), "Ok");

test_v_t!(u31, "t31", "s31");
test_v_t!(u32, "t32", "s32");
test_v_t!(u33, "t33", "s33");
test_v_t!(u34, "t34", "s34");
test_v_t!(u35, "t35", "s35");
test_v_t!(u36, "t36", "s36");
test_v_t!(u37, "t37", "s37");
test_v_t!(u38, "t38", "s38");
test_v_t!(u39, "t39", "s39");
test_v_t!(u40, "t40", "s40");

test_v_t!(u41, "t41", "s41");
test_v_t!(u42, "t42", "s42");
test_v_t!(u43, "t43", "s43");
test_v_t!(u44, "t44", "s44");
test_v_t!(u45, "t45", "s45");
test_v_t!(u46, "t46", "s46");
test_v_t!(u47, "t47", "s47");
test_v_t!(u48, "t48", "s48");
test_v_t!(u49, "t49", "s49");
test_v_t!(u50, "t50", "s50");

test_v_t!(u51, "t51", "s51");
test_v_t!(u52, "t52", "s52");
test_v_t!(u53, "t53", "s53");
test_v_t!(u54, "t54", "s54");
test_v_t!(u55, "t55", "s55");
test_v_t!(u56, "t56", "s56");
test_v_t!(u57, "t57", "s57");
test_v_t!(u58, "t58", "s58");
test_v_t!(u59, "t59", "s59");
test_v_t!(u60, "t60", "s60");

test_v_t!(u61, "t61", "s61");
test_v_t!(u62, "t62", "s62");
test_v_t!(u63, "t63", "s63");
test_v_t!(u64, "t64", "s64");
test_v_t!(u65, "t65", "s65");
test_v_t!(u66, "t66", "s66");
test_v_t!(u67, "t67", "s67");
test_v_t!(u68, "t68", "s68");
test_v_t!(u69, "t69", "s69");
test_v_t!(u70, "t70", "s70");

test_v_t!(u71, "t71", "s71");
test_v_t!(u72, "t72", "s72");
test_v_t!(u73, "t73", "s73");
test_v_t!(u74, "t74", "s74");
test_v_t!(u75, "t75", "s75");
test_v_t!(u76, "t76", "s76");
test_v_t!(u77, "t77", "s77");
test_v_t!(u78, "t78", "s78");
test_v_t!(u79, "t79", "s79");
test_v_t!(u80, "t80", "s80");
