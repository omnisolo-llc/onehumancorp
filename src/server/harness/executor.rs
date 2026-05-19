use super::sandbox::{SandboxManager, SandboxAdapter};
use sqlx::PgPool;
use std::time::Instant;
use opentelemetry::{global, KeyValue};
use opentelemetry::metrics::{Histogram, Counter};
use opentelemetry::trace::Tracer;

pub struct LocalShellTask {
    manager: SandboxManager,
    tenant_id: String,
    duration_histogram: Histogram<f64>,
    io_counter: Counter<u64>,
}

impl LocalShellTask {
    pub fn new(pool: Option<PgPool>, tenant_id: String) -> Self {
        let meter = global::meter("ohc.harness.executor");
        let duration_histogram = meter.f64_histogram("ohc_harness_command_duration_seconds").build();
        let io_counter = meter.u64_counter("ohc_harness_io_bytes_total").build();

        LocalShellTask {
            manager: SandboxManager::new(pool),
            tenant_id,
            duration_histogram,
            io_counter,
        }
    }

    pub async fn update_config(&mut self, policy_json: &str) -> Result<(), String> {
        self.manager.update_config(policy_json).await
    }

    pub async fn execute(&self, cmd: &str) -> Result<String, String> {
        let tracer = global::tracer("ohc.harness.executor");
        let _span = tracer.start("execute_command");
        let start = Instant::now();

        let command_prefix = cmd.split_whitespace().next().unwrap_or("unknown").to_string();

        let wrapped_cmd = match self.manager.wrap_command(cmd).await {
            Ok(c) => c,
            Err(e) => {
                let duration = start.elapsed().as_secs_f64();
                self.duration_histogram.record(duration, &[
                    KeyValue::new("tenant_id", self.tenant_id.clone()),
                    KeyValue::new("command_prefix", command_prefix.clone()),
                    KeyValue::new("exit_code", "error"),
                ]);
                self.io_counter.add(0, &[
                    KeyValue::new("tenant_id", self.tenant_id.clone()),
                    KeyValue::new("stream_type", "stdout"),
                ]);
                self.io_counter.add(e.len() as u64, &[
                    KeyValue::new("tenant_id", self.tenant_id.clone()),
                    KeyValue::new("stream_type", "stderr"),
                ]);
                return Err(self.manager.annotate_error(e, String::new()));
            }
        };

        // In a real execution, we would run `wrapped_cmd` using `tokio::process::Command`
        // For the scope of this harness executor logic, we just return the wrapped command
        // or execute it if needed. Let's return the wrapped command as a success placeholder
        // to show interception logic.

        let duration = start.elapsed().as_secs_f64();
        self.duration_histogram.record(duration, &[
            KeyValue::new("tenant_id", self.tenant_id.clone()),
            KeyValue::new("command_prefix", command_prefix.clone()),
            KeyValue::new("exit_code", "0"),
        ]);

        // Simulating stdout output length for successful execution placeholder
        let output = format!("Executing: {}", wrapped_cmd);

        self.io_counter.add(output.len() as u64, &[
            KeyValue::new("tenant_id", self.tenant_id.clone()),
            KeyValue::new("stream_type", "stdout"),
        ]);

        self.io_counter.add(0, &[
            KeyValue::new("tenant_id", self.tenant_id.clone()),
            KeyValue::new("stream_type", "stderr"),
        ]);

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_allowed_command_execution() {
        let task = LocalShellTask::new(None, "test-tenant".to_string());
        let result = task.execute("echo 'hello'").await;
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.contains("Executing: bash -c \"set -e; umask 077; echo 'hello'\""));
    }

    #[tokio::test]
    async fn test_denied_command_execution() {
        let task = LocalShellTask::new(None, "test-tenant".to_string());
        let result = task.execute("sudo rm -rf /").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("SANDBOX_FAILURE"));
        assert!(err.contains("Command execution denied by sandbox policy"));
    }

    #[tokio::test]
    async fn test_dynamic_config_update() {
        let mut task = LocalShellTask::new(None, "test-tenant".to_string());

        let result1 = task.execute("curl http://example.com").await;
        assert!(result1.is_ok());

        let policy = r#"{
            "disabled_commands": ["curl"]
        }"#;

        task.update_config(policy).await.unwrap();

        let result2 = task.execute("curl http://example.com").await;
        assert!(result2.is_err());

        let msg = result2.unwrap_err();
        assert!(msg.contains("Command execution denied by sandbox policy"));
    }

    #[tokio::test]
    async fn test_dynamic_config_wrapper_update() {
        let mut task = LocalShellTask::new(None, "test-tenant".to_string());

        let policy = r#"{
            "read_only_paths": ["/etc", "/var"],
            "blocked_domains": ["evil.com"]
        }"#;

        task.update_config(policy).await.unwrap();

        let result = task.execute("echo 'hello'").await;
        assert!(result.is_ok());
        let msg = result.unwrap();
        assert!(msg.contains("export READ_ONLY_PATHS='/etc:/var'"));
        assert!(msg.contains("export BLOCKED_DOMAINS='evil.com'"));
    }
}
