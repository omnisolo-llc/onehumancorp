use std::sync::Arc;

use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;

use super::ast::ASTParser;
use super::permissions::PermissionEvaluator;
use super::wrapper::BashWrapper;
use crate::telemetry::ViolationStore;


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SandboxPolicy {
    #[serde(default)]
    pub disabled_commands: Vec<String>,
    #[serde(default)]
    pub disabled_patterns: Vec<String>,
    #[serde(default)]
    pub read_only_paths: Vec<String>,
    #[serde(default)]
    pub blocked_domains: Vec<String>,
    #[serde(default)]
    pub allowed_domains: Vec<String>,
    #[serde(default)]
    pub seccomp_fd: Option<i32>,
    #[serde(default)]
    pub socat_socket_path: Option<String>,
    #[serde(default)]
    pub socat_proxy_port: Option<u16>,
}


#[async_trait]
pub trait SandboxAdapter: Send + Sync {
    async fn wrap_command(&self, cmd: &str) -> Result<String, String>;
    async fn update_config(&mut self, policy_json: &str) -> Result<(), String>;
    fn annotate_error(&self, err: String, stdout: String) -> String;
}

pub struct SandboxManager {
    os_adapter: Box<dyn SandboxAdapter>,
}

impl SandboxManager {
    pub fn get_policy(&self) -> SandboxPolicy {
        // Not used right now but can be stored if needed.
        SandboxPolicy::default()
    }

    pub fn new(pool: Option<PgPool>) -> Self {
        #[cfg(target_os = "linux")]
        {
            SandboxManager {
                os_adapter: Box::new(crate::sandbox::LinuxSandbox::new(pool)),
            }
        }
        #[cfg(target_os = "macos")]
        {
            SandboxManager {
                os_adapter: Box::new(crate::sandbox::MacOsSandbox::new(pool)),
            }
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            struct FallbackAdapter;
            #[async_trait]
            impl SandboxAdapter for FallbackAdapter {
                async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
                    Ok(format!("bash -c \"{}\"", cmd.replace("\"", "\\\"")))
                }
                async fn update_config(&mut self, _policy_json: &str) -> Result<(), String> {
                    Ok(())
                }
                fn annotate_error(&self, err: String, stdout: String) -> String {
                    format!("SANDBOX_FAILURE: {}\nSTDOUT:\n{}", err, stdout)
                }
            }
            SandboxManager {
                os_adapter: Box::new(FallbackAdapter),
            }
        }
    }
}

#[async_trait]
impl SandboxAdapter for SandboxManager {
    async fn wrap_command(&self, cmd: &str) -> Result<String, String> {
        self.os_adapter.wrap_command(cmd).await
    }

    async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        self.os_adapter.update_config(policy_json).await
    }

    fn annotate_error(&self, err: String, stdout: String) -> String {
        self.os_adapter.annotate_error(err, stdout)
    }
}
