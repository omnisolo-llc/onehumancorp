use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use tokio::net::{TcpListener, TcpStream};
use tokio::process::{Child, Command};
use tokio::time::{Instant, sleep};

use super::process_env::apply_isolated_environment;

const DEFAULT_READINESS_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_millis(50);
const MAX_EPHEMERAL_SPAWN_ATTEMPTS: usize = 3;
const MINIMUM_OWNERSHIP_SETTLE_INTERVAL: Duration = Duration::from_millis(50);

#[derive(Clone, PartialEq)]
pub struct HttpProcessConfig {
    pub executable: String,
    pub args: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub working_directory: Option<PathBuf>,
    pub preferred_address: SocketAddr,
    pub readiness_timeout: Duration,
    pub poll_interval: Duration,
}

impl std::fmt::Debug for HttpProcessConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HttpProcessConfig")
            .field("executable", &self.executable)
            .field("arg_count", &self.args.len())
            .field(
                "environment_keys",
                &self.environment.keys().collect::<Vec<_>>(),
            )
            .field("working_directory", &self.working_directory)
            .field("preferred_address", &self.preferred_address)
            .field("readiness_timeout", &self.readiness_timeout)
            .field("poll_interval", &self.poll_interval)
            .finish()
    }
}

impl HttpProcessConfig {
    pub fn new<I, S>(executable: impl Into<String>, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            executable: executable.into(),
            args: args.into_iter().map(Into::into).collect(),
            environment: BTreeMap::new(),
            working_directory: None,
            preferred_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            readiness_timeout: DEFAULT_READINESS_TIMEOUT,
            poll_interval: DEFAULT_POLL_INTERVAL,
        }
    }

    pub fn with_environment(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    pub fn with_working_directory(mut self, working_directory: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    pub fn with_preferred_address(mut self, preferred_address: SocketAddr) -> Self {
        self.preferred_address = preferred_address;
        self
    }

    pub fn with_readiness(mut self, timeout: Duration, poll_interval: Duration) -> Self {
        self.readiness_timeout = timeout;
        self.poll_interval = poll_interval;
        self
    }
}

#[derive(Debug)]
pub enum HttpProcessError {
    NonLoopbackAddress(SocketAddr),
    Bind(std::io::Error),
    Spawn(std::io::Error),
    ReadinessTimeout { address: SocketAddr },
    EarlyExit { status: Option<i32> },
    Poll(std::io::Error),
}

impl std::fmt::Display for HttpProcessError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonLoopbackAddress(address) => {
                write!(formatter, "HTTP harness address is not loopback: {address}")
            }
            Self::Bind(error) => write!(formatter, "failed to allocate HTTP harness port: {error}"),
            Self::Spawn(error) => write!(formatter, "failed to spawn HTTP harness: {error}"),
            Self::ReadinessTimeout { address } => {
                write!(formatter, "HTTP harness readiness timed out at {address}")
            }
            Self::EarlyExit { status } => {
                write!(
                    formatter,
                    "HTTP harness exited before readiness: {status:?}"
                )
            }
            Self::Poll(error) => write!(formatter, "failed to poll HTTP harness process: {error}"),
        }
    }
}

impl std::error::Error for HttpProcessError {}

pub struct HttpProcessRuntime {
    child: Child,
    address: SocketAddr,
    base_url: String,
}

impl std::fmt::Debug for HttpProcessRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HttpProcessRuntime")
            .field("address", &self.address)
            .field("base_url", &self.base_url)
            .field("process_id", &self.child.id())
            .finish()
    }
}

impl HttpProcessRuntime {
    pub async fn spawn(config: HttpProcessConfig) -> Result<Self, HttpProcessError> {
        if !config.preferred_address.ip().is_loopback() {
            return Err(HttpProcessError::NonLoopbackAddress(
                config.preferred_address,
            ));
        }
        let max_attempts = if config.preferred_address.port() == 0 {
            MAX_EPHEMERAL_SPAWN_ATTEMPTS
        } else {
            1
        };
        for attempt in 1..=max_attempts {
            match Self::spawn_once(&config).await {
                Err(HttpProcessError::EarlyExit { .. }) if attempt < max_attempts => continue,
                result => return result,
            }
        }
        unreachable!("the bounded HTTP harness spawn loop always returns")
    }

    async fn spawn_once(config: &HttpProcessConfig) -> Result<Self, HttpProcessError> {
        let reservation = TcpListener::bind(config.preferred_address)
            .await
            .map_err(HttpProcessError::Bind)?;
        let address = reservation.local_addr().map_err(HttpProcessError::Bind)?;
        let base_url = format!("http://{address}");
        let args = config
            .args
            .iter()
            .map(|argument| substitute(argument, address, &base_url))
            .collect::<Vec<_>>();
        let environment = config
            .environment
            .iter()
            .map(|(key, value)| (key, substitute(value, address, &base_url)))
            .collect::<Vec<_>>();

        let mut command = Command::new(&config.executable);
        command.args(args);
        apply_isolated_environment(&mut command, environment);
        command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        if let Some(working_directory) = &config.working_directory {
            command.current_dir(working_directory);
        }

        // Release the reserved loopback port immediately before the child binds it.
        drop(reservation);
        let mut child = command.spawn().map_err(HttpProcessError::Spawn)?;
        let deadline = Instant::now() + config.readiness_timeout;
        loop {
            if let Some(status) = child.try_wait().map_err(HttpProcessError::Poll)? {
                return Err(HttpProcessError::EarlyExit {
                    status: status.code(),
                });
            }
            if TcpStream::connect(address).await.is_ok() {
                // A reserved ephemeral port can be claimed by an unrelated
                // concurrently-starting process in the short handoff between
                // releasing the reservation and the child binding. Require
                // the child to remain alive across a settling interval and
                // require the listener to remain reachable before admitting
                // work; an exiting child otherwise may not be reaped after a
                // single scheduler yield.
                sleep(config.poll_interval.max(MINIMUM_OWNERSHIP_SETTLE_INTERVAL)).await;
                if let Some(status) = child.try_wait().map_err(HttpProcessError::Poll)? {
                    return Err(HttpProcessError::EarlyExit {
                        status: status.code(),
                    });
                }
                if TcpStream::connect(address).await.is_err() {
                    continue;
                }
                return Ok(Self {
                    child,
                    address,
                    base_url,
                });
            }
            if Instant::now() >= deadline {
                let _ = child.start_kill();
                let _ = child.wait().await;
                return Err(HttpProcessError::ReadinessTimeout { address });
            }
            sleep(config.poll_interval).await;
        }
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn process_id(&self) -> Option<u32> {
        self.child.id()
    }

    pub async fn shutdown(mut self) -> Result<(), std::io::Error> {
        if self.child.try_wait()?.is_none() {
            self.child.kill().await?;
        }
        self.child.wait().await?;
        Ok(())
    }
}

impl Drop for HttpProcessRuntime {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}

fn substitute(value: &str, address: SocketAddr, base_url: &str) -> String {
    value
        .replace("{address}", &address.to_string())
        .replace("{host}", &address.ip().to_string())
        .replace("{port}", &address.port().to_string())
        .replace("{base_url}", base_url)
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
    use std::sync::{Mutex, OnceLock};
    use std::time::Duration;

    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    use super::{HttpProcessConfig, HttpProcessError, HttpProcessRuntime};

    const CHILD_ENV: &str = "OMNISOLO_HTTP_RUNTIME_TEST_CHILD";

    #[test]
    fn mock_http_child_process() {
        let Ok(mut mode) = std::env::var(CHILD_ENV) else {
            return;
        };
        if mode == "exit" {
            std::process::exit(23);
        }
        if mode == "sleep" {
            std::thread::sleep(Duration::from_secs(30));
            return;
        }
        if mode == "exit_once" {
            let marker = std::env::var("OMNISOLO_HTTP_TEST_RETRY_MARKER").unwrap();
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(marker)
            {
                Ok(_) => std::process::exit(24),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    mode = "serve".to_owned();
                }
                Err(error) => panic!("create retry marker: {error}"),
            }
        }
        let environment_mode = mode == "environment";
        assert!(environment_mode || mode == "serve");

        let address = std::env::var("OMNISOLO_HTTP_TEST_ADDRESS").unwrap();
        let listener = TcpListener::bind(address).unwrap();
        let body = if environment_mode {
            format!(
                "{}:{}:{}",
                std::env::var_os("UNRELATED_DEPLOYMENT_SECRET").is_some(),
                std::env::var_os("OPENAI_API_KEY").is_some(),
                std::env::var_os("PATH").is_some()
            )
        } else {
            "ok".to_owned()
        };
        for connection in listener.incoming() {
            let mut connection = connection.unwrap();
            let mut request = [0_u8; 512];
            let _ = connection.read(&mut request);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            connection.write_all(response.as_bytes()).unwrap();
        }
    }

    fn child_config(mode: &str) -> HttpProcessConfig {
        HttpProcessConfig::new(
            std::env::current_exe().unwrap().to_string_lossy(),
            [
                "--exact",
                "middleware::http_runtime::tests::mock_http_child_process",
            ],
        )
        .with_environment(CHILD_ENV, mode)
        .with_environment("OMNISOLO_HTTP_TEST_ADDRESS", "{address}")
        .with_readiness(Duration::from_secs(2), Duration::from_millis(10))
    }

    fn environment_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[tokio::test]
    async fn ambient_parent_environment_is_not_inherited() {
        let _guard = environment_lock().lock().unwrap();
        unsafe {
            std::env::set_var("UNRELATED_DEPLOYMENT_SECRET", "CANARY-AMBIENT-7KQ9");
        }
        let runtime = HttpProcessRuntime::spawn(
            child_config("environment").with_environment("OPENAI_API_KEY", "explicit-key"),
        )
        .await
        .unwrap();
        let mut stream = TcpStream::connect(runtime.address()).await.unwrap();
        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .await
            .unwrap();
        let mut response = Vec::new();
        tokio::time::timeout(Duration::from_secs(1), stream.read_to_end(&mut response))
            .await
            .unwrap()
            .unwrap();

        unsafe {
            std::env::remove_var("UNRELATED_DEPLOYMENT_SECRET");
        }
        let response = String::from_utf8(response).unwrap();
        assert!(
            response.ends_with("false:true:true"),
            "response: {response}"
        );
        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn binds_only_loopback_substitutes_address_and_becomes_ready() {
        let runtime = HttpProcessRuntime::spawn(child_config("serve"))
            .await
            .unwrap();
        assert_eq!(runtime.address().ip(), IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(runtime.base_url(), format!("http://{}", runtime.address()));
        tokio::net::TcpStream::connect(runtime.address())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn reports_early_child_exit() {
        let error = HttpProcessRuntime::spawn(child_config("exit"))
            .await
            .unwrap_err();
        assert!(matches!(
            error,
            HttpProcessError::EarlyExit { status: Some(23) }
        ));
    }

    #[tokio::test]
    async fn retries_an_early_exit_when_using_an_ephemeral_port() {
        let marker = std::env::temp_dir().join(format!(
            "omnisolo-http-runtime-retry-{}",
            uuid::Uuid::new_v4()
        ));
        let runtime = HttpProcessRuntime::spawn(
            child_config("exit_once")
                .with_environment("OMNISOLO_HTTP_TEST_RETRY_MARKER", marker.to_string_lossy()),
        )
        .await
        .unwrap();
        tokio::net::TcpStream::connect(runtime.address())
            .await
            .unwrap();
        runtime.shutdown().await.unwrap();
        std::fs::remove_file(marker).unwrap();
    }

    #[tokio::test]
    async fn reports_spawn_failures() {
        let error = HttpProcessRuntime::spawn(HttpProcessConfig::new(
            "/definitely/missing/http-harness",
            std::iter::empty::<String>(),
        ))
        .await
        .unwrap_err();
        assert!(matches!(error, HttpProcessError::Spawn(_)));
    }

    #[tokio::test]
    async fn reports_bounded_readiness_timeout() {
        let config = child_config("sleep")
            .with_readiness(Duration::from_millis(100), Duration::from_millis(10));
        let error = HttpProcessRuntime::spawn(config).await.unwrap_err();
        assert!(matches!(error, HttpProcessError::ReadinessTimeout { .. }));
    }

    #[tokio::test]
    async fn rejects_non_loopback_preferred_addresses() {
        let error = HttpProcessRuntime::spawn(
            child_config("serve").with_preferred_address(SocketAddr::from(([0, 0, 0, 0], 0))),
        )
        .await
        .unwrap_err();
        assert!(matches!(error, HttpProcessError::NonLoopbackAddress(_)));
    }

    #[tokio::test]
    async fn drop_kills_the_child_process() {
        let runtime = HttpProcessRuntime::spawn(child_config("serve"))
            .await
            .unwrap();
        let process_id = runtime.process_id().unwrap();
        drop(runtime);

        let mut exited = false;
        for _ in 0..50 {
            let status = std::process::Command::new("/bin/sh")
                .args(["-c", &format!("kill -0 {process_id} 2>/dev/null")])
                .status()
                .unwrap();
            if !status.success() {
                exited = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert!(
            exited,
            "HTTP harness child {process_id} survived runtime drop"
        );
    }

    #[test]
    fn substitutes_every_supported_endpoint_placeholder() {
        let address = SocketAddr::from(([127, 0, 0, 1], 43123));
        assert_eq!(
            super::substitute(
                "--listen={address};--host={host};--port={port};--url={base_url}",
                address,
                "http://127.0.0.1:43123",
            ),
            "--listen=127.0.0.1:43123;--host=127.0.0.1;--port=43123;--url=http://127.0.0.1:43123"
        );
    }

    #[test]
    fn config_debug_never_exposes_argument_or_environment_values() {
        let config = HttpProcessConfig::new("harness", ["--api-key", "secret-canary"])
            .with_environment("OPENAI_API_KEY", "secret-canary");
        let debug = format!("{config:?}");
        assert!(!debug.contains("secret-canary"));
        assert!(debug.contains("OPENAI_API_KEY"));
    }
}
