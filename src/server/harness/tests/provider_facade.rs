use std::sync::Arc;

use serde_json::{Value, json};
use server_harness::middleware::provider_facade::ProviderFacade;
use server_harness::middleware::types::{
    ModelApiDialect, ReasoningEffort, ResolvedModelSelection,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, oneshot};
use tokio::task::JoinHandle;

const UPSTREAM_SECRET: &str = "upstream-secret-canary";

fn selection(model_id: &str) -> ResolvedModelSelection {
    ResolvedModelSelection {
        provider_route: "openai-compatible".to_owned(),
        model_id: model_id.to_owned(),
        reasoning_effort: Some(ReasoningEffort::Max),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: None,
        max_output_tokens: Some(128),
        capabilities: Default::default(),
        binding_revision: "facade-test-v1".to_owned(),
        binding_digest: "sha256:facade-test".to_owned(),
        metadata: Default::default(),
    }
}

struct UpstreamFixture {
    base_url: String,
    authorizations: Arc<Mutex<Vec<String>>>,
    requests: Arc<Mutex<Vec<String>>>,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<()>>,
}

impl UpstreamFixture {
    async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let authorizations = Arc::new(Mutex::new(Vec::new()));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let authorizations_for_task = Arc::clone(&authorizations);
        let requests_for_task = Arc::clone(&requests);
        let (shutdown, mut shutdown_receiver) = oneshot::channel();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_receiver => break,
                    result = listener.accept() => {
                        let Ok((mut stream, _)) = result else { break };
                        let request = read_request(&mut stream).await.unwrap();
                        let (path, authorization, body) = parse_request(&request);
                        authorizations_for_task.lock().await.push(authorization);
                        requests_for_task.lock().await.push(path.clone());
                        let response_body = if path == "/v1/models" {
                            json!({"object":"list","data":[]}).to_string()
                        } else {
                            let model = body
                                .get("model")
                                .and_then(Value::as_str)
                                .unwrap_or("gpt-5.6-luna");
                            json!({
                                "id":"resp_facade_1",
                                "object":"response",
                                "status":"completed",
                                "model":model,
                                "output":[{"type":"message","content":[{"type":"output_text","text":"upstream response"}]}],
                                "usage":{"input_tokens":3,"output_tokens":2,"total_tokens":5}
                            }).to_string()
                        };
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            response_body.len(),
                            response_body
                        );
                        stream.write_all(response.as_bytes()).await.unwrap();
                    }
                }
            }
        });
        Self {
            base_url: format!("http://{address}/v1"),
            authorizations,
            requests,
            shutdown: Some(shutdown),
            task: Some(task),
        }
    }

    fn url(&self) -> &str {
        &self.base_url
    }

    async fn last_authorization(&self) -> Option<String> {
        self.authorizations.lock().await.last().cloned()
    }

    async fn request_count(&self) -> usize {
        self.requests.lock().await.len()
    }

    async fn shutdown(mut self) {
        let _ = self.shutdown.take().unwrap().send(());
        self.task.take().unwrap().await.unwrap();
    }
}

async fn read_request(stream: &mut tokio::net::TcpStream) -> std::io::Result<Vec<u8>> {
    let mut request = Vec::new();
    let mut buffer = [0_u8; 4096];
    loop {
        let read = stream.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
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
                    .and_then(|value| value.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        if request.len() >= headers_end + 4 + content_length {
            break;
        }
    }
    Ok(request)
}

fn parse_request(request: &[u8]) -> (String, String, Value) {
    let headers_end = request
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .unwrap();
    let headers = String::from_utf8_lossy(&request[..headers_end]);
    let mut lines = headers.lines();
    let path = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or_default()
        .to_owned();
    let authorization = lines
        .find_map(|line| {
            line.strip_prefix("Authorization:")
                .or_else(|| line.strip_prefix("authorization:"))
                .map(str::trim)
                .unwrap_or_default()
                .to_owned()
                .into()
        })
        .unwrap_or_default();
    let body = serde_json::from_slice(&request[headers_end + 4..]).unwrap_or(Value::Null);
    (path, authorization, body)
}

#[tokio::test]
async fn facade_forwards_model_and_upstream_authorization_without_exposing_the_key() {
    let upstream = UpstreamFixture::start().await;
    let facade = ProviderFacade::start(upstream.url(), UPSTREAM_SECRET, selection("gpt-5.6-luna"))
        .await
        .unwrap();
    let response = reqwest::Client::new()
        .post(format!("{}/responses", facade.route().base_url()))
        .bearer_auth(facade.route().token())
        .json(&json!({
            "model":"gpt-5.6-luna",
            "input":"hello",
            "stream":false
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(
        upstream.last_authorization().await.as_deref(),
        Some("Bearer upstream-secret-canary")
    );
    assert_eq!(upstream.request_count().await, 1);
    let debug = format!("{facade:?}");
    assert!(!debug.contains(UPSTREAM_SECRET));
    assert!(!debug.contains(facade.route().token()));
    facade.shutdown().await.unwrap();
    upstream.shutdown().await;
}

#[tokio::test]
async fn facade_rejects_wrong_token_model_method_and_unallowlisted_route() {
    let upstream = UpstreamFixture::start().await;
    let facade = ProviderFacade::start(upstream.url(), UPSTREAM_SECRET, selection("gpt-5.6-luna"))
        .await
        .unwrap();
    let client = reqwest::Client::new();
    let wrong_token = client
        .post(format!("{}/responses", facade.route().base_url()))
        .bearer_auth("wrong-token")
        .json(&json!({"model":"gpt-5.6-luna","input":"hello"}))
        .send()
        .await
        .unwrap();
    assert_eq!(wrong_token.status(), reqwest::StatusCode::UNAUTHORIZED);

    let wrong_model = client
        .post(format!("{}/responses", facade.route().base_url()))
        .bearer_auth(facade.route().token())
        .json(&json!({"model":"other-model","input":"hello"}))
        .send()
        .await
        .unwrap();
    assert_eq!(wrong_model.status(), reqwest::StatusCode::BAD_REQUEST);

    let wrong_method = client
        .get(format!("{}/responses", facade.route().base_url()))
        .bearer_auth(facade.route().token())
        .send()
        .await
        .unwrap();
    assert_eq!(wrong_method.status(), reqwest::StatusCode::METHOD_NOT_ALLOWED);

    let admin = client
        .post(format!("{}/admin", facade.route().base_url()))
        .bearer_auth(facade.route().token())
        .send()
        .await
        .unwrap();
    assert_eq!(admin.status(), reqwest::StatusCode::NOT_FOUND);
    assert_eq!(upstream.request_count().await, 0);

    facade.shutdown().await.unwrap();
    upstream.shutdown().await;
}
