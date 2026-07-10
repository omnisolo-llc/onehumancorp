use std::fs;
use std::process::Output;
use std::time::Duration;
use tempfile::{tempdir, TempDir};
use tokio::process::Command as AsyncCommand;
use tokio::time::timeout;

pub type SandboxResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait::async_trait]
pub trait ExecutionEnvironment: Send + Sync {
    async fn execute_context(&self, command: String, work_dir: String) -> SandboxResult<String>;
}

pub struct LocalEnvironment {
    dir: TempDir,
}

impl LocalEnvironment {
    pub fn new() -> SandboxResult<Self> {
        let dir = tempdir()?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700))?;
        }

        Ok(Self { dir })
    }

    pub fn dir_path(&self) -> &std::path::Path {
        self.dir.path()
    }
}

#[async_trait::async_trait]
impl ExecutionEnvironment for LocalEnvironment {
    async fn execute_context(&self, command: String, work_dir: String) -> SandboxResult<String> {
        self.execute(&command, &work_dir, Duration::from_secs(30)).await
            .map(|out| String::from_utf8_lossy(&out.stdout).to_string())
    }
}

impl LocalEnvironment {
    pub async fn execute(&self, cmd: &str, work_dir: &str, timeout_dur: Duration) -> SandboxResult<Output> {
        // Wrap command for Bash execution to disable extended globs
        let wrapped_cmd = format!("shopt -u extglob 2>/dev/null || true; cd '{}'; {}", work_dir, cmd);

        let dir_str = self.dir.path().to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "failed to convert temp dir path to string")
        })?;

        let mut command = AsyncCommand::new("bash");
        command.arg("-c").arg(wrapped_cmd);

        // Force TMPDIR to sandbox directory
        command.env("TMPDIR", dir_str);

        // Override HOME to temporary directory for isolation
        let home_dir = self.dir.path().join(".agent-home");
        fs::create_dir_all(&home_dir).unwrap_or_default();
        command.env("HOME", home_dir.to_str().unwrap_or(dir_str));

        // Scrub sensitive environment variables
        command.env_remove("OHC_API_KEY");
        command.env_remove("GH_TOKEN");
        command.env_remove("GITHUB_TOKEN");
        command.env_remove("OTEL_EXPORTER_OTLP_HEADERS");

        let rusage_start = {
            let mut rusage = std::mem::MaybeUninit::<libc::rusage>::uninit();
            unsafe {
                libc::getrusage(libc::RUSAGE_CHILDREN, rusage.as_mut_ptr());
                rusage.assume_init()
            }
        };

        let child = command.output();

        match timeout(timeout_dur, child).await {
            Ok(output_result) => {
                let output = output_result?;

                let mut rusage_end = std::mem::MaybeUninit::<libc::rusage>::uninit();
                unsafe {
                    libc::getrusage(libc::RUSAGE_CHILDREN, rusage_end.as_mut_ptr());
                    let rusage_end = rusage_end.assume_init();

                    let utime_sec = rusage_end.ru_utime.tv_sec - rusage_start.ru_utime.tv_sec;
                    #[cfg(target_os = "linux")]
                    let utime_usec = rusage_end.ru_utime.tv_usec - rusage_start.ru_utime.tv_usec;
                    #[cfg(target_os = "linux")]
                    let stime_usec = rusage_end.ru_stime.tv_usec - rusage_start.ru_stime.tv_usec;
                    #[cfg(not(target_os = "linux"))]
                    let utime_usec = rusage_end.ru_utime.tv_usec as i64 - rusage_start.ru_utime.tv_usec as i64;
                    #[cfg(not(target_os = "linux"))]
                    let stime_usec = rusage_end.ru_stime.tv_usec as i64 - rusage_start.ru_stime.tv_usec as i64;
                    let stime_sec = rusage_end.ru_stime.tv_sec - rusage_start.ru_stime.tv_sec;
                    let cpu_usage = (utime_sec as f64 + utime_usec as f64 / 1_000_000.0) + (stime_sec as f64 + stime_usec as f64 / 1_000_000.0);
                    let mem_bytes = (rusage_end.ru_maxrss) as f64 * 1024.0;
                    let net_io = (rusage_end.ru_inblock + rusage_end.ru_oublock) as f64;
                    ::server_telemetry::record_sandbox_cpu_usage("local_sandbox", cpu_usage);
                    ::server_telemetry::record_sandbox_memory_bytes("local_sandbox", mem_bytes);
                    ::server_telemetry::record_sandbox_network_io("local_sandbox", net_io);
                }

                Ok(output)
            }
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Command execution timed out").into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_sandbox_execute_tmpdir() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();
        let output = sm.execute_context("echo $TMPDIR".to_string(), work_dir).await.unwrap();

        assert_eq!(output.trim(), sm.dir_path().to_str().unwrap());
    }

    #[tokio::test]
    async fn test_sandbox_execute_shopt() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();
        let output = sm.execute_context("shopt | grep extglob".to_string(), work_dir).await.unwrap();

        assert!(output.contains("extglob\toff") || output.contains("extglob        \toff") || output.contains("extglob\t off") || output.contains("extglob") && output.contains("off"));
    }

    #[tokio::test]
    async fn test_sandbox_execute_timeout() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();
        let result = sm.execute("sleep 1", &work_dir, Duration::from_millis(10)).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Command execution timed out");
    }

    #[tokio::test]
    async fn test_sandbox_environment_scrubbing() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();

        let output = sm.execute_context("echo ${GITHUB_TOKEN:-not_found}".to_string(), work_dir).await.unwrap();

        // It should output not_found because the environment variable is stripped out from the command context
        assert_eq!(output.trim(), "not_found");
    }

    #[tokio::test]
    async fn test_sandbox_home_override() {
        let sm = LocalEnvironment::new().unwrap();
        let work_dir = sm.dir_path().to_str().unwrap().to_string();

        let output = sm.execute_context("echo $HOME".to_string(), work_dir).await.unwrap();

        let expected_home = sm.dir_path().join(".agent-home");
        assert_eq!(output.trim(), expected_home.to_str().unwrap());
    }
}
