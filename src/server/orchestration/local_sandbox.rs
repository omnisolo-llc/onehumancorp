use async_trait::async_trait;
use std::sync::Arc;
use tokio::process::Command;
use crate::orchestration::sandbox::{OHCSandboxManager, SandboxConfig, ViolationEvent};
use crate::orchestration::sandbox_ask::SandboxAskCallback;

pub struct LocalSandbox {
    config: SandboxConfig,
    callback: Option<Arc<dyn SandboxAskCallback>>,
}

impl LocalSandbox {
    pub fn new(config: SandboxConfig, callback: Option<Arc<dyn SandboxAskCallback>>) -> Self {
        Self { config, callback }
    }
}

#[async_trait]
impl OHCSandboxManager for LocalSandbox {
    async fn execute(&self, cmd: &str) -> Result<(bool, String, String), ViolationEvent> {
        // Check deny-list directories
        for deny_dir in &self.config.deny_list_dirs {
            if cmd.contains(deny_dir) {
                let reason = format!("Command attempts to access deny-listed directory: {}", deny_dir);
                if let Some(cb) = &self.callback {
                    if !cb.ask_for_permission(cmd, &reason).await {
                        return Err(ViolationEvent {
                            reason,
                            command: cmd.to_string(),
                        });
                    }
                } else {
                    return Err(ViolationEvent {
                        reason,
                        command: cmd.to_string(),
                    });
                }
            }
        }

        // Check disabled commands
        for disabled_cmd in &self.config.disabled_commands {
            if cmd.contains(disabled_cmd) {
                let reason = format!("Command attempts to run disabled command: {}", disabled_cmd);
                if let Some(cb) = &self.callback {
                    if !cb.ask_for_permission(cmd, &reason).await {
                        return Err(ViolationEvent {
                            reason,
                            command: cmd.to_string(),
                        });
                    }
                } else {
                    return Err(ViolationEvent {
                        reason,
                        command: cmd.to_string(),
                    });
                }
            }
        }

        match Command::new("sh").arg("-c").arg(cmd).output().await {
            Ok(output) => {
                if output.status.success() {
                    Ok((true, String::from_utf8_lossy(&output.stdout).to_string(), "".to_string()))
                } else {
                    Ok((false, "".to_string(), String::from_utf8_lossy(&output.stderr).to_string()))
                }
            }
            Err(e) => Ok((false, "".to_string(), e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCallback {
        allow: bool,
    }

    #[async_trait]
    impl SandboxAskCallback for MockCallback {
        async fn ask_for_permission(&self, _command: &str, _reason: &str) -> bool {
            self.allow
        }
    }

    #[tokio::test]
    async fn test_local_sandbox_execute_valid() {
        let config = SandboxConfig {
            deny_list_dirs: vec!["/etc".to_string()],
            disabled_commands: vec!["rm".to_string()],
            ..Default::default()
        };
        let sandbox = LocalSandbox::new(config, None);
        let (success, stdout, _) = sandbox.execute("echo 'hello'").await.unwrap();
        assert!(success);
        assert_eq!(stdout.trim(), "hello");
    }

    #[tokio::test]
    async fn test_local_sandbox_execute_deny_list_dir() {
        let config = SandboxConfig {
            deny_list_dirs: vec!["/etc".to_string()],
            ..Default::default()
        };
        let sandbox = LocalSandbox::new(config, None);
        let result = sandbox.execute("cat /etc/passwd").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.reason, "Command attempts to access deny-listed directory: /etc");
        assert_eq!(err.command, "cat /etc/passwd");
    }

    #[tokio::test]
    async fn test_local_sandbox_execute_disabled_command() {
        let config = SandboxConfig {
            disabled_commands: vec!["rm -rf /".to_string()],
            ..Default::default()
        };
        let sandbox = LocalSandbox::new(config, None);
        let result = sandbox.execute("sudo rm -rf /").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.reason, "Command attempts to run disabled command: rm -rf /");
        assert_eq!(err.command, "sudo rm -rf /");
    }

    #[tokio::test]
    async fn test_local_sandbox_execute_with_callback_allow() {
        let config = SandboxConfig {
            deny_list_dirs: vec!["/etc".to_string()],
            ..Default::default()
        };
        let callback = Arc::new(MockCallback { allow: true });
        let sandbox = LocalSandbox::new(config, Some(callback));
        let (_success, _, _) = sandbox.execute("cat /etc/hostname").await.unwrap();
    }

    #[tokio::test]
    async fn test_local_sandbox_execute_with_callback_deny() {
        let config = SandboxConfig {
            deny_list_dirs: vec!["/etc".to_string()],
            ..Default::default()
        };
        let callback = Arc::new(MockCallback { allow: false });
        let sandbox = LocalSandbox::new(config, Some(callback));
        let result = sandbox.execute("cat /etc/hostname").await;
        assert!(result.is_err());
    }
}
