use std::collections::HashMap;

pub struct TooltipRegistry;

impl TooltipRegistry {
    pub fn get(key: &str) -> String {
        match key {
            "help_btn" => "Click for help".to_string(),
            "save_btn" => "Save your changes".to_string(),
            _ => "".to_string(),
        }
    }
}
