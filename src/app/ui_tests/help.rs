use crate::app;
use slint::ComponentHandle;

fn create() -> app::HelpCenter { crate::ui_tests::init(); app::HelpCenter::new().unwrap() }

// --- Specialized / Flow Tests ---

#[test] fn help_flow_search_sync() {
    let ui = create();
    ui.set_search_query("billing".into());
    assert_eq!(ui.get_search_query(), "billing");
}

#[test] fn help_xss_query() {
    let ui = create();
    let xss = "<img src=x onerror=alert('help')>";
    ui.set_search_query(xss.into());
    assert_eq!(ui.get_search_query(), xss);
}

#[test] fn help_injection_query() {
    let ui = create();
    let inj = "search'); DROP TABLE articles; --";
    ui.set_search_query(inj.into());
    assert_eq!(ui.get_search_query(), inj);
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_search_query() {
    let ui = create();
    ui.set_search_query("how to add products".into());
    assert_eq!(ui.get_search_query(), "how to add products");
    ui.set_search_query("connecting instagram".into());
    assert_eq!(ui.get_search_query(), "connecting instagram");
    ui.set_search_query("payment methods".into());
    assert_eq!(ui.get_search_query(), "payment methods");
}

#[test]
fn test_help_center_search_filtering() {
    let ui = create();
    let all_articles = vec![
        app::HelpArticle { category: "My Store".into(), title: "How to add products".into(), description: "Learn how to list new items.".into() },
        app::HelpArticle { category: "Getting Started".into(), title: "Set up your store".into(), description: "Follow our simple guide.".into() },
    ];
    let all_articles_rc = std::rc::Rc::new(all_articles.clone());
    ui.set_articles(slint::ModelRc::new(slint::VecModel::from(all_articles)));

    let hc_weak = ui.as_weak();
    ui.on_execute_search(move || {
        if let Some(ui) = hc_weak.upgrade() {
            let query = ui.get_search_query().to_string().to_lowercase();
            let filtered: Vec<app::HelpArticle> = all_articles_rc.iter().filter(|a| {
                a.title.to_lowercase().contains(&query)
            }).cloned().collect();
            ui.set_articles(slint::ModelRc::new(slint::VecModel::from(filtered)));
        }
    });

    ui.set_search_query("add products".into());
    ui.invoke_execute_search();
    use slint::Model;
    assert_eq!(ui.get_articles().row_count(), 1);
    assert_eq!(ui.get_articles().row_data(0).unwrap().title, "How to add products");
}
