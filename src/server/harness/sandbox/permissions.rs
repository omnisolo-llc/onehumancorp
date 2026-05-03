use regex::Regex;

pub struct PermissionEvaluator {
    disabled_commands: Vec<String>,
    disabled_patterns: Vec<Regex>,
}

impl PermissionEvaluator {
    pub fn new() -> Self {
        let disabled_commands = vec![
            "rm -rf /".to_string(),
            "mkfs".to_string(),
        ];

        let disabled_patterns = vec![
            Regex::new(r"(?i)\bsudo\b").unwrap(),
            Regex::new(r"(?i)\bchown\b").unwrap(),
        ];

        PermissionEvaluator {
            disabled_commands,
            disabled_patterns,
        }
    }

    pub fn evaluate(&self, cmd: &str) -> bool {
        for disabled in &self.disabled_commands {
            if cmd.contains(disabled) {
                return false;
            }
        }

        for pattern in &self.disabled_patterns {
            if pattern.is_match(cmd) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_command() {
        let evaluator = PermissionEvaluator::new();
        assert!(evaluator.evaluate("echo 'hello world'"));
        assert!(evaluator.evaluate("ls -l /tmp"));
    }

    #[test]
    fn test_disabled_command() {
        let evaluator = PermissionEvaluator::new();
        assert!(!evaluator.evaluate("rm -rf /"));
        assert!(!evaluator.evaluate("mkfs.ext4 /dev/sda1"));
    }

    #[test]
    fn test_disabled_pattern() {
        let evaluator = PermissionEvaluator::new();
        assert!(!evaluator.evaluate("sudo apt-get update"));
        assert!(!evaluator.evaluate("SUDO rm -rf /tmp/*"));
        assert!(!evaluator.evaluate("chown root:root /etc/passwd"));
    }
}
