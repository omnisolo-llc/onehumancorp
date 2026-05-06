use crate::app;

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
fn test_documentation_features_present() {
    crate::ui_tests::init();
    // This test explicitly verifies that the requested components are exposed.
    let _hc = crate::app::HelpCenter::new().unwrap();
    let _chat = crate::app::AiHelpChat::new().unwrap();
    let _walkthrough = crate::app::InteractiveWalkthrough::new().unwrap();
    let _video = crate::app::VideoTutorials::new().unwrap();
    let _api_docs = crate::app::ApiDocs::new().unwrap();
    let _release_notes = crate::app::ReleaseNotes::new().unwrap();
}
