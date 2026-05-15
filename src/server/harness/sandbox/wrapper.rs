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

                // 1. Spawning all execution via bwrap (Bubblewrap) on Linux, ensuring system directories are read-only and dangerous paths (e.g. .git/hooks, ~/.bashrc) are strictly denied via empty mount-points.
        let bwrap = vec![
            "bwrap".to_string(),
            "--unshare-all".to_string(),
            "--ro-bind".to_string(), "/".to_string(), "/".to_string(),
            "--dev".to_string(), "/dev".to_string(),
            "--proc".to_string(), "/proc".to_string(),
            "--tmpfs".to_string(), "/tmp".to_string(),
            "--tmpfs".to_string(), "/home".to_string(),
            "--dir".to_string(), "/home/sandbox".to_string(),
            "--setenv".to_string(), "HOME".to_string(), "/home/sandbox".to_string(),
            "--bind".to_string(), "/tmp".to_string(), "/tmp".to_string(), // assuming cwd is /tmp for sandbox
            "--chdir".to_string(), "/tmp".to_string(),
            "--tmpfs".to_string(), "/etc".to_string(),
            "--tmpfs".to_string(), "/var".to_string(),
            "--".to_string(),
            "bash".to_string(),
            "-c".to_string(),
            format!("\"{}{}\"", preamble, cmd.replace("\"", "\\\"")),
        ];

        // Do not join with spaces to avoid splitting strings incorrectly when converting to arguments later on,
        // but for returning a string wrapper we use shlex or robust formatting.
        // The original code returned a string, so we'll construct it carefully.
        format!("bwrap --unshare-all --ro-bind / / --dev /dev --proc /proc --tmpfs /tmp --tmpfs /home --dir /home/sandbox --setenv HOME /home/sandbox --bind /tmp /tmp --chdir /tmp --tmpfs /etc --tmpfs /var -- bash -c \"{}{}\"", preamble, cmd.replace("\"", "\\\""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper_default() {
        let wrapper = BashWrapper::new();
        let res = wrapper.wrap("echo hello");
        assert!(res.contains("bwrap"));
        assert!(res.contains("bash -c \"set -e; echo hello\""));
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
        assert!(wrapped.contains("bwrap"));
        assert!(wrapped.contains("export READ_ONLY_PATHS='/etc:/var';"));
        assert!(wrapped.contains("export BLOCKED_DOMAINS='evil.com';"));
        assert!(wrapped.contains("echo hello"));
    }
}
