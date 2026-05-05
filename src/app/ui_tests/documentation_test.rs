#[cfg(test)]
mod tests {
    use crate::app;

    #[test]
    fn test_e2e_documentation_help_center_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let login_ui = app::Login::new().unwrap();
        login_ui.invoke_login("test@example.com".into(), "password123".into());
        let dashboard_ui = app::Dashboard::new().unwrap();
        let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let opened_clone = opened.clone();
        dashboard_ui.on_open_help_center(move || { *opened_clone.borrow_mut() = true; });
        dashboard_ui.invoke_open_help_center();
        assert!(*opened.borrow(), "Help Center should open");
    }

    #[test]
    fn test_e2e_documentation_interactive_walkthrough_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let dashboard_ui = app::Dashboard::new().unwrap();
        let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let opened_clone = opened.clone();
        dashboard_ui.on_open_interactive_walkthrough(move || { *opened_clone.borrow_mut() = true; });
        dashboard_ui.invoke_open_interactive_walkthrough();
        assert!(*opened.borrow(), "Walkthrough should open");
    }

    #[test]
    fn test_e2e_documentation_ai_chat_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let dashboard_ui = app::Dashboard::new().unwrap();
        let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let opened_clone = opened.clone();
        dashboard_ui.on_open_ai_chat(move || { *opened_clone.borrow_mut() = true; });
        dashboard_ui.invoke_open_ai_chat();
        assert!(*opened.borrow(), "AI Chat should open");
    }

    #[test]
    fn test_e2e_documentation_video_tutorials_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let dashboard_ui = app::Dashboard::new().unwrap();
        let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let opened_clone = opened.clone();
        dashboard_ui.on_open_video_tutorials(move || { *opened_clone.borrow_mut() = true; });
        dashboard_ui.invoke_open_video_tutorials();
        assert!(*opened.borrow(), "Video Tutorials should open");
    }

    #[test]
    fn test_e2e_documentation_api_docs_flow() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }
        let dashboard_ui = app::Dashboard::new().unwrap();
        let opened = std::rc::Rc::new(std::cell::RefCell::new(false));
        let opened_clone = opened.clone();
        dashboard_ui.on_open_api_docs(move || { *opened_clone.borrow_mut() = true; });
        dashboard_ui.invoke_open_api_docs();
        assert!(*opened.borrow(), "API Docs should open");
    }
}
