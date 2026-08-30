use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener as StdTcpListener};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use uuid::Uuid;

use server_harness::middleware::http_runtime::HttpProcessConfig;
use server_harness::middleware::openhands::{
    CapabilityDowngradePolicy, OPENHANDS_AGENT_SERVER_VERSION, OpenHandsAdapterConfig,
    OpenHandsConversationConfig, OpenHandsError, OpenHandsEventCategory, OpenHandsHttpAdapter,
    OpenHandsLaunchConfig, OpenHandsProviderErrorKind, OpenHandsReasoningSupport,
    OpenHandsStreamItem, classify_litellm_error, decode_native_event,
    decode_native_event_with_config, prepare_conversation_request,
    prepare_conversation_request_with_config, sanitize_native_json,
};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};

const CONVERSATION_ID: &str = "10000000-0000-0000-0000-000000000001";
const CHILD_MODE_ENV: &str = "OMNISOLO_OPENHANDS_TEST_CHILD";

fn selection(effort: ReasoningEffort) -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "portable-openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(effort),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: Some(400_000),
        max_output_tokens: Some(128_000),
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test".to_owned(),
        metadata: [("api_key".to_owned(), json!("metadata-secret-canary"))]
            .into_iter()
            .collect(),
    }
}

fn adapter_config(reasoning: OpenHandsReasoningSupport) -> OpenHandsAdapterConfig {
    OpenHandsAdapterConfig::new(
        "/workspace/project",
        selection(ReasoningEffort::Max),
        "https://llmapi.omnisolo.co/v1",
    )
    .with_reasoning_support(reasoning)
    .with_timeouts(
        Duration::from_secs(2),
        Duration::from_secs(2),
        Duration::from_millis(10),
    )
}

#[test]
fn portable_model_and_selected_base_url_translate_to_openhands_llm_config() {
    let prepared = prepare_conversation_request(
        &adapter_config(OpenHandsReasoningSupport::MaxAccepted),
        None,
    )
    .unwrap();

    assert_eq!(prepared.body["agent"]["kind"], "Agent");
    assert_eq!(
        prepared.body["agent"]["llm"]["model"],
        "openai/gpt-5.6-luna"
    );
    assert_eq!(
        prepared.body["agent"]["llm"]["base_url"],
        "https://llmapi.omnisolo.co/v1"
    );
    assert_eq!(prepared.body["agent"]["llm"]["reasoning_effort"], "max");
    assert_eq!(prepared.body["agent"]["llm"]["api_mode"], "responses");
    assert_eq!(
        prepared.body["confirmation_policy"],
        json!({"kind": "AlwaysConfirm"})
    );
    assert_eq!(
        prepared.body["workspace"],
        json!({"kind": "LocalWorkspace", "working_dir": "/workspace/project"})
    );
    assert!(prepared.downgrades.is_empty());

    let wire = serde_json::to_string(&prepared.body).unwrap();
    assert!(!wire.contains("metadata-secret-canary"));
    assert!(!wire.contains("api_key"));
}

#[test]
fn pinned_1_43_1_always_emits_max_reasoning_without_extension_bypass() {
    let prepared = prepare_conversation_request(
        &adapter_config(OpenHandsReasoningSupport::MaxUnsupported),
        Some(CONVERSATION_ID),
    )
    .unwrap();

    assert_eq!(prepared.body["conversation_id"], CONVERSATION_ID);
    assert_eq!(OPENHANDS_AGENT_SERVER_VERSION, "1.43.1");
    assert_eq!(prepared.body["agent"]["llm"]["reasoning_effort"], "max");
    assert_eq!(prepared.body["agent"]["llm"]["api_mode"], "responses");
    assert!(prepared.downgrades.is_empty());
    prepared
        .authorize(CapabilityDowngradePolicy::Reject)
        .unwrap();
    prepared
        .authorize(CapabilityDowngradePolicy::Accept)
        .unwrap();
}

#[test]
fn native_events_map_real_event_shapes_and_preserve_sanitized_json() {
    let cases = [
        ("ActionEvent", OpenHandsEventCategory::Action),
        ("ObservationEvent", OpenHandsEventCategory::Observation),
        ("MessageEvent", OpenHandsEventCategory::Message),
        ("FutureUpstreamEvent", OpenHandsEventCategory::Native),
    ];

    for (kind, expected_category) in cases {
        let native = json!({
            "id": format!("event-{kind}"),
            "kind": kind,
            "payload": {
                "safe": [1, {"nested": true, "apiKey": "event-secret-canary"}],
                "authorization": {"token": "event-secret-canary"}
            }
        });
        let event = decode_native_event(&native).unwrap();
        assert_eq!(event.category, expected_category);
        assert_eq!(
            event.native_cursor.as_deref(),
            Some(format!("event-{kind}").as_str())
        );
        assert_eq!(event.native["payload"]["safe"][0], 1);
        assert_eq!(event.native["payload"]["safe"][1]["nested"], true);
        assert_eq!(event.native["payload"]["safe"][1]["apiKey"], "[REDACTED]");
        assert_eq!(event.native["payload"]["authorization"], "[REDACTED]");
        assert!(!event.native.to_string().contains("event-secret-canary"));
    }

    let sanitized = sanitize_native_json(&json!({
        "array": [{"refresh_token": "secret"}, "safe"],
        "cookieValue": "secret",
        "number": 7
    }));
    assert_eq!(sanitized["array"][0]["refresh_token"], "[REDACTED]");
    assert_eq!(sanitized["array"][1], "safe");
    assert_eq!(sanitized["cookieValue"], "[REDACTED]");
    assert_eq!(sanitized["number"], 7);
}

#[test]
fn configured_secret_is_redacted_from_arbitrary_native_and_final_text() {
    let config = adapter_config(OpenHandsReasoningSupport::MaxAccepted)
        .with_provider_api_key("exact-configured-secret");
    assert!(!format!("{config:?}").contains("exact-configured-secret"));

    let event = decode_native_event_with_config(
        &json!({
            "id": "secret-output",
            "kind": "MessageEvent",
            "source": "agent",
            "stdout": "prefix exact-configured-secret suffix",
            "content": {"text": "exact-configured-secret"},
            "detail": ["safe", "before exact-configured-secret after"],
            "llm_message": {
                "content": [{
                    "type": "text",
                    "text": "final exact-configured-secret answer"
                }]
            }
        }),
        &config,
    )
    .unwrap();

    assert_eq!(event.native["stdout"], "prefix [REDACTED] suffix");
    assert_eq!(event.native["content"]["text"], "[REDACTED]");
    assert_eq!(event.native["detail"][0], "safe");
    assert_eq!(event.native["detail"][1], "before [REDACTED] after");
    assert_eq!(event.final_text.as_deref(), Some("final [REDACTED] answer"));
    assert!(!format!("{event:?}").contains("exact-configured-secret"));
}

#[test]
fn litellm_failures_have_stable_provider_classification_and_sanitized_native_detail() {
    let cases = [
        (
            401,
            "AuthenticationError",
            OpenHandsProviderErrorKind::Authentication,
            false,
        ),
        (
            429,
            "RateLimitError",
            OpenHandsProviderErrorKind::RateLimited,
            true,
        ),
        (
            400,
            "context_length_exceeded",
            OpenHandsProviderErrorKind::ContextLengthExceeded,
            false,
        ),
        (
            404,
            "model_not_found",
            OpenHandsProviderErrorKind::ModelUnavailable,
            false,
        ),
        (
            504,
            "APITimeoutError",
            OpenHandsProviderErrorKind::Timeout,
            true,
        ),
        (
            503,
            "ServiceUnavailableError",
            OpenHandsProviderErrorKind::Unavailable,
            true,
        ),
        (
            400,
            "BadRequestError",
            OpenHandsProviderErrorKind::InvalidRequest,
            false,
        ),
    ];

    for (status, marker, kind, retryable) in cases {
        let error = classify_litellm_error(
            status,
            &json!({
                "detail": {
                    "type": marker,
                    "message": marker,
                    "api_key": "provider-secret-canary"
                }
            }),
        );
        assert_eq!(error.kind, kind);
        assert_eq!(error.retryable, retryable);
        assert_eq!(error.status, Some(status));
        assert!(!error.native.to_string().contains("provider-secret-canary"));
        assert!(!error.to_string().contains(marker));
    }
}

#[test]
fn all_pinned_conversation_error_classifications_remain_typed_and_retryable() {
    let cases = [
        ("auth", OpenHandsProviderErrorKind::Authentication, false),
        ("quota", OpenHandsProviderErrorKind::Quota, false),
        ("rate_limit", OpenHandsProviderErrorKind::RateLimited, true),
        ("config", OpenHandsProviderErrorKind::Configuration, false),
        ("transient", OpenHandsProviderErrorKind::Transient, true),
        (
            "agent_action",
            OpenHandsProviderErrorKind::AgentAction,
            true,
        ),
        ("internal", OpenHandsProviderErrorKind::Internal, false),
        ("unknown", OpenHandsProviderErrorKind::Unknown, false),
    ];
    for (native_kind, expected, retryable) in cases {
        let event = decode_native_event(&json!({
            "kind": "ConversationErrorEvent",
            "code": "PinnedError",
            "detail": "safe detail",
            "classification": {
                "kind": native_kind,
                "retryable": retryable,
                "user_action": if retryable { "retry" } else { "none" },
                "error_id": "stable-id"
            }
        }))
        .unwrap();
        let provider = event.provider_error.unwrap();
        assert_eq!(provider.kind, expected);
        assert_eq!(provider.retryable, retryable);
        assert_eq!(provider.status, None);
        assert_eq!(provider.native["classification"]["kind"], native_kind);
        assert_eq!(provider.native["classification"]["error_id"], "stable-id");
    }
}

#[test]
fn model_base_url_and_dialect_validation_is_strict() {
    for model in [
        " gpt-5.6-luna",
        "gpt-5.6-luna ",
        "gpt 5",
        "openai/",
        "openai/gpt/extra",
        "../gpt",
        "gpt?variant",
        "gpt#fragment",
        "gpt\\alias",
        "gpt\nmodel",
    ] {
        let mut config = OpenHandsConversationConfig::new(
            "/workspace/project",
            selection(ReasoningEffort::Max),
            "https://llmapi.omnisolo.co/v1",
        );
        config.resolved_model.model_id = model.to_owned();
        assert!(
            matches!(
                prepare_conversation_request_with_config(&config, None),
                Err(OpenHandsError::InvalidRequest(_))
            ),
            "model should be rejected: {model:?}"
        );
    }

    for base_url in [
        " https://llm.example/v1",
        "https://llm.example/v1 ",
        "ftp://llm.example/v1",
        "http:///v1",
        "https://user:pass@llm.example/v1",
        "https://llm.example:0/v1",
        "https://llm.example:70000/v1",
        "https://llm.example:notaport/v1",
        "https://bad_host.example/v1",
        "https://llm.example/../secret",
        "https://llm.example/v1?key=value",
        "https://llm.example/v1#fragment",
        "https://llm.example\\v1",
    ] {
        let config = OpenHandsConversationConfig::new(
            "/workspace/project",
            selection(ReasoningEffort::Max),
            base_url,
        );
        assert!(
            matches!(
                prepare_conversation_request_with_config(&config, None),
                Err(OpenHandsError::InvalidRequest(_))
            ),
            "base URL should be rejected: {base_url:?}"
        );
    }

    let mut unsupported = OpenHandsConversationConfig::new(
        "/workspace/project",
        selection(ReasoningEffort::Max),
        "https://llm.example/v1",
    );
    unsupported.resolved_model.api_dialect = ModelApiDialect::AnthropicMessages;
    assert!(matches!(
        prepare_conversation_request_with_config(&unsupported, None),
        Err(OpenHandsError::InvalidRequest(_))
    ));
}

#[derive(Clone, Copy)]
enum MockMode {
    Normal,
    HangConversation,
    PagedResumedHistory,
    ProviderError,
    ProviderNotFound,
    ProviderInvalidRequest,
    NullMetricsWithUsageStats,
    RouterNotFound,
    RouterValidation,
    TerminalConversationError,
    WaitingForConfirmation,
    IdleBeforeRunningFinishAction,
    RunningUntilCancelled,
    SecretBoundary,
    WrongVersion,
}

#[derive(Clone, Debug)]
struct RecordedRequest {
    method: String,
    path: String,
    body: Value,
}

struct MockServer {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
    task: tokio::task::JoinHandle<()>,
}

struct FakeResponsesProvider {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
    task: tokio::task::JoinHandle<()>,
}

impl FakeResponsesProvider {
    async fn start() -> Self {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&requests);
        let task = tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    return;
                };
                let captured = Arc::clone(&captured);
                tokio::spawn(async move {
                    let Some((method, path, body)) = read_request(&mut stream).await else {
                        return;
                    };
                    captured
                        .lock()
                        .unwrap()
                        .push(RecordedRequest { method, path, body });
                    write_response(
                        &mut stream,
                        200,
                        &json!({
                            "id": "resp_openhands_e2e",
                            "object": "response",
                            "created_at": 1,
                            "status": "completed",
                            "model": "gpt-5.6-luna",
                            "output": [{
                                "id": "fc_openhands_e2e",
                                "type": "function_call",
                                "status": "completed",
                                "name": "finish",
                                "call_id": "call_openhands_e2e",
                                "arguments": "{\"message\":\"real server provider prompt completed\"}"
                            }],
                            "parallel_tool_calls": false,
                            "usage": {
                                "input_tokens": 10,
                                "output_tokens": 5,
                                "total_tokens": 15
                            }
                        }),
                    )
                    .await;
                });
            }
        });
        Self {
            address,
            requests,
            task,
        }
    }

    fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().unwrap().clone()
    }
}

impl Drop for FakeResponsesProvider {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl MockServer {
    async fn start(mode: MockMode) -> Self {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&requests);
        let task = tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    return;
                };
                let captured = Arc::clone(&captured);
                tokio::spawn(async move {
                    serve_mock_request(stream, mode, captured).await;
                });
            }
        });
        Self {
            address,
            requests,
            task,
        }
    }

    fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().unwrap().clone()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

async fn serve_mock_request(
    mut stream: TcpStream,
    mode: MockMode,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
) {
    let Some((method, path, body)) = read_request(&mut stream).await else {
        return;
    };
    requests.lock().unwrap().push(RecordedRequest {
        method: method.clone(),
        path: path.clone(),
        body: body.clone(),
    });
    let recorded = requests.lock().unwrap().clone();
    let prompt_index = recorded.iter().rposition(|request| {
        request.method == "POST"
            && request.path == format!("/api/conversations/{CONVERSATION_ID}/events")
    });
    let prompt_seen = prompt_index.is_some();
    let fresh_searches_after_prompt = prompt_index
        .map(|index| {
            recorded[index + 1..]
                .iter()
                .filter(|request| {
                    request.method == "GET"
                        && request.path.contains("/events/search?")
                        && !request.path.contains("&page_id=")
                })
                .count()
        })
        .unwrap_or(0);
    let conversation_polls_after_prompt = prompt_index
        .map(|index| {
            recorded[index + 1..]
                .iter()
                .filter(|request| {
                    request.method == "GET"
                        && request.path == format!("/api/conversations/{CONVERSATION_ID}")
                })
                .count()
        })
        .unwrap_or(0);

    if matches!(mode, MockMode::HangConversation)
        && method == "POST"
        && path == "/api/conversations"
        && body["conversation_id"] == "hang"
    {
        tokio::time::sleep(Duration::from_secs(5)).await;
        return;
    }

    let (status, response) = match (method.as_str(), path.as_str()) {
        ("GET", "/ready") => (200, json!({"status": "ready"})),
        ("GET", "/server_info") => (
            200,
            json!({
                "title": "OpenHands Agent Server",
                "version": if matches!(mode, MockMode::WrongVersion) {
                    "1.42.1"
                } else {
                    OPENHANDS_AGENT_SERVER_VERSION
                },
                "uptime": 1,
                "idle_time": 0
            }),
        ),
        ("POST", "/api/conversations") if matches!(mode, MockMode::ProviderError) => (
            429,
            json!({
                "detail": {
                    "type": "RateLimitError",
                    "message": "litellm.RateLimitError",
                    "api_key": "http-provider-secret-canary"
                }
            }),
        ),
        ("POST", "/api/conversations") if matches!(mode, MockMode::ProviderNotFound) => (
            404,
            json!({
                "detail": {
                    "type": "litellm.NotFoundError",
                    "code": "model_not_found",
                    "message": "provider model not found"
                }
            }),
        ),
        ("POST", "/api/conversations") if matches!(mode, MockMode::ProviderInvalidRequest) => (
            422,
            json!({
                "detail": {
                    "type": "litellm.BadRequestError",
                    "code": "invalid_request_error",
                    "message": "provider rejected the request"
                }
            }),
        ),
        ("POST", "/api/conversations") if matches!(mode, MockMode::RouterNotFound) => (
            404,
            json!({"detail": {"message": "route not found", "api_key": "router-secret-canary"}}),
        ),
        ("POST", "/api/conversations") if matches!(mode, MockMode::RouterValidation) => (
            422,
            json!({"detail": [{"loc": ["body", "agent"], "msg": "invalid request"}]}),
        ),
        ("POST", "/api/conversations") => {
            let id = body
                .get("conversation_id")
                .and_then(Value::as_str)
                .unwrap_or(CONVERSATION_ID);
            (
                if id == CONVERSATION_ID { 201 } else { 200 },
                json!({
                    "id": id,
                    "execution_status": "idle",
                    "agent": {"llm": {"model": "openai/gpt-5.6-luna"}},
                    "detail": if matches!(mode, MockMode::SecretBoundary) {
                        "created with exact-configured-secret"
                    } else {
                        "safe"
                    }
                }),
            )
        }
        ("GET", path)
            if path == format!("/api/conversations/{CONVERSATION_ID}")
                && matches!(mode, MockMode::PagedResumedHistory) =>
        {
            (
                200,
                json!({
                    "id": CONVERSATION_ID,
                    "execution_status": if fresh_searches_after_prompt < 2 {
                        "running"
                    } else {
                        "finished"
                    }
                }),
            )
        }
        ("GET", path)
            if path == format!("/api/conversations/{CONVERSATION_ID}")
                && matches!(mode, MockMode::WaitingForConfirmation) =>
        {
            (
                200,
                json!({
                    "id": CONVERSATION_ID,
                    "execution_status": "waiting_for_confirmation"
                }),
            )
        }
        ("GET", path)
            if path == format!("/api/conversations/{CONVERSATION_ID}")
                && matches!(mode, MockMode::NullMetricsWithUsageStats) =>
        {
            (
                200,
                json!({
                    "id": CONVERSATION_ID,
                    "execution_status": "finished",
                    "metrics": null,
                    "stats": {
                        "usage_to_metrics": {
                            "agent": {
                                "accumulated_token_usage": {
                                    "prompt_tokens": 23,
                                    "completion_tokens": 9
                                }
                            }
                        }
                    }
                }),
            )
        }
        ("GET", path)
            if path == format!("/api/conversations/{CONVERSATION_ID}")
                && matches!(mode, MockMode::IdleBeforeRunningFinishAction) =>
        {
            let execution_status = match conversation_polls_after_prompt {
                0 | 1 => "idle",
                2 => "running",
                _ => "finished",
            };
            (
                200,
                json!({"id": CONVERSATION_ID, "execution_status": execution_status}),
            )
        }
        ("GET", path)
            if path == format!("/api/conversations/{CONVERSATION_ID}")
                && matches!(mode, MockMode::RunningUntilCancelled) =>
        {
            (
                200,
                json!({"id": CONVERSATION_ID, "execution_status": "running"}),
            )
        }
        ("GET", path)
            if path == format!("/api/conversations/{CONVERSATION_ID}")
                && matches!(mode, MockMode::TerminalConversationError) =>
        {
            (
                200,
                json!({
                    "id": CONVERSATION_ID,
                    "execution_status": "error",
                    "last_error": {"message": "generic state error must not win"}
                }),
            )
        }
        ("GET", path) if path == format!("/api/conversations/{CONVERSATION_ID}") => (
            200,
            json!({
                "id": CONVERSATION_ID,
                "execution_status": "finished",
                "metrics": {
                    "accumulated_token_usage": {
                        "prompt_tokens": 11,
                        "completion_tokens": 7
                    }
                }
            }),
        ),
        ("DELETE", path) if path == format!("/api/conversations/{CONVERSATION_ID}") => {
            (200, json!({"success": true}))
        }
        ("POST", path) if path == format!("/api/conversations/{CONVERSATION_ID}/events") => {
            (200, json!({"success": true}))
        }
        ("GET", path)
            if path.starts_with(&format!(
                "/api/conversations/{CONVERSATION_ID}/events/search"
            )) && matches!(mode, MockMode::PagedResumedHistory) =>
        {
            let page_id = path.split("&page_id=").nth(1);
            let response = if !prompt_seen {
                match page_id {
                    None => json!({
                        "items": [{"id": "history-1", "kind": "MessageEvent", "source": "user", "message": {"text": "old prompt"}}],
                        "next_page_id": "baseline-next"
                    }),
                    Some("baseline-next") => json!({
                        "items": [{"id": "history-2", "kind": "ObservationEvent", "observation": {"stdout": "old output"}}],
                        "next_page_id": null
                    }),
                    _ => json!({"items": [], "next_page_id": null}),
                }
            } else if fresh_searches_after_prompt == 1 {
                match page_id {
                    None => json!({
                        "items": [
                            {"id": "history-1", "kind": "MessageEvent", "source": "user", "message": {"text": "old prompt"}},
                            {"id": "turn-action", "kind": "ActionEvent", "action": {"command": "pwd"}}
                        ],
                        "next_page_id": "poll-one-next"
                    }),
                    Some("poll-one-next") => json!({
                        "items": [
                            {"id": "history-2", "kind": "ObservationEvent", "observation": {"stdout": "old output"}},
                            {"id": "turn-observation", "kind": "ObservationEvent", "observation": {"stdout": "/workspace/project"}}
                        ],
                        "next_page_id": null
                    }),
                    _ => json!({"items": [], "next_page_id": null}),
                }
            } else {
                match page_id {
                    None => json!({
                        "items": [
                            {"id": "history-1", "kind": "MessageEvent", "source": "user", "message": {"text": "old prompt"}},
                            {"id": "turn-action", "kind": "ActionEvent", "action": {"command": "pwd"}}
                        ],
                        "next_page_id": "poll-two-next"
                    }),
                    Some("poll-two-next") => json!({
                        "items": [
                            {"id": "history-2", "kind": "ObservationEvent", "observation": {"stdout": "old output"}},
                            {"id": "turn-observation", "kind": "ObservationEvent", "observation": {"stdout": "/workspace/project"}},
                            {"id": "turn-message", "kind": "MessageEvent", "source": "agent", "message": {"text": "fresh result"}}
                        ],
                        "next_page_id": null
                    }),
                    _ => json!({"items": [], "next_page_id": null}),
                }
            };
            (200, response)
        }
        ("GET", path)
            if path.starts_with(&format!(
                "/api/conversations/{CONVERSATION_ID}/events/search"
            )) && matches!(mode, MockMode::IdleBeforeRunningFinishAction) =>
        {
            (
                200,
                json!({
                    "items": if prompt_seen { json!([{
                        "id": "finish-action",
                        "kind": "ActionEvent",
                        "source": "agent",
                        "tool_name": "finish",
                        "action": {"kind": "FinishAction", "message": "event fallback must not win"}
                    }]) } else { json!([]) },
                    "next_page_id": null
                }),
            )
        }
        ("GET", path)
            if path.starts_with(&format!(
                "/api/conversations/{CONVERSATION_ID}/events/search"
            )) && matches!(mode, MockMode::WaitingForConfirmation) =>
        {
            (
                200,
                json!({
                    "items": if prompt_seen { json!([{
                        "id": "pending-action",
                        "kind": "ActionEvent",
                        "source": "agent",
                        "action": {"kind": "ExecuteBashAction", "command": "git status"}
                    }]) } else { json!([]) },
                    "next_page_id": null
                }),
            )
        }
        ("GET", path)
            if path.starts_with(&format!(
                "/api/conversations/{CONVERSATION_ID}/events/search"
            )) && matches!(mode, MockMode::TerminalConversationError) =>
        {
            (
                200,
                json!({
                    "items": if prompt_seen { json!([{
                        "id": "conversation-error",
                        "kind": "ConversationErrorEvent",
                        "source": "environment",
                        "code": "LLMRateLimitError",
                        "detail": "provider throttled the request",
                        "classification": {
                            "kind": "rate_limit",
                            "retryable": true,
                            "user_action": "retry"
                        },
                        "diagnostic": {"api_key": "background-secret-canary"}
                    }]) } else { json!([]) },
                    "next_page_id": null
                }),
            )
        }
        ("GET", path)
            if path.starts_with(&format!(
                "/api/conversations/{CONVERSATION_ID}/events/search"
            )) =>
        {
            let items = if !prompt_seen {
                json!([])
            } else if matches!(mode, MockMode::SecretBoundary) {
                json!([
                    {
                        "id": "secret-observation",
                        "kind": "ObservationEvent",
                        "observation": {
                            "stdout": "prefix exact-configured-secret suffix",
                            "content": {"text": "exact-configured-secret"},
                            "detail": ["safe", "exact-configured-secret"]
                        }
                    },
                    {
                        "id": "secret-final",
                        "kind": "MessageEvent",
                        "source": "agent",
                        "llm_message": {
                            "content": [{"type": "text", "text": "final exact-configured-secret answer"}]
                        }
                    }
                ])
            } else {
                json!([
                    {
                        "id": "event-1",
                        "kind": "ActionEvent",
                        "action": {"command": "pwd", "api_key": "stream-secret-canary"}
                    },
                    {
                        "id": "event-2",
                        "kind": "ObservationEvent",
                        "observation": {"stdout": "/workspace/project"}
                    },
                    {
                        "id": "event-4",
                        "kind": "MessageEvent",
                        "source": "agent",
                        "llm_message": {
                            "content": [{"type": "text", "text": "done"}]
                        }
                    }
                ])
            };
            (
                200,
                json!({
                    "items": items,
                    "next_page_id": null
                }),
            )
        }
        ("POST", path)
            if path
                == format!(
                    "/api/conversations/{CONVERSATION_ID}/events/respond_to_confirmation"
                ) =>
        {
            (200, json!({"success": true}))
        }
        ("POST", path) if path == format!("/api/conversations/{CONVERSATION_ID}/interrupt") => {
            (200, json!({"success": true}))
        }
        ("GET", path)
            if path == format!("/api/conversations/{CONVERSATION_ID}/agent_final_response") =>
        {
            (
                200,
                json!({
                    "response": if matches!(mode, MockMode::IdleBeforeRunningFinishAction) {
                        "authoritative exact-configured-secret response"
                    } else {
                        "authoritative finish response"
                    }
                }),
            )
        }
        _ => (404, json!({"detail": "not found"})),
    };
    write_response(&mut stream, status, &response).await;
}

async fn read_request(stream: &mut TcpStream) -> Option<(String, String, Value)> {
    let mut bytes = Vec::new();
    let header_end;
    loop {
        let mut chunk = [0_u8; 1024];
        let count = stream.read(&mut chunk).await.ok()?;
        if count == 0 {
            return None;
        }
        bytes.extend_from_slice(&chunk[..count]);
        if let Some(index) = bytes.windows(4).position(|part| part == b"\r\n\r\n") {
            header_end = index + 4;
            break;
        }
    }
    let headers = String::from_utf8_lossy(&bytes[..header_end]);
    let mut lines = headers.lines();
    let mut request_line = lines.next()?.split_whitespace();
    let method = request_line.next()?.to_owned();
    let path = request_line.next()?.to_owned();
    let content_length = lines
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .unwrap_or(0);
    while bytes.len() < header_end + content_length {
        let mut chunk = [0_u8; 1024];
        let count = stream.read(&mut chunk).await.ok()?;
        if count == 0 {
            break;
        }
        bytes.extend_from_slice(&chunk[..count]);
    }
    let body = if content_length == 0 {
        Value::Null
    } else {
        serde_json::from_slice(&bytes[header_end..header_end + content_length]).ok()?
    };
    Some((method, path, body))
}

async fn write_response(stream: &mut TcpStream, status: u16, body: &Value) {
    let body = serde_json::to_vec(body).unwrap();
    let reason = match status {
        200 => "OK",
        201 => "Created",
        404 => "Not Found",
        429 => "Too Many Requests",
        _ => "Error",
    };
    let headers = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(headers.as_bytes()).await.unwrap();
    stream.write_all(&body).await.unwrap();
}

#[tokio::test]
async fn connect_waits_for_readiness_and_create_resume_delete_use_official_routes() {
    let server = MockServer::start(MockMode::Normal).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();

    assert!(adapter.address().ip().is_loopback());
    assert!(adapter.is_ready());
    let created = adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    assert_eq!(created.id, CONVERSATION_ID);
    assert_eq!(created.status, "idle");

    let resumed = adapter
        .resume_conversation(CONVERSATION_ID, CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    assert_eq!(resumed.id, CONVERSATION_ID);
    adapter.delete_conversation(CONVERSATION_ID).await.unwrap();

    let requests = server.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(requests[0].path, "/ready");
    assert_eq!(requests[1].path, "/server_info");
    assert_eq!(requests[2].path, "/api/conversations");
    assert!(requests[2].body.get("conversation_id").is_none());
    assert_eq!(requests[3].body["conversation_id"], CONVERSATION_ID);
    assert_eq!(requests[4].method, "DELETE");
}

#[tokio::test]
async fn one_server_accepts_distinct_per_conversation_model_base_and_dialect() {
    let server = MockServer::start(MockMode::Normal).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    let responses = OpenHandsConversationConfig::new(
        "/workspace/one",
        selection(ReasoningEffort::Max),
        "https://responses.example.test/v1",
    );
    let mut chat_selection = selection(ReasoningEffort::High);
    chat_selection.model_id = "gpt-4.1-mini".to_owned();
    chat_selection.api_dialect = ModelApiDialect::OpenAiChatCompletions;
    let chat = OpenHandsConversationConfig::new(
        "/workspace/two",
        chat_selection,
        "http://127.0.0.1:4100/openai/v1",
    );

    adapter
        .create_conversation_with_config(&responses, CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    adapter
        .resume_conversation_with_config(CONVERSATION_ID, &chat, CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();

    let creates = server
        .requests()
        .into_iter()
        .filter(|request| request.method == "POST" && request.path == "/api/conversations")
        .collect::<Vec<_>>();
    assert_eq!(creates.len(), 2);
    assert_eq!(
        creates[0].body["agent"]["llm"]["model"],
        "openai/gpt-5.6-luna"
    );
    assert_eq!(
        creates[0].body["agent"]["llm"]["base_url"],
        "https://responses.example.test/v1"
    );
    assert_eq!(creates[0].body["agent"]["llm"]["api_mode"], "responses");
    assert_eq!(
        creates[0].body["workspace"]["working_dir"],
        "/workspace/one"
    );
    assert_eq!(
        creates[1].body["agent"]["llm"]["model"],
        "openai/gpt-4.1-mini"
    );
    assert_eq!(
        creates[1].body["agent"]["llm"]["base_url"],
        "http://127.0.0.1:4100/openai/v1"
    );
    assert_eq!(creates[1].body["agent"]["llm"]["api_mode"], "chat");
    assert_eq!(
        creates[1].body["workspace"]["working_dir"],
        "/workspace/two"
    );
}

#[test]
fn pinned_agent_server_launch_builder_uses_worker_executable_and_loopback_placeholders() {
    let launch =
        OpenHandsLaunchConfig::agent_server(adapter_config(OpenHandsReasoningSupport::MaxAccepted));
    let process = launch.process_config();

    assert_eq!(OPENHANDS_AGENT_SERVER_VERSION, "1.43.1");
    assert_eq!(process.executable, "openhands-agent-server");
    assert_eq!(process.args, ["--host", "{host}", "--port", "{port}"]);
    assert!(process.preferred_address.ip().is_loopback());
}

#[tokio::test]
async fn readiness_rejects_an_agent_server_with_the_wrong_version() {
    let server = MockServer::start(MockMode::WrongVersion).await;
    let error = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap_err();

    assert!(matches!(
        error,
        OpenHandsError::UnsupportedServerVersion {
            expected: "1.43.1",
            actual
        } if actual == "1.42.1"
    ));
}

#[tokio::test]
async fn pinned_server_image_contract_and_installed_binary_are_verified() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dockerfile = manifest_dir.join("../../../deploy/docker/Dockerfile.harness-worker");
    let package_contract = fs::read_to_string(dockerfile).unwrap();
    assert!(package_contract.contains(
        "ghcr.io/openhands/agent-server:1.43.1-python@sha256:6f5c614cdab68150d6365e5be1051de4a005375dc46249e17ae2349ec93a9cc0 AS openhands"
    ));

    if std::process::Command::new("/bin/sh")
        .args(["-c", "command -v openhands-agent-server >/dev/null 2>&1"])
        .status()
        .unwrap()
        .success()
    {
        let mut config = adapter_config(OpenHandsReasoningSupport::MaxUnsupported).with_timeouts(
            Duration::from_secs(30),
            Duration::from_secs(5),
            Duration::from_millis(50),
        );
        config.workspace = std::env::current_dir().unwrap();
        let mut adapter = OpenHandsHttpAdapter::launch(OpenHandsLaunchConfig::agent_server(config))
            .await
            .unwrap();
        let conversation = adapter
            .create_conversation(CapabilityDowngradePolicy::Reject)
            .await
            .unwrap();
        assert!(conversation.downgrades.is_empty());
        adapter.shutdown().await.unwrap();
    }
}

#[tokio::test]
async fn installed_pinned_server_executes_a_real_provider_prompt_when_available() {
    if !std::process::Command::new("/bin/sh")
        .args(["-c", "command -v openhands-agent-server >/dev/null 2>&1"])
        .status()
        .unwrap()
        .success()
    {
        eprintln!("SKIP: openhands-agent-server is not installed");
        return;
    }

    let provider = FakeResponsesProvider::start().await;
    let mut config = OpenHandsAdapterConfig::new(
        std::env::current_dir().unwrap(),
        selection(ReasoningEffort::Max),
        format!("http://{}/v1", provider.address),
    )
    .with_timeouts(
        Duration::from_secs(60),
        Duration::from_secs(60),
        Duration::from_millis(50),
    );
    config.reasoning_support = OpenHandsReasoningSupport::MaxUnsupported;
    let mut adapter = OpenHandsHttpAdapter::launch(
        OpenHandsLaunchConfig::agent_server(config).with_api_key("loopback-e2e-key"),
    )
    .await
    .unwrap();
    let conversation = adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    let mut stream = adapter
        .prompt_stream(&conversation.id, "Finish with the provider-supplied result")
        .await
        .unwrap();
    let mut completion = None;
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            OpenHandsStreamItem::Event(_) => {}
            OpenHandsStreamItem::ApprovalRequired { .. } => {
                adapter
                    .respond_to_approval(&conversation.id, true, "E2E finish approval")
                    .await
                    .unwrap();
            }
            OpenHandsStreamItem::Completed { final_text, .. } => completion = final_text,
        }
    }
    assert_eq!(
        completion.as_deref(),
        Some("real server provider prompt completed")
    );
    let provider_requests = provider.requests();
    assert!(!provider_requests.is_empty());
    assert!(
        provider_requests
            .iter()
            .any(|request| request.path.ends_with("/responses"))
    );
    assert!(provider_requests.iter().any(|request| {
        request.body["model"] == "openai/gpt-5.6-luna" || request.body["model"] == "gpt-5.6-luna"
    }));
    adapter.shutdown().await.unwrap();
}

#[tokio::test]
async fn untrusted_max_downgrade_flag_cannot_change_conversation_creation() {
    let server = MockServer::start(MockMode::Normal).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxUnsupported),
    )
    .await
    .unwrap();

    let conversation = adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    assert!(conversation.downgrades.is_empty());
    let create = server
        .requests()
        .into_iter()
        .find(|request| request.method == "POST" && request.path == "/api/conversations")
        .unwrap();
    assert_eq!(create.body["agent"]["llm"]["reasoning_effort"], "max");
    assert_eq!(create.body["agent"]["llm"]["api_mode"], "responses");
}

#[tokio::test]
async fn prompt_returns_event_stream_with_actions_observations_approval_final_text_and_usage() {
    let server = MockServer::start(MockMode::Normal).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();

    let mut stream = adapter
        .prompt_stream(CONVERSATION_ID, "Inspect the repository")
        .await
        .unwrap();
    let mut categories = Vec::new();
    let mut completed = None;
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            OpenHandsStreamItem::Event(event) => {
                assert!(!event.native.to_string().contains("stream-secret-canary"));
                categories.push(event.category);
            }
            OpenHandsStreamItem::ApprovalRequired { action } => {
                assert_eq!(action.category, OpenHandsEventCategory::Action);
                categories.push(OpenHandsEventCategory::Approval);
            }
            OpenHandsStreamItem::Completed { final_text, usage } => {
                completed = Some((final_text, usage));
            }
        }
    }

    assert_eq!(
        categories,
        vec![
            OpenHandsEventCategory::Action,
            OpenHandsEventCategory::Observation,
            OpenHandsEventCategory::Message,
        ]
    );
    let (final_text, usage) = completed.unwrap();
    assert_eq!(final_text.as_deref(), Some("done"));
    assert_eq!(
        usage.unwrap()["accumulated_token_usage"]["prompt_tokens"],
        11
    );

    let prompt = server
        .requests()
        .into_iter()
        .find(|request| request.path.ends_with("/events"))
        .unwrap();
    assert_eq!(prompt.body["role"], "user");
    assert_eq!(prompt.body["content"][0]["text"], "Inspect the repository");
    assert_eq!(prompt.body["run"], true);
}

#[tokio::test]
async fn configured_secret_is_redacted_across_http_conversation_events_and_completion() {
    let server = MockServer::start(MockMode::SecretBoundary).await;
    let config = adapter_config(OpenHandsReasoningSupport::MaxAccepted)
        .with_provider_api_key("exact-configured-secret");
    let mut adapter = OpenHandsHttpAdapter::connect(server.address, config)
        .await
        .unwrap();
    let conversation = adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    assert_eq!(conversation.native["detail"], "created with [REDACTED]");

    let mut stream = adapter
        .prompt_stream(CONVERSATION_ID, "exercise redaction")
        .await
        .unwrap();
    let mut final_text = None;
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            OpenHandsStreamItem::Event(event) => {
                assert!(!event.native.to_string().contains("exact-configured-secret"));
                if event.kind == "ObservationEvent" {
                    assert_eq!(
                        event.native["observation"]["stdout"],
                        "prefix [REDACTED] suffix"
                    );
                }
            }
            OpenHandsStreamItem::ApprovalRequired { .. } => panic!("unexpected approval"),
            OpenHandsStreamItem::Completed {
                final_text: text, ..
            } => final_text = text,
        }
    }
    assert_eq!(final_text.as_deref(), Some("final [REDACTED] answer"));
}

#[tokio::test]
async fn resumed_history_and_prior_pages_are_not_reemitted_across_poll_searches() {
    let server = MockServer::start(MockMode::PagedResumedHistory).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    adapter
        .resume_conversation(CONVERSATION_ID, CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();

    let mut stream = adapter
        .prompt_stream(CONVERSATION_ID, "new prompt")
        .await
        .unwrap();
    let mut event_ids = Vec::new();
    let mut completion = None;
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            OpenHandsStreamItem::Event(event) => {
                event_ids.push(event.native_cursor.unwrap());
            }
            OpenHandsStreamItem::ApprovalRequired { .. } => {
                panic!("unexpected approval requirement")
            }
            OpenHandsStreamItem::Completed { final_text, .. } => completion = final_text,
        }
    }

    assert_eq!(
        event_ids,
        ["turn-action", "turn-observation", "turn-message"]
    );
    assert_eq!(completion.as_deref(), Some("fresh result"));

    let searches = server
        .requests()
        .into_iter()
        .filter(|request| request.path.contains("/events/search?"))
        .map(|request| request.path)
        .collect::<Vec<_>>();
    assert!(
        searches
            .iter()
            .any(|path| path.contains("page_id=baseline-next"))
    );
    assert!(
        searches
            .iter()
            .any(|path| path.contains("page_id=poll-one-next"))
    );
    assert!(
        searches
            .iter()
            .any(|path| path.contains("page_id=poll-two-next"))
    );
}

#[tokio::test]
async fn stale_idle_does_not_finish_and_finish_action_uses_authoritative_final_response() {
    let server = MockServer::start(MockMode::IdleBeforeRunningFinishAction).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted)
            .with_provider_api_key("exact-configured-secret"),
    )
    .await
    .unwrap();
    adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();

    let mut stream = adapter
        .prompt_stream(CONVERSATION_ID, "finish through the tool")
        .await
        .unwrap();
    let mut final_text = None;
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            OpenHandsStreamItem::Event(_) => {}
            OpenHandsStreamItem::ApprovalRequired { .. } => panic!("unexpected approval"),
            OpenHandsStreamItem::Completed {
                final_text: text, ..
            } => final_text = text,
        }
    }

    assert_eq!(
        final_text.as_deref(),
        Some("authoritative [REDACTED] response")
    );
    let requests = server.requests();
    assert!(
        requests
            .iter()
            .filter(|request| request.path == format!("/api/conversations/{CONVERSATION_ID}"))
            .count()
            >= 3
    );
    assert!(requests.iter().any(|request| {
        request.path == format!("/api/conversations/{CONVERSATION_ID}/agent_final_response")
    }));
}

#[tokio::test]
async fn waiting_for_confirmation_surfaces_the_real_pending_action_shape() {
    let server = MockServer::start(MockMode::WaitingForConfirmation).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    let conversation = adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    let create = server
        .requests()
        .into_iter()
        .find(|request| request.method == "POST" && request.path == "/api/conversations")
        .unwrap();
    assert_eq!(
        create.body["confirmation_policy"],
        json!({"kind": "AlwaysConfirm"})
    );

    let mut stream = adapter
        .prompt_stream(&conversation.id, "check status")
        .await
        .unwrap();
    let mut action_seen = false;
    let approval = loop {
        match stream.next().await.unwrap().unwrap() {
            OpenHandsStreamItem::Event(event) => {
                assert_eq!(event.kind, "ActionEvent");
                assert_eq!(event.category, OpenHandsEventCategory::Action);
                action_seen = true;
            }
            OpenHandsStreamItem::ApprovalRequired { action } => break action,
            OpenHandsStreamItem::Completed { .. } => panic!("approval was not surfaced"),
        }
    };
    assert!(action_seen);
    assert_eq!(approval.kind, "ActionEvent");
    assert_eq!(approval.native_cursor.as_deref(), Some("pending-action"));
    assert_eq!(approval.native["action"]["kind"], "ExecuteBashAction");
}

#[tokio::test]
async fn null_metrics_fall_back_to_stats_usage_to_metrics() {
    let server = MockServer::start(MockMode::NullMetricsWithUsageStats).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();

    let mut stream = adapter
        .prompt_stream(CONVERSATION_ID, "measure usage")
        .await
        .unwrap();
    let usage = loop {
        match stream.next().await.unwrap().unwrap() {
            OpenHandsStreamItem::Event(_) => {}
            OpenHandsStreamItem::ApprovalRequired { .. } => {
                panic!("unexpected approval requirement")
            }
            OpenHandsStreamItem::Completed { usage, .. } => break usage.unwrap(),
        }
    };

    assert_eq!(
        usage["agent"]["accumulated_token_usage"]["prompt_tokens"],
        23
    );
    assert_eq!(
        usage["agent"]["accumulated_token_usage"]["completion_tokens"],
        9
    );
}

#[tokio::test]
async fn approvals_and_cancellation_use_native_agent_server_operations() {
    let server = MockServer::start(MockMode::Normal).await;
    let adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();

    adapter
        .respond_to_approval(CONVERSATION_ID, true, "approved by policy")
        .await
        .unwrap();
    adapter.cancel(CONVERSATION_ID).await.unwrap();

    let requests = server.requests();
    let approval = requests
        .iter()
        .find(|request| request.path.ends_with("/events/respond_to_confirmation"))
        .unwrap();
    assert_eq!(
        approval.path,
        format!("/api/conversations/{CONVERSATION_ID}/events/respond_to_confirmation")
    );
    assert_eq!(
        approval.body,
        json!({"accept": true, "reason": "approved by policy"})
    );
    let cancellation = requests
        .iter()
        .find(|request| request.path.ends_with("/interrupt"))
        .unwrap();
    assert_eq!(
        cancellation.path,
        format!("/api/conversations/{CONVERSATION_ID}/interrupt")
    );
}

#[tokio::test]
async fn dropping_owned_prompt_stream_cancels_the_native_conversation() {
    let server = MockServer::start(MockMode::RunningUntilCancelled).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();

    let stream = adapter
        .prompt_stream(CONVERSATION_ID, "keep running")
        .await
        .unwrap();
    drop(stream);

    for _ in 0..50 {
        if server
            .requests()
            .iter()
            .any(|request| request.path.ends_with("/interrupt"))
        {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("dropping the owned stream did not call the native interrupt route");
}

#[tokio::test]
async fn http_provider_errors_and_request_timeouts_are_typed_without_leaking_bodies() {
    let provider_server = MockServer::start(MockMode::ProviderError).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        provider_server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    let error = adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap_err();
    let OpenHandsError::Provider(provider) = error else {
        panic!("expected provider error");
    };
    assert_eq!(provider.kind, OpenHandsProviderErrorKind::RateLimited);
    assert!(provider.retryable);
    assert!(
        !provider
            .native
            .to_string()
            .contains("http-provider-secret-canary")
    );

    let timeout_server = MockServer::start(MockMode::HangConversation).await;
    let config = adapter_config(OpenHandsReasoningSupport::MaxAccepted).with_timeouts(
        Duration::from_secs(1),
        Duration::from_millis(50),
        Duration::from_millis(10),
    );
    let mut adapter = OpenHandsHttpAdapter::connect(timeout_server.address, config)
        .await
        .unwrap();
    let error = adapter
        .resume_conversation("hang", CapabilityDowngradePolicy::Reject)
        .await
        .unwrap_err();
    assert!(matches!(error, OpenHandsError::Timeout));
}

#[tokio::test]
async fn router_404_and_422_are_not_misclassified_as_provider_failures() {
    for (mode, expected_status) in [
        (MockMode::RouterNotFound, 404),
        (MockMode::RouterValidation, 422),
    ] {
        let server = MockServer::start(mode).await;
        let mut adapter = OpenHandsHttpAdapter::connect(
            server.address,
            adapter_config(OpenHandsReasoningSupport::MaxAccepted),
        )
        .await
        .unwrap();
        let error = adapter
            .create_conversation(CapabilityDowngradePolicy::Reject)
            .await
            .unwrap_err();
        let OpenHandsError::Router(router) = error else {
            panic!("expected router error, got {error:?}");
        };
        assert_eq!(router.status, expected_status);
        assert!(!router.native.to_string().contains("router-secret-canary"));
    }
}

#[tokio::test]
async fn provider_shaped_404_and_422_are_not_misclassified_as_router_failures() {
    for (mode, expected_status, expected_kind) in [
        (
            MockMode::ProviderNotFound,
            404,
            OpenHandsProviderErrorKind::ModelUnavailable,
        ),
        (
            MockMode::ProviderInvalidRequest,
            422,
            OpenHandsProviderErrorKind::InvalidRequest,
        ),
    ] {
        let server = MockServer::start(mode).await;
        let mut adapter = OpenHandsHttpAdapter::connect(
            server.address,
            adapter_config(OpenHandsReasoningSupport::MaxAccepted),
        )
        .await
        .unwrap();
        let error = adapter
            .create_conversation(CapabilityDowngradePolicy::Reject)
            .await
            .unwrap_err();
        let OpenHandsError::Provider(provider) = error else {
            panic!("expected provider error, got {error:?}");
        };
        assert_eq!(provider.status, Some(expected_status));
        assert_eq!(provider.kind, expected_kind);
    }
}

#[tokio::test]
async fn terminal_background_error_uses_conversation_error_classification() {
    let server = MockServer::start(MockMode::TerminalConversationError).await;
    let mut adapter = OpenHandsHttpAdapter::connect(
        server.address,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap();
    adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap();
    let mut stream = adapter
        .prompt_stream(CONVERSATION_ID, "trigger background error")
        .await
        .unwrap();

    let event = match stream.next().await.unwrap().unwrap() {
        OpenHandsStreamItem::Event(event) => event,
        other => panic!("expected native error event, got {other:?}"),
    };
    let decoded = event.provider_error.as_ref().unwrap();
    assert_eq!(decoded.kind, OpenHandsProviderErrorKind::RateLimited);
    assert!(decoded.retryable);
    assert_eq!(decoded.status, None);
    assert!(
        !decoded
            .native
            .to_string()
            .contains("background-secret-canary")
    );

    let error = stream.next().await.unwrap().unwrap_err();
    let OpenHandsError::Provider(provider) = error else {
        panic!("expected classified provider error");
    };
    assert_eq!(provider.kind, OpenHandsProviderErrorKind::RateLimited);
    assert!(provider.retryable);
    assert_eq!(provider.status, None);
    assert_eq!(provider.native["classification"]["kind"], "rate_limit");
    assert!(
        !provider
            .native
            .to_string()
            .contains("background-secret-canary")
    );
}

#[test]
fn mock_openhands_child() {
    let Ok(mode) = std::env::var(CHILD_MODE_ENV) else {
        return;
    };
    if mode == "exit" {
        std::process::exit(27);
    }
    if mode == "sleep" {
        std::thread::sleep(Duration::from_secs(30));
        return;
    }

    if let Ok(capture_path) = std::env::var("OMNISOLO_OPENHANDS_CAPTURE") {
        let keys = [
            "HOME",
            "XDG_CONFIG_HOME",
            "XDG_DATA_HOME",
            "OPENHANDS_AGENT_SERVER_CONFIG_PATH",
            "OH_CONVERSATIONS_PATH",
            "TMUX_TMPDIR",
            "OPENAI_API_KEY",
            "LLM_API_KEY",
        ];
        let captured = keys
            .into_iter()
            .map(|key| (key, std::env::var(key).unwrap_or_default()))
            .collect::<HashMap<_, _>>();
        fs::write(capture_path, serde_json::to_vec(&captured).unwrap()).unwrap();
    }

    let address = std::env::var("OMNISOLO_OPENHANDS_ADDRESS").unwrap();
    let listener = StdTcpListener::bind(address).unwrap();
    if mode == "exit_after_probe" {
        let _ = listener.accept();
        std::process::exit(29);
    }
    let mut http_requests = 0;
    for connection in listener.incoming() {
        let mut connection = connection.unwrap();
        let mut request = [0_u8; 4096];
        let count = connection.read(&mut request).unwrap_or(0);
        if count == 0 {
            continue;
        }
        http_requests += 1;
        let request = String::from_utf8_lossy(&request[..count]);
        let body: &[u8] = if request.starts_with("GET /ready ") {
            b"{\"status\":\"ready\"}"
        } else if request.starts_with("GET /server_info ") {
            b"{\"title\":\"OpenHands Agent Server\",\"version\":\"1.43.1\",\"uptime\":1,\"idle_time\":0}"
        } else {
            b"{}"
        };
        write!(
            connection,
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        )
        .unwrap();
        connection.write_all(body).unwrap();
        if mode == "serve_once" && http_requests == 2 {
            return;
        }
    }
}

fn child_process_config(mode: &str) -> HttpProcessConfig {
    HttpProcessConfig::new(
        std::env::current_exe().unwrap().to_string_lossy(),
        ["--exact", "mock_openhands_child", "--nocapture"],
    )
    .with_environment(CHILD_MODE_ENV, mode)
    .with_environment("OMNISOLO_OPENHANDS_ADDRESS", "{address}")
    .with_readiness(Duration::from_millis(500), Duration::from_millis(10))
}

async fn process_has_exited(process_id: u32) -> bool {
    for _ in 0..100 {
        let alive = std::process::Command::new("/bin/sh")
            .args(["-c", &format!("kill -0 {process_id} 2>/dev/null")])
            .status()
            .unwrap()
            .success();
        if !alive {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    false
}

#[tokio::test]
async fn launch_uses_http_runtime_isolated_homes_child_only_key_and_bounded_shutdown() {
    let parent = std::env::temp_dir().join(format!("omnisolo-openhands-test-{}", Uuid::new_v4()));
    fs::create_dir_all(&parent).unwrap();
    let capture_path = parent.join("captured.json");
    let process = child_process_config("serve")
        .with_environment("OMNISOLO_OPENHANDS_CAPTURE", capture_path.to_string_lossy());
    let launch = OpenHandsLaunchConfig::new(
        process,
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .with_api_key("child-secret-canary");
    let debug = format!("{launch:?}");
    assert!(!debug.contains("child-secret-canary"));

    let adapter = OpenHandsHttpAdapter::launch(launch).await.unwrap();
    let process_id = adapter.process_id().unwrap();
    let isolated_home = adapter.isolated_home().unwrap().to_owned();
    let captured: HashMap<String, String> =
        serde_json::from_slice(&fs::read(&capture_path).unwrap()).unwrap();
    assert_eq!(captured["OPENAI_API_KEY"], "child-secret-canary");
    assert_eq!(captured["LLM_API_KEY"], "child-secret-canary");
    for key in [
        "HOME",
        "XDG_CONFIG_HOME",
        "XDG_DATA_HOME",
        "OPENHANDS_AGENT_SERVER_CONFIG_PATH",
        "OH_CONVERSATIONS_PATH",
        "TMUX_TMPDIR",
    ] {
        assert!(
            Path::new(&captured[key]).starts_with(&isolated_home),
            "{key}"
        );
    }
    assert_ne!(captured["XDG_CONFIG_HOME"], captured["XDG_DATA_HOME"]);

    adapter.shutdown().await.unwrap();
    #[cfg(target_os = "linux")]
    assert!(
        !Path::new(&format!("/proc/{process_id}")).exists(),
        "shutdown returned before the child was killed and waited"
    );
    assert!(process_has_exited(process_id).await);
    assert!(!isolated_home.exists());
    fs::remove_dir_all(parent).unwrap();
}

#[tokio::test]
async fn launch_reports_early_exit_readiness_timeout_and_post_ready_process_exit() {
    let early = OpenHandsLaunchConfig::new(
        child_process_config("exit"),
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    );
    let error = OpenHandsHttpAdapter::launch(early).await.unwrap_err();
    assert!(matches!(
        error,
        OpenHandsError::ProcessExited { status: Some(27) }
    ));

    let sleeping = OpenHandsLaunchConfig::new(
        child_process_config("sleep"),
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    );
    let error = OpenHandsHttpAdapter::launch(sleeping).await.unwrap_err();
    assert!(matches!(error, OpenHandsError::ReadinessTimeout { .. }));

    let exits_after_probe = OpenHandsLaunchConfig::new(
        child_process_config("exit_after_probe"),
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    );
    let error = OpenHandsHttpAdapter::launch(exits_after_probe)
        .await
        .unwrap_err();
    assert!(matches!(error, OpenHandsError::ProcessExited { .. }));

    let once = OpenHandsLaunchConfig::new(
        child_process_config("serve_once"),
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    );
    let mut adapter = OpenHandsHttpAdapter::launch(once).await.unwrap();
    tokio::time::sleep(Duration::from_millis(25)).await;
    let error = adapter
        .create_conversation(CapabilityDowngradePolicy::Reject)
        .await
        .unwrap_err();
    assert!(matches!(error, OpenHandsError::ProcessExited { .. }));
}

#[tokio::test]
async fn connect_rejects_non_loopback_addresses() {
    let error = OpenHandsHttpAdapter::connect(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8000),
        adapter_config(OpenHandsReasoningSupport::MaxAccepted),
    )
    .await
    .unwrap_err();
    assert!(matches!(error, OpenHandsError::NonLoopbackAddress(_)));
}
