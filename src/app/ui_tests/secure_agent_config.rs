use crate::app;

fn create() -> app::SecureHelperConfig { crate::ui_tests::init(); app::SecureHelperConfig::new().unwrap() }

// --- Hacking / Corner Cases ---

#[test] fn secure_xss_token() {
    let ui = create();
    let xss = "<script>alert('token')</script>";
    ui.set_token(xss.into());
    assert_eq!(ui.get_token(), xss);
}

#[test] fn secure_injection_error() {
    let ui = create();
    let inj = "Error'); DROP TABLE secrets; --";
    ui.set_error_text(inj.into());
    assert_eq!(ui.get_error_text(), inj);
}

#[test] fn secure_long_token() {
    let ui = create();
    let long = "spiffe://ohc.os/helper/".to_string() + &"a".repeat(1000);
    ui.set_token(long.clone().into());
    assert_eq!(ui.get_token(), long);
}

// --- Interaction / Flow Tests ---

#[test] fn secure_flow_save_callback() {
    let ui = create();
    let called_token = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let c = called_token.clone();
    ui.on_save_config(move |t| { *c.borrow_mut() = t.to_string(); });
    
    ui.set_token("my-token".into());
    ui.invoke_save_config("my-token".into());
    assert_eq!(*called_token.borrow(), "my-token");
}

// --- Unique Scenarios with Verification ---

// --- Consolidated Verified Tests ---

#[test]
fn create_verify_token() {
    let ui = create();
    ui.set_token("valid-token-123".into());
    assert_eq!(ui.get_token(), "valid-token-123");
    ui.set_token("spiffe://test/1".into());
    assert_eq!(ui.get_token(), "spiffe://test/1");
    ui.set_token("t11".into());
    assert_eq!(ui.get_token(), "t11");
}

#[test]
fn create_verify_error_text() {
    let ui = create();
    ui.set_error_text("Invalid SPIFFE format".into());
    assert_eq!(ui.get_error_text(), "Invalid SPIFFE format");
    ui.set_error_text("e31".into());
    assert_eq!(ui.get_error_text(), "e31");
    ui.set_error_text("e32".into());
    assert_eq!(ui.get_error_text(), "e32");
}
