#[cfg(test)]
mod tests {
    
    #[test]
    fn test_checklist_ui_instantiation() {
        if std::env::var("DISPLAY").is_err() && std::env::var("WAYLAND_DISPLAY").is_err() { return; }

        let _ui = crate::app::WelcomeChecklist::new().unwrap();
        // Just verify it doesn't crash on instantiation now that duplicate handler is removed.
    }
}
