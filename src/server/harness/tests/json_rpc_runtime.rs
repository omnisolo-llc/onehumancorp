use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use server_harness::middleware::json_rpc::{
    JsonRpcError, JsonRpcErrorObject, JsonRpcId, JsonRpcProcessConfig, JsonRpcProcessRuntime,
};

fn environment_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[tokio::test]
async fn runtime_correlates_out_of_order_responses_and_routes_notifications() {
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"
while IFS= read -r line; do
  method=$(printf '%s' "$line" | sed -n 's/.*"method":"\([^"]*\)".*/\1/p')
  case "$method" in
    first) printf '%s\n' '{"jsonrpc":"2.0","id":2,"result":{"result":"second"}}'; printf '%s\n' '{"jsonrpc":"2.0","method":"progress","params":{"step":1}}' ;;
    second) printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"result":"first"}}' ;;
  esac
done
"#,
    ))
    .await
    .unwrap();
    let mut notifications = runtime.subscribe_notifications();

    let first = runtime.request("first", serde_json::json!({"value": 1}));
    let second = runtime.request("second", serde_json::json!({"value": 2}));
    let (first, second) = tokio::join!(first, second);

    assert_eq!(first.unwrap(), serde_json::json!({"result": "first"}));
    assert_eq!(second.unwrap(), serde_json::json!({"result": "second"}));
    assert_eq!(notifications.recv().await.unwrap().method, "progress");
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn runtime_exposes_server_requests_and_accepts_a_matching_response() {
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"
while IFS= read -r line; do
  method=$(printf '%s' "$line" | sed -n 's/.*"method":"\([^"]*\)".*/\1/p')
  if [ "$method" = "start" ]; then
    printf '%s\n' '{"jsonrpc":"2.0","id":9001,"method":"item/commandExecution/requestApproval","params":{"command":"echo test"}}'
    read response
    printf '%s\n' '{"jsonrpc":"2.0","id":1,"result":{"accepted":true}}'
  fi
done
"#,
    ))
    .await
    .unwrap();
    let mut requests = runtime.subscribe_server_requests();
    let request_runtime = runtime.clone();
    let request_task = tokio::spawn(async move {
        request_runtime
            .request("start", serde_json::Value::Null)
            .await
    });

    let request = tokio::time::timeout(Duration::from_secs(2), requests.recv())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(request.method, "item/commandExecution/requestApproval");
    runtime
        .respond_server_request(request.id, serde_json::json!({"decision": "accept"}))
        .await
        .unwrap();
    assert_eq!(
        request_task.await.unwrap().unwrap(),
        serde_json::json!({"accepted": true})
    );
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn runtime_reports_malformed_frames_and_request_timeouts() {
    let malformed =
        JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("printf '%s\\n' '{not-json}'"))
            .await
            .unwrap();
    assert!(matches!(
        malformed.next_message().await,
        Err(JsonRpcError::Malformed(_))
    ));

    let timeout_runtime =
        JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("read line; sleep 2"))
            .await
            .unwrap();
    assert!(matches!(
        timeout_runtime
            .request_with_timeout("slow", serde_json::Value::Null, Duration::from_millis(20))
            .await,
        Err(JsonRpcError::Timeout)
    ));
    timeout_runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn runtime_can_use_the_headerless_codex_jsonl_wire_format() {
    let runtime = JsonRpcProcessRuntime::spawn(
        JsonRpcProcessConfig::shell(
            r#"
read line
if printf '%s' "$line" | grep -q '"jsonrpc"'; then exit 41; fi
id=$(printf '%s' "$line" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
printf '{"id":%s,"result":{"wire":"headerless"}}\n' "$id"
"#,
        )
        .without_jsonrpc_header(),
    )
    .await
    .unwrap();
    assert_eq!(
        runtime
            .request("initialize", serde_json::Value::Null)
            .await
            .unwrap(),
        serde_json::json!({"wire": "headerless"})
    );
    runtime.shutdown().await.unwrap();
}

#[tokio::test]
async fn ambient_parent_environment_is_not_inherited() {
    let _guard = environment_lock().lock().unwrap();
    unsafe {
        std::env::set_var("UNRELATED_DEPLOYMENT_SECRET", "CANARY-AMBIENT-7KQ9");
    }

    let mut config = JsonRpcProcessConfig::shell(
        r#"
read line
id=$(printf '%s' "$line" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p')
ambient=false
[ -n "${UNRELATED_DEPLOYMENT_SECRET:-}" ] && ambient=true
explicit=false
[ -n "${OPENAI_API_KEY:-}" ] && explicit=true
path=false
[ -n "${PATH:-}" ] && path=true
printf '{"jsonrpc":"2.0","id":%s,"result":{"ambient":%s,"explicit":%s,"path":%s}}\n' "$id" "$ambient" "$explicit" "$path"
"#,
    );
    config.environment = HashMap::from([("OPENAI_API_KEY".to_owned(), "explicit-key".to_owned())]);
    let runtime = JsonRpcProcessRuntime::spawn(config).await.unwrap();
    let result = runtime
        .request("environment-check", serde_json::Value::Null)
        .await
        .unwrap();

    unsafe {
        std::env::remove_var("UNRELATED_DEPLOYMENT_SECRET");
    }
    assert_eq!(
        result,
        serde_json::json!({"ambient": false, "explicit": true, "path": true})
    );
    runtime.shutdown().await.unwrap();
}

#[test]
fn json_rpc_ids_keep_numeric_and_string_values_distinct() {
    let numeric =
        server_harness::middleware::json_rpc::JsonRpcId::from_value(serde_json::json!(1)).unwrap();
    let string =
        server_harness::middleware::json_rpc::JsonRpcId::from_value(serde_json::json!("1"))
            .unwrap();
    assert_ne!(numeric, string);
    assert_eq!(numeric.as_value(), serde_json::json!(1));
    assert_eq!(string.as_value(), serde_json::json!("1"));
}

#[tokio::test]
async fn runtime_rejects_ambiguous_and_malformed_response_frames() {
    let ambiguous = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"read line; printf '%s\n' '{"id":1,"result":{},"error":{"code":-1,"message":"ambiguous"}}'"#,
    ))
    .await
    .unwrap();
    assert!(matches!(
        ambiguous.request("ambiguous", serde_json::Value::Null).await,
        Err(JsonRpcError::InvalidMessage(message)) if message.contains("both result and error")
    ));

    let malformed_error = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"read line; printf '%s\n' '{"id":1,"error":[]}'"#,
    ))
    .await
    .unwrap();
    assert!(matches!(
        malformed_error
            .request("malformed-error", serde_json::Value::Null)
            .await,
        Err(JsonRpcError::InvalidMessage(message)) if message.contains("error must be an object")
    ));

    let missing_result = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"read line; printf '%s\n' '{"id":1}'"#,
    ))
    .await
    .unwrap();
    assert!(matches!(
        missing_result
            .request("missing-result", serde_json::Value::Null)
            .await,
        Err(JsonRpcError::InvalidMessage(message)) if message.contains("result or error")
    ));
}

#[tokio::test]
async fn runtime_reports_failed_requests_and_unknown_server_responses() {
    let failed = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"printf '%s\n' '{"id":1,"error":{"code":-32001,"message":"denied","data":{"reason":"policy"}}}'"#,
    ))
    .await
    .unwrap();
    assert!(matches!(
        failed.request("denied", serde_json::Value::Null).await,
        Err(JsonRpcError::RequestFailed(JsonRpcErrorObject { code: -32001, message, data: Some(_) })) if message == "denied"
    ));

    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell("exit 0"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(matches!(
        runtime.request("closed", serde_json::Value::Null).await,
        Err(JsonRpcError::ProcessExited)
    ));
    assert!(matches!(
        runtime.notify("closed", serde_json::Value::Null).await,
        Err(JsonRpcError::ProcessExited)
    ));
    assert!(matches!(
        runtime
            .respond_server_request(JsonRpcId::from_number(99), serde_json::json!({}))
            .await,
        Err(JsonRpcError::UnknownRequest(_))
    ));
}

#[test]
fn json_rpc_error_display_covers_transport_failures() {
    let id = JsonRpcId::from_number(7);
    let errors = [
        JsonRpcError::Spawn("spawn".to_owned()),
        JsonRpcError::Io("io".to_owned()),
        JsonRpcError::Malformed("malformed".to_owned()),
        JsonRpcError::InvalidMessage("invalid".to_owned()),
        JsonRpcError::Timeout,
        JsonRpcError::ProcessExited,
        JsonRpcError::UnknownRequest(id),
        JsonRpcError::RequestFailed(JsonRpcErrorObject {
            code: -1,
            message: "failed".to_owned(),
            data: None,
        }),
        JsonRpcError::Cancelled,
    ];
    for error in errors {
        assert!(!error.to_string().is_empty());
    }
}

#[tokio::test]
async fn unused_diagnostic_observer_cannot_block_rpc_completion() {
    let runtime = JsonRpcProcessRuntime::spawn(JsonRpcProcessConfig::shell(
        r#"
read request
i=0
while [ "$i" -lt 600 ]; do
  printf '%s\n' '{"method":"progress","params":{}}'
  i=$((i + 1))
done
printf '%s\n' '{"id":1,"result":{"completed":true}}'
read request
"#,
    ))
    .await
    .unwrap();
    let completion = tokio::time::timeout(
        Duration::from_secs(3),
        runtime.request("run", serde_json::Value::Null),
    )
    .await;
    runtime.shutdown().await.unwrap();
    assert_eq!(
        completion
            .expect("unused observer stalled protocol dispatch")
            .unwrap(),
        serde_json::json!({"completed":true})
    );
    assert!(
        matches!(runtime.next_message().await, Err(JsonRpcError::InvalidMessage(message)) if message.contains("lagged"))
    );
}
