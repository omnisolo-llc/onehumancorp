use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}

#[derive(Debug, Clone)]
pub struct PermissionRule {
    pub tool_name: String,
    pub pattern: String,
    pub action: PermissionAction,
}

pub struct PermissionManager {
    rules: HashMap<String, Vec<PermissionRule>>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, org_id: String, rule: PermissionRule) {
        self.rules.entry(org_id).or_default().push(rule);
    }

    pub fn check_permission(&self, org_id: &str, tool_name: &str, args: &str) -> PermissionAction {
        if let Some(rules) = self.rules.get(org_id) {
            for rule in rules {
                if rule.tool_name == tool_name {
                    if let Ok(re) = Regex::new(&rule.pattern) {
                        if re.is_match(args) {
                            return rule.action.clone();
                        }
                    } else if rule.pattern == args {
                        return rule.action.clone();
                    }
                }
            }
        }
        PermissionAction::Allow
    }
}
