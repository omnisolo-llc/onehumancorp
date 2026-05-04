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

#[test] fn help_flow_execute_search_callback() {
    let ui = create();
    let called = std::rc::Rc::new(std::cell::RefCell::new(false));
    let c = called.clone();
    ui.on_execute_search(move || { *c.borrow_mut() = true; });

    ui.set_search_query("test callback".into());
    ui.invoke_execute_search();

    assert!(*called.borrow(), "execute_search callback should be invoked");
}



#[test] fn help_flow_empty_search() {
    let ui = create();
    ui.set_search_query("".into());
    assert_eq!(ui.get_search_query(), "");
}

#[test] fn help_flow_long_search() {
    let ui = create();
    let long_query = "a".repeat(1000);
    ui.set_search_query(long_query.clone().into());
    assert_eq!(ui.get_search_query(), long_query);
}

#[test] fn help_flow_special_chars_search() {
    let ui = create();
    ui.set_search_query(r#"!@#$%^&*()_+=-{}[]|:;"\'<>,.?/~`"#.into());
    assert_eq!(ui.get_search_query(), r#"!@#$%^&*()_+=-{}[]|:;"\'<>,.?/~`"#);
}

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
