use std::collections::BTreeSet;
use std::time::Duration;

use serde_json::{Value, json};
use server_harness::middleware::harness::{
    HarnessAdapter, HarnessDescriptor, HarnessIntegrationMode, HarnessProtocolKind,
    HarnessSessionRequest, OmniSoloHarnessAdapterBridge,
};
use server_harness::middleware::inference::OpenAiResponsesClient;
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use uuid::Uuid;

const SECRET_CANARY: &str = "omnisolo-provider-secret-canary";

fn selection() -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: "gpt-5.6-luna".to_owned(),
        reasoning_effort: Some(ReasoningEffort::Max),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: None,
        max_output_tokens: Some(128_000),
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "binding-v1".to_owned(),
        binding_digest: "sha256:test-binding".to_owned(),
        metadata: Default::default(),
    }
}

fn descriptor() -> HarnessDescriptor {
    HarnessDescriptor {
        harness_id: "omnisolo".to_owned(),
        adapter_id: "omnisolo.native".to_owned(),
        display_name: "OmniSolo harness".to_owned(),
        implementation_version: "test".to_owned(),
        protocol_kind: HarnessProtocolKind::OmniSolo,
        integration_mode: HarnessIntegrationMode::Native,
        protocol_version: "omnisolo.worker.v1".to_owned(),
        source_revision: None,
        schema_revision: Some("omnisolo.session.v2".to_owned()),
        capabilities: BTreeSet::new(),
        worker_image: None,
        worker_pool: Some("omnisolo".to_owned()),
        state_locality: Some("test".to_owned()),
        native_extension_namespace: "omnisolo".to_owned(),
        metadata: Default::default(),
    }
}

#[tokio::test]
async fn responses_transport_routes_model_and_effort_without_persisting_the_key() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        let mut buffer = [0_u8; 4096];
        loop {
            let read = stream.read(&mut buffer).await.unwrap();
            assert!(read > 0, "request ended before its JSON body arrived");
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
            if request.len() >= headers_end + 4 + content_length {
                break;
            }
        }
        let headers_end = request
            .windows(4)
            .position(|part| part == b"\r\n\r\n")
            .unwrap();
        let headers = String::from_utf8_lossy(&request[..headers_end]);
        assert!(headers.starts_with("POST /v1/responses HTTP/1.1"));
        assert!(
            headers
                .to_ascii_lowercase()
                .contains(&format!("authorization: bearer {SECRET_CANARY}"))
        );
        let body: Value = serde_json::from_slice(&request[headers_end + 4..]).unwrap();
        assert_eq!(body["model"], "gpt-5.6-luna");
        assert_eq!(body["reasoning"]["effort"], "max");
        assert_eq!(body["input"], "Return the transport marker only");
        assert_eq!(body["max_output_tokens"], 128_000);
        assert_eq!(body["stream"], false);

        let response = json!({
            "id": "resp_transport_1",
            "status": "completed",
            "model": "gpt-5.6-luna",
            "output": [{
                "type": "message",
                "role": "assistant",
                "content": [{"type": "output_text", "text": "omnisolo-transport-ok"}]
            }],
            "usage": {
                "input_tokens": 7,
                "output_tokens": 4,
                "total_tokens": 11,
                "input_tokens_details": {"cached_tokens": 2}
            }
        });
        let response = serde_json::to_vec(&response).unwrap();
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
    });

    let client = OpenAiResponsesClient::new(
        format!("http://{address}/v1"),
        SECRET_CANARY,
        Duration::from_secs(2),
    )
    .unwrap();
    assert!(!format!("{client:?}").contains(SECRET_CANARY));
    let result = client
        .execute(&selection(), "Return the transport marker only")
        .await
        .unwrap();
    assert_eq!(result.response_id, "resp_transport_1");
    assert_eq!(result.model, "gpt-5.6-luna");
    assert_eq!(result.text, "omnisolo-transport-ok");
    assert_eq!(result.usage.input_tokens, 7);
    assert_eq!(result.usage.output_tokens, 4);
    assert_eq!(result.usage.cached_tokens, 2);
    assert_eq!(result.binding_revision, "binding-v1");
    assert_eq!(result.binding_digest, "sha256:test-binding");
    assert!(
        !serde_json::to_string(&result)
            .unwrap()
            .contains(SECRET_CANARY)
    );
    server.await.unwrap();
}

#[tokio::test]
async fn omnisolo_bridge_executes_provider_inference_and_preserves_binding_provenance() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
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
            let length = headers
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length:")
                        .map(str::trim)
                        .and_then(|value| value.parse::<usize>().ok())
                })
                .unwrap();
            if request.len() >= headers_end + 4 + length {
                break;
            }
        }
        let response = serde_json::to_vec(&json!({
            "id": "resp_bridge_1",
            "status": "completed",
            "model": "gpt-5.6-luna",
            "output": [{
                "type": "message",
                "role": "assistant",
                "content": [{"type": "output_text", "text": "omnisolo-bridge-ok"}]
            }],
            "usage": {"input_tokens": 5, "output_tokens": 3, "total_tokens": 8}
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
    });

    let client = OpenAiResponsesClient::new(
        format!("http://{address}/v1"),
        SECRET_CANARY,
        Duration::from_secs(2),
    )
    .unwrap();
    let mut bridge = OmniSoloHarnessAdapterBridge::with_provider_client(descriptor(), client);
    let request = HarnessSessionRequest::new("tenant", Uuid::new_v4(), Uuid::new_v4())
        .with_task(Uuid::new_v4(), "provider-backed objective")
        .with_resolved_model(selection());
    let native = bridge.create_session(request.clone()).await.unwrap();
    let execution = bridge
        .execute(
            request,
            "attempt-provider-1",
            "Return the bridge marker only",
            Some(&native.native_session_id),
        )
        .await
        .unwrap();

    assert_eq!(execution.final_text.as_deref(), Some("omnisolo-bridge-ok"));
    assert_eq!(execution.usage.as_ref().unwrap()["input_tokens"], 5);
    assert_eq!(execution.usage.as_ref().unwrap()["output_tokens"], 3);
    assert!(execution.events.iter().any(|event| {
        event.event_type == "assistant.text_chunk"
            && event.payload["content"] == "omnisolo-bridge-ok"
    }));
    let binding = execution
        .events
        .iter()
        .find(|event| event.event_type == "inference.model_binding")
        .unwrap();
    assert!(binding.durable);
    assert_eq!(binding.payload["model_id"], "gpt-5.6-luna");
    assert_eq!(binding.payload["binding_revision"], "binding-v1");
    assert_eq!(binding.payload["binding_digest"], "sha256:test-binding");
    assert!(
        !serde_json::to_string(&execution)
            .unwrap()
            .contains(SECRET_CANARY)
    );
    server.await.unwrap();
}
