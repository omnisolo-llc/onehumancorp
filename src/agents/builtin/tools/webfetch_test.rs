use super::*;
use axum::{
    Router,
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    routing::get,
};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn execute_webfetch(url: String) -> Result<String, ToolError> {
    let tool = webfetch_tool();
    crate::ToolExecutor::execute(&*tool.execute, json!({"url": url})).await
}

async fn counted_response(State(count): State<Arc<AtomicUsize>>) -> &'static str {
    count.fetch_add(1, Ordering::SeqCst);
    "private response"
}

async fn redirect_chain(Path(step): Path<usize>) -> impl IntoResponse {
    if step < 6 {
        Redirect::temporary(&format!("/redirect/{next}", next = step + 1)).into_response()
    } else {
        "done".into_response()
    }
}

async fn five_redirect_chain(Path(step): Path<usize>) -> impl IntoResponse {
    if step < 5 {
        Redirect::temporary(&format!("/five/{next}", next = step + 1)).into_response()
    } else {
        "done".into_response()
    }
}

#[tokio::test]
async fn webfetch_rejects_private_addresses_without_sending_request() {
    temp_env::async_with_vars([("OHC_AGENT_ALLOW_PRIVATE_NETWORK", None::<&str>)], async {
        let count = Arc::new(AtomicUsize::new(0));
        let app = Router::new()
            .route("/", get(counted_response))
            .with_state(count.clone());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let result = execute_webfetch(format!("http://{address}/")).await;
        server.abort();

        assert!(result.is_err());
        assert_eq!(count.load(Ordering::SeqCst), 0);
        assert!(result.unwrap_err().to_string().contains("network policy"));
    })
    .await;
}

#[tokio::test]
async fn webfetch_rejects_oversized_chunked_responses() {
    temp_env::async_with_vars(
        [("OHC_AGENT_ALLOW_PRIVATE_NETWORK", Some("true"))],
        async {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server = tokio::spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = [0_u8; 4096];
                let _ = stream.read(&mut request).await;
                let _ = stream
                    .write_all(
                        b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n",
                    )
                    .await;
                for chunk in [vec![b'a'; 600 * 1024], vec![b'b'; 600 * 1024]] {
                    let _ = stream
                        .write_all(format!("{:X}\r\n", chunk.len()).as_bytes())
                        .await;
                    let _ = stream.write_all(&chunk).await;
                    let _ = stream.write_all(b"\r\n").await;
                }
                let _ = stream.write_all(b"0\r\n\r\n").await;
            });

            let result = execute_webfetch(format!("http://{address}/large")).await;
            server.abort();

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("exceeds 1 MiB"));
        },
    )
    .await;
}

#[tokio::test]
async fn webfetch_rejects_more_than_five_redirects() {
    temp_env::async_with_vars([("OHC_AGENT_ALLOW_PRIVATE_NETWORK", Some("true"))], async {
        let app = Router::new().route("/redirect/{step}", get(redirect_chain));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let result = execute_webfetch(format!("http://{address}/redirect/0")).await;
        server.abort();

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("too many redirects")
        );
    })
    .await;
}

#[tokio::test]
async fn webfetch_rejects_redirect_targets_on_loopback() {
    let current = Url::parse("https://example.com/start").unwrap();
    let result = validated_redirect_target(&current, "http://127.0.0.1/admin", false).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("network policy"));
}

#[tokio::test]
async fn webfetch_allows_exactly_five_redirects() {
    temp_env::async_with_vars([("OHC_AGENT_ALLOW_PRIVATE_NETWORK", Some("true"))], async {
        let app = Router::new().route("/five/{step}", get(five_redirect_chain));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let result = execute_webfetch(format!("http://{address}/five/0"))
            .await
            .unwrap();
        server.abort();

        assert_eq!(result, "done");
    })
    .await;
}

#[tokio::test]
async fn webfetch_truncates_multibyte_text_on_a_character_boundary() {
    temp_env::async_with_vars([("OHC_AGENT_ALLOW_PRIVATE_NETWORK", Some("true"))], async {
        let body = "a".to_string() + &"🧪".repeat(10_001);
        let app = Router::new().route("/unicode", get(move || async move { body.clone() }));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });

        let result = execute_webfetch(format!("http://{address}/unicode"))
            .await
            .unwrap();
        server.abort();

        assert!(result.ends_with("... (truncated)"));
        assert_eq!(
            result.trim_end_matches("... (truncated)").chars().count(),
            10_000
        );
    })
    .await;
}

#[tokio::test]
async fn test_webfetch_strip_html() {
    let html =
        "<html><head><title>Test</title></head><body><h1>Hello</h1><p>World</p></body></html>";
    let text = strip_html(html);
    assert_eq!(text, "Test Hello World");
}

#[test]
fn test_strip_html_complex() {
    let html = r#"
    <html>
        <head><title>Title</title></head>
        <body>
            <p>Paragraph 1</p>
            <div>
                <a href='x'>Link</a>
                <span>Text</span>
            </div>
            <script>let x = 1;</script>
        </body>
    </html>
    "#;
    let text = strip_html(html);
    assert!(text.contains("Paragraph 1"));
    assert!(text.contains("Link"));
    assert!(text.contains("Text"));
    assert!(text.contains("let x = 1;"));
}

#[tokio::test]
async fn test_webfetch_tool_validation() {
    let tool = webfetch_tool();

    // Testing missing required parameter via validation.
    let invalid_args = json!({});
    let res = crate::ToolExecutor::execute(&*tool.execute, invalid_args).await;

    assert!(res.is_err());
    let err_msg = res.unwrap_err().to_string();
    assert!(err_msg.contains("Validation Error (Pydantic-first tool schema)"));
}
