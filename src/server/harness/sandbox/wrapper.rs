use super::manager::SandboxPolicy;

pub struct BashWrapper {
    read_only_paths: Vec<String>,
    blocked_domains: Vec<String>,
    socat_socket_path: Option<String>,
    socat_proxy_port: Option<u16>,
}

impl BashWrapper {
    pub fn new() -> Self {
        BashWrapper {
            read_only_paths: Vec::new(),
            blocked_domains: Vec::new(),
            socat_socket_path: None,
            socat_proxy_port: None,
        }
    }

    pub fn update_policy(&mut self, policy: SandboxPolicy) {
        self.read_only_paths = policy.read_only_paths;
        self.blocked_domains = policy.blocked_domains;
        self.socat_socket_path = policy.socat_socket_path;
        self.socat_proxy_port = policy.socat_proxy_port;
    }

    pub fn wrap(&self, cmd: &str) -> String {
        // Enforce state management / I/O instrumenting based on config
        let mut preamble = String::new();
        if let (Some(path), Some(port)) = (&self.socat_socket_path, self.socat_proxy_port) {
            let escaped_path = path.replace("'", "'\\''");
            preamble.push_str(&format!(
                "socat UNIX-LISTEN:'{}',fork TCP:127.0.0.1:{} & \n\
                 SOCAT_PID=$!\n\
                 trap 'kill $SOCAT_PID 2>/dev/null || true' EXIT\n\
                 while [ ! -S '{}' ]; do sleep 0.05; done\n",
                escaped_path, port, escaped_path
            ));
        }

        preamble.push_str("set -e; umask 077; ");
        // simple representation of instrumentation
        if !self.read_only_paths.is_empty() {
            // For assistant-class isolation, we simulate read-only enforcement
            preamble.push_str(&format!("export READ_ONLY_PATHS='{}'; ", self.read_only_paths.join(":")));
        }
        if !self.blocked_domains.is_empty() {
            preamble.push_str(&format!("export BLOCKED_DOMAINS='{}'; ", self.blocked_domains.join(",")));
        }

        format!("bash -c \"{}{}\"", preamble, cmd.replace("\"", "\\\""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper_default() {
        let wrapper = BashWrapper::new();
        assert_eq!(wrapper.wrap("echo hello"), "bash -c \"set -e; umask 077; echo hello\"");
    }

    #[test]
    fn test_wrapper_with_policy() {
        let mut wrapper = BashWrapper::new();
        let policy = SandboxPolicy {
            disabled_commands: vec![],
            disabled_patterns: vec![],
            read_only_paths: vec!["/etc".to_string(), "/var".to_string()],
            blocked_domains: vec!["evil.com".to_string()],
            seccomp_fd: None,
            socat_socket_path: Some("/tmp/test.sock".to_string()),
            socat_proxy_port: Some(8080),
        };
        wrapper.update_policy(policy);

        let wrapped = wrapper.wrap("echo hello");
        assert!(wrapped.contains("export READ_ONLY_PATHS='/etc:/var';"));
        assert!(wrapped.contains("export BLOCKED_DOMAINS='evil.com';"));
        assert!(wrapped.contains("echo hello"));
        assert!(wrapped.contains("socat UNIX-LISTEN:'/tmp/test.sock',fork TCP:127.0.0.1:8080 & \nSOCAT_PID=$!\ntrap 'kill $SOCAT_PID 2>/dev/null || true' EXIT\nwhile [ ! -S '/tmp/test.sock' ]; do sleep 0.05; done"));
    }
}
