use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;

use super::manager::{SandboxAdapter, SandboxPolicy};

#[cfg(target_os = "linux")]
use super::linux_sandbox::LinuxSandbox;

#[cfg(target_os = "macos")]
use super::macos_sandbox::MacOsSandbox;

use super::proxy::NetworkProxy;

use async_trait::async_trait;

#[async_trait]
pub trait SandboxHarness: Send + Sync {
    async fn execute(&self, cmd: &str) -> Result<String, String>;
    async fn update_policy(&mut self, policy_json: &str) -> Result<(), String>;
}

pub struct OHCSandboxHarness {
    adapter: Box<dyn SandboxAdapter>,
    policy: Arc<RwLock<SandboxPolicy>>,
    proxy_addr: Option<SocketAddr>,
}

impl OHCSandboxHarness {
    pub async fn new(pool: Option<sqlx::PgPool>) -> Result<Self, std::io::Error> {
        let policy = Arc::new(RwLock::new(SandboxPolicy::default()));

        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let local_addr = listener.local_addr()?;

        let proxy = NetworkProxy::new(policy.clone(), local_addr);

        tokio::spawn(async move {
            let policy_clone = proxy.get_policy();
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    let p = policy_clone.clone();
                    tokio::spawn(async move {
                        let _ = NetworkProxy::handle_connection(stream, p).await;
                    });
                }
            }
        });

        #[cfg(target_os = "macos")]
        let adapter = Box::new(MacOsSandbox::new(pool));

        #[cfg(target_os = "linux")]
        let adapter = Box::new(LinuxSandbox::new(pool));

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        let adapter = Box::new(super::manager::SandboxManager::new(pool));

        Ok(Self {
            adapter,
            policy,
            proxy_addr: Some(local_addr),
        })
    }
}

#[async_trait]
impl SandboxHarness for OHCSandboxHarness {
    async fn execute(&self, cmd: &str) -> Result<String, String> {
        // inject proxy settings into the command
        let proxy_cmd = if let Some(addr) = self.proxy_addr {
            format!("export HTTP_PROXY=http://{} HTTPS_PROXY=http://{} ALL_PROXY=socks5://{}; {}", addr, addr, addr, cmd)
        } else {
            cmd.to_string()
        };
        self.adapter.wrap_command(&proxy_cmd).await
    }

    async fn update_policy(&mut self, policy_json: &str) -> Result<(), String> {
        if let Ok(new_policy) = serde_json::from_str::<SandboxPolicy>(policy_json) {
            let mut p = self.policy.write().await;
            *p = new_policy;
        }

        self.adapter.update_config(policy_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_harness_creation() {
        let harness = OHCSandboxHarness::new(None).await.unwrap();
        let policy_json = r#"{
            "read_only_paths": ["/etc", "/var/log"],
            "blocked_domains": ["evil.com"]
        }"#;

        let mut mut_harness = harness;
        mut_harness.update_policy(policy_json).await.unwrap();

        let policy = mut_harness.policy.read().await;
        assert!(policy.blocked_domains.contains(&"evil.com".to_string()));
    }
}
