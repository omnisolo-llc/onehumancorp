use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use ::server_telemetry::record_bubblewrap_violation;
use super::ast::ASTParser;
use super::manager::{SandboxAdapter, SandboxPolicy};
use super::permissions::PermissionEvaluator;
use crate::telemetry::ViolationStore;

pub struct LinuxSandbox {
    bridge: Option<Arc<crate::harness::network::bridge::NetworkBridgeProxy>>,
    evaluator: PermissionEvaluator,
    policy: SandboxPolicy,
    violation_store: Arc<ViolationStore>,
}

impl LinuxSandbox {
    pub fn new(pool: Option<PgPool>) -> Self {
        let violation_store = Arc::new(ViolationStore::new(pool.clone()));
        LinuxSandbox {
            bridge: None,
            evaluator: PermissionEvaluator::new(),
            policy: SandboxPolicy::default(),
            violation_store,
        }
    }

    fn generate_bwrap_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        // Basic isolation
        args.push("--unshare-all".to_string());
        args.push("--die-with-parent".to_string());

        // Mount system directories
        args.push("--dev".to_string());
        args.push("/dev".to_string());
        args.push("--proc".to_string());
        args.push("/proc".to_string());

        args.push("--bind".to_string());
        args.push("/".to_string());
        args.push("/".to_string());

        // Handle network restrictions. For strict isolation, if there are ANY blocked domains,
        // we drop the network entirely by not providing `--share-net` (so it unshares).
        // If a proxy bridge is active, we also bind its socket so the sandbox can use it.
        if self.policy.blocked_domains.is_empty() {
            args.push("--share-net".to_string());
        } else if let Some(bridge) = &self.bridge {
            args.push("--bind".to_string());
            args.push(bridge.socket_path().to_string());
            args.push(bridge.socket_path().to_string());
        }

        // Now, for every read-only path, we bind it as ro-bind to restrict writes.
        // In bwrap, later bind mounts override earlier ones. So binding / as rw,
        // and then binding specific dirs as ro works.
        for path in &self.policy.read_only_paths {
            args.push("--ro-bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        // Drop capabilities
        args.push("--cap-drop".to_string());
        args.push("ALL".to_string());

        args
    }
}

#[async_trait]
impl SandboxAdapter for LinuxSandbox {
    async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
        let mut ast_parser = ASTParser::new();
        if let Err(reason) = ast_parser.parse_for_security(cmd) {
            let details = json!({ "command": cmd, "reason": reason });
            let _ = self.violation_store.record_violation(
                "system",
                "unknown_agent",
                "unknown_session",
                "ast_security_violation",
                details
            ).await;
            record_bubblewrap_violation("unknown_agent", "unknown_task", "ast_security_violation");
            return Err(reason);
        }

        if !self.evaluator.evaluate(cmd) {
            let details = json!({ "command": cmd });
            let _ = self.violation_store.record_violation(
                "system",
                "unknown_agent",
                "unknown_session",
                "command_execution",
                details
            ).await;
            record_bubblewrap_violation("unknown_agent", "unknown_task", "command_execution_denied");
            return Err("Command execution denied by sandbox policy".to_string());
        }

        let bwrap_args = self.generate_bwrap_args();
        let bwrap_args_str = bwrap_args.join(" ");
        let escaped_cmd = cmd.replace("\"", "\\\"");

        let mut prefix = String::new();
        prefix.push_str("set -e; umask 077; ");
        if !self.policy.blocked_domains.is_empty() {
            prefix.push_str(&format!("export BLOCKED_DOMAINS='{}'; ", self.policy.blocked_domains.join(",")));
            if let Some(bridge) = &self.bridge {
                // bwrap doesn't natively expose unix:// for traditional http tools, so we bridge it via internal socat
                let proxy_port = 8080;
                let socket_path = bridge.socket_path();
                prefix.push_str(&format!("socat TCP-LISTEN:{},fork UNIX-CONNECT:{} & ", proxy_port, socket_path));
                let proxy_url = format!("http://127.0.0.1:{}", proxy_port);
                prefix.push_str(&format!("export HTTP_PROXY='{}'; ", proxy_url));
                prefix.push_str(&format!("export HTTPS_PROXY='{}'; ", proxy_url));
                prefix.push_str(&format!("export ALL_PROXY='{}'; ", proxy_url));
            }
        }

        Ok(format!("bwrap {} -- bash -c \"{}{}\"", bwrap_args_str, prefix, escaped_cmd))
    }

    async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        let policy: SandboxPolicy = serde_json::from_str(policy_json)
            .map_err(|e| format!("Invalid policy JSON: {}", e))?;

        self.evaluator.update_policy(policy.clone());

        if !policy.blocked_domains.is_empty() {
            let bridge = crate::harness::network::bridge::NetworkBridgeProxy::new(policy.blocked_domains.clone()).await?;
            self.bridge = Some(Arc::new(bridge));
        } else {
            self.bridge = None;
        }

        self.policy = policy;

        Ok(())
    }

    fn annotate_error(&self, err: String, stdout: String) -> String {
        format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_bwrap_args_default() {
        let sandbox = LinuxSandbox::new(None);
        let args = sandbox.generate_bwrap_args();
        assert!(args.contains(&"--unshare-all".to_string()));
        assert!(args.contains(&"--die-with-parent".to_string()));
        assert!(args.contains(&"--share-net".to_string()));
        assert!(args.contains(&"--bind".to_string()));
        assert!(args.contains(&"--cap-drop".to_string()));
        assert!(!args.contains(&"--ro-bind".to_string()));
    }

    #[tokio::test]
    async fn test_update_config_and_args() {
        let mut sandbox = LinuxSandbox::new(None);
        let policy_json = r#"{
            "read_only_paths": ["/etc", "/var/log"],
            "blocked_domains": ["evil.com"]
        }"#;

        sandbox.update_config(policy_json).await.unwrap();
        let args = sandbox.generate_bwrap_args();

        assert!(args.contains(&"--unshare-all".to_string()));
        assert!(!args.contains(&"--share-net".to_string())); // Because blocked domains is not empty
        // Ensure proxy socket is bound
        let bridge_socket = sandbox.bridge.as_ref().unwrap().socket_path().to_string();
        assert!(args.contains(&"--bind".to_string()));
        assert!(args.contains(&bridge_socket));

        assert!(args.contains(&"--ro-bind".to_string()));
        assert!(args.contains(&"/etc".to_string()));
        assert!(args.contains(&"/var/log".to_string()));
    }

    #[tokio::test]
    async fn test_wrap_command_injects_http_proxy() {
        let mut sandbox = LinuxSandbox::new(None);
        let policy_json = r#"{
            "blocked_domains": ["evil.com"]
        }"#;
        sandbox.update_config(policy_json).await.unwrap();

        let wrapped = sandbox.wrap_command("curl example.com").await.unwrap();
        assert!(wrapped.contains("export HTTP_PROXY"));
        assert!(wrapped.contains("export HTTPS_PROXY"));
        assert!(wrapped.contains("export ALL_PROXY"));

        let bridge_socket = sandbox.bridge.as_ref().unwrap().socket_path().to_string();
        assert!(wrapped.contains(&format!("UNIX-CONNECT:{}", bridge_socket)));
    }

    #[tokio::test]
    async fn test_wrap_command_allowed() {
        let sandbox = LinuxSandbox::new(None);
        let wrapped = sandbox.wrap_command("echo 'hello world'").await.unwrap();

        assert!(wrapped.starts_with("bwrap --unshare-all"));
        assert!(wrapped.contains("bash -c \"set -e; umask 077; echo 'hello world'\""));
    }

    #[tokio::test]
    async fn test_wrap_command_ast_denied() {
        let sandbox = LinuxSandbox::new(None);
        let result = sandbox.wrap_command("echo test > >(cat)").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Dangerous pattern detected: >() process substitution");
    }

    #[tokio::test]
    async fn test_wrap_command_evaluator_denied() {
        let sandbox = LinuxSandbox::new(None);
        let result = sandbox.wrap_command("rm -rf /").await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Command execution denied by sandbox policy");
    }
}
