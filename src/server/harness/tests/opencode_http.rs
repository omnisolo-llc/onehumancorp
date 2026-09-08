use std::collections::BTreeSet;
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

use server_harness::middleware::harness::HarnessAdapterError;
use server_harness::middleware::opencode::{
    OPENCODE_API_KEY_REFERENCE, OPENCODE_CONFIG_HOME_ENV, OPENCODE_DATA_HOME_ENV,
    OPENCODE_PROVIDER_ID, OpenCodeEventCorrelation, OpenCodeEventDecoder, OpenCodeHttpAdapter,
    OpenCodeIsolatedHome, OpenCodeProcessConfig, build_opencode_config,
};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};

const SECRET_CANARY: &str = "opencode-secret-canary";
const SESSION_ID: &str = "ses_contract_1";

fn selection() -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(ReasoningEffort::Max),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: Some(400_000),
        max_output_tokens: Some(128_000),
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test".to_owned(),
        metadata: [
            ("api_key".to_owned(), json!(SECRET_CANARY)),
            ("base_url".to_owned(), json!("https://must-not-win.test/v1")),
        ]
        .into_iter()
        .collect(),
    }
}

fn correlation() -> OpenCodeEventCorrelation {
    OpenCodeEventCorrelation {
        session_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        task_id: Some(Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()),
        turn_id: Some(Uuid::parse_str("00000000-0000-0000-0000-000000000003").unwrap()),
        attempt_id: "attempt-1".to_owned(),
        native_session_id: SESSION_ID.to_owned(),
        admitted_user_message_id: None,
    }
}

fn temporary_parent(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("omnisolo-opencode-{label}-{}", Uuid::new_v4()));
    fs::create_dir(&path).unwrap();
    path
}

#[test]
fn generated_config_is_responses_compatible_portable_and_secret_free() {
    let config = build_opencode_config(&selection(), "https://llmapi.omnisolo.co/v1/").unwrap();
    let provider = &config["provider"][OPENCODE_PROVIDER_ID];
    let model = &provider["models"]["gpt-5.6-luna"];

    assert_eq!(provider["npm"], "@ai-sdk/openai");
    assert_eq!(
        provider["options"]["baseURL"],
        "https://llmapi.omnisolo.co/v1"
    );
    assert_eq!(
        provider["options"]["apiKey"],
        format!("{{env:{OPENCODE_API_KEY_REFERENCE}}}")
    );
    assert_eq!(provider["options"]["includeUsage"], true);
    assert_eq!(model["variants"]["max"]["reasoningEffort"], "max");
    assert_eq!(model["limit"]["context"], 400_000);
    assert_eq!(model["limit"]["output"], 128_000);

    let serialized = serde_json::to_string(&config).unwrap();
    assert!(serialized.contains(OPENCODE_API_KEY_REFERENCE));
    assert!(!serialized.contains(SECRET_CANARY));
    assert!(!serialized.contains("must-not-win.test"));
}

#[test]
fn config_generation_rejects_non_responses_and_malformed_routing() {
    let mut wrong_dialect = selection();
    wrong_dialect.api_dialect = ModelApiDialect::OpenAiChatCompletions;
    assert!(matches!(
        build_opencode_config(&wrong_dialect, "https://example.test/v1"),
        Err(HarnessAdapterError::InvalidRequest(_))
    ));

    for base_url in [
        "file:///tmp/api",
        "https://user:pass@example.test/v1",
        "https://example.test/v1?key=value",
        "https://example.test/v1#fragment",
        "https://example.test/white space",
    ] {
        assert!(
            matches!(
                build_opencode_config(&selection(), base_url),
                Err(HarnessAdapterError::InvalidRequest(_))
            ),
            "accepted malformed URL {base_url}"
        );
    }

    let mut empty_model = selection();
    empty_model.model_id.clear();
    assert!(matches!(
        build_opencode_config(&empty_model, "https://example.test/v1"),
        Err(HarnessAdapterError::InvalidRequest(_))
    ));

    let mut custom_effort = selection();
    custom_effort.reasoning_effort = Some(ReasoningEffort::Custom);
    assert!(matches!(
        build_opencode_config(&custom_effort, "https://example.test/v1"),
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
}

#[test]
fn config_generation_enforces_provider_route_and_structural_url_validity() {
    let mut unsupported = selection();
    unsupported.provider_route = "anthropic".to_owned();
    assert!(matches!(
        build_opencode_config(&unsupported, "https://example.test/v1"),
        Err(HarnessAdapterError::InvalidRequest(_))
    ));

    for base_url in [
        "http://example.test:bad/v1",
        "http://[::1/v1",
        "http:///missing-host",
    ] {
        assert!(
            matches!(
                build_opencode_config(&selection(), base_url),
                Err(HarnessAdapterError::InvalidRequest(_))
            ),
            "accepted structurally invalid URL {base_url}"
        );
    }
}

#[test]
fn isolated_home_uses_private_xdg_config_and_data_and_cleans_up() {
    let parent = temporary_parent("home");
    let root;
    {
        let home =
            OpenCodeIsolatedHome::create_in(&parent, &selection(), "https://llmapi.omnisolo.co/v1")
                .unwrap();
        root = home.root().to_owned();
        assert!(home.config_home().starts_with(&root));
        assert!(home.data_home().starts_with(&root));
        assert_eq!(
            home.environment()[OPENCODE_CONFIG_HOME_ENV],
            home.config_home()
        );
        assert_eq!(home.environment()[OPENCODE_DATA_HOME_ENV], home.data_home());

        let config = fs::read_to_string(home.config_path()).unwrap();
        assert!(config.contains("@ai-sdk/openai"));
        assert!(!config.contains("@ai-sdk/openai-compatible"));
        assert!(config.contains("{env:OPENAI_API_KEY}"));
        assert!(!config.contains(SECRET_CANARY));

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&root).unwrap().permissions().mode() & 0o777,
                0o700
            );
            assert_eq!(
                fs::metadata(home.config_path())
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
    }
    assert!(!root.exists());
    fs::remove_dir_all(parent).unwrap();
}

#[test]
fn launch_spec_is_opencode_serve_on_allocated_loopback_port_and_redacts_debug() {
    let parent = temporary_parent("launch-spec");
    let home =
        OpenCodeIsolatedHome::create_in(&parent, &selection(), "https://llmapi.omnisolo.co/v1")
            .unwrap();
    let config = OpenCodeProcessConfig::new("opencode")
        .with_environment("OPENAI_API_KEY", SECRET_CANARY)
        .with_working_directory(parent.clone())
        .with_preferred_address(SocketAddr::from(([127, 0, 0, 1], 0)));
    let launch = config.http_process_config(&home);

    assert_eq!(launch.executable, "opencode");
    assert_eq!(
        launch.args,
        [
            "serve",
            "--hostname",
            "{host}",
            "--port",
            "{port}",
            "--pure"
        ]
    );
    assert_eq!(
        launch.preferred_address.ip(),
        IpAddr::V4(Ipv4Addr::LOCALHOST)
    );
    assert_eq!(
        launch.environment[OPENCODE_CONFIG_HOME_ENV],
        home.config_home().to_string_lossy()
    );
    assert_eq!(
        launch.environment[OPENCODE_DATA_HOME_ENV],
        home.data_home().to_string_lossy()
    );
    assert!(!format!("{config:?}").contains(SECRET_CANARY));
    assert!(!format!("{launch:?}").contains(SECRET_CANARY));

    drop(home);
    fs::remove_dir_all(parent).unwrap();
}

#[tokio::test]
async fn launches_real_pinned_server_with_bounded_health_and_session_lifecycle() {
    let parent = temporary_parent("real-server");
    let config = OpenCodeProcessConfig::new("opencode")
        .with_environment("OPENAI_API_KEY", SECRET_CANARY)
        .with_isolation_parent(parent.clone())
        .with_readiness(Duration::from_secs(10), Duration::from_millis(20))
        .with_request_timeout(Duration::from_secs(3));
    let adapter = OpenCodeHttpAdapter::spawn(config, selection(), "https://llmapi.omnisolo.co/v1")
        .await
        .unwrap();

    assert!(adapter.address().ip().is_loopback());
    assert!(adapter.process_id().is_some());
    assert_eq!(adapter.health().await.unwrap().version, "1.18.15");
    let session = adapter
        .create_session("Task 8 contract", &selection())
        .await
        .unwrap();
    assert!(session.id.starts_with("ses"));
    let fetched = adapter.get_session(&session.id).await.unwrap();
    assert_eq!(fetched.id, session.id);
    adapter.delete_session(&session.id).await.unwrap();

    let isolated_root = adapter.isolated_root().unwrap().to_owned();
    let process_id = adapter.process_id().unwrap();
    drop(adapter);
    for _ in 0..100 {
        if !isolated_root.exists() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert!(!isolated_root.exists());
    #[cfg(unix)]
    {
        let mut exited = false;
        for _ in 0..100 {
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
            "OpenCode process {process_id} survived adapter drop"
        );
    }
    fs::remove_dir_all(parent).unwrap();
}

#[tokio::test]
async fn real_pinned_server_executes_a_prompt_through_openai_responses() {
    let provider_listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
    let provider_address = provider_listener.local_addr().unwrap();

    let parent = temporary_parent("real-provider-prompt");
    let config = OpenCodeProcessConfig::new("opencode")
        .with_environment("OPENAI_API_KEY", SECRET_CANARY)
        .with_isolation_parent(parent.clone())
        .with_readiness(Duration::from_secs(30), Duration::from_millis(20))
        .with_request_timeout(Duration::from_secs(15));
    let adapter = OpenCodeHttpAdapter::spawn(
        config,
        selection(),
        &format!("http://{provider_address}/v1"),
    )
    .await
    .unwrap();
    let session = adapter
        .create_session("Responses provider prompt", &selection())
        .await
        .unwrap();
    let provider = tokio::spawn(serve_one_responses_request(provider_listener));
    let result = adapter
        .prompt(&session.id, "Return the provider marker only", &selection())
        .await
        .unwrap();
    let provider_body = provider.await.unwrap();

    assert_eq!(
        result.final_text.as_deref(),
        Some("opencode-provider-ok"),
        "native OpenCode prompt result: {:#?}; provider request: {provider_body:#?}",
        result.native,
    );
    assert!(result.usage.is_some());
    assert!(
        !serde_json::to_string(&provider_body)
            .unwrap()
            .contains(SECRET_CANARY)
    );
    adapter.shutdown().await.unwrap();
    fs::remove_dir_all(parent).unwrap();
}

async fn serve_one_responses_request(provider_listener: TcpListener) -> Value {
    let (mut stream, _) = tokio::time::timeout(Duration::from_secs(30), provider_listener.accept())
        .await
        .expect("OpenCode did not call the Responses provider")
        .unwrap();
    let (method, path, body) = read_request(&mut stream).await.unwrap();
    assert_eq!(method, "POST");
    assert_eq!(path, "/v1/responses");
    assert_eq!(body["model"], "gpt-5.6-luna");
    assert_eq!(body["reasoning"]["effort"], "max");
    write_responses_stream(&mut stream, "opencode-provider-ok").await;
    body
}

async fn write_responses_stream(stream: &mut TcpStream, text: &str) {
    let response_id = "resp_opencode_contract_1";
    let message_id = "msg_opencode_contract_1";
    let completed = json!({
        "id": response_id,
        "object": "response",
        "created_at": 1_777_777_777,
        "status": "completed",
        "model": "gpt-5.6-luna",
        "output": [{
            "id": message_id,
            "type": "message",
            "role": "assistant",
            "status": "completed",
            "content": [{"type":"output_text","text":text,"annotations":[],"logprobs":[]}]
        }],
        "parallel_tool_calls": true,
        "tool_choice": "auto",
        "tools": [],
        "usage": {
            "input_tokens": 6,
            "input_tokens_details": {"cached_tokens": 0},
            "output_tokens": 4,
            "output_tokens_details": {"reasoning_tokens": 1},
            "total_tokens": 10
        }
    });
    let events = vec![
        json!({"type":"response.created","sequence_number":0,"response":{
            "id":response_id,"object":"response","created_at":1_777_777_777,
            "status":"in_progress","model":"gpt-5.6-luna","output":[],"parallel_tool_calls":true,
            "tool_choice":"auto","tools":[],"usage":null
        }}),
        json!({"type":"response.in_progress","sequence_number":1,"response":{
            "id":response_id,"object":"response","created_at":1_777_777_777,
            "status":"in_progress","model":"gpt-5.6-luna","output":[],"parallel_tool_calls":true,
            "tool_choice":"auto","tools":[],"usage":null
        }}),
        json!({"type":"response.output_item.added","sequence_number":2,"output_index":0,
            "item":{"id":message_id,"type":"message","role":"assistant","status":"in_progress","content":[]}}),
        json!({"type":"response.content_part.added","sequence_number":3,"item_id":message_id,
            "output_index":0,"content_index":0,"part":{"type":"output_text","text":"","annotations":[],"logprobs":[]}}),
        json!({"type":"response.output_text.delta","sequence_number":4,"item_id":message_id,
            "output_index":0,"content_index":0,"delta":text,"logprobs":[]}),
        json!({"type":"response.output_text.done","sequence_number":5,"item_id":message_id,
            "output_index":0,"content_index":0,"text":text,"logprobs":[]}),
        json!({"type":"response.content_part.done","sequence_number":6,"item_id":message_id,
            "output_index":0,"content_index":0,
            "part":{"type":"output_text","text":text,"annotations":[],"logprobs":[]}}),
        json!({"type":"response.output_item.done","sequence_number":7,"output_index":0,
            "item":{"id":message_id,"type":"message","role":"assistant","status":"completed",
                "content":[{"type":"output_text","text":text,"annotations":[],"logprobs":[]}]}}),
        json!({"type":"response.completed","sequence_number":8,"response":completed}),
    ];
    stream
        .write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();
    for event in events {
        let event_type = event["type"].as_str().unwrap();
        let frame = format!("event: {event_type}\ndata: {event}\n\n");
        stream.write_all(frame.as_bytes()).await.unwrap();
    }
    stream.write_all(b"data: [DONE]\n\n").await.unwrap();
}

#[tokio::test]
async fn explicit_shutdown_reaps_managed_process_before_removing_isolated_home() {
    let parent = temporary_parent("explicit-shutdown");
    let config = OpenCodeProcessConfig::new("opencode")
        .with_environment("OPENAI_API_KEY", SECRET_CANARY)
        .with_isolation_parent(parent.clone())
        .with_readiness(Duration::from_secs(10), Duration::from_millis(20))
        .with_request_timeout(Duration::from_secs(3));
    let adapter = OpenCodeHttpAdapter::spawn(config, selection(), "https://llmapi.omnisolo.co/v1")
        .await
        .unwrap();
    let isolated_root = adapter.isolated_root().unwrap().to_owned();
    let process_id = adapter.process_id().unwrap();

    adapter.shutdown().await.unwrap();

    assert!(!isolated_root.exists());
    #[cfg(unix)]
    assert!(
        !std::process::Command::new("/bin/sh")
            .args(["-c", &format!("kill -0 {process_id} 2>/dev/null")])
            .status()
            .unwrap()
            .success(),
        "OpenCode process {process_id} was not reaped before home removal"
    );
    fs::remove_dir_all(parent).unwrap();
}

#[tokio::test]
async fn synchronous_prompt_maps_selected_provider_model_text_usage_and_abort() {
    let server = MockOpenCodeServer::start(MockMode::Contract).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(2)).unwrap();

    let result = adapter
        .prompt(SESSION_ID, "Answer exactly once", &selection())
        .await
        .unwrap();
    assert_eq!(result.final_text.as_deref(), Some("hello world"));
    assert_eq!(result.usage.as_ref().unwrap()["tokens"]["input"], 3);
    assert_eq!(result.usage.as_ref().unwrap()["tokens"]["output"], 2);
    assert_eq!(result.usage.as_ref().unwrap()["tokens"]["reasoning"], 1);
    assert_eq!(result.usage.as_ref().unwrap()["cost"], 0.125);
    adapter.abort(SESSION_ID).await.unwrap();

    let requests = server.requests().await;
    let prompt = requests
        .iter()
        .find(|request| request.path == format!("/session/{SESSION_ID}/message"))
        .unwrap();
    assert_eq!(prompt.method, "POST");
    assert_eq!(prompt.body["model"]["providerID"], OPENCODE_PROVIDER_ID);
    assert_eq!(prompt.body["model"]["modelID"], "gpt-5.6-luna");
    assert_eq!(prompt.body["variant"], "max");
    assert_eq!(prompt.body["parts"][0]["type"], "text");
    assert_eq!(prompt.body["parts"][0]["text"], "Answer exactly once");
    assert!(requests.iter().any(|request| {
        request.method == "POST" && request.path == format!("/session/{SESSION_ID}/abort")
    }));
    server.shutdown().await;
}

#[tokio::test]
async fn async_prompt_subscribes_sse_and_maps_native_events_usage_and_errors() {
    let server = MockOpenCodeServer::start(MockMode::Contract).await;
    let adapter = OpenCodeHttpAdapter::connect_with_secret_values(
        server.address(),
        Duration::from_secs(2),
        [SECRET_CANARY.to_owned()],
    )
    .unwrap();
    let mut events = adapter
        .prompt_async(correlation(), "Stream the answer", &selection())
        .await
        .unwrap();

    let mut decoded = Vec::new();
    loop {
        let event = events.next_event().await.unwrap();
        let terminal = event.terminal;
        decoded.push(event);
        if terminal {
            break;
        }
    }
    assert_eq!(
        decoded
            .iter()
            .map(|event| event.event.event_type.as_str())
            .collect::<Vec<_>>(),
        [
            "harness.connected",
            "assistant.message_updated",
            "assistant.text_chunk",
            "assistant.reasoning",
            "assistant.final",
        ]
    );
    assert_eq!(decoded[2].event.payload["content"], "hello world");
    assert_eq!(decoded[3].event.payload["content"], "carefully");
    assert_eq!(decoded[4].final_text.as_deref(), Some("hello world"));
    assert_eq!(decoded[4].usage.as_ref().unwrap()["tokens"]["total"], 6);
    for event in &decoded {
        assert!(event.event.payload.get("native").is_some());
        assert!(!event.event.payload.to_string().contains(SECRET_CANARY));
    }
    assert_eq!(
        decoded[2].event.payload["native"]["properties"]["apiKey"],
        "<redacted>"
    );

    let requests = server.requests().await;
    let event_index = requests
        .iter()
        .position(|request| request.path == "/event")
        .unwrap();
    let prompt_index = requests
        .iter()
        .position(|request| request.path == format!("/session/{SESSION_ID}/prompt_async"))
        .unwrap();
    assert!(
        event_index < prompt_index,
        "SSE must subscribe before async prompt admission"
    );
    server.shutdown().await;
}

#[tokio::test]
async fn every_async_prompt_admits_a_unique_native_user_message_id() {
    let server = MockOpenCodeServer::start(MockMode::Contract).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(2)).unwrap();

    let first = adapter
        .prompt_async(correlation(), "first", &selection())
        .await
        .unwrap();
    drop(first);
    tokio::time::sleep(Duration::from_millis(25)).await;
    let second = adapter
        .prompt_async(correlation(), "second", &selection())
        .await
        .unwrap();
    drop(second);

    let message_ids = server
        .requests()
        .await
        .into_iter()
        .filter(|request| request.path.ends_with("/prompt_async"))
        .map(|request| {
            request.body["messageID"]
                .as_str()
                .unwrap_or_default()
                .to_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(message_ids.len(), 2);
    assert!(message_ids.iter().all(|id| id.starts_with("msg_")));
    assert_ne!(message_ids[0], message_ids[1]);
    server.shutdown().await;
}

#[tokio::test]
async fn overlapping_async_turns_for_one_session_are_rejected() {
    let server = MockOpenCodeServer::start(MockMode::Contract).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(2)).unwrap();
    let first = adapter
        .prompt_async(correlation(), "first", &selection())
        .await
        .unwrap();

    assert!(matches!(
        adapter
            .prompt_async(correlation(), "overlap", &selection())
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));

    drop(first);
    server.shutdown().await;
}

#[tokio::test]
async fn active_async_turn_can_explicitly_cancel_and_release_its_session() {
    let server = MockOpenCodeServer::start(MockMode::Pending).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(2)).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "pending", &selection())
        .await
        .unwrap();

    stream.cancel().await.unwrap();
    let replacement = adapter
        .prompt_async(correlation(), "replacement", &selection())
        .await
        .unwrap();
    drop(replacement);

    let aborts = server
        .requests()
        .await
        .into_iter()
        .filter(|request| request.path == format!("/session/{SESSION_ID}/abort"))
        .count();
    assert_eq!(aborts, 1);
    server.shutdown().await;
}

#[tokio::test]
async fn dropping_active_async_turn_sends_best_effort_abort() {
    let server = MockOpenCodeServer::start(MockMode::Pending).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(2)).unwrap();
    let stream = adapter
        .prompt_async(correlation(), "drop me", &selection())
        .await
        .unwrap();

    drop(stream);
    let mut abort_observed = false;
    for _ in 0..100 {
        abort_observed = server
            .requests()
            .await
            .iter()
            .any(|request| request.path == format!("/session/{SESSION_ID}/abort"));
        if abort_observed {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert!(
        abort_observed,
        "dropping an admitted turn orphaned generation"
    );
    server.shutdown().await;
}

#[tokio::test]
async fn async_event_timeout_aborts_and_releases_the_active_turn() {
    let server = MockOpenCodeServer::start(MockMode::Pending).await;
    let adapter =
        OpenCodeHttpAdapter::connect(server.address(), Duration::from_millis(50)).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "timeout", &selection())
        .await
        .unwrap();
    assert_eq!(
        stream.next_event().await.unwrap().event.event_type,
        "harness.connected"
    );

    assert!(matches!(
        stream.next_event().await,
        Err(HarnessAdapterError::Timeout)
    ));
    let replacement = adapter
        .prompt_async(correlation(), "after timeout", &selection())
        .await
        .unwrap();
    drop(replacement);
    assert!(
        server
            .requests()
            .await
            .iter()
            .any(|request| request.path == format!("/session/{SESSION_ID}/abort"))
    );
    server.shutdown().await;
}

#[test]
fn event_decoder_correlates_assistant_and_parts_through_admitted_parent_id() {
    let mut turn = correlation();
    turn.admitted_user_message_id = Some("msg_admitted_user".to_owned());
    let mut decoder = OpenCodeEventDecoder::new(turn);

    let unrelated = decoder
        .decode(assistant_message_event(
            "evt_other",
            "msg_other_assistant",
            "msg_other_user",
            false,
        ))
        .unwrap();
    assert!(unrelated.is_none());
    let admitted = decoder
        .decode(assistant_message_event(
            "evt_admitted",
            "msg_admitted_assistant",
            "msg_admitted_user",
            false,
        ))
        .unwrap();
    assert!(admitted.is_some());

    let unrelated_part = decoder
        .decode(text_delta_event(
            "evt_part_other",
            "msg_other_assistant",
            "wrong",
        ))
        .unwrap();
    assert!(unrelated_part.is_none());
    let admitted_part = decoder
        .decode(text_delta_event(
            "evt_part_admitted",
            "msg_admitted_assistant",
            "kept",
        ))
        .unwrap()
        .unwrap();
    assert_eq!(admitted_part.event.payload["content"], "kept");
}

#[test]
fn assistant_is_final_only_with_completed_time_and_finish_evidence() {
    let mut turn = correlation();
    turn.admitted_user_message_id = Some("msg_user_completion".to_owned());
    let mut decoder = OpenCodeEventDecoder::new(turn);

    let in_progress = decoder
        .decode(assistant_message_event(
            "evt_in_progress",
            "msg_assistant_completion",
            "msg_user_completion",
            false,
        ))
        .unwrap()
        .unwrap();
    assert_eq!(in_progress.event.event_type, "assistant.message_updated");
    assert!(!in_progress.terminal);
    assert_eq!(in_progress.final_text, None);

    decoder
        .decode(text_delta_event(
            "evt_completion_delta",
            "msg_assistant_completion",
            "finished text",
        ))
        .unwrap()
        .unwrap();
    let completed = decoder
        .decode(assistant_message_event(
            "evt_completed",
            "msg_assistant_completion",
            "msg_user_completion",
            true,
        ))
        .unwrap()
        .unwrap();
    assert_eq!(completed.event.event_type, "assistant.final");
    assert!(completed.terminal);
    assert_eq!(completed.final_text.as_deref(), Some("finished text"));
}

#[test]
fn tool_call_completion_keeps_turn_open_for_the_next_assistant_message() {
    let mut turn = correlation();
    turn.admitted_user_message_id = Some("user-tools".into());
    let mut decoder = OpenCodeEventDecoder::new(turn);
    decoder
        .decode(assistant_message_event(
            "start",
            "assistant-tools",
            "user-tools",
            false,
        ))
        .unwrap();
    decoder
        .decode(text_delta_event(
            "thinking",
            "assistant-tools",
            "Calling a tool",
        ))
        .unwrap();
    let mut step = assistant_message_event("tool-step", "assistant-tools", "user-tools", true);
    step["properties"]["info"]["finish"] = json!("tool-calls");
    let step = decoder.decode(step).unwrap().unwrap();
    assert!(
        !step.terminal,
        "a tool call completes a model step, not the native turn"
    );
    assert!(step.final_text.is_none());
    assert!(
        decoder
            .decode(assistant_message_event(
                "next",
                "assistant-final",
                "user-tools",
                false
            ))
            .unwrap()
            .is_some()
    );
    decoder
        .decode(text_delta_event(
            "answer",
            "assistant-final",
            "Retrieved actual service value",
        ))
        .unwrap();
    let done = decoder
        .decode(assistant_message_event(
            "done",
            "assistant-final",
            "user-tools",
            true,
        ))
        .unwrap()
        .unwrap();
    assert!(done.terminal);
    assert_eq!(
        done.final_text.as_deref(),
        Some("Retrieved actual service value")
    );
}

#[test]
fn documented_native_error_union_maps_terminal_outcomes() {
    for (error, expected_type, expected_class) in [
        (
            json!({"name":"MessageAbortedError","data":{"message":"stopped"}}),
            "turn.cancelled",
            "aborted",
        ),
        (
            json!({"name":"ProviderAuthError","data":{"providerID":OPENCODE_PROVIDER_ID,"message":"bad key"}}),
            "turn.failed",
            "authentication",
        ),
        (
            json!({"name":"APIError","data":{"message":"slow down","statusCode":429,"isRetryable":true}}),
            "turn.failed",
            "rate_limit",
        ),
        (
            json!({"name":"ContextOverflowError","data":{"message":"too much context"}}),
            "turn.failed",
            "context_length",
        ),
    ] {
        let mut turn = correlation();
        turn.admitted_user_message_id = Some("msg_error_user".to_owned());
        let mut decoder = OpenCodeEventDecoder::new(turn);
        let event = decoder
            .decode(assistant_error_event(error))
            .unwrap()
            .unwrap();
        assert_eq!(event.event.event_type, expected_type);
        assert_eq!(event.error_class.as_deref(), Some(expected_class));
        assert!(event.terminal);
    }
}

#[test]
fn durable_native_errors_whitelist_fields_and_scrub_exact_credentials() {
    let mut decoder =
        OpenCodeEventDecoder::new_with_secrets(correlation(), [SECRET_CANARY.to_owned()]);
    let failed = decoder
        .decode(json!({
            "id":"evt_safe_error",
            "type":"session.error",
            "properties":{
                "sessionID":SESSION_ID,
                "tokens":{"input":3,"output":2,"total":5},
                "error":{
                    "name":"APIError",
                    "data":{
                        "message":format!("provider echoed {SECRET_CANARY}"),
                        "statusCode":429,
                        "isRetryable":true,
                        "responseHeaders":{"authorization":SECRET_CANARY},
                        "responseBody":format!("raw {SECRET_CANARY}"),
                        "metadata":{"request":"unsafe"}
                    }
                }
            }
        }))
        .unwrap()
        .unwrap();
    let native = &failed.event.payload["native"];

    assert_eq!(native["properties"]["tokens"]["total"], 5);
    assert_eq!(
        native["properties"]["error"]["data"]["message"],
        "provider echoed <redacted>"
    );
    assert!(
        native["properties"]["error"]["data"]
            .get("responseHeaders")
            .is_none()
    );
    assert!(
        native["properties"]["error"]["data"]
            .get("responseBody")
            .is_none()
    );
    assert!(
        native["properties"]["error"]["data"]
            .get("metadata")
            .is_none()
    );
    assert!(!native.to_string().contains(SECRET_CANARY));
}

#[test]
fn text_snapshot_event_includes_snapshot_content() {
    let mut turn = correlation();
    turn.admitted_user_message_id = Some("msg_snapshot_user".to_owned());
    let mut decoder = OpenCodeEventDecoder::new(turn);
    decoder
        .decode(assistant_message_event(
            "evt_snapshot_assistant",
            "msg_snapshot_assistant",
            "msg_snapshot_user",
            false,
        ))
        .unwrap()
        .unwrap();

    let snapshot = decoder
        .decode(json!({
            "id":"evt_snapshot",
            "type":"message.part.updated",
            "properties":{
                "sessionID":SESSION_ID,
                "time":2,
                "part":{
                    "id":"prt_snapshot",
                    "sessionID":SESSION_ID,
                    "messageID":"msg_snapshot_assistant",
                    "type":"text",
                    "text":"complete snapshot"
                }
            }
        }))
        .unwrap()
        .unwrap();
    assert_eq!(snapshot.event.event_type, "assistant.text_snapshot");
    assert_eq!(snapshot.event.payload["content"], "complete snapshot");
}

#[test]
fn event_decoder_maps_provider_errors_and_fails_closed_on_malformed_native_json() {
    let mut decoder = OpenCodeEventDecoder::new(correlation());
    let failed = decoder
        .decode(json!({
            "id":"evt_error",
            "type":"session.error",
            "properties": {
                "sessionID": SESSION_ID,
                "error": {
                    "name":"ProviderAuthError",
                    "data": {
                        "providerID": OPENCODE_PROVIDER_ID,
                        "message":"invalid credential",
                        "apiKey": SECRET_CANARY
                    }
                }
            }
        }))
        .unwrap()
        .unwrap();
    assert_eq!(failed.event.event_type, "turn.failed");
    assert_eq!(failed.error_class.as_deref(), Some("authentication"));
    assert!(failed.terminal);
    assert!(
        failed.event.payload["native"]["properties"]["error"]["data"]
            .get("apiKey")
            .is_none()
    );
    assert!(!failed.event.payload.to_string().contains(SECRET_CANARY));

    let mut malformed_turn = correlation();
    malformed_turn.admitted_user_message_id = Some("msg_bad_user".to_owned());
    let mut decoder = OpenCodeEventDecoder::new(malformed_turn);
    decoder
        .decode(assistant_message_event(
            "evt_bad_assistant",
            "msg_bad_assistant",
            "msg_bad_user",
            false,
        ))
        .unwrap()
        .unwrap();

    for malformed in [
        Value::Null,
        json!({}),
        json!({"id":"evt_bad","type":"message.part.delta","properties":{}}),
        json!({
            "id":"evt_bad",
            "type":"message.part.delta",
            "properties": {
                "sessionID": SESSION_ID,
                "messageID":"msg_bad_assistant",
                "partID":"prt_1",
                "field":"text",
                "delta": 7
            }
        }),
    ] {
        let result = decoder.decode(malformed.clone());
        assert!(
            matches!(result, Err(HarnessAdapterError::InvalidResponse(_))),
            "malformed frame was not rejected: {malformed:#?}; result: {result:#?}"
        );
    }
}

#[tokio::test]
async fn malformed_success_http_and_sse_responses_fail_closed() {
    for mode in [MockMode::MalformedSession, MockMode::MalformedPrompt] {
        let server = MockOpenCodeServer::start(mode).await;
        let adapter =
            OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(1)).unwrap();
        let result = match mode {
            MockMode::MalformedSession => adapter.get_session(SESSION_ID).await.map(|_| ()),
            MockMode::MalformedPrompt => adapter
                .prompt(SESSION_ID, "bad", &selection())
                .await
                .map(|_| ()),
            _ => unreachable!(),
        };
        assert!(matches!(
            result,
            Err(HarnessAdapterError::InvalidResponse(_))
        ));
        server.shutdown().await;
    }

    let server = MockOpenCodeServer::start(MockMode::MalformedSse).await;
    let adapter = OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(1)).unwrap();
    let mut stream = adapter
        .prompt_async(correlation(), "bad stream", &selection())
        .await
        .unwrap();
    assert!(matches!(
        stream.next_event().await,
        Err(HarnessAdapterError::InvalidResponse(_))
    ));
    server.shutdown().await;
}

#[tokio::test]
async fn successful_session_and_prompt_payloads_require_documented_fields() {
    for mode in [MockMode::SparseSession, MockMode::SparsePrompt] {
        let server = MockOpenCodeServer::start(mode).await;
        let adapter =
            OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(1)).unwrap();
        let result = match mode {
            MockMode::SparseSession => adapter.get_session(SESSION_ID).await.map(|_| ()),
            MockMode::SparsePrompt => adapter
                .prompt(SESSION_ID, "strict", &selection())
                .await
                .map(|_| ()),
            _ => unreachable!(),
        };
        assert!(matches!(
            result,
            Err(HarnessAdapterError::InvalidResponse(_))
        ));
        server.shutdown().await;
    }
}

#[tokio::test]
async fn sse_rejects_oversized_lines_and_events_before_json_decode() {
    for (mode, expected) in [
        (MockMode::OversizedSseLine, "line exceeded"),
        (MockMode::OversizedSseEvent, "event exceeded"),
    ] {
        let server = MockOpenCodeServer::start(mode).await;
        let adapter =
            OpenCodeHttpAdapter::connect(server.address(), Duration::from_secs(2)).unwrap();
        let mut stream = adapter
            .prompt_async(correlation(), "bounded", &selection())
            .await
            .unwrap();
        let error = stream.next_event().await.unwrap_err();
        assert!(matches!(error, HarnessAdapterError::InvalidResponse(_)));
        assert!(error.to_string().contains(expected), "{error}");
        server.shutdown().await;
    }
}

#[tokio::test]
async fn maps_http_errors_timeouts_process_exit_and_non_loopback_binding() {
    let rejected = MockOpenCodeServer::start(MockMode::RateLimited).await;
    let adapter = OpenCodeHttpAdapter::connect(rejected.address(), Duration::from_secs(1)).unwrap();
    let error = adapter.get_session(SESSION_ID).await.unwrap_err();
    assert!(matches!(error, HarnessAdapterError::Remote(_)));
    assert!(error.to_string().contains("rate_limit"));
    rejected.shutdown().await;

    let slow = MockOpenCodeServer::start(MockMode::Slow).await;
    let adapter = OpenCodeHttpAdapter::connect(slow.address(), Duration::from_millis(50)).unwrap();
    assert!(matches!(
        adapter.get_session(SESSION_ID).await,
        Err(HarnessAdapterError::Timeout)
    ));
    slow.shutdown().await;

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
    let exited_address = listener.local_addr().unwrap();
    drop(listener);
    let adapter = OpenCodeHttpAdapter::connect(exited_address, Duration::from_millis(100)).unwrap();
    assert!(matches!(
        adapter.get_session(SESSION_ID).await,
        Err(HarnessAdapterError::ProcessExited)
    ));

    assert!(matches!(
        OpenCodeHttpAdapter::connect(
            SocketAddr::from(([0, 0, 0, 0], 12345)),
            Duration::from_secs(1)
        ),
        Err(HarnessAdapterError::InvalidRequest(_))
    ));

    let parent = temporary_parent("process-errors");
    let early_exit = OpenCodeProcessConfig::new("/bin/sh")
        .with_prefix_args(["-c", "exit 23", "--"])
        .with_isolation_parent(parent.clone())
        .with_readiness(Duration::from_secs(1), Duration::from_millis(10));
    assert!(matches!(
        OpenCodeHttpAdapter::spawn(early_exit, selection(), "https://llmapi.omnisolo.co/v1").await,
        Err(HarnessAdapterError::ProcessExited)
    ));

    let readiness_timeout = OpenCodeProcessConfig::new("/bin/sh")
        .with_prefix_args(["-c", "sleep 30", "--"])
        .with_isolation_parent(parent.clone())
        .with_readiness(Duration::from_millis(75), Duration::from_millis(10));
    assert!(matches!(
        OpenCodeHttpAdapter::spawn(
            readiness_timeout,
            selection(),
            "https://llmapi.omnisolo.co/v1"
        )
        .await,
        Err(HarnessAdapterError::Timeout)
    ));

    let non_loopback = OpenCodeProcessConfig::new("opencode")
        .with_isolation_parent(parent.clone())
        .with_preferred_address(SocketAddr::from(([0, 0, 0, 0], 0)));
    assert!(matches!(
        OpenCodeHttpAdapter::spawn(non_loopback, selection(), "https://llmapi.omnisolo.co/v1")
            .await,
        Err(HarnessAdapterError::InvalidRequest(_))
    ));
    fs::remove_dir_all(parent).unwrap();
}

#[tokio::test]
async fn spawn_retries_a_hung_health_probe_before_the_readiness_deadline() {
    let parent = temporary_parent("hung-health-retry");
    let script = concat!(
        "import os,socket,threading,time\n",
        "host,port=os.environ['TEST_ADDRESS'].rsplit(':',1)\n",
        "listener=socket.socket()\n",
        "listener.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1)\n",
        "listener.bind((host,int(port)))\n",
        "listener.listen()\n",
        "lock=threading.Lock()\n",
        "health_requests=[0]\n",
        "def handle(connection):\n",
        " data=b''\n",
        " connection.settimeout(1)\n",
        " try:\n",
        "  while b'\\r\\n\\r\\n' not in data:\n",
        "   chunk=connection.recv(1024)\n",
        "   if not chunk: break\n",
        "   data+=chunk\n",
        " except (TimeoutError,socket.timeout): pass\n",
        " if not data:\n",
        "  connection.close()\n",
        "  return\n",
        " with lock:\n",
        "  health_requests[0]+=1\n",
        "  attempt=health_requests[0]\n",
        " if attempt==1:\n",
        "  time.sleep(30)\n",
        "  connection.close()\n",
        "  return\n",
        " body=b'{\"healthy\":true,\"version\":\"1.18.15\"}'\n",
        " response=b'HTTP/1.1 200 OK\\r\\nContent-Type: application/json\\r\\nContent-Length: '+str(len(body)).encode()+b'\\r\\nConnection: close\\r\\n\\r\\n'+body\n",
        " connection.sendall(response)\n",
        " connection.close()\n",
        "while True:\n",
        " connection,_=listener.accept()\n",
        " threading.Thread(target=handle,args=(connection,),daemon=True).start()\n",
    );
    let config = OpenCodeProcessConfig::new("python3")
        .with_prefix_args(["-c", script])
        .with_environment("TEST_ADDRESS", "{address}")
        .with_isolation_parent(parent.clone())
        .with_readiness(Duration::from_secs(3), Duration::from_millis(10))
        .with_request_timeout(Duration::from_secs(30));

    let adapter = OpenCodeHttpAdapter::spawn(config, selection(), "https://llmapi.omnisolo.co/v1")
        .await
        .unwrap();

    adapter.shutdown().await.unwrap();
    fs::remove_dir_all(parent).unwrap();
}

#[tokio::test]
async fn spawn_uses_one_absolute_deadline_for_tcp_and_health_readiness() {
    let parent = temporary_parent("absolute-readiness");
    let script = concat!(
        "import os,socket,time\n",
        "host,port=os.environ['TEST_ADDRESS'].rsplit(':',1)\n",
        "listener=socket.socket()\n",
        "listener.setsockopt(socket.SOL_SOCKET,socket.SO_REUSEADDR,1)\n",
        "listener.bind((host,int(port)))\n",
        "listener.listen()\n",
        "while True:\n",
        " connection,_=listener.accept()\n",
        " time.sleep(30)\n",
    );
    let config = OpenCodeProcessConfig::new("python3")
        .with_prefix_args(["-c", script])
        .with_environment("TEST_ADDRESS", "{address}")
        .with_isolation_parent(parent.clone())
        .with_readiness(Duration::from_millis(150), Duration::from_millis(10))
        .with_request_timeout(Duration::from_secs(2));

    let started = Instant::now();
    assert!(matches!(
        OpenCodeHttpAdapter::spawn(config, selection(), "https://llmapi.omnisolo.co/v1").await,
        Err(HarnessAdapterError::Timeout)
    ));
    assert!(
        started.elapsed() < Duration::from_millis(750),
        "readiness exceeded its absolute deadline: {:?}",
        started.elapsed()
    );
    fs::remove_dir_all(parent).unwrap();
}

#[derive(Clone, Copy, Debug)]
enum MockMode {
    Contract,
    Pending,
    MalformedSession,
    MalformedPrompt,
    SparseSession,
    SparsePrompt,
    OversizedSseLine,
    OversizedSseEvent,
    MalformedSse,
    RateLimited,
    Slow,
}

#[derive(Clone, Debug)]
struct CapturedRequest {
    method: String,
    path: String,
    body: Value,
}

struct MockOpenCodeServer {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<CapturedRequest>>>,
    _admitted_message_id: Arc<Mutex<Option<String>>>,
    shutdown: Option<oneshot::Sender<()>>,
}

impl MockOpenCodeServer {
    async fn start(mode: MockMode) -> Self {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let server_requests = Arc::clone(&requests);
        let admitted_message_id = Arc::new(Mutex::new(None));
        let server_message_id = Arc::clone(&admitted_message_id);
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => break,
                    accepted = listener.accept() => {
                        let Ok((stream, _)) = accepted else { break };
                        let requests = Arc::clone(&server_requests);
                        let admitted_message_id = Arc::clone(&server_message_id);
                        tokio::spawn(async move {
                            handle_mock_connection(stream, mode, requests, admitted_message_id).await;
                        });
                    }
                }
            }
        });
        Self {
            address,
            requests,
            _admitted_message_id: admitted_message_id,
            shutdown: Some(shutdown_tx),
        }
    }

    fn address(&self) -> SocketAddr {
        self.address
    }

    async fn requests(&self) -> Vec<CapturedRequest> {
        self.requests.lock().await.clone()
    }

    async fn shutdown(mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
    }
}

async fn handle_mock_connection(
    mut stream: TcpStream,
    mode: MockMode,
    requests: Arc<Mutex<Vec<CapturedRequest>>>,
    admitted_message_id: Arc<Mutex<Option<String>>>,
) {
    let Some((method, path, body)) = read_request(&mut stream).await else {
        return;
    };
    requests.lock().await.push(CapturedRequest {
        method: method.clone(),
        path: path.clone(),
        body: body.clone(),
    });

    if matches!(mode, MockMode::Slow) {
        tokio::time::sleep(Duration::from_secs(5)).await;
        return;
    }
    if matches!(mode, MockMode::RateLimited) {
        write_json(
            &mut stream,
            429,
            json!({"name":"APIError","message":"too many requests"}),
        )
        .await;
        return;
    }
    if path == "/event" {
        let headers =
            "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\n";
        let _ = stream.write_all(headers.as_bytes()).await;
        if matches!(mode, MockMode::Pending) {
            let _ = stream
                .write_all(b"data: {\"id\":\"evt_connected\",\"type\":\"server.connected\",\"properties\":{}}\n\n")
                .await;
            tokio::time::sleep(Duration::from_secs(5)).await;
            return;
        }
        if matches!(mode, MockMode::OversizedSseLine) {
            let oversized = format!("data: \"{}\"\n\n", "x".repeat(70 * 1024));
            let _ = stream.write_all(oversized.as_bytes()).await;
            return;
        }
        if matches!(mode, MockMode::OversizedSseEvent) {
            for _ in 0..5 {
                let line = format!("data: {}\n", " ".repeat(60 * 1024));
                let _ = stream.write_all(line.as_bytes()).await;
            }
            let _ = stream.write_all(b"data: {\"id\":\"evt_large\",\"type\":\"server.connected\",\"properties\":{}}\n\n").await;
            return;
        }
        let sse = match mode {
            MockMode::MalformedSse => "data: {not-json}\n\n".to_owned(),
            _ => {
                let mut admitted = None;
                for _ in 0..200 {
                    admitted = admitted_message_id.lock().await.clone();
                    if admitted.is_some() {
                        break;
                    }
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
                contract_sse(admitted.as_deref().unwrap_or("msg_user_1"))
            }
        };
        let _ = stream.write_all(sse.as_bytes()).await;
        return;
    }

    if method == "POST" && path.ends_with("/prompt_async") {
        *admitted_message_id.lock().await = body
            .get("messageID")
            .and_then(Value::as_str)
            .map(str::to_owned);
    }
    let response = match (method.as_str(), path.as_str()) {
        ("GET", "/global/health") => json!({"healthy":true,"version":"1.18.15"}),
        ("POST", "/session") => session_json(),
        ("GET", path) if path == format!("/session/{SESSION_ID}") => {
            if matches!(mode, MockMode::MalformedSession) {
                json!({"id":7})
            } else if matches!(mode, MockMode::SparseSession) {
                json!({"id":SESSION_ID})
            } else {
                session_json()
            }
        }
        ("DELETE", path) if path == format!("/session/{SESSION_ID}") => json!(true),
        ("POST", path) if path == format!("/session/{SESSION_ID}/message") => {
            if matches!(mode, MockMode::MalformedPrompt) {
                json!({"info":{"role":"assistant"},"parts":"wrong"})
            } else if matches!(mode, MockMode::SparsePrompt) {
                json!({
                    "info":{
                        "sessionID":SESSION_ID,
                        "role":"assistant",
                        "cost":0,
                        "tokens":{"input":0,"output":0,"reasoning":0,"cache":{"read":0,"write":0}}
                    },
                    "parts":[]
                })
            } else {
                prompt_json()
            }
        }
        ("POST", path) if path == format!("/session/{SESSION_ID}/abort") => json!(true),
        ("POST", path) if path == format!("/session/{SESSION_ID}/prompt_async") => {
            write_empty(&mut stream, 204).await;
            return;
        }
        _ => {
            write_json(&mut stream, 404, json!({"name":"NotFoundError"})).await;
            return;
        }
    };
    write_json(&mut stream, 200, response).await;
}

async fn read_request(stream: &mut TcpStream) -> Option<(String, String, Value)> {
    let mut bytes = Vec::new();
    let header_end;
    loop {
        let mut buffer = [0_u8; 1024];
        let read = stream.read(&mut buffer).await.ok()?;
        if read == 0 {
            return None;
        }
        bytes.extend_from_slice(&buffer[..read]);
        if let Some(offset) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
            header_end = offset + 4;
            break;
        }
    }
    let headers = String::from_utf8(bytes[..header_end].to_vec()).ok()?;
    let request_line = headers.lines().next()?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next()?.to_owned();
    let path = request_parts.next()?.split('?').next()?.to_owned();
    let content_length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .unwrap_or(0);
    while bytes.len() < header_end + content_length {
        let mut buffer = [0_u8; 1024];
        let read = stream.read(&mut buffer).await.ok()?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    let body = if content_length == 0 {
        Value::Null
    } else {
        serde_json::from_slice(&bytes[header_end..header_end + content_length]).ok()?
    };
    Some((method, path, body))
}

async fn write_json(stream: &mut TcpStream, status: u16, value: Value) {
    let body = serde_json::to_vec(&value).unwrap();
    let reason = match status {
        200 => "OK",
        429 => "Too Many Requests",
        _ => "Error",
    };
    let headers = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(headers.as_bytes()).await;
    let _ = stream.write_all(&body).await;
}

async fn write_empty(stream: &mut TcpStream, status: u16) {
    let response =
        format!("HTTP/1.1 {status} No Content\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
    let _ = stream.write_all(response.as_bytes()).await;
}

fn session_json() -> Value {
    json!({
        "id": SESSION_ID,
        "slug":"contract",
        "projectID":"global",
        "directory":"/workspace",
        "title":"Task 8 contract",
        "version":"1.18.15",
        "time":{"created":1,"updated":1},
        "model": {
            "providerID": OPENCODE_PROVIDER_ID,
            "id":"gpt-5.6-luna",
            "variant":"max"
        }
    })
}

fn prompt_json() -> Value {
    json!({
        "info": {
            "id":"msg_assistant_1",
            "sessionID":SESSION_ID,
            "role":"assistant",
            "time":{"created":1,"completed":2},
            "parentID":"msg_user_1",
            "modelID":"gpt-5.6-luna",
            "providerID":OPENCODE_PROVIDER_ID,
            "mode":"build",
            "agent":"build",
            "path":{"cwd":"/workspace","root":"/workspace"},
            "cost":0.125,
            "tokens":{
                "total":6,
                "input":3,
                "output":2,
                "reasoning":1,
                "cache":{"read":0,"write":0}
            },
            "finish":"stop"
        },
        "parts":[
            {"id":"prt_reasoning","sessionID":SESSION_ID,"messageID":"msg_assistant_1","type":"reasoning","text":"carefully","time":{"start":1,"end":1}},
            {"id":"prt_text","sessionID":SESSION_ID,"messageID":"msg_assistant_1","type":"text","text":"hello world"}
        ]
    })
}

fn assistant_message_event(
    event_id: &str,
    assistant_message_id: &str,
    parent_id: &str,
    completed: bool,
) -> Value {
    let mut info = prompt_json()["info"].clone();
    info["id"] = json!(assistant_message_id);
    info["parentID"] = json!(parent_id);
    if !completed {
        info["time"].as_object_mut().unwrap().remove("completed");
        info.as_object_mut().unwrap().remove("finish");
    }
    json!({
        "id":event_id,
        "type":"message.updated",
        "properties":{"sessionID":SESSION_ID,"info":info}
    })
}

fn assistant_error_event(error: Value) -> Value {
    let mut event = assistant_message_event(
        "evt_assistant_error",
        "msg_error_assistant",
        "msg_error_user",
        false,
    );
    event["properties"]["info"]["error"] = error;
    event
}

fn text_delta_event(event_id: &str, assistant_message_id: &str, delta: &str) -> Value {
    json!({
        "id":event_id,
        "type":"message.part.delta",
        "properties":{
            "sessionID":SESSION_ID,
            "messageID":assistant_message_id,
            "partID":format!("prt_{event_id}"),
            "field":"text",
            "delta":delta
        }
    })
}

fn contract_sse(parent_message_id: &str) -> String {
    let mut final_info = prompt_json()["info"].clone();
    final_info["parentID"] = json!(parent_message_id);
    let events = [
        json!({"id":"evt_connected","type":"server.connected","properties":{}}),
        assistant_message_event(
            "evt_message_started",
            "msg_assistant_1",
            parent_message_id,
            false,
        ),
        json!({
            "id":"evt_text",
            "type":"message.part.delta",
            "properties":{
                "sessionID":SESSION_ID,
                "messageID":"msg_assistant_1",
                "partID":"prt_text",
                "field":"text",
                "delta":"hello world",
                "apiKey":SECRET_CANARY,
                "detail":format!("provider echoed {SECRET_CANARY}")
            }
        }),
        json!({
            "id":"evt_reasoning",
            "type":"message.part.delta",
            "properties":{
                "sessionID":SESSION_ID,
                "messageID":"msg_assistant_1",
                "partID":"prt_reasoning",
                "field":"reasoning",
                "delta":"carefully"
            }
        }),
        json!({
            "id":"evt_message",
            "type":"message.updated",
            "properties":{"sessionID":SESSION_ID,"info":final_info}
        }),
        json!({
            "id":"evt_idle",
            "type":"session.idle",
            "properties":{"sessionID":SESSION_ID}
        }),
    ];
    events
        .into_iter()
        .map(|event| format!("data: {event}\n\n"))
        .collect()
}
