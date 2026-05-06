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
