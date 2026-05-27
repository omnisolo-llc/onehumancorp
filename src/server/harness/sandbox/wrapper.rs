use super::manager::SandboxPolicy;
#[cfg(target_os = "linux")]
use crate::harness::network::proxy::NetworkBridgeProxy;

pub struct BashWrapper {
    read_only_paths: Vec<String>,
    blocked_domains: Vec<String>,
    #[cfg(target_os = "linux")]
    network_proxy: Option<NetworkBridgeProxy>,
}

impl BashWrapper {
    pub fn new() -> Self {
        BashWrapper {
            read_only_paths: Vec::new(),
            blocked_domains: Vec::new(),
            #[cfg(target_os = "linux")]
            network_proxy: None,
        }
    }

    pub fn update_policy(&mut self, policy: SandboxPolicy) {
        self.read_only_paths = policy.read_only_paths;
        self.blocked_domains = policy.blocked_domains;

        #[cfg(target_os = "linux")]
        {
            if !self.blocked_domains.is_empty() {
                let mut proxy = NetworkBridgeProxy::new(self.blocked_domains.clone());
                let _ = proxy.start();
                self.network_proxy = Some(proxy);
            } else {
                self.network_proxy = None;
            }
        }
    }

    pub fn wrap(&self, cmd: &str) -> String {
        // Enforce state management / I/O instrumenting based on config
        let mut preamble = String::from("set -e; umask 077; ");
        // simple representation of instrumentation
        if !self.read_only_paths.is_empty() {
            // For assistant-class isolation, we simulate read-only enforcement
            preamble.push_str(&format!("export READ_ONLY_PATHS='{}'; ", self.read_only_paths.join(":")));
        }
        if !self.blocked_domains.is_empty() {
            preamble.push_str(&format!("export BLOCKED_DOMAINS='{}'; ", self.blocked_domains.join(",")));
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(proxy) = &self.network_proxy {
                preamble.push_str(&format!("export HTTP_PROXY='unix://{}'; ", proxy.socket_path()));
                preamble.push_str(&format!("export HTTPS_PROXY='unix://{}'; ", proxy.socket_path()));
                preamble.push_str(&format!("export ALL_PROXY='unix://{}'; ", proxy.socket_path()));
            }
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
        };
        wrapper.update_policy(policy);

        let wrapped = wrapper.wrap("echo hello");
        assert!(wrapped.contains("export READ_ONLY_PATHS='/etc:/var';"));
        assert!(wrapped.contains("export BLOCKED_DOMAINS='evil.com';"));
        #[cfg(target_os = "linux")]
        assert!(wrapped.contains("export HTTP_PROXY='unix:///tmp/ohc-agent-http-"));
        assert!(wrapped.contains("echo hello"));
    }
}
