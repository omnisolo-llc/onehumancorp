pub fn is_standalone_mode() -> bool {
    std::env::var("STANDALONE_MODE").unwrap_or_else(|_| "true".to_string()) == "true" ||
    std::env::var("OHC_STANDALONE").unwrap_or_else(|_| "false".to_string()) == "true"
}

pub fn is_cloud_mode() -> bool {
    !is_standalone_mode()
}
