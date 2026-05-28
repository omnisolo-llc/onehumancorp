use regex::Regex;
use super::manager::SandboxPolicy;

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

    pub fn update_policy(&mut self, policy: SandboxPolicy) {
        self.disabled_commands.extend(policy.disabled_commands);
        for pattern in policy.disabled_patterns {
            if let Ok(re) = Regex::new(&pattern) {
                self.disabled_patterns.push(re);
            }
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

    #[test]
    fn test_update_policy() {
        let mut evaluator = PermissionEvaluator::new();
        let policy = SandboxPolicy {
            disabled_commands: vec!["curl".to_string()],
            disabled_patterns: vec![r"(?i)\bwget\b".to_string()],
            read_only_paths: vec![],
            blocked_domains: vec![], seccomp_fd: None, socat_socket_path: None, socat_proxy_port: None,
        };
        evaluator.update_policy(policy);

        assert!(!evaluator.evaluate("curl http://example.com"));
        assert!(!evaluator.evaluate("wget http://example.com"));
        assert!(evaluator.evaluate("echo 'hello world'"));
    }
}
