pub fn is_standalone_runtime() -> bool {
    fn parse_bool(value: &str) -> Option<bool> {
        match value.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "y" | "on" => Some(true),
            "0" | "false" | "no" | "n" | "off" => Some(false),
            _ => None,
        }
    }

    if let Ok(value) = std::env::var("OHC_STANDALONE_MODE") {
        if let Some(parsed) = parse_bool(&value) {
            return parsed;
        }
    }
    if let Ok(value) = std::env::var("OHC_SOURCE_MODE") {
        match value.trim().to_ascii_lowercase().as_str() {
            "standalone" | "desktop" => return true,
            "cloud" | "cluster" | "headless" => return false,
            _ => {}
        }
    }

    true
}

pub fn is_cloud_runtime() -> bool {
    !is_standalone_runtime()
}
