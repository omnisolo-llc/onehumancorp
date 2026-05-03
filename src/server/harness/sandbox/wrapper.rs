use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use tempfile::TempDir;

pub struct BashWrapper;

impl BashWrapper {
    pub fn new() -> Self {
        BashWrapper
    }

    pub async fn execute(&self, cmd: &str, temp_dir: &TempDir, timeout_duration: Duration) -> Result<String, String> {
        let wrapper_cmd = format!("shopt -u extglob 2>/dev/null || true; {}", cmd);

        let mut command = Command::new("bash");
        command.arg("-c").arg(&wrapper_cmd);
        command.env("TMPDIR", temp_dir.path());
        command.current_dir(temp_dir.path());

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let output = timeout(timeout_duration, command.output()).await
            .map_err(|_| "Command execution timed out".to_string())?
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(format!("Command failed with exit code {}\nSTDOUT:\n{}\nSTDERR:\n{}",
                output.status.code().unwrap_or(-1), stdout, stderr))
        }
    }
}

pub struct PowerShellWrapper;

impl PowerShellWrapper {
    pub fn new() -> Self {
        PowerShellWrapper
    }

    pub async fn execute(&self, cmd: &str, temp_dir: &TempDir, timeout_duration: Duration) -> Result<String, String> {
        let mut command = Command::new("pwsh");
        command.arg("-Command").arg(cmd);
        command.env("TMP", temp_dir.path());
        command.env("TEMP", temp_dir.path());
        command.current_dir(temp_dir.path());

        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        let output = timeout(timeout_duration, command.output()).await
            .map_err(|_| "Command execution timed out".to_string())?
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(format!("Command failed with exit code {}\nSTDOUT:\n{}\nSTDERR:\n{}",
                output.status.code().unwrap_or(-1), stdout, stderr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bash_wrapper_tmpdir() {
        let temp_dir = TempDir::new().unwrap();
        let wrapper = BashWrapper::new();

        // Ensure TMPDIR is correctly scoped
        let result = wrapper.execute("echo $TMPDIR", &temp_dir, Duration::from_secs(5)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), temp_dir.path().to_str().unwrap());
    }

    #[tokio::test]
    async fn test_bash_wrapper_extglob() {
        let temp_dir = TempDir::new().unwrap();
        let wrapper = BashWrapper::new();

        // Ensure extglob is disabled
        let result = wrapper.execute("shopt extglob", &temp_dir, Duration::from_secs(5)).await;
        // In bash, shopt without arguments exits with 0 if true, 1 if false
        // The execute function will return an Err if the exit code is not 0
        // Wait, "shopt extglob" returns 1 if disabled.
        // Let's just output it and check string.
        let result = wrapper.execute("shopt extglob || true", &temp_dir, Duration::from_secs(5)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("off"));
    }

    #[tokio::test]
    async fn test_bash_wrapper_timeout() {
        let temp_dir = TempDir::new().unwrap();
        let wrapper = BashWrapper::new();

        // Ensure timeout correctly kills long-running commands
        let result = wrapper.execute("sleep 2", &temp_dir, Duration::from_secs(1)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Command execution timed out");
    }
}
