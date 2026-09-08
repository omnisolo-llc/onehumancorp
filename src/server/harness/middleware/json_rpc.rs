use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{Mutex, broadcast, oneshot};
use tokio::time::timeout;

use super::process_env::apply_isolated_environment;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonRpcId {
    raw: Value,
    key: String,
}

impl JsonRpcId {
    fn numeric(value: u64) -> Self {
        Self::from_raw(Value::from(value)).expect("numeric JSON-RPC ids are always valid")
    }

    pub fn from_number(value: u64) -> Self {
        Self::numeric(value)
    }

    pub fn as_value(&self) -> Value {
        self.raw.clone()
    }

    pub fn from_value(raw: Value) -> Result<Self, JsonRpcError> {
        Self::from_raw(raw)
    }

    fn from_raw(raw: Value) -> Result<Self, JsonRpcError> {
        if raw.is_null() || !(raw.is_string() || raw.is_number()) {
            return Err(JsonRpcError::InvalidMessage(
                "JSON-RPC id must be a string or number".to_owned(),
            ));
        }
        Ok(Self {
            key: raw.to_string(),
            raw,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JsonRpcNotification {
    pub method: String,
    pub params: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JsonRpcServerRequest {
    pub id: JsonRpcId,
    pub method: String,
    pub params: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JsonRpcErrorObject {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsonRpcError {
    Spawn(String),
    Io(String),
    Malformed(String),
    InvalidMessage(String),
    Timeout,
    ProcessExited,
    UnknownRequest(JsonRpcId),
    RequestFailed(JsonRpcErrorObject),
    Cancelled,
}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spawn(error) => write!(formatter, "failed to spawn JSON-RPC process: {error}"),
            Self::Io(error) => write!(formatter, "JSON-RPC I/O error: {error}"),
            Self::Malformed(error) => write!(formatter, "malformed JSON-RPC frame: {error}"),
            Self::InvalidMessage(error) => write!(formatter, "invalid JSON-RPC message: {error}"),
            Self::Timeout => formatter.write_str("JSON-RPC request timed out"),
            Self::ProcessExited => formatter.write_str("JSON-RPC process exited"),
            Self::UnknownRequest(id) => {
                write!(formatter, "unknown JSON-RPC request id: {}", id.key)
            }
            Self::RequestFailed(error) => {
                write!(formatter, "JSON-RPC request failed: {}", error.message)
            }
            Self::Cancelled => formatter.write_str("JSON-RPC request cancelled"),
        }
    }
}

impl std::error::Error for JsonRpcError {}

pub type JsonRpcResult = Result<Value, JsonRpcError>;

#[derive(Clone)]
pub struct JsonRpcProcessConfig {
    pub executable: String,
    pub args: Vec<String>,
    pub environment: HashMap<String, String>,
    pub request_timeout: Duration,
    pub include_jsonrpc_header: bool,
}

impl std::fmt::Debug for JsonRpcProcessConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("JsonRpcProcessConfig")
            .field("executable", &self.executable)
            .field("args", &self.args)
            .field(
                "environment_keys",
                &self.environment.keys().collect::<Vec<_>>(),
            )
            .field("request_timeout", &self.request_timeout)
            .field("include_jsonrpc_header", &self.include_jsonrpc_header)
            .finish()
    }
}

impl JsonRpcProcessConfig {
    pub fn shell(script: impl Into<String>) -> Self {
        Self {
            executable: "/bin/sh".to_owned(),
            args: vec!["-c".to_owned(), script.into()],
            environment: HashMap::new(),
            request_timeout: Duration::from_secs(30),
            include_jsonrpc_header: true,
        }
    }

    pub fn without_jsonrpc_header(mut self) -> Self {
        self.include_jsonrpc_header = false;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsonRpcInbound {
    Response {
        id: JsonRpcId,
        result: JsonRpcResult,
    },
    Notification(JsonRpcNotification),
    ServerRequest(JsonRpcServerRequest),
}

struct RuntimeState {
    writer: Mutex<ChildStdin>,
    process: Mutex<Child>,
    pending: Mutex<HashMap<String, oneshot::Sender<JsonRpcResult>>>,
    server_requests: Mutex<HashMap<String, JsonRpcId>>,
    notifications: broadcast::Sender<JsonRpcNotification>,
    server_request_events: broadcast::Sender<JsonRpcServerRequest>,
    inbound_tx: broadcast::Sender<Result<JsonRpcInbound, JsonRpcError>>,
    inbound_rx: Mutex<broadcast::Receiver<Result<JsonRpcInbound, JsonRpcError>>>,
    next_id: AtomicU64,
    request_timeout: Duration,
    include_jsonrpc_header: bool,
    closed: AtomicBool,
}

#[derive(Clone)]
pub struct JsonRpcProcessRuntime {
    inner: Arc<RuntimeState>,
}

impl std::fmt::Debug for JsonRpcProcessRuntime {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("JsonRpcProcessRuntime")
            .field("closed", &self.inner.closed.load(Ordering::Acquire))
            .finish()
    }
}

impl JsonRpcProcessRuntime {
    pub async fn spawn(config: JsonRpcProcessConfig) -> Result<Self, JsonRpcError> {
        let mut command = Command::new(&config.executable);
        command.args(&config.args);
        apply_isolated_environment(&mut command, &config.environment);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let mut process = command
            .spawn()
            .map_err(|error| JsonRpcError::Spawn(error.to_string()))?;
        let stdin = take_piped(process.stdin.take(), "stdin")?;
        let stdout = take_piped(process.stdout.take(), "stdout")?;
        let (notifications, _) = broadcast::channel(256);
        let (server_request_events, _) = broadcast::channel(256);
        // Optional diagnostic observation must never backpressure protocol dispatch.
        let (inbound_tx, inbound_rx) = broadcast::channel(256);
        let runtime = Self {
            inner: Arc::new(RuntimeState {
                writer: Mutex::new(stdin),
                process: Mutex::new(process),
                pending: Mutex::new(HashMap::new()),
                server_requests: Mutex::new(HashMap::new()),
                notifications,
                server_request_events,
                inbound_tx,
                inbound_rx: Mutex::new(inbound_rx),
                next_id: AtomicU64::new(1),
                request_timeout: config.request_timeout,
                include_jsonrpc_header: config.include_jsonrpc_header,
                closed: AtomicBool::new(false),
            }),
        };
        let reader_runtime = runtime.clone();
        tokio::spawn(async move {
            reader_runtime.read_stdout(BufReader::new(stdout)).await;
        });
        Ok(runtime)
    }

    pub fn subscribe_notifications(&self) -> broadcast::Receiver<JsonRpcNotification> {
        self.inner.notifications.subscribe()
    }

    pub fn subscribe_server_requests(&self) -> broadcast::Receiver<JsonRpcServerRequest> {
        self.inner.server_request_events.subscribe()
    }

    pub async fn request(&self, method: &str, params: Value) -> JsonRpcResult {
        self.request_with_timeout(method, params, self.inner.request_timeout)
            .await
    }

    pub async fn request_with_timeout(
        &self,
        method: &str,
        params: Value,
        request_timeout: Duration,
    ) -> JsonRpcResult {
        self.request_with_optional_params(method, Some(params), request_timeout)
            .await
    }

    pub async fn request_without_params(&self, method: &str) -> JsonRpcResult {
        self.request_without_params_with_timeout(method, self.inner.request_timeout)
            .await
    }

    pub async fn request_without_params_with_timeout(
        &self,
        method: &str,
        request_timeout: Duration,
    ) -> JsonRpcResult {
        self.request_with_optional_params(method, None, request_timeout)
            .await
    }

    async fn request_with_optional_params(
        &self,
        method: &str,
        params: Option<Value>,
        request_timeout: Duration,
    ) -> JsonRpcResult {
        if self.inner.closed.load(Ordering::Acquire) {
            return Err(JsonRpcError::ProcessExited);
        }
        let id = JsonRpcId::numeric(self.inner.next_id.fetch_add(1, Ordering::AcqRel));
        let (sender, receiver) = oneshot::channel();
        self.inner
            .pending
            .lock()
            .await
            .insert(id.key.clone(), sender);
        let mut message = json!({
            "id": id.raw,
            "method": method,
        });
        if let Some(params) = params
            && let Some(object) = message.as_object_mut()
        {
            object.insert("params".to_owned(), params);
        }
        self.add_jsonrpc_header(&mut message);
        if let Err(error) = self.write_message(message).await {
            self.inner.pending.lock().await.remove(&id.key);
            return Err(error);
        }
        match timeout(request_timeout, receiver).await {
            Ok(result) => resolve_pending_result(result),
            Err(_) => {
                self.inner.pending.lock().await.remove(&id.key);
                Err(JsonRpcError::Timeout)
            }
        }
    }

    pub async fn notify(&self, method: &str, params: Value) -> Result<(), JsonRpcError> {
        if self.inner.closed.load(Ordering::Acquire) {
            return Err(JsonRpcError::ProcessExited);
        }
        let mut message = json!({
            "method": method,
            "params": params,
        });
        self.add_jsonrpc_header(&mut message);
        self.write_message(message).await
    }

    pub async fn next_message(&self) -> Result<JsonRpcInbound, JsonRpcError> {
        let mut receiver = self.inner.inbound_rx.lock().await;
        match receiver.recv().await {
            Ok(message) => message,
            Err(broadcast::error::RecvError::Closed) => Err(JsonRpcError::ProcessExited),
            Err(broadcast::error::RecvError::Lagged(count)) => Err(JsonRpcError::InvalidMessage(
                format!("diagnostic observer lagged by {count} messages"),
            )),
        }
    }

    pub async fn respond_server_request(
        &self,
        id: JsonRpcId,
        result: Value,
    ) -> Result<(), JsonRpcError> {
        self.respond_server_request_message(id, json!({"result": result}))
            .await
    }

    pub async fn respond_server_error(
        &self,
        id: JsonRpcId,
        error: JsonRpcErrorObject,
    ) -> Result<(), JsonRpcError> {
        self.respond_server_request_message(
            id,
            json!({
                "error": {
                    "code": error.code,
                    "message": error.message,
                    "data": error.data,
                }
            }),
        )
        .await
    }

    pub async fn shutdown(&self) -> Result<(), JsonRpcError> {
        if self.inner.closed.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        self.fail_pending(JsonRpcError::Cancelled).await;
        let mut process = self.inner.process.lock().await;
        process_kill_result(process.kill().await)?;
        process
            .wait()
            .await
            .map_err(|error| JsonRpcError::Io(error.to_string()))?;
        Ok(())
    }

    async fn write_message(&self, message: Value) -> Result<(), JsonRpcError> {
        let bytes = serde_json::to_vec(&message)
            .map_err(|error| JsonRpcError::Malformed(error.to_string()))?;
        let mut writer = self.inner.writer.lock().await;
        writer
            .write_all(&bytes)
            .await
            .map_err(|error| JsonRpcError::Io(error.to_string()))?;
        writer
            .write_all(b"\n")
            .await
            .map_err(|error| JsonRpcError::Io(error.to_string()))?;
        writer
            .flush()
            .await
            .map_err(|error| JsonRpcError::Io(error.to_string()))
    }

    async fn respond_server_request_message(
        &self,
        id: JsonRpcId,
        body: Value,
    ) -> Result<(), JsonRpcError> {
        let known = self
            .inner
            .server_requests
            .lock()
            .await
            .remove(&id.key)
            .is_some();
        if !known {
            return Err(JsonRpcError::UnknownRequest(id));
        }
        let mut message = serde_json::Map::new();
        message.insert("id".to_owned(), id.raw);
        if let Some(body) = body.as_object() {
            for (key, value) in body {
                message.insert(key.clone(), value.clone());
            }
        }
        let mut message = Value::Object(message);
        self.add_jsonrpc_header(&mut message);
        self.write_message(message).await
    }

    fn add_jsonrpc_header(&self, message: &mut Value) {
        if self.inner.include_jsonrpc_header
            && let Some(object) = message.as_object_mut()
        {
            object.insert("jsonrpc".to_owned(), Value::String("2.0".to_owned()));
        }
    }

    async fn read_stdout<R>(&self, mut stdout: BufReader<R>)
    where
        R: AsyncRead + Unpin,
    {
        let mut line = String::new();
        loop {
            line.clear();
            match stdout.read_line(&mut line).await {
                Ok(0) => {
                    self.inner.closed.store(true, Ordering::Release);
                    self.fail_pending(JsonRpcError::ProcessExited).await;
                    let _ = self.inner.inbound_tx.send(Err(JsonRpcError::ProcessExited));
                    break;
                }
                Ok(_) if line.trim().is_empty() => continue,
                Ok(_) => match parse_message(line.trim()) {
                    Ok(message) => {
                        self.route_message(message).await;
                    }
                    Err(error) => {
                        self.inner.closed.store(true, Ordering::Release);
                        self.fail_pending(error.clone()).await;
                        let _ = self.inner.inbound_tx.send(Err(error));
                        break;
                    }
                },
                Err(error) => {
                    self.fail_stdout_read(error).await;
                    break;
                }
            }
        }
    }

    async fn route_message(&self, message: JsonRpcInbound) {
        match &message {
            JsonRpcInbound::Response { id, result } => {
                if let Some(sender) = self.inner.pending.lock().await.remove(&id.key) {
                    let _ = sender.send(result.clone());
                }
            }
            JsonRpcInbound::Notification(notification) => {
                let _ = self.inner.notifications.send(notification.clone());
            }
            JsonRpcInbound::ServerRequest(request) => {
                self.inner
                    .server_requests
                    .lock()
                    .await
                    .insert(request.id.key.clone(), request.id.clone());
                let _ = self.inner.server_request_events.send(request.clone());
            }
        }
        let _ = self.inner.inbound_tx.send(Ok(message));
    }

    async fn fail_pending(&self, error: JsonRpcError) {
        let pending = std::mem::take(&mut *self.inner.pending.lock().await);
        for sender in pending.into_values() {
            let _ = sender.send(Err(error.clone()));
        }
    }

    async fn fail_stdout_read(&self, error: std::io::Error) {
        let error = JsonRpcError::Io(error.to_string());
        self.inner.closed.store(true, Ordering::Release);
        self.fail_pending(error.clone()).await;
        let _ = self.inner.inbound_tx.send(Err(error));
    }
}

fn take_piped<T>(pipe: Option<T>, stream: &str) -> Result<T, JsonRpcError> {
    pipe.ok_or_else(|| JsonRpcError::InvalidMessage(format!("JSON-RPC {stream} was not piped")))
}

fn resolve_pending_result(
    result: Result<JsonRpcResult, oneshot::error::RecvError>,
) -> JsonRpcResult {
    match result {
        Ok(result) => result,
        Err(_) => Err(JsonRpcError::ProcessExited),
    }
}

fn classify_kill_error(error: std::io::Error) -> Option<JsonRpcError> {
    if error.kind() == std::io::ErrorKind::InvalidInput {
        None
    } else {
        Some(JsonRpcError::Io(error.to_string()))
    }
}

fn process_kill_result(result: Result<(), std::io::Error>) -> Result<(), JsonRpcError> {
    match result {
        Ok(()) => Ok(()),
        Err(error) => classify_kill_error(error).map_or(Ok(()), Err),
    }
}

fn parse_message(line: &str) -> Result<JsonRpcInbound, JsonRpcError> {
    let value: Value =
        serde_json::from_str(line).map_err(|error| JsonRpcError::Malformed(error.to_string()))?;
    let object = value.as_object().ok_or_else(|| {
        JsonRpcError::InvalidMessage("JSON-RPC frame must be an object".to_owned())
    })?;
    if let Some(method) = object.get("method").and_then(Value::as_str) {
        let params = object.get("params").cloned().unwrap_or(Value::Null);
        if let Some(raw_id) = object.get("id") {
            return Ok(JsonRpcInbound::ServerRequest(JsonRpcServerRequest {
                id: JsonRpcId::from_raw(raw_id.clone())?,
                method: method.to_owned(),
                params,
            }));
        }
        return Ok(JsonRpcInbound::Notification(JsonRpcNotification {
            method: method.to_owned(),
            params,
        }));
    }
    let raw_id = object
        .get("id")
        .cloned()
        .ok_or_else(|| JsonRpcError::InvalidMessage("response is missing id".to_owned()))?;
    let id = JsonRpcId::from_raw(raw_id)?;
    if object.contains_key("result") && object.contains_key("error") {
        return Err(JsonRpcError::InvalidMessage(
            "response cannot contain both result and error".to_owned(),
        ));
    }
    let result = if let Some(result) = object.get("result") {
        Ok(result.clone())
    } else if let Some(error) = object.get("error") {
        let error = error.as_object().ok_or_else(|| {
            JsonRpcError::InvalidMessage("JSON-RPC error must be an object".to_owned())
        })?;
        Err(JsonRpcError::RequestFailed(JsonRpcErrorObject {
            code: error.get("code").and_then(Value::as_i64).unwrap_or(-1),
            message: error
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("unknown JSON-RPC error")
                .to_owned(),
            data: error.get("data").cloned(),
        }))
    } else {
        return Err(JsonRpcError::InvalidMessage(
            "response must contain result or error".to_owned(),
        ));
    };
    Ok(JsonRpcInbound::Response { id, result })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_message_rejects_non_object_and_missing_response_ids() {
        assert!(matches!(
            take_piped::<()>(None, "stdin"),
            Err(JsonRpcError::InvalidMessage(message)) if message.contains("stdin")
        ));
        assert!(matches!(
            parse_message("[]"),
            Err(JsonRpcError::InvalidMessage(message)) if message.contains("must be an object")
        ));
        assert!(matches!(
            parse_message("{}"),
            Err(JsonRpcError::InvalidMessage(message)) if message.contains("missing id")
        ));
    }

    #[tokio::test]
    async fn runtime_debug_shutdown_and_write_failures_are_observable() {
        let mut debug_config = JsonRpcProcessConfig::shell("sleep 1");
        debug_config
            .environment
            .insert("OPENAI_API_KEY".to_owned(), "secret-value".to_owned());
        let debug_config_text = format!("{debug_config:?}");
        assert!(!debug_config_text.contains("secret-value"));
        assert!(debug_config_text.contains("environment_keys"));

        let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("sleep 1"))
            .await
            .unwrap();
        assert!(format!("{runtime:?}").contains("JsonRpcProcessRuntime"));
        runtime.shutdown().await.unwrap();
        runtime.shutdown().await.unwrap();

        let broken =
            JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("exec 0<&-; sleep 1"))
                .await
                .unwrap();
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert!(matches!(
            broken.request("broken", Value::Null).await,
            Err(JsonRpcError::Io(_)) | Err(JsonRpcError::ProcessExited)
        ));
        broken.shutdown().await.unwrap();

        let (sender, receiver) = oneshot::channel::<JsonRpcResult>();
        drop(sender);
        assert!(matches!(
            resolve_pending_result(Err(receiver.await.unwrap_err())),
            Err(JsonRpcError::ProcessExited)
        ));
        assert!(
            classify_kill_error(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "already exited",
            ))
            .is_none()
        );
        assert!(matches!(
            classify_kill_error(std::io::Error::other("kill failed")),
            Some(JsonRpcError::Io(_))
        ));
    }

    #[tokio::test]
    async fn runtime_maps_closed_pending_requests_and_non_object_server_bodies() {
        let exited = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
            "read line; exec 1>&-; sleep 1",
        ))
        .await
        .unwrap();
        assert!(matches!(
            exited.request("exit", Value::Null).await,
            Err(JsonRpcError::ProcessExited)
        ));
        exited.shutdown().await.unwrap();

        let runtime =
            JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("read line; sleep 1"))
                .await
                .unwrap();
        let id = JsonRpcId::numeric(55);
        runtime
            .inner
            .server_requests
            .lock()
            .await
            .insert(id.key.clone(), id.clone());
        runtime
            .respond_server_request_message(id, Value::Null)
            .await
            .unwrap();
        runtime.shutdown().await.unwrap();

        let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("sleep 1"))
            .await
            .unwrap();
        runtime
            .fail_stdout_read(std::io::Error::other("stdout failed"))
            .await;
        assert!(matches!(
            runtime.next_message().await,
            Err(JsonRpcError::Io(message)) if message == "stdout failed"
        ));
        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn explicit_null_params_remain_distinct_from_omitted_params() {
        let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
            r#"
read explicit_null
if ! printf '%s' "$explicit_null" | grep -q '"id":1' \
  || ! printf '%s' "$explicit_null" | grep -q '"jsonrpc":"2.0"' \
  || ! printf '%s' "$explicit_null" | grep -q '"method":"explicit-null"' \
  || ! printf '%s' "$explicit_null" | grep -q '"params":null'; then
  exit 41
fi
printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":true}'

read omitted
if ! printf '%s' "$omitted" | grep -q '"id":2' \
  || ! printf '%s' "$omitted" | grep -q '"jsonrpc":"2.0"' \
  || ! printf '%s' "$omitted" | grep -q '"method":"omitted"' \
  || printf '%s' "$omitted" | grep -q '"params"'; then
  exit 42
fi
printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":true}'
"#,
        ))
        .await
        .unwrap();

        assert_eq!(
            runtime.request("explicit-null", Value::Null).await.unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            runtime.request_without_params("omitted").await.unwrap(),
            Value::Bool(true)
        );
        runtime.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn runtime_handles_stdout_reader_failures_and_kill_results() {
        assert!(process_kill_result(Ok(())).is_ok());
        assert!(
            process_kill_result(Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "already exited",
            )))
            .is_ok()
        );
        assert!(matches!(
            process_kill_result(Err(std::io::Error::other("kill failed"))),
            Err(JsonRpcError::Io(message)) if message == "kill failed"
        ));

        let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("sleep 1"))
            .await
            .unwrap();
        runtime.read_stdout(BufReader::new(FailingReader)).await;
        assert!(matches!(
            runtime.next_message().await,
            Err(JsonRpcError::Io(message)) if message == "stdout failed"
        ));
        runtime.shutdown().await.unwrap();
    }

    struct FailingReader;

    impl tokio::io::AsyncRead for FailingReader {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _context: &mut std::task::Context<'_>,
            _buffer: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::task::Poll::Ready(Err(std::io::Error::other("stdout failed")))
        }
    }
}
