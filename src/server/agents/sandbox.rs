use std::fs;
use std::process::{Output, Stdio};
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


        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());

        // Use spawn to get a handle for monitoring
        let child = command.spawn().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;


        let pid_opt = child.id();
        let stop_poller = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop_poller_clone = stop_poller.clone();

        if let Some(pid) = pid_opt {
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
                loop {
                    interval.tick().await;
                    if stop_poller_clone.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    // Collect CPU/Memory from /proc/{pid}/stat
                    let stat_path = format!("/proc/{}/statm", pid);
                    if let Ok(statm) = tokio::fs::read_to_string(&stat_path).await {
                        let parts: Vec<&str> = statm.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(rss) = parts[1].parse::<i64>() {
                                // rss is in pages. typically 4096 bytes per page
                                let memory_bytes = rss * 4096;
                                ::server_telemetry::record_sandbox_memory_bytes("unknown", "unknown", "unknown", memory_bytes);
                            }
                        }
                    }

                    let stat_path_cpu = format!("/proc/{}/stat", pid);
                    if let Ok(stat) = tokio::fs::read_to_string(&stat_path_cpu).await {
                        let parts: Vec<&str> = stat.split_whitespace().collect();
                        if parts.len() >= 15 {
                            if let (Ok(utime), Ok(stime)) = (parts[13].parse::<f64>(), parts[14].parse::<f64>()) {
                                // Very rough approximation, sysconf(_SC_CLK_TCK) is usually 100
                                let cpu_usage = (utime + stime) / 100.0;
                                ::server_telemetry::record_sandbox_cpu_usage("unknown", "unknown", "unknown", cpu_usage);
                            }
                        }
                    }

                    // Simple network I/O from /proc/net/dev (though this is global, not isolated unless in network namespace)
                    // If in container, it might be isolated. We just sum up the received/transmitted bytes for eth0
                    if let Ok(net_dev) = tokio::fs::read_to_string("/proc/net/dev").await {
                        let mut total_bytes: i64 = 0;
                        for line in net_dev.lines().skip(2) {
                            let parts: Vec<&str> = line.split_whitespace().collect();
                            if parts.len() >= 10 {
                                if let (Ok(rx), Ok(tx)) = (parts[1].parse::<i64>(), parts[9].parse::<i64>()) {
                                    total_bytes += rx + tx;
                                }
                            }
                        }
                        if total_bytes > 0 {
                            ::server_telemetry::record_sandbox_network_io("unknown", "unknown", "unknown", total_bytes);
                        }
                    }
                }
            });
        }

        let child_future = child.wait_with_output();

        let res = match timeout(timeout_dur, child_future).await {
            Ok(output_result) => output_result.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>),
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Command execution timed out").into()),
        };

        stop_poller.store(true, std::sync::atomic::Ordering::Relaxed);

        res
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
