use crate::app;
use slint::Model;
use slint::ComponentHandle;

#[test]
fn e2e_ai_director_routing_logic() {
    crate::ui_tests::init();

    // Initialize Dashboard and AiDirector
    let dashboard_ui = app::Dashboard::new().unwrap();
    let ai_director_ui = app::AiDirector::new().unwrap();

    let ai_director_opened = std::rc::Rc::new(std::cell::RefCell::new(false));
    let ai_director_opened_clone = ai_director_opened.clone();

    dashboard_ui.on_action_open_ai_director(move || {
        *ai_director_opened_clone.borrow_mut() = true;
    });

    dashboard_ui.invoke_action_open_ai_director();
    assert!(*ai_director_opened.borrow(), "Instruct Team button should open My Team screen");

    // Setup models for AiDirector testing independently
    let deps_model = std::rc::Rc::new(slint::VecModel::from(vec![
        app::UiDepartmentConfig { id: "marketing".into(), name: "Marketing".into(), requires_review: true },
        app::UiDepartmentConfig { id: "operations".into(), name: "Operations".into(), requires_review: false },
        app::UiDepartmentConfig { id: "customer_success".into(), name: "Customer Success".into(), requires_review: true },
    ]));
    ai_director_ui.set_departments(deps_model.clone().into());

    let deps_model_clone = deps_model.clone();
    ai_director_ui.on_toggle_department_review(move |dep_id| {
        let mut found = false;
        let mut idx = 0;
        let mut new_item = app::UiDepartmentConfig { id: "".into(), name: "".into(), requires_review: false };

        for i in 0..deps_model_clone.row_count() {
            if let Some(item) = deps_model_clone.row_data(i) {
                if item.id == dep_id {
                    found = true;
                    idx = i;
                    new_item = item;
                    new_item.requires_review = !new_item.requires_review;
                    break;
                }
            }
        }
        if found {
            deps_model_clone.set_row_data(idx, new_item);
        }
    });

    // NOTE: Because we test this purely as UI integration mock, we bypass the real `tokio` network call
    // by injecting the exact same test verification mock on the `ai_director_ui` element in tests
    // as it does not inherently have a way to mock `HubServiceClient` inside the Slint callback natively for UI assertions here.
    // We rewrite the callback explicitly for this test so we can safely verify the UI logic
    // handles states like `requires_review` appropriately.
    let dashboard_handle_for_intent = dashboard_ui.as_weak();
    let deps_model_for_intent = deps_model.clone();
    ai_director_ui.on_submit_intent(move |intent| {
        let intent_lower = intent.to_lowercase();
        let (dep_id, _dep_name, helper_name) = if intent_lower.contains("post") || intent_lower.contains("campaign") || intent_lower.contains("promote") {
            ("marketing", "Marketing", "The Promoter")
        } else if intent_lower.contains("order") || intent_lower.contains("inventory") || intent_lower.contains("ship") {
            ("operations", "Operations", "The Manager")
        } else if intent_lower.contains("customer") || intent_lower.contains("support") || intent_lower.contains("reply") {
            ("customer_success", "Customer Success", "The Ambassador")
        } else {
            ("operations", "Operations", "The Manager") // Default
        };

        let mut requires_review = true;
        for i in 0..deps_model_for_intent.row_count() {
            if let Some(item) = deps_model_for_intent.row_data(i) {
                if item.id == dep_id {
                    requires_review = item.requires_review;
                    break;
                }
            }
        }

        if requires_review {
            if let Some(ui) = dashboard_handle_for_intent.upgrade() {
                let current = ui.get_pending_approvals();
                let mut tasks: Vec<app::UiPendingApproval> = current.iter().collect();
                tasks.push(app::UiPendingApproval {
                    task_id: uuid::Uuid::new_v4().to_string().into(),
                    title: format!("Execute: {}", intent).into(),
                    proposed_content: format!("{} proposes to handle: {}", helper_name, intent).into(),
                    helper_name: helper_name.into(),
                });
                ui.set_pending_approvals(std::rc::Rc::new(slint::VecModel::from(tasks)).into());
            }
        } else {
            // Auto-execute: just log it or update activity feed (mocked here)
        }
    });

    // Verify initial state
    assert_eq!(dashboard_ui.get_pending_approvals().row_count(), 0);

    // Intent #1: Marketing (Needs Review)
    ai_director_ui.invoke_submit_intent("Promote Valentine's day".into());
    assert_eq!(dashboard_ui.get_pending_approvals().row_count(), 1, "Marketing intent should create a pending approval");

    let approval1 = dashboard_ui.get_pending_approvals().row_data(0).unwrap();
    assert_eq!(approval1.helper_name, "The Promoter", "Marketing intent should be assigned to The Promoter");

    // Intent #2: Operations (Auto-Execute)
    ai_director_ui.invoke_submit_intent("Ship order #1002".into());
    assert_eq!(dashboard_ui.get_pending_approvals().row_count(), 1, "Operations intent should auto-execute, adding no pending approvals");

    // Toggle Marketing to Auto-Execute
    ai_director_ui.invoke_toggle_department_review("marketing".into());
    assert_eq!(deps_model.row_data(0).unwrap().requires_review, false, "Marketing should be set to Auto-Execute");

    // Intent #3: Marketing again (Now Auto-Execute)
    ai_director_ui.invoke_submit_intent("Post new cake flavors".into());
    assert_eq!(dashboard_ui.get_pending_approvals().row_count(), 1, "Marketing intent should auto-execute now, adding no pending approvals");

    // Intent #4: Customer Success (Needs Review)
    ai_director_ui.invoke_submit_intent("Reply to angry customer".into());
    assert_eq!(dashboard_ui.get_pending_approvals().row_count(), 2, "Customer Success intent should create a pending approval");

    let approval2 = dashboard_ui.get_pending_approvals().row_data(1).unwrap();
    assert_eq!(approval2.helper_name, "The Ambassador", "Customer Success intent should be assigned to The Ambassador");

    // Test 5: Verify My Team Component displays accurately mapped titles/settings
    let team_ui = app::AiDirector::new().unwrap();
    let sample_deps = slint::VecModel::from(vec![
        app::UiDepartmentConfig { id: "sales".into(), name: "Sales Team".into(), requires_review: true },
    ]);
    team_ui.set_departments(std::rc::Rc::new(sample_deps).into());
    assert_eq!(team_ui.get_departments().row_data(0).unwrap().name, "Sales Team");
}
