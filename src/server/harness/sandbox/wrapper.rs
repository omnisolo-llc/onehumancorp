use super::manager::SandboxPolicy;
use std::time::Instant;
use crate::telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency};

pub struct BashWrapper {
    read_only_paths: Vec<String>,
    blocked_domains: Vec<String>,
}

impl BashWrapper {
    pub fn new() -> Self {
        BashWrapper {
            read_only_paths: Vec::new(),
            blocked_domains: Vec::new(),
        }
    }

    pub fn update_policy(&mut self, policy: SandboxPolicy) {
        self.read_only_paths = policy.read_only_paths;
        self.blocked_domains = policy.blocked_domains;
    }

    pub fn wrap(&self, cmd: &str) -> String {
        record_bubblewrap_spawn();
        let start = Instant::now();

        // Enforce state management / I/O instrumenting based on config
        let mut preamble = String::from("set -e; ");
        // simple representation of instrumentation
        if !self.read_only_paths.is_empty() {
            // For assistant-class isolation, we simulate read-only enforcement
            preamble.push_str(&format!("export READ_ONLY_PATHS='{}'; ", self.read_only_paths.join(":")));
        }
        if !self.blocked_domains.is_empty() {
            preamble.push_str(&format!("export BLOCKED_DOMAINS='{}'; ", self.blocked_domains.join(",")));
        }

        let wrapped = format!("bash -c \"{}{}\"", preamble, cmd.replace("\"", "\\\""));

        let latency = start.elapsed().as_secs_f64();
        record_bubblewrap_execution_latency(latency);

        wrapped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper_default() {
        let wrapper = BashWrapper::new();
        assert_eq!(wrapper.wrap("echo hello"), "bash -c \"set -e; echo hello\"");
    }

    #[test]
    fn test_wrapper_with_policy() {
        let mut wrapper = BashWrapper::new();
        let policy = SandboxPolicy {
            disabled_commands: vec![],
            disabled_patterns: vec![],
            read_only_paths: vec!["/etc".to_string(), "/var".to_string()],
            blocked_domains: vec!["evil.com".to_string()],
        };
        wrapper.update_policy(policy);

        let wrapped = wrapper.wrap("echo hello");
        assert!(wrapped.contains("export READ_ONLY_PATHS='/etc:/var';"));
        assert!(wrapped.contains("export BLOCKED_DOMAINS='evil.com';"));
        assert!(wrapped.contains("echo hello"));
    }
}
