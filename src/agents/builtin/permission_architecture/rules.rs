use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub tool_name: String,
    // Simple path prefix matching for arguments containing "path" or "filepath"
    pub allowed_path_prefix: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuleEngine {
    auto_approve_rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self {
            auto_approve_rules: rules,
        }
    }

    pub fn check_auto_approve(&self, tool_name: &str, arguments: &serde_json::Value) -> bool {
        for rule in &self.auto_approve_rules {
            if rule.tool_name == tool_name {
                if let Some(prefix) = &rule.allowed_path_prefix {
                    let path_val = arguments.get("path")
                        .or_else(|| arguments.get("filepath"))
                        .or_else(|| arguments.get("file_path"));

                    if let Some(path_str) = path_val.and_then(|v| v.as_str()) {
                        if path_str.starts_with(prefix) {
                            return true;
                        }
                    }
                } else {
                    // No prefix constraint, just match on tool name
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_engine_exact_tool_match() {
        let rules = vec![Rule {
            tool_name: "read_file".to_string(),
            allowed_path_prefix: None,
        }];
        let engine = RuleEngine::new(rules);

        assert!(engine.check_auto_approve("read_file", &serde_json::json!({})));
        assert!(!engine.check_auto_approve("write_file", &serde_json::json!({})));
    }

    #[test]
    fn test_rule_engine_path_prefix_match() {
        let rules = vec![Rule {
            tool_name: "read_file".to_string(),
            allowed_path_prefix: Some("/tmp/".to_string()),
        }];
        let engine = RuleEngine::new(rules);

        assert!(engine.check_auto_approve("read_file", &serde_json::json!({"path": "/tmp/test.txt"})));
        assert!(engine.check_auto_approve("read_file", &serde_json::json!({"filepath": "/tmp/sub/file"})));
        assert!(!engine.check_auto_approve("read_file", &serde_json::json!({"path": "/etc/passwd"})));
        assert!(!engine.check_auto_approve("read_file", &serde_json::json!({}))); // no path arg provided
    }
}
