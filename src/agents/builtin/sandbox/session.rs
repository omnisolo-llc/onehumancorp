#![allow(dead_code)]

use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use tokio::sync::RwLock;
use regex::Regex;
use crate::harness::ASTValidator;
use ::server_telemetry::{record_bubblewrap_spawn, record_bubblewrap_execution_latency, record_bubblewrap_violation};
use std::time::Instant;

pub struct ShellSession {
    pub session_id: String,
    pub sandbox_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub current_cwd: RwLock<PathBuf>,
    blocked_patterns: Vec<Regex>,
    ast_validator: ASTValidator,
}

impl ShellSession {
    pub async fn new(session_id: &str, sandbox_dir: &str) -> Result<Self, String> {
        let sandbox_path = Path::new(sandbox_dir);
        fs::create_dir_all(sandbox_path).await.map_err(|e| e.to_string())?;

        let memory_dir = sandbox_path.join("memory");
        fs::create_dir_all(&memory_dir).await.map_err(|e| e.to_string())?;

        let env_snapshot_path = sandbox_path.join("env_snapshot.sh");
        if !env_snapshot_path.exists() {
            fs::write(&env_snapshot_path, b"").await.map_err(|e| e.to_string())?;
        }

        let blocked_patterns = vec![
            Regex::new(r"(?i)\bsudo\b").unwrap(),
            Regex::new(r"(?i)\brm\s+-rf\s+/").unwrap(),
            Regex::new(r"(?i)\bchown\b").unwrap(),
            Regex::new(r"(?i)\bchmod\b").unwrap(),
            Regex::new(r"<\(").unwrap(),
            Regex::new(r">\(").unwrap(),
            Regex::new(r"=\(").unwrap(),
        ];

        Ok(ShellSession {
            session_id: session_id.to_string(),
            sandbox_dir: sandbox_path.to_path_buf(),
            memory_dir,
            current_cwd: RwLock::new(sandbox_path.to_path_buf()),
            blocked_patterns,
            ast_validator: ASTValidator::new(),
        })
    }

    pub fn validate(&self, command: &str) -> Result<(), String> {
        for pattern in &self.blocked_patterns {
            if pattern.is_match(command) {
                record_bubblewrap_violation("local_agent", "unknown_task", "regex_policy_violation");
                return Err(format!("command violates security policy: matched {}", pattern));
            }
        }
        if let Err(e) = self.ast_validator.validate(command) {
            record_bubblewrap_violation("local_agent", "unknown_task", "ast_policy_violation");
            return Err(e);
        }
        Ok(())
    }

    pub async fn run_stateful_command(&self, command: &str) -> Result<String, String> {
        self.validate(command)?;

        let env_snapshot_path = self.sandbox_dir.join("env_snapshot.sh");
        let cwd_snapshot_path = self.sandbox_dir.join("cwd_snapshot.txt");

        let memory_dir_export = format!("export OHC_MEMORY_DIR='{}';", self.memory_dir.display());

        let wrapper_cmd = format!(
            "{} source '{}' 2>/dev/null || true; {{ {}; }}; declare -p | grep -v '^declare -[a-zA-Z-]*r' > '{}'; pwd -P > '{}'",
            memory_dir_export, env_snapshot_path.display(), command, env_snapshot_path.display(), cwd_snapshot_path.display()
        );

        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(wrapper_cmd);
        
        let cwd = self.current_cwd.read().await.clone();
        cmd.current_dir(cwd);
        
        cmd.env_clear();
        cmd.env("PATH", "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin");

        // Check if bwrap is available (cached)
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

        let is_bwrap_available = BWRAP_AVAILABLE.load(std::sync::atomic::Ordering::Relaxed);

        if is_bwrap_available {
            let mut bwrap_args = vec![
                "--unshare-pid".to_string(),
                "--unshare-uts".to_string(),
                "--unshare-ipc".to_string(),
                "--unshare-cgroup".to_string(),
                "--proc".to_string(), "/proc".to_string(),
                "--dev".to_string(), "/dev".to_string(),
                "--tmpfs".to_string(), "/tmp".to_string(),
                "--ro-bind".to_string(), "/".to_string(), "/".to_string(),
                "--bind".to_string(), self.sandbox_dir.to_string_lossy().to_string(), self.sandbox_dir.to_string_lossy().to_string(),
                "--".to_string(),
                "bash".to_string(),
                "-c".to_string(),
            ];
            bwrap_args.push(command.to_string());

            let mut bwrap_cmd = Command::new("bwrap");
            bwrap_cmd.args(&bwrap_args);

            let cwd = self.current_cwd.read().await.clone();
            bwrap_cmd.current_dir(cwd);
            bwrap_cmd.env_clear();
            bwrap_cmd.env("PATH", "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin");

            // We do not override cmd entirely, instead we simply use bwrap_cmd to execute
            record_bubblewrap_spawn("local_agent", "unknown_task");
            let start = Instant::now();
            let output = bwrap_cmd.output().await.map_err(|e| e.to_string())?;
            let latency = start.elapsed().as_secs_f64() * 1000.0;
            record_bubblewrap_execution_latency("local_agent", "unknown_task", latency);

            if let Ok(cwd_bytes) = fs::read(&cwd_snapshot_path).await {
                let cwd_str = String::from_utf8_lossy(&cwd_bytes).trim().to_string();
                if !cwd_str.is_empty() {
                    let mut cwd = self.current_cwd.write().await;
                    *cwd = PathBuf::from(cwd_str);
                }
            }

            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                return Ok(stdout);
            } else {
                return Err(format!("command failed: {}
stderr: {}", stdout, stderr));
            }
        }

        let output = cmd.output().await.map_err(|e| e.to_string())?;

        if let Ok(cwd_bytes) = fs::read(&cwd_snapshot_path).await {
            let cwd_str = String::from_utf8_lossy(&cwd_bytes).trim().to_string();
            if !cwd_str.is_empty() {
                let mut cwd = self.current_cwd.write().await;
                *cwd = PathBuf::from(cwd_str);
            }
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        if output.status.success() {
            Ok(stdout)
        } else {
            Err(format!("command failed: {}\nstderr: {}", stdout, stderr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shell_session() {
        let dir = "/tmp/test_session";
        let _ = tokio::fs::remove_dir_all(dir).await;
        let session = ShellSession::new("sess1", dir).await.unwrap();

        let out = session.run_stateful_command("echo hello").await.unwrap();
        assert!(out.contains("hello"));

        let out = session.run_stateful_command("export FOO=bar").await.unwrap();
        let out = session.run_stateful_command("echo $FOO").await.unwrap();
        assert!(out.contains("bar"));

        // Test memory directory export
        let out = session.run_stateful_command("echo $OHC_MEMORY_DIR").await.unwrap();
        assert!(out.contains("memory"));

        let _ = tokio::fs::remove_dir_all(dir).await;
    }

    #[tokio::test]
    async fn test_shell_session_validation() {
        let dir = "/tmp/test_session_val";
        let session = ShellSession::new("sess2", dir).await.unwrap();

        let res = session.run_stateful_command("sudo apt-get update").await;
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("violates security policy"));

        let _ = tokio::fs::remove_dir_all(dir).await;
    }
}
