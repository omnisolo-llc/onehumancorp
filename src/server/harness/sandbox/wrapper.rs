use super::manager::SandboxPolicy;
use tracing::instrument;

pub struct BashWrapper {
    read_only_paths: Vec<String>,
    blocked_domains: Vec<String>,
    dangerously_disable_sandbox: bool,
}

impl BashWrapper {
    pub fn new() -> Self {
        BashWrapper {
            read_only_paths: Vec::new(),
            blocked_domains: Vec::new(),
            dangerously_disable_sandbox: false,
        }
    }

    pub fn update_policy(&mut self, policy: SandboxPolicy) {
        self.read_only_paths = policy.read_only_paths;
        self.blocked_domains = policy.blocked_domains;
        self.dangerously_disable_sandbox = policy.dangerously_disable_sandbox;
    }

    pub fn dangerously_disable_sandbox(&self) -> bool {
        self.dangerously_disable_sandbox
    }

    #[instrument(skip(self), fields(command = %cmd))]
    pub fn wrap(&self, cmd: &str) -> String {
        if self.dangerously_disable_sandbox {
            return format!("bash -c \"{}\"", cmd.replace("\"", "\\\""));
        }

        let mut bwrap_args = vec![
            "bwrap".to_string(),
            "--ro-bind".to_string(), "/usr".to_string(), "/usr".to_string(),
            "--ro-bind".to_string(), "/lib".to_string(), "/lib".to_string(),
            "--bind".to_string(), ".".to_string(), ".".to_string(),
            "--unshare-net".to_string(),
        ];

        let mut preamble = String::from("set -e; umask 077; ");
        if !self.read_only_paths.is_empty() {
            for path in &self.read_only_paths {
                bwrap_args.push("--ro-bind".to_string());
                bwrap_args.push(path.to_string());
                bwrap_args.push(path.to_string());
            }
            preamble.push_str(&format!("export READ_ONLY_PATHS='{}'; ", self.read_only_paths.join(":")));
        }
        if !self.blocked_domains.is_empty() {
            preamble.push_str(&format!("export BLOCKED_DOMAINS='{}'; ", self.blocked_domains.join(",")));
        }

        format!("{} bash -c \"{}{}\"", bwrap_args.join(" "), preamble, cmd.replace("\"", "\\\""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapper_default() {
        let wrapper = BashWrapper::new();
        assert_eq!(wrapper.wrap("echo hello"), "bwrap --ro-bind /usr /usr --ro-bind /lib /lib --bind . . --unshare-net bash -c \"set -e; umask 077; echo hello\"");
    }

    #[test]
    fn test_wrapper_with_policy() {
        let mut wrapper = BashWrapper::new();
        let policy = SandboxPolicy {
            disabled_commands: vec![],
            disabled_patterns: vec![],
            read_only_paths: vec!["/etc".to_string(), "/var".to_string()],
            blocked_domains: vec!["evil.com".to_string()],
            dangerously_disable_sandbox: false,
        };
        wrapper.update_policy(policy);

        let wrapped = wrapper.wrap("echo hello");
        assert!(wrapped.contains("export READ_ONLY_PATHS='/etc:/var';"));
        assert!(wrapped.contains("export BLOCKED_DOMAINS='evil.com';"));
        assert!(wrapped.contains("echo hello"));
        assert!(wrapped.contains("bwrap --ro-bind /usr /usr --ro-bind /lib /lib --bind . . --unshare-net"));
        assert!(wrapped.contains("--ro-bind /etc /etc"));
        assert!(wrapped.contains("--ro-bind /var /var"));
    }

    #[test]
    fn test_dangerously_disable_sandbox() {
        let mut wrapper = BashWrapper::new();
        let policy = SandboxPolicy {
            disabled_commands: vec![],
            disabled_patterns: vec![],
            read_only_paths: vec!["/etc".to_string()],
            blocked_domains: vec![],
            dangerously_disable_sandbox: true,
        };
        wrapper.update_policy(policy);

        let wrapped = wrapper.wrap("echo hello");
        assert_eq!(wrapped, "bash -c \"echo hello\"");
    }
}
