//! Real loopback HTTP contracts for the production provider implementations.
use super::*;
use axum::{Json, Router, body::Body, http::StatusCode, routing::post};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio_stream::StreamExt;

// The legacy production circuit breaker is process-wide. Provider and chaos
// contracts must share this guard before resetting or exercising it so a sibling
// fixture cannot reset failures or hide the current contract's actual request.
static LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
pub(crate) async fn isolated() -> tokio::sync::MutexGuard<'static, ()> {
    let guard = LOCK.lock().await;
    get_circuit_breaker().reset_for_tests();
    guard
}
pub(crate) struct HttpFixture {
    base: String,
    observed: Arc<Mutex<Vec<Value>>>,
    server: tokio::task::JoinHandle<()>,
}
impl Drop for HttpFixture {
    fn drop(&mut self) {
        self.server.abort();
    }
}
impl HttpFixture {
    fn minimax(&self) -> MinimaxClient {
        let mut client = MinimaxClient::new("public-local-fixture-credential".into());
        client.url = format!("{}/v1/chat/completions", self.base);
        client.embed_url = format!("{}/v1/embeddings", self.base);
        client
    }
    pub(crate) fn local(&self) -> LocalLLMClient {
        let mut client = LocalLLMClient::new();
        client.endpoint = format!("{}/api/generate", self.base);
        client.embed_endpoint = format!("{}/api/embeddings", self.base);
        client.model = "fixture-model".into();
        client
    }

    pub(crate) fn request_count(&self) -> usize {
        self.observed.lock().unwrap().len()
    }
}
pub(crate) async fn http_fixture(status: StatusCode, body: Value) -> HttpFixture {
    response_fixture(status, vec![serde_json::to_vec(&body).unwrap()]).await
}
async fn response_fixture(status: StatusCode, chunks: Vec<Vec<u8>>) -> HttpFixture {
    let observed = Arc::new(Mutex::new(Vec::new()));
    let seen = observed.clone();
    let app = Router::new().fallback(post(move |Json(request): Json<Value>| {
        seen.lock().unwrap().push(request);
        let stream = tokio_stream::iter(
            chunks
                .clone()
                .into_iter()
                .map(Ok::<_, std::convert::Infallible>),
        )
        .throttle(Duration::from_millis(2));
        async move { (status, Body::from_stream(stream)) }
    }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    HttpFixture {
        base,
        observed,
        server,
    }
}
fn completion(text: &str) -> Value {
    json!({"choices":[{"message":{"content":text},"finish_reason":"stop"}],
        "base_resp":{"status_code":0}})
}

#[tokio::test]
async fn sentinel_credentials_never_fabricate_text_streams_or_embeddings() {
    let _guard = isolated().await;
    for key in [
        "",
        " ",
        "fake-key",
        "dummy_key",
        "ci-builder",
        "mock-key",
        "test-key",
        "my-placeholder",
    ] {
        let client = MinimaxClient::new(key.into());
        assert!(
            client.reason("Maya bakery").await.is_err(),
            "sentinel {key:?} invented business data"
        );
        let chunks: Vec<_> = client.reason_stream("Maya bakery").await.collect().await;
        assert_eq!(chunks.len(), 1);
        assert!(
            chunks[0].is_err(),
            "sentinel {key:?} invented streamed data"
        );
        assert!(
            client.generate_embedding("Bakery").await.is_err(),
            "sentinel {key:?} invented an embedding"
        );
    }
}
#[test]
fn text_endpoint_uses_current_official_host() {
    assert_eq!(
        MinimaxClient::new(String::new()).url,
        "https://api.minimax.io/v1/chat/completions"
    );
}
#[tokio::test]
async fn actual_http_text_is_preserved_instead_of_test_build_synthesis() {
    let _guard = isolated().await;
    let fixture = http_fixture(StatusCode::OK, completion("Actual local HTTP answer")).await;
    let client = fixture.minimax();
    for _ in 0..2 {
        assert_eq!(
            client.reason("Maya owns a bakery").await.unwrap(),
            "Actual local HTTP answer"
        );
    }
    let observed = fixture.observed.lock().unwrap();
    assert_eq!(
        observed.len(),
        1,
        "real completed response should be cached"
    );
    assert_eq!(observed[0]["stream"], false);
    assert!(
        observed[0]["messages"][0]["content"]
            .as_str()
            .unwrap()
            .contains("Maya")
    );
}
#[tokio::test]
async fn incomplete_empty_and_provider_error_completions_are_rejected() {
    let _guard = isolated().await;
    for body in [
        json!({"choices":[]}),
        json!({"choices":[{"message":{"content":"partial"},"finish_reason":"length"}]}),
        json!({"choices":[{"message":{"content":"partial"}}]}),
        completion(""),
        completion("   "),
        json!({"choices":[{"message":{"content":"bogus"},"finish_reason":"stop"}],"base_resp":{"status_code":1000}}),
    ] {
        get_circuit_breaker().reset_for_tests();
        let fixture = http_fixture(StatusCode::OK, body.clone()).await;
        assert!(
            fixture.minimax().reason("input").await.is_err(),
            "accepted incomplete/error response {body}"
        );
        assert!(
            !fixture.observed.lock().unwrap().is_empty(),
            "must exercise actual transport"
        );
    }
}
#[tokio::test]
async fn stream_preserves_http_text_across_utf8_and_event_boundaries() {
    let _guard = isolated().await;
    let sse = concat!(
        ": heartbeat\r\n\r\n",
        "data: {\"choices\":[{\"delta\":{\"role\":\"assistant\"},\"finish_reason\":null}]}\r\n\r\n",
        "data: {\"choices\":[{\"delta\":{\"content\":\"Café 世界\"},\"finish_reason\":null}]}\r\n\r\n",
        "data: {\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\r\n\r\n",
        "data: [DONE]\r\n\r\n"
    );
    let chunks = sse.as_bytes().chunks(5).map(<[u8]>::to_vec).collect();
    let fixture = response_fixture(StatusCode::OK, chunks).await;
    let parts: Result<Vec<_>, _> = fixture
        .minimax()
        .reason_stream("Maya bakery")
        .await
        .collect()
        .await;
    assert_eq!(parts.unwrap().concat(), "Café 世界");
    let observed = fixture.observed.lock().unwrap();
    assert_eq!(observed.len(), 1);
    assert_eq!(observed[0]["stream"], true);
}
#[tokio::test]
async fn malformed_incomplete_empty_and_error_streams_are_not_success() {
    let _guard = isolated().await;
    for body in [
        "data: {broken}\n\n",
        "data: {\"choices\":[{\"delta\":{\"content\":\"partial\"},\"finish_reason\":null}]}\n\n",
        "data: {\"choices\":[{\"delta\":{\"content\":\"partial\"},\"finish_reason\":\"length\"}]}\n\ndata: [DONE]\n\n",
        "data: [DONE]\n\n",
        "data: {\"base_resp\":{\"status_code\":1000}}\n\n",
        "data: {\"error\":{\"message\":\"denied\"}}\n\n",
    ] {
        let fixture = response_fixture(StatusCode::OK, vec![body.as_bytes().to_vec()]).await;
        let parts: Vec<_> = fixture
            .minimax()
            .reason_stream("input")
            .await
            .collect()
            .await;
        assert!(
            parts.iter().any(Result::is_err),
            "accepted stream {body:?}: {parts:?}"
        );
        assert_eq!(fixture.observed.lock().unwrap().len(), 1);
    }
}
#[tokio::test]
async fn stream_http_errors_surface_without_fallback() {
    let _guard = isolated().await;
    let fixture = http_fixture(StatusCode::BAD_GATEWAY, json!({"error":"unavailable"})).await;
    let parts: Vec<_> = fixture
        .minimax()
        .reason_stream("input")
        .await
        .collect()
        .await;
    assert_eq!(parts.len(), 1);
    assert!(parts[0].is_err());
    assert_eq!(fixture.observed.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn minimax_embedding_uses_actual_provider_vector_and_request() {
    let _guard = isolated().await;
    let fixture = http_fixture(
        StatusCode::OK,
        json!({"vectors":[[0.75,-0.25,0.0]],"base_resp":{"status_code":0}}),
    )
    .await;
    assert_eq!(
        fixture
            .minimax()
            .generate_embedding("Actual input")
            .await
            .unwrap(),
        vec![0.75, -0.25, 0.0]
    );
    let observed = fixture.observed.lock().unwrap();
    assert_eq!(observed.len(), 1);
    assert_eq!(observed[0]["texts"], json!(["Actual input"]));
    assert_eq!(observed[0]["model"], "embo-01");
}
#[tokio::test]
async fn minimax_embedding_rejects_provider_errors_empty_and_invalid_vectors() {
    let _guard = isolated().await;
    for body in [
        json!({"vectors":[[0.5]],"base_resp":{"status_code":1000}}),
        json!({"vectors":[]}),
        json!({"vectors":[[]]}),
        json!({"vectors":[["not a number"]]}),
        json!({"vectors":[[1e100]]}),
    ] {
        get_circuit_breaker().reset_for_tests();
        let fixture = http_fixture(StatusCode::OK, body.clone()).await;
        assert!(
            fixture.minimax().generate_embedding("input").await.is_err(),
            "accepted {body}"
        );
        assert!(!fixture.observed.lock().unwrap().is_empty());
    }
}
#[tokio::test]
async fn local_text_requires_completed_nonempty_matching_provider_response() {
    let _guard = isolated().await;
    for body in [
        json!({"model":"fixture-model","response":"partial","done":false}),
        json!({"model":"foreign-model","response":"other","done":true,"done_reason":"stop"}),
        json!({"model":"fixture-model","response":"","done":true,"done_reason":"stop"}),
    ] {
        get_circuit_breaker().reset_for_tests();
        let fixture = http_fixture(StatusCode::OK, body.clone()).await;
        assert!(
            fixture.local().reason("input").await.is_err(),
            "accepted {body}"
        );
        assert!(!fixture.observed.lock().unwrap().is_empty());
    }
}
#[tokio::test]
async fn local_text_preserves_real_text_without_fabricating_usage() {
    let _guard = isolated().await;
    let fixture = http_fixture(
        StatusCode::OK,
        json!({"model":"fixture-model","response":"real answer","done":true,"done_reason":"stop"}),
    )
    .await;
    assert_eq!(
        fixture.local().reason("input").await.unwrap(),
        "real answer"
    );
    let observed = fixture
        .local()
        .reason_with_usage("input", 128)
        .await
        .unwrap();
    assert_eq!(observed.text, "real answer");
    assert_eq!(observed.counts, None);
    assert_eq!(observed.duration_ns, None);
}
#[tokio::test]
async fn local_embeddings_preserve_real_vectors_and_reject_malformed_values() {
    let _guard = isolated().await;
    let fixture = http_fixture(StatusCode::OK, json!({"embedding":[0.75,-0.25,0.0]})).await;
    assert_eq!(
        fixture.local().generate_embedding("input").await.unwrap(),
        vec![0.75, -0.25, 0.0]
    );
    assert_eq!(fixture.observed.lock().unwrap()[0]["prompt"], "input");
    for body in [
        json!({"embedding":[]}),
        json!({"embedding":["bad"]}),
        json!({"embedding":[1e100]}),
    ] {
        let fixture = http_fixture(StatusCode::OK, body).await;
        assert!(fixture.local().generate_embedding("input").await.is_err());
    }
}

#[tokio::test]
async fn completion_and_embedding_http_errors_cannot_invent_success() {
    let _guard = isolated().await;
    for status in [StatusCode::UNAUTHORIZED, StatusCode::INTERNAL_SERVER_ERROR] {
        get_circuit_breaker().reset_for_tests();
        let fixture = http_fixture(status, json!({"error":"unavailable"})).await;
        assert!(fixture.minimax().reason("input").await.is_err());
        assert!(!fixture.observed.lock().unwrap().is_empty());
        get_circuit_breaker().reset_for_tests();
        let fixture = http_fixture(status, json!({"error":"unavailable"})).await;
        assert!(fixture.minimax().generate_embedding("input").await.is_err());
        assert!(!fixture.observed.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn successful_http_bodies_are_bounded_before_json_decoding() {
    let _guard = isolated().await;
    let mut body = completion("actual answer");
    body["vectors"] = json!([[0.75]]);
    body["embedding"] = json!([0.75]);
    body["response"] = json!("local answer");
    body["model"] = json!("fixture-model");
    body["done"] = json!(true);
    body["padding"] = json!("x".repeat(10 * 1024 * 1024));
    for call in 0..4 {
        get_circuit_breaker().reset_for_tests();
        let fixture = http_fixture(StatusCode::OK, body.clone()).await;
        let rejected = match call {
            0 => fixture.minimax().reason("input").await.is_err(),
            1 => fixture.minimax().generate_embedding("input").await.is_err(),
            2 => fixture.local().reason("input").await.is_err(),
            _ => fixture.local().generate_embedding("input").await.is_err(),
        };
        assert!(rejected, "adapter {call} accepted oversized HTTP body");
        assert_eq!(fixture.observed.lock().unwrap().len(), 1);
    }
}
#[tokio::test]
async fn completed_text_has_an_output_size_bound() {
    let _guard = isolated().await;
    let fixture = http_fixture(StatusCode::OK, completion(&"x".repeat(8 * 1024 * 1024 + 1))).await;
    assert!(fixture.minimax().reason("input").await.is_err());
    assert_eq!(fixture.observed.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn embedding_dimension_and_vector_counts_are_bounded() {
    let _guard = isolated().await;
    let oversized = vec![0.5; 65_537];
    let fixture = http_fixture(
        StatusCode::OK,
        json!({"vectors":[oversized.clone()],"embedding":oversized}),
    )
    .await;
    assert!(fixture.minimax().generate_embedding("input").await.is_err());
    assert!(fixture.local().generate_embedding("input").await.is_err());
    get_circuit_breaker().reset_for_tests();
    let fixture = http_fixture(StatusCode::OK, json!({"vectors":[[0.75],[-0.25]]})).await;
    assert!(
        fixture
            .minimax()
            .generate_embedding("one input")
            .await
            .is_err()
    );
    assert_eq!(fixture.observed.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn embedding_input_must_be_nonempty_and_bounded_before_dispatch() {
    let _guard = isolated().await;
    for input in [String::new(), " ".into(), "x".repeat(1_048_577)] {
        let fixture = http_fixture(
            StatusCode::OK,
            json!({"vectors":[[0.75]],"embedding":[0.75]}),
        )
        .await;
        assert!(fixture.minimax().generate_embedding(&input).await.is_err());
        assert!(fixture.local().generate_embedding(&input).await.is_err());
        assert!(fixture.observed.lock().unwrap().is_empty());
    }
}
#[tokio::test]
async fn stream_rejects_multiple_choices_content_after_finish_and_missing_done_marker() {
    let _guard = isolated().await;
    for body in [
        concat!(
            "data: {\"choices\":[{\"index\":0,\"delta\":{\"content\":\"one\"},\"finish_reason\":\"stop\"},{\"index\":1,\"delta\":{\"content\":\"two\"},\"finish_reason\":\"stop\"}]}\n\n",
            "data: [DONE]\n\n"
        ),
        concat!(
            "data: {\"choices\":[{\"index\":1,\"delta\":{\"content\":\"wrong choice\"},\"finish_reason\":\"stop\"}]}\n\n",
            "data: [DONE]\n\n"
        ),
        concat!(
            "data: {\"choices\":[{\"delta\":{\"content\":\"one\"},\"finish_reason\":\"stop\"}]}\n\n",
            "data: {\"choices\":[{\"delta\":{\"content\":\"late\"},\"finish_reason\":null}]}\n\n",
            "data: [DONE]\n\n"
        ),
        "data: {\"choices\":[{\"delta\":{\"content\":\"one\"},\"finish_reason\":\"stop\"}]}\n\n",
    ] {
        let fixture = response_fixture(StatusCode::OK, vec![body.as_bytes().to_vec()]).await;
        let chunks: Vec<_> = fixture
            .minimax()
            .reason_stream("input")
            .await
            .collect()
            .await;
        assert!(
            chunks.iter().any(Result::is_err),
            "accepted ambiguous or incomplete stream: {chunks:?}"
        );
        assert_eq!(fixture.observed.lock().unwrap().len(), 1);
    }
}
#[tokio::test]
async fn failed_inference_is_never_automatically_dispatched_again() {
    let _guard = isolated().await;
    for call in 0..4 {
        get_circuit_breaker().reset_for_tests();
        let fixture = http_fixture(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({"error":"unknown outcome"}),
        )
        .await;
        let rejected = match call {
            0 => fixture.minimax().reason("input").await.is_err(),
            1 => fixture.minimax().generate_embedding("input").await.is_err(),
            2 => fixture.local().reason("input").await.is_err(),
            _ => fixture.local().generate_embedding("input").await.is_err(),
        };
        assert!(rejected);
        assert_eq!(
            fixture.observed.lock().unwrap().len(),
            1,
            "adapter {call} retried inference"
        );
    }
}
