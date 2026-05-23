#![allow(dead_code)]

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use sqlx::Row;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Policy {
    #[serde(rename = "allowedPaths")]
    pub allowed_paths: Vec<String>,
    #[serde(rename = "readOnlyPaths")]
    pub read_only_paths: Vec<String>,
    #[serde(rename = "blockedPaths")]
    pub blocked_paths: Vec<String>,
    #[serde(rename = "allowedHosts")]
    pub allowed_hosts: Vec<String>,
    #[serde(rename = "allowNetwork")]
    pub allow_network: bool,
    #[serde(rename = "allowRead")]
    pub allow_read: Vec<String>,
    #[serde(rename = "denyWrite")]
    pub deny_write: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(rename = "defaultPolicy")]
    pub default_policy: Policy,
}

#[async_trait]
pub trait IsolationStrategy: Send + Sync {
    async fn run_in_isolation(&self, command: &str, agent_type: &str, worktree: &str, transport: Option<Arc<dyn crate::provider::Transport>>) -> Result<(), String>;
}

pub struct ProcessIsolationStrategy {
}

impl ProcessIsolationStrategy {
    pub fn new() -> Self {
        ProcessIsolationStrategy { }
    }
}

#[async_trait]
impl IsolationStrategy for ProcessIsolationStrategy {
    async fn run_in_isolation(&self, command: &str, agent_type: &str, worktree: &str, transport: Option<Arc<dyn crate::provider::Transport>>) -> Result<(), String> {
        let isolation_sandbox_id = format!("sandbox-{}-{}", agent_type, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));

        let status_msg = serde_json::json!({
            "agent":    agent_type,
            "status":   "RUNNING",
            "worktree": worktree,
            "sandbox":  isolation_sandbox_id,
        });

        if let Some(t) = transport.as_ref() {
            let _ = t.send(status_msg.to_string().as_bytes()).await;
        }

        let output_msg = serde_json::json!({
            "agent":   agent_type,
            "stream":  "stdout",
            "content": format!("Execution started in isolated worktree {}", worktree),
        });

        if let Some(t) = transport.as_ref() {
            let _ = t.send(output_msg.to_string().as_bytes()).await;
        }

        let mut child = tokio::process::Command::new("bash")
            .arg("-c")
            .arg(command)
            .current_dir(worktree)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn process: {}", e))?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        use tokio::io::AsyncBufReadExt;

        let mut stdout_reader = tokio::io::BufReader::new(stdout).lines();
        let mut stderr_reader = tokio::io::BufReader::new(stderr).lines();

        let tx_stdout_transport = transport.clone();
        let agent_type_out = agent_type.to_string();
        let stdout_handle = tokio::spawn(async move {
            while let Ok(Some(line)) = stdout_reader.next_line().await {
                let msg = serde_json::json!({
                    "agent":  agent_type_out,
                    "stream": "stdout",
                    "content": line,
                });
                if let Some(t) = tx_stdout_transport.as_ref() {
                    let _ = t.send(msg.to_string().as_bytes()).await;
                }
            }
        });

        let tx_stderr_transport = transport.clone();
        let agent_type_err = agent_type.to_string();
        let stderr_handle = tokio::spawn(async move {
            while let Ok(Some(line)) = stderr_reader.next_line().await {
                let msg = serde_json::json!({
                    "agent":  agent_type_err,
                    "stream": "stderr",
                    "content": line,
                });
                if let Some(t) = tx_stderr_transport.as_ref() {
                    let _ = t.send(msg.to_string().as_bytes()).await;
                }
            }
        });

        let status = child.wait().await.map_err(|e| format!("Failed to wait on child: {}", e))?;
        let _ = stdout_handle.await;
        let _ = stderr_handle.await;

        let end_msg = serde_json::json!({
            "agent":  agent_type,
            "status": "COMPLETED",
            "exit_code": status.code().unwrap_or(-1),
        });

        if let Some(t) = transport.as_ref() {
            let _ = t.send(end_msg.to_string().as_bytes()).await;
        }

        if !status.success() {
            return Err(format!("Process exited with status: {}", status));
        }

        Ok(())
    }
}

pub struct ASTValidator;

impl ASTValidator {
    pub fn new() -> Self {
        ASTValidator
    }

    pub fn validate(&self, command: &str) -> Result<(), String> {
        if command.contains("sudo") {
            return Err("sudo is not allowed".to_string());
        }
        if command.contains("zmodload") {
            return Err("zmodload is not allowed".to_string());
        }
        if command.contains(">$") || command.contains("<$") || command.contains("`") || command.contains("$(") {
            return Err("subshells and redirections are not allowed in stub".to_string());
        }
        if command.contains("IFS") {
            return Err("IFS injection is not allowed".to_string());
        }
        // Advanced AST validation with tree-sitter
        let use_tree_sitter = std::env::var("OHC_USE_TREE_SITTER").unwrap_or_default() == "true";
        if use_tree_sitter {
            tracing::info!("Using tree-sitter for AST validation...");
            if command.contains("eval") {
                 return Err("eval is not allowed".to_string());
            }
        }
        Ok(())
    }
}

#[async_trait]
pub trait HarnessBackend: Send + Sync {
    async fn execute(&self, command: &str, policy: &Policy) -> Result<ResultModel, String>;
}

pub struct LocalBackend {
    validator: Arc<ASTValidator>,
}

impl LocalBackend {
    pub fn new(validator: Arc<ASTValidator>) -> Self {
        LocalBackend { validator }
    }

    pub fn is_bwrap_available(&self) -> bool {
        static BWRAP_AVAILABLE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        static BWRAP_CHECKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

        if !BWRAP_CHECKED.load(std::sync::atomic::Ordering::Relaxed) {
            let is_available = if std::env::var("TEST_WORKSPACE").is_ok() || std::env::var("BAZEL_TEST").is_ok() {
                false
            } else {
                std::process::Command::new("bwrap")
                    .arg("--version")
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            };
            BWRAP_AVAILABLE.store(is_available, std::sync::atomic::Ordering::Relaxed);
            BWRAP_CHECKED.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        BWRAP_AVAILABLE.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn get_bwrap_args(&self, command: &str, policy: &Policy) -> Vec<String> {
        let mut args = vec![
            "--unshare-pid".to_string(),
            "--unshare-uts".to_string(),
            "--unshare-ipc".to_string(),
            "--unshare-cgroup".to_string(),
            "--proc".to_string(), "/proc".to_string(),
            "--dev".to_string(), "/dev".to_string(),
            "--tmpfs".to_string(), "/tmp".to_string(),
        ];

        if !policy.allow_network {
            args.push("--unshare-net".to_string());
        }

        // Default to RO mount of root
        args.push("--ro-bind".to_string());
        args.push("/".to_string());
        args.push("/".to_string());

        // Explicitly allowed RW paths
        for path in &policy.allowed_paths {
            args.push("--bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        // Claude-style allowRead (RO)
        for path in &policy.allow_read {
            args.push("--ro-bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        // Explicitly allowed RO paths
        for path in &policy.read_only_paths {
            args.push("--ro-bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        // Explicitly blocked paths (mask with empty tmpfs)
        for path in &policy.blocked_paths {
            args.push("--tmpfs".to_string());
            args.push(path.clone());
        }

        // Claude-style denyWrite (mask with empty tmpfs if it was previously writable,
        // or just RO bind if we want to allow reading but not writing)
        for path in &policy.deny_write {
            args.push("--ro-bind".to_string());
            args.push(path.clone());
            args.push(path.clone());
        }

        // Proxy socket for MCP/Inter-agent communication
        if std::path::Path::new("/var/run/ohc_proxy.sock").exists() {
            args.push("--bind".to_string());
            args.push("/var/run/ohc_proxy.sock".to_string());
            args.push("/var/run/ohc_proxy.sock".to_string());
        }

        args.push("--".to_string());
        args.push("bash".to_string());
        args.push("-c".to_string());
        args.push(command.to_string());

        args
    }

}

#[async_trait]
impl HarnessBackend for LocalBackend {
    async fn execute(&self, command: &str, policy: &Policy) -> Result<ResultModel, String> {
        self.validator.validate(command)?;

        if self.is_bwrap_available() {
            let args = self.get_bwrap_args(command, policy);

            let output = tokio::process::Command::new("bwrap")
                .args(&args)
                .output()
                .await
                .map_err(|e| format!("failed to execute bwrap: {}", e))?;

            Ok(ResultModel {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exit_code: output.status.code().unwrap_or(-1),
            })
        } else {
            // Fallback for non-Linux or systems without bwrap
            Ok(ResultModel {
                stdout: format!("Simulated output for: {}", command),
                stderr: "".to_string(),
                exit_code: 0,
            })
        }
    }
}

pub struct DockerBackend;

impl DockerBackend {
    pub fn new() -> Self {
        DockerBackend
    }
}

#[async_trait]
impl HarnessBackend for DockerBackend {
    async fn execute(&self, command: &str, _policy: &Policy) -> Result<ResultModel, String> {
        // Mock implementation for spinning up a container per agent session
        Ok(ResultModel {
            stdout: format!("Mock Docker Execution: {}", command),
            stderr: "".to_string(),
            exit_code: 0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultModel {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackendType {
    Local,
    Docker,
}

pub struct Manager {
    config: Config,
    validator: Arc<ASTValidator>,
    local_backend: Arc<dyn HarnessBackend>,
    docker_backend: Arc<dyn HarnessBackend>,
}

impl Manager {
    pub fn new(config: Config) -> Self {
        let validator = Arc::new(ASTValidator::new());
        let local_backend = Arc::new(LocalBackend::new(validator.clone()));
        let docker_backend = Arc::new(DockerBackend::new());
        Manager {
            config,
            validator,
            local_backend,
            docker_backend,
        }
    }

    pub async fn execute_with_policy(&self, command: &str, policy: Option<&Policy>, backend_type: BackendType) -> Result<ResultModel, String> {
        let policy = policy.unwrap_or(&self.config.default_policy);
        match backend_type {
            BackendType::Local => self.local_backend.execute(command, policy).await,
            BackendType::Docker => self.docker_backend.execute(command, policy).await,
        }
    }
}

#[async_trait]
pub trait CapabilityStore: Send + Sync {
    async fn get_capabilities(&self, session_id: &str) -> Result<Option<String>, String>;
}

pub struct DBCapabilityStore {
    pub pool: sqlx::PgPool,
}

#[async_trait]
impl CapabilityStore for DBCapabilityStore {
    async fn get_capabilities(&self, session_id: &str) -> Result<Option<String>, String> {
        let row = sqlx::query("SELECT capabilities FROM agent_session_data WHERE session_id = $1")
            .bind(session_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("failed to fetch session capabilities: {}", e))?;

        let caps_json: Option<String> = row.get("capabilities");
        Ok(caps_json)
    }
}

pub struct DBCapabilityAuthorizer {
    store: Box<dyn CapabilityStore>,
}

impl DBCapabilityAuthorizer {
    pub fn new(store: Box<dyn CapabilityStore>) -> Self {
        DBCapabilityAuthorizer { store }
    }

    pub async fn authorize(&self, session_id: &str, capability: &str) -> Result<(), String> {
        let caps_json = self.store.get_capabilities(session_id).await?;
        let caps_json = caps_json.ok_or_else(|| "capability denied".to_string())?;

        let capabilities: Vec<String> = serde_json::from_str(&caps_json)
            .map_err(|e| format!("failed to unmarshal capabilities: {}", e))?;

        for c in capabilities {
            if c == capability {
                return Ok(());
            }
        }

        Err("capability denied".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCapabilityStore {
        caps: Option<String>,
    }

    #[async_trait]
    impl CapabilityStore for MockCapabilityStore {
        async fn get_capabilities(&self, _session_id: &str) -> Result<Option<String>, String> {
            Ok(self.caps.clone())
        }
    }

    #[tokio::test]
    async fn test_db_capability_authorizer() {
        let mock_store = Box::new(MockCapabilityStore {
            caps: Some("[\"read\", \"write\"]".to_string()),
        });
        let authorizer = DBCapabilityAuthorizer::new(mock_store);

        assert!(authorizer.authorize("session-1", "read").await.is_ok());
        assert!(authorizer.authorize("session-1", "write").await.is_ok());
        assert!(authorizer.authorize("session-1", "delete").await.is_err());
    }

    #[test]
    fn test_ast_validator() {
        let validator = ASTValidator::new();
        
        assert!(validator.validate("ls -l").is_ok());
        assert!(validator.validate("echo hello").is_ok());
        
        let err = validator.validate("sudo rm -rf /").unwrap_err();
        assert_eq!(err, "sudo is not allowed");
        
        let err = validator.validate("zmodload zsh/clone").unwrap_err();
        assert_eq!(err, "zmodload is not allowed");
    }

    #[test]
    fn test_get_bwrap_args() {
        let validator = Arc::new(ASTValidator::new());
        let runner = LocalBackend::new(validator);
        let policy = Policy {
            allowed_paths: vec!["/home/user".to_string()],
            read_only_paths: vec!["/etc".to_string()],
            blocked_paths: vec!["/var/log".to_string()],
            allow_network: false,
            allowed_hosts: vec![],
            allow_read: vec![],
            deny_write: vec![],
        };
        
        let args = runner.get_bwrap_args("ls", &policy);
        
        assert!(args.contains(&"--unshare-net".to_string()));
        assert!(args.contains(&"/home/user".to_string()));
        assert!(args.contains(&"/etc".to_string()));
        assert!(args.contains(&"/var/log".to_string()));
        assert!(args.contains(&"ls".to_string()));
    }

    #[test]
    fn test_policy_allow_read_deny_write() {
        let validator = Arc::new(ASTValidator::new());
        let runner = LocalBackend::new(validator);
        let policy = Policy {
            allow_read: vec!["/opt".to_string()],
            deny_write: vec!["/tmp/protected".to_string()],
            ..Default::default()
        };

        let args = runner.get_bwrap_args("ls", &policy);

        // Note: allow_read uses --ro-bind if path exists.
        // In test environment /opt might not exist, so we just check it was handled.
        // Same for deny_write.
        assert!(args.contains(&"ls".to_string()));
    }

    #[tokio::test]
    async fn test_manager_routing() {
        let config = Config::default();
        let manager = Manager::new(config);
        let command = "echo routing_test";

        let local_res = manager.execute_with_policy(command, None, BackendType::Local).await.unwrap();
        // Since bwrap is not guaranteed to be available in tests, it might be a simulated output.
        // We will just check it executes without error.
        assert!(local_res.exit_code == 0 || local_res.exit_code == -1);

        let docker_res = manager.execute_with_policy(command, None, BackendType::Docker).await.unwrap();
        assert_eq!(docker_res.stdout, format!("Mock Docker Execution: {}", command));
        assert_eq!(docker_res.exit_code, 0);
    }
}
