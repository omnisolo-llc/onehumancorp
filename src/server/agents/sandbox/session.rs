use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::process::Command;
use tokio::sync::RwLock;

pub struct ShellSession {
    pub session_id: String,
    pub sandbox_dir: PathBuf,
    pub current_cwd: RwLock<PathBuf>,
}

impl ShellSession {
    pub async fn new(session_id: &str, sandbox_dir: &str) -> Result<Self, String> {
        let sandbox_path = Path::new(sandbox_dir);
        fs::create_dir_all(sandbox_path).await.map_err(|e| e.to_string())?;

        let env_snapshot_path = sandbox_path.join("env_snapshot.sh");
        if !env_snapshot_path.exists() {
            fs::write(&env_snapshot_path, b"").await.map_err(|e| e.to_string())?;
        }

        Ok(ShellSession {
            session_id: session_id.to_string(),
            sandbox_dir: sandbox_path.to_path_buf(),
            current_cwd: RwLock::new(sandbox_path.to_path_buf()),
        })
    }

    pub async fn run_stateful_command(&self, command: &str) -> Result<String, String> {
        let env_snapshot_path = self.sandbox_dir.join("env_snapshot.sh");
        let cwd_snapshot_path = self.sandbox_dir.join("cwd_snapshot.txt");

        let wrapper_cmd = format!(
            "source '{}' 2>/dev/null || true; {{ {}; }}; declare -p > '{}'; pwd -P > '{}'",
            env_snapshot_path.display(), command, env_snapshot_path.display(), cwd_snapshot_path.display()
        );

        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(wrapper_cmd);

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
        let session = ShellSession::new("sess1", dir).await.unwrap();

        let out = session.run_stateful_command("echo hello").await.unwrap();
        assert!(out.contains("hello"));

        let out = session.run_stateful_command("export FOO=bar").await.unwrap();
        let out = session.run_stateful_command("echo $FOO").await.unwrap();
        assert!(out.contains("bar"));

        let _ = tokio::fs::remove_dir_all(dir).await;
    }
}
