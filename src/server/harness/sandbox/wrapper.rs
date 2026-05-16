use super::manager::SandboxPolicy;

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

        let inner_cmd = format!("bash -c \"{}{}\"", preamble, cmd.replace("\"", "\\\""));

        #[cfg(target_os = "linux")]
        {
            format!("bwrap --ro-bind /bin /bin --ro-bind /usr /usr --ro-bind /lib /lib --ro-bind-try /lib64 /lib64 --ro-bind /etc/alternatives /etc/alternatives --dev /dev --proc /proc --tmpfs /tmp --unshare-pid --unshare-ipc -- {}", inner_cmd)
        }
        #[cfg(target_os = "macos")]
        {
            let profile = "(version 1)\n(deny default)\n(allow process-exec (regex #\"^/bin/.*\"))\n(allow file-read* (subpath \"/bin\"))\n";
            format!("sandbox-exec -p '{}' {}", profile, inner_cmd)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            inner_cmd
        }
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
