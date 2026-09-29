use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use serde_json::{Value, json};
use server_harness::middleware::harness::{
    HarnessAdapter, HarnessProtocolKind, HarnessSessionRequest, ProcessHarnessAdapter,
    ProcessHarnessSpec,
};
use server_harness::middleware::protocol::{AttemptOperation, HarnessExecutionItem};
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use uuid::Uuid;

fn resolved_model() -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(ReasoningEffort::Max),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: None,
        max_output_tokens: None,
        capabilities: Default::default(),
        binding_revision: "shim-test".to_owned(),
        binding_digest: "sha256:shim-test".to_owned(),
        metadata: Default::default(),
    }
}

async fn responses_provider() -> (SocketAddr, tokio::task::JoinHandle<(String, String, Value)>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let read = stream.read(&mut buffer).await.unwrap();
            assert!(read > 0);
            request.extend_from_slice(&buffer[..read]);
            let Some(headers_end) = request.windows(4).position(|part| part == b"\r\n\r\n") else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..headers_end]);
            let content_length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .map(str::trim)
                        .and_then(|value| value.parse::<usize>().ok())
                })
                .unwrap();
            if request.len() < headers_end + 4 + content_length {
                continue;
            }
            let mut lines = headers.lines();
            let request_line = lines.next().unwrap_or_default().to_owned();
            let authorization = lines
                .find_map(|line| {
                    line.strip_prefix("Authorization:")
                        .or_else(|| line.strip_prefix("authorization:"))
                        .map(|value| value.trim().to_owned())
                })
                .unwrap_or_default();
            let body: Value =
                serde_json::from_slice(&request[headers_end + 4..headers_end + 4 + content_length])
                    .unwrap();
            let response = serde_json::to_vec(&json!({
                "id": "shim-response-1",
                "status": "completed",
                "model": "gpt-5.6-luna",
                "output": [{
                    "type": "message",
                    "role": "assistant",
                    "content": [{"type": "output_text", "text": "shim-provider-ok"}]
                }],
                "usage": {
                    "input_tokens": 5,
                    "output_tokens": 3,
                    "total_tokens": 8
                }
            }))
            .unwrap();
            stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        response.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
            stream.write_all(&response).await.unwrap();
            return (request_line, authorization, body);
        }
    });
    (address, task)
}

#[tokio::test]
async fn openai_compatible_shim_dispatches_sessions_and_prompts_over_the_facade() {
    let (provider_address, provider) = responses_provider().await;
    let secret = "shim-scoped-token-canary";
    let shim = Path::new(env!("CARGO_MANIFEST_DIR")).join("sidecars/openai_compatible_shim.py");
    let fixture_dir = std::env::temp_dir().join(format!("omnisolo-shim-cli-{}", Uuid::new_v4()));
    std::fs::create_dir_all(&fixture_dir).unwrap();
    let fixture = fixture_dir.join("aider");
    std::fs::write(&fixture, r#"#!/usr/bin/env python3
import json, os, sys, urllib.request
prompt = sys.stdin.read()
assert '--message-file' in sys.argv
request = urllib.request.Request(os.environ['OPENAI_API_BASE_URL'] + '/responses',
    data=json.dumps({'model':os.environ['OPENAI_MODEL'], 'input':prompt}).encode(),
    headers={'Authorization':'Bearer ' + os.environ['OPENAI_API_KEY'], 'Content-Type':'application/json'})
response = json.load(urllib.request.urlopen(request))
print(response['output'][0]['content'][0]['text'])
"#).unwrap();
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&fixture, std::fs::Permissions::from_mode(0o755)).unwrap();
    let selection = resolved_model();
    let request = HarnessSessionRequest::new("tenant-shim", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "exercise the OpenAI-compatible shim")
        .with_resolved_model(selection.clone());
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command(
            "python3",
            ["-u".to_owned(), shim.to_string_lossy().into_owned()],
            "aider",
        )
        .with_protocol(HarnessProtocolKind::OpenAiCompatibleShim)
        .with_model_routing(selection, Some(format!("http://{provider_address}/v1")))
        .with_environment(
            "PATH",
            format!(
                "{}:{}",
                fixture_dir.display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .with_environment("OPENAI_API_KEY", secret)
        .with_environment(
            "OPENAI_API_BASE_URL",
            format!("http://{provider_address}/v1"),
        )
        .with_environment("OPENAI_MODEL", "gpt-5.6-luna")
        .with_environment("OPENAI_REASONING_EFFORT", "max")
        .with_timeout(Duration::from_secs(3)),
    );

    let native = adapter.create_session(request.clone()).await.unwrap();
    assert_eq!(
        native.native_session_id,
        format!("shim:{}", request.session_id)
    );
    let mut stream = adapter
        .attempt_stream(
            AttemptOperation::Execute,
            request,
            "shim-attempt-1",
            "Answer through the shared provider",
            Some(&native.native_session_id),
        )
        .await
        .unwrap();
    let mut events = Vec::new();
    let mut final_text = None;
    let mut usage = None;
    while let Some(item) = stream.next().await {
        match item.unwrap() {
            HarnessExecutionItem::Event(event) => events.push(event),
            HarnessExecutionItem::Completed {
                final_text: completed_text,
                usage: completed_usage,
            } => {
                final_text = completed_text;
                usage = completed_usage;
            }
        }
    }
    assert_eq!(final_text.as_deref(), Some("shim-provider-ok"));
    assert_eq!(events[0].payload["integration_mode"], "openai_compatible");
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "inference.model_binding")
            .count(),
        1
    );
    assert!(events.iter().any(|event| {
        event.event_type == "assistant.final" && event.payload["text"] == "shim-provider-ok"
    }));
    assert_eq!(usage.as_ref().unwrap()["total_tokens"], 8);
    let (request_line, authorization, body) = provider.await.unwrap();
    assert_eq!(request_line, "POST /v1/responses HTTP/1.1");
    assert_eq!(authorization, format!("Bearer {secret}"));
    assert_eq!(body["model"], "gpt-5.6-luna");
    assert_eq!(body["input"], "Answer through the shared provider");
    std::fs::remove_dir_all(fixture_dir).unwrap();
    assert!(
        !events
            .iter()
            .any(|event| event.payload.to_string().contains(secret))
    );
}

#[cfg(unix)]
#[tokio::test]
async fn dropping_shim_attempt_reaps_cli_and_removes_its_home() {
    use std::os::unix::fs::PermissionsExt;
    let directory = std::env::temp_dir().join(format!("shim-drop-{}", Uuid::new_v4()));
    std::fs::create_dir_all(&directory).unwrap();
    let fixture = directory.join("aider");
    std::fs::write(&fixture, r#"#!/usr/bin/env python3
import json, os, pathlib, time
pathlib.Path(__file__).with_suffix('.state').write_text(json.dumps({'pid':os.getpid(),'home':os.environ['HOME']}))
time.sleep(60)
"#).unwrap();
    std::fs::set_permissions(&fixture, std::fs::Permissions::from_mode(0o755)).unwrap();
    let shim = Path::new(env!("CARGO_MANIFEST_DIR")).join("sidecars/openai_compatible_shim.py");
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("python3", [shim.to_string_lossy().into_owned()], "aider")
            .with_protocol(HarnessProtocolKind::OpenAiCompatibleShim)
            .with_environment(
                "PATH",
                format!("{}:{}", directory.display(), std::env::var("PATH").unwrap()),
            )
            .with_environment("OPENAI_API_KEY", "drop-fixture-token")
            .with_environment("OPENAI_API_BASE_URL", "http://127.0.0.1:1/v1")
            .with_environment("OPENAI_MODEL", "gpt-5.6-luna")
            .with_timeout(Duration::from_secs(90)),
    );
    let attempt = tokio::spawn(async move {
        let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
        adapter.execute(request, "drop-attempt", "wait", None).await
    });
    let state = fixture.with_extension("state");
    tokio::time::timeout(Duration::from_secs(5), async {
        while !state.exists() {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .unwrap();
    let state: Value = serde_json::from_slice(&std::fs::read(state).unwrap()).unwrap();
    attempt.abort();
    let _ = attempt.await;
    tokio::time::timeout(Duration::from_secs(5), async {
        while Path::new(state["home"].as_str().unwrap()).exists()
            || Path::new(&format!("/proc/{}", state["pid"])).exists()
        {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("dropping adapter must reap CLI and remove temporary home");
    std::fs::remove_dir_all(directory).unwrap();
}

#[cfg(unix)]
#[tokio::test]
async fn shim_stream_admits_prompt_without_blocking_and_cancel_reaps_cli() {
    use std::os::unix::fs::PermissionsExt;
    let directory = std::env::temp_dir().join(format!("shim-drop-{}", Uuid::new_v4()));
    std::fs::create_dir_all(&directory).unwrap();
    let fixture = directory.join("aider");
    std::fs::write(&fixture, r#"#!/usr/bin/env python3
import json, os, pathlib, time
pathlib.Path(__file__).with_suffix('.state').write_text(json.dumps({'pid':os.getpid(),'home':os.environ['HOME']}))
time.sleep(60)
"#).unwrap();
    std::fs::set_permissions(&fixture, std::fs::Permissions::from_mode(0o755)).unwrap();
    let shim = Path::new(env!("CARGO_MANIFEST_DIR")).join("sidecars/openai_compatible_shim.py");
    let mut adapter = ProcessHarnessAdapter::new(
        ProcessHarnessSpec::command("python3", [shim.to_string_lossy().into_owned()], "aider")
            .with_protocol(HarnessProtocolKind::OpenAiCompatibleShim)
            .with_environment(
                "PATH",
                format!("{}:{}", directory.display(), std::env::var("PATH").unwrap()),
            )
            .with_environment("OPENAI_API_KEY", "drop-fixture-token")
            .with_environment("OPENAI_API_BASE_URL", "http://127.0.0.1:1/v1")
            .with_environment("OPENAI_MODEL", "gpt-5.6-luna")
            .with_timeout(Duration::from_secs(90)),
    );
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4());
    let stream = tokio::time::timeout(
        Duration::from_millis(500),
        adapter.attempt_stream(
            AttemptOperation::Execute,
            request.clone(),
            "cancel-attempt",
            "wait",
            None,
        ),
    )
    .await
    .expect("shim stream admission must release the adapter before CLI completion")
    .unwrap();
    let state = fixture.with_extension("state");
    tokio::time::timeout(Duration::from_secs(5), async {
        while !state.exists() {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .unwrap();
    let state: Value = serde_json::from_slice(&std::fs::read(state).unwrap()).unwrap();
    adapter
        .control_attempt(request, "cancel-attempt", "cancel", "", None)
        .await
        .unwrap();
    drop(stream);
    tokio::time::timeout(Duration::from_secs(5), async {
        while Path::new(state["home"].as_str().unwrap()).exists()
            || Path::new(&format!("/proc/{}", state["pid"])).exists()
        {
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
    .await
    .expect("dropping adapter must reap CLI and remove temporary home");
    std::fs::remove_dir_all(directory).unwrap();
}
