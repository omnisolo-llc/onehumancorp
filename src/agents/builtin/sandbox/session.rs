#![allow(dead_code)]

use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use tokio::sync::RwLock;
use regex::Regex;

pub struct ShellSession {
    pub session_id: String,
    pub sandbox_dir: PathBuf,
    pub memory_dir: PathBuf,
    pub current_cwd: RwLock<PathBuf>,
    blocked_patterns: Vec<Regex>,
    pub bwrap_available: bool,
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

        let bwrap_available = tokio::process::Command::new("bwrap")
            .arg("--version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false);

        Ok(ShellSession {
            session_id: session_id.to_string(),
            sandbox_dir: sandbox_path.to_path_buf(),
            memory_dir,
            current_cwd: RwLock::new(sandbox_path.to_path_buf()),
            blocked_patterns,
            bwrap_available,
        })
    }

    pub fn validate(&self, command: &str) -> Result<(), String> {
        for pattern in &self.blocked_patterns {
            if pattern.is_match(command) {
                return Err(format!("command violates security policy: matched {}", pattern));
            }
        }
        // TODO: Implement AST validation similar to Go BashASTValidator
        Ok(())
    }

    pub fn build_command(&self, wrapper_cmd: &str) -> Command {
        if self.bwrap_available {
            let mut bwrap_cmd = Command::new("bwrap");
            bwrap_cmd.arg("--unshare-pid")
                     .arg("--unshare-uts")
                     .arg("--proc").arg("/proc")
                     .arg("--dev").arg("/dev")
                     .arg("--ro-bind").arg("/usr").arg("/usr")
                     .arg("--ro-bind").arg("/bin").arg("/bin")
                     .arg("--ro-bind").arg("/etc").arg("/etc");

            if Path::new("/run").exists() {
                bwrap_cmd.arg("--ro-bind").arg("/run").arg("/run");
            }
            if Path::new("/var/run").exists() {
                bwrap_cmd.arg("--ro-bind").arg("/var/run").arg("/var/run");
            }
            if Path::new("/lib").exists() {
                bwrap_cmd.arg("--ro-bind").arg("/lib").arg("/lib");
            }
            if Path::new("/lib64").exists() {
                bwrap_cmd.arg("--ro-bind").arg("/lib64").arg("/lib64");
            }

            bwrap_cmd.arg("--tmpfs").arg("/tmp")
                     .arg("--bind").arg(&self.sandbox_dir).arg(&self.sandbox_dir)
                     .arg("--").arg("bash").arg("-c").arg(wrapper_cmd);
            bwrap_cmd
        } else {
            let mut bash_cmd = Command::new("bash");
            bash_cmd.arg("-c").arg(wrapper_cmd);
            bash_cmd
        }
    }

    pub async fn run_stateful_command(&self, command: &str) -> Result<String, String> {
        self.validate(command)?;

        let env_snapshot_path = self.sandbox_dir.join("env_snapshot.sh");
        let cwd_snapshot_path = self.sandbox_dir.join("cwd_snapshot.txt");

        let memory_dir_export = format!("export OHC_MEMORY_DIR='{}';", self.memory_dir.display());

        let wrapper_cmd = format!(
            "{} source '{}' 2>/dev/null || true; {{ {}; }}; declare -p > '{}'; pwd -P > '{}'",
            memory_dir_export, env_snapshot_path.display(), command, env_snapshot_path.display(), cwd_snapshot_path.display()
        );

        let mut cmd = self.build_command(&wrapper_cmd);
        
        let cwd = self.current_cwd.read().await.clone();
        cmd.current_dir(cwd);
        
        cmd.env_clear();
        cmd.env("PATH", "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin");

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

    #[tokio::test]
    async fn test_shell_session_bwrap_args() {
        let dir = "/tmp/test_session_bwrap";
        let _ = tokio::fs::remove_dir_all(dir).await;
        let mut session = ShellSession::new("sess3", dir).await.unwrap();

        // Force bwrap mode for test evaluation
        session.bwrap_available = true;
        let cmd = session.build_command("echo test");
        let cmd_str = format!("{:?}", cmd);

        assert!(cmd_str.contains("bwrap"));
        assert!(cmd_str.contains("--unshare-pid"));
        assert!(cmd_str.contains("--tmpfs"));
        assert!(cmd_str.contains("--bind"));
        assert!(cmd_str.contains("/tmp/test_session_bwrap"));

        // Force bash fallback
        session.bwrap_available = false;
        let cmd2 = session.build_command("echo test");
        let cmd_str2 = format!("{:?}", cmd2);

        assert!(!cmd_str2.contains("bwrap"));
        assert!(cmd_str2.contains("bash"));
        assert!(cmd_str2.contains("-c"));

        let _ = tokio::fs::remove_dir_all(dir).await;
    }
}
