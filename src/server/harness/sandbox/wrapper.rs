pub struct BashWrapper;

impl BashWrapper {
    pub fn new() -> Self {
        BashWrapper
    }

    pub fn wrap_args(&self, cmd_args: &[String]) -> Vec<String> {
        let joined_args = cmd_args.join(" ");
        // Avoid naive shell interpolation and command injection risks.
        // Note: For real safety, we should pass arguments properly without a single shell string,
        // but since we need to enforce 'set -e' and wrap it, we escape single quotes.
        let safe_cmd = joined_args.replace("'", "'\''");
        let bash_c_arg = format!("set -e; {}", safe_cmd);

        let mut base_cmd = vec!["bash".to_string(), "-c".to_string(), bash_c_arg];

        #[cfg(target_os = "linux")]
        {
            let mut bwrap = vec![
                "bwrap".to_string(),
                "--unshare-all".to_string(),
                "--share-net".to_string(),
                "--ro-bind".to_string(), "/".to_string(), "/".to_string(),
                "--bind".to_string(), "/tmp/agent_workspace".to_string(), "/tmp/agent_workspace".to_string(),
                "--dev".to_string(), "/dev".to_string(),
            ];
            bwrap.append(&mut base_cmd);
            bwrap
        }
        #[cfg(target_os = "macos")]
        {
            let mut sandbox = vec![
                "sandbox-exec".to_string(),
                "-p".to_string(),
                "(version 1)(allow default)(deny file-write*)".to_string(),
            ];
            sandbox.append(&mut base_cmd);
            sandbox
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            base_cmd
        }
    }

    // Fallback for tests expecting string wrapping
    pub fn wrap(&self, cmd: &str) -> String {
        let args = vec![cmd.to_string()];
        let wrapped_args = self.wrap_args(&args);

        // Very basic join for string-based test cases that were already added
        let joined = wrapped_args.into_iter().map(|s| {
            if s.contains(' ') || s.contains(';') || s.contains('\'') {
                format!("'{}'", s.replace("'", "'\''"))
            } else {
                s
            }
        }).collect::<Vec<String>>().join(" ");

        joined
    }
}

impl Default for BashWrapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_basic() {
        let wrapper = BashWrapper::new();
        let cmd = "echo hello";
        let wrapped_args = wrapper.wrap_args(&vec!["echo".to_string(), "hello".to_string()]);

        #[cfg(target_os = "linux")]
        assert_eq!(wrapped_args, vec!["bwrap", "--unshare-all", "--share-net", "--ro-bind", "/", "/", "--bind", "/tmp/agent_workspace", "/tmp/agent_workspace", "--dev", "/dev", "bash", "-c", "set -e; echo hello"]);

        #[cfg(target_os = "macos")]
        assert_eq!(wrapped_args, vec!["sandbox-exec", "-p", "(version 1)(allow default)(deny file-write*)", "bash", "-c", "set -e; echo hello"]);

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        assert_eq!(wrapped_args, vec!["bash", "-c", "set -e; echo hello"]);
    }
}
