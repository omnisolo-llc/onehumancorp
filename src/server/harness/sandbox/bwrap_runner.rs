use std::process::Stdio;
use tokio::process::Command;

pub struct BwrapRunner {
    pub allow_net: bool,
    pub socat_socket_path: Option<String>,
    pub socat_proxy_port: Option<u16>,
}

impl BwrapRunner {
    pub fn new(allow_net: bool) -> Self {
        BwrapRunner {
            allow_net,
            socat_socket_path: None,
            socat_proxy_port: None,
        }
    }

    pub fn with_socat_proxy(mut self, socket_path: String, port: u16) -> Self {
        self.socat_socket_path = Some(socket_path);
        self.socat_proxy_port = Some(port);
        self
    }

    pub fn generate_bwrap_args(&self, cmd: &str, workspace: &str) -> Vec<String> {
        let mut bwrap_args = vec![
            "--unshare-all".to_string(),
            "--ro-bind".to_string(), "/".to_string(), "/".to_string(),
            "--bind".to_string(), workspace.to_string(), workspace.to_string(),
        ];

        if self.allow_net {
            bwrap_args.push("--share-net".to_string());
        }

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

        let final_cmd = format!("bash -c \"{}\"", format!("{}{}", preamble, cmd).replace("\"", "\\\""));

        bwrap_args.push("--".to_string());
        bwrap_args.push("bash".to_string());
        bwrap_args.push("-c".to_string());
        bwrap_args.push(final_cmd);

        bwrap_args
    }

    pub async fn run_in_sandbox(&self, cmd: &str, workspace: &str) -> Result<String, String> {
        let bwrap_args = self.generate_bwrap_args(cmd, workspace);

        let output = Command::new("bwrap")
            .args(&bwrap_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                ::server_telemetry::record_sandbox_violation("bwrap_execution_failed");
                format!("Failed to spawn bwrap: {}", e)
            })?;

        let exit_code = output.status.code().unwrap_or(-1);
        if exit_code == 13 || exit_code == 126 || exit_code != 0 {
            ::server_telemetry::record_sandbox_violation("bwrap_policy_violation");
        }

        if !output.status.success() {
            return Err(format!(
                "bwrap exited with error: {}\n{}",
                exit_code,
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bwrap_args_formatting() {
        let runner = BwrapRunner::new(false);
        let args = runner.generate_bwrap_args("echo hello", "/workspace");
        assert_eq!(args[0], "--unshare-all");
        assert_eq!(args[1], "--ro-bind");
        assert_eq!(args[2], "/");
        assert_eq!(args[3], "/");
        assert_eq!(args[4], "--bind");
        assert_eq!(args[5], "/workspace");
        assert_eq!(args[6], "/workspace");
        assert_eq!(args[7], "--");
        assert_eq!(args[8], "bash");
        assert_eq!(args[9], "-c");
        assert_eq!(args[10], "bash -c \"set -e; umask 077; echo hello\"");
    }

    #[test]
    fn test_bwrap_args_formatting_with_net() {
        let runner = BwrapRunner::new(true).with_socat_proxy("/tmp/sock".to_string(), 8080);
        let args = runner.generate_bwrap_args("echo hello", "/workspace");
        assert!(args.contains(&"--share-net".to_string()));
        let last_arg = args.last().unwrap();
        assert!(last_arg.contains("socat UNIX-LISTEN:'/tmp/sock'"));
        assert!(last_arg.contains("TCP:127.0.0.1:8080"));
    }
}
