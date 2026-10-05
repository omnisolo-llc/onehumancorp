use omnisolo_builtin_agent_core::types::ToolError;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use super::{
    Tool,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
};

const MAX_RESPONSE_BYTES: usize = 1_048_576;

#[derive(Deserialize)]
struct WebSearchArgs {
    query: String,
}

struct WebSearchExecutor {
    client: Client,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<WebSearchArgs> for WebSearchExecutor {
    async fn execute_typed(&self, args: WebSearchArgs) -> Result<String, ToolError> {
        let query = &args.query;

        // Use DuckDuckGo HTML endpoint (no API key required).
        let url = format!(
            "https://html.duckduckgo.com/html/?q={}",
            urlencoding::encode(query)
        );

        let resp = self
            .client
            .get(&url)
            .header("User-Agent", "OmniSolo-Agent/1.0")
            .send()
            .await
            .map_err(|e| format!("websearch: {}", e))
            .map_err(|e| ToolError::LlmRecoverable(e.to_string()))?;

        if !resp.status().is_success() {
            return Ok(format!(
                "Search for '{}' returned HTTP {}",
                query,
                resp.status()
            ));
        }

        let body = read_search_body(resp).await?;

        // Extract result snippets from DuckDuckGo HTML
        let results = extract_ddg_results(&body)?;
        if results.is_empty() {
            return Ok(format!("No results found for: {}", query));
        }

        Ok(results.join("\n\n"))
    }
}

async fn read_search_body(mut response: reqwest::Response) -> Result<String, ToolError> {
    let too_large = || ToolError::LlmRecoverable("websearch: response exceeds 1 MiB".to_string());
    if response
        .content_length()
        .is_some_and(|length| length > MAX_RESPONSE_BYTES as u64)
    {
        return Err(too_large());
    }
    let mut body = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|error| ToolError::LlmRecoverable(format!("websearch: read body: {error}")))?
    {
        if chunk.len() > MAX_RESPONSE_BYTES.saturating_sub(body.len()) {
            return Err(too_large());
        }
        body.extend_from_slice(&chunk);
    }

    // Retain reqwest's text decoding (including any enabled charset support)
    // after the streamed byte limit, before allocating a DOM.
    let mut buffered = axum::http::Response::new(body);
    *buffered.headers_mut() = response.headers().clone();
    reqwest::Response::from(buffered)
        .text()
        .await
        .map_err(|error| ToolError::LlmRecoverable(format!("websearch: read body: {error}")))
}

fn extract_ddg_results(html: &str) -> Result<Vec<String>, ToolError> {
    let document = super::html_parser::parse_html(html).map_err(|limit| {
        ToolError::LlmRecoverable(format!(
            "websearch: HTML parsing complexity limit exceeded ({limit})"
        ))
    })?;
    // Keep concatenated descendant text (including script/style), trimming only
    // the edges. HTML5 DOM order and class membership determine matches; comments
    // and inert template contents cannot become snippets. Skip empty matches.
    Ok(document
        .select(".result__snippet")
        .nodes()
        .iter()
        .map(|node| node.text().trim().to_string())
        .filter(|text| !text.is_empty())
        .take(5)
        .collect())
}

pub fn websearch_tool() -> Tool {
    Tool {
        name: "WebSearch".to_string(),
        description: "Search the web for information. Returns a list of result snippets."
            .to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query."
                }
            },
            "required": ["query"]
        }),
        execute: Arc::new(PydanticAdapter::new(WebSearchExecutor {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap(),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snippet_text() {
        for (html, expected) in [
            ("<b>bold</b>", "bold"),
            ("hello <a href='x'>world</a>", "hello world"),
            ("no tags here", "no tags here"),
        ] {
            assert_eq!(
                extract_ddg_results(&format!("<div class='result__snippet'>{html}</div>")).unwrap(),
                vec![expected]
            );
        }
    }

    #[test]
    fn test_extract_ddg_results() {
        let html = r#"
            <div>
                <a class="result__snippet" href="x">
                    This is <b>result 1</b> snippet.
                </a>
            </div>
            <div>
                <a class="result__snippet" href="y">
                    This is <i>result 2</i> snippet.
                </a>
            </div>
        "#;

        let results = extract_ddg_results(html).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "This is result 1 snippet.");
        assert_eq!(results[1], "This is result 2 snippet.");
    }

    #[test]
    fn test_extract_ddg_results_empty() {
        let html = "<html><body>No results here</body></html>";
        let results = extract_ddg_results(html).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn complexity_failure_does_not_return_partial_snippets() {
        let html = format!(
            "<a class='result__snippet'>Partial result</a>{}",
            "<div>".repeat(256)
        );
        assert!(
            extract_ddg_results(&html)
                .unwrap_err()
                .to_string()
                .contains("complexity limit")
        );
    }

    #[test]
    fn extracts_structural_snippets_with_quoted_attributes_and_entities() {
        assert_eq!(
            extract_ddg_results(
                r#"<A class="extra result__snippet" title="a > b">Fish &amp; Chips &#x1F9EA;</A>"#
            )
            .unwrap(),
            vec!["Fish & Chips 🧪"]
        );
    }

    #[test]
    fn ignores_comments_attributes_and_nonmatching_classes() {
        assert_eq!(
            extract_ddg_results(
                r#"<!-- <a class="result__snippet">Comment</a> -->
                <a title="result__snippet">Attribute</a>
                <a class="result__snippet_extra">Wrong class</a>
                <a class="result__snippet">Real</a>"#
            )
            .unwrap(),
            vec!["Real"]
        );
    }

    #[test]
    fn repairs_unclosed_snippets_and_accepts_non_anchor_elements() {
        assert_eq!(
            extract_ddg_results(
                r#"<a class="result__snippet">First<b> bold</b><a class="result__snippet">Second</a><div class="result__snippet">Third"#
            ).unwrap(),
            vec!["First bold", "Second", "Third"]
        );
    }

    #[test]
    fn preserves_raw_descendant_text_and_trims_only_edges() {
        assert_eq!(
            extract_ddg_results(
                "<a class='result__snippet'>  ice<b>cream</b>\n  today\
                 <script>if (a < b) run();</script><style>.x > p { color: red; }</style>  </a>"
            )
            .unwrap(),
            vec!["icecream\n  todayif (a < b) run();.x > p { color: red; }"]
        );
    }

    #[test]
    fn excludes_inert_templates_and_takes_first_five_nonempty_results() {
        let mut html = String::from(
            "<template><a class='result__snippet'>Inert</a></template>\
             <a class='result__snippet'> </a>",
        );
        for index in 1..=7 {
            html.push_str(&format!("<a class='result__snippet'>{index}</a>"));
        }
        assert_eq!(
            extract_ddg_results(&html).unwrap(),
            vec!["1", "2", "3", "4", "5"]
        );
    }

    async fn local_response(bytes: Vec<u8>) -> reqwest::Response {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut request = [0_u8; 4096];
            assert!(stream.read(&mut request).await.unwrap() > 0);
            // A size rejection can close the connection before this finishes.
            let _ = stream.write_all(&bytes).await;
        });
        Client::builder()
            .no_proxy()
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .unwrap()
            .get(format!("http://{address}/"))
            .send()
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn rejects_oversized_content_length_before_reading_body() {
        let response = local_response(
            b"HTTP/1.1 200 OK\r\nContent-Length: 1048577\r\nConnection: close\r\n\r\n".to_vec(),
        )
        .await;
        let error = read_search_body(response).await.unwrap_err();
        assert!(error.to_string().contains("exceeds 1 MiB"));
    }

    #[tokio::test]
    async fn rejects_oversized_chunked_body() {
        let mut bytes =
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n".to_vec();
        for byte in [b'a', b'b'] {
            bytes.extend_from_slice(b"96000\r\n"); // 600 KiB per chunk, no Content-Length.
            bytes.extend(std::iter::repeat_n(byte, 600 * 1024));
            bytes.extend_from_slice(b"\r\n");
        }
        bytes.extend_from_slice(b"0\r\n\r\n");
        let error = read_search_body(local_response(bytes).await)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("exceeds 1 MiB"));
    }

    #[tokio::test]
    async fn reports_truncated_body_instead_of_empty_results() {
        let response = local_response(
            b"HTTP/1.1 200 OK\r\nContent-Length: 50\r\nConnection: close\r\n\r\nshort".to_vec(),
        )
        .await;
        let error = read_search_body(response).await.unwrap_err();
        assert!(error.to_string().contains("read body"));
    }

    #[tokio::test]
    async fn accepts_exactly_one_mib_and_extracts_unicode_snippet() {
        let mut body = "<a class='result__snippet'>Café 🧪 &amp; tea</a>"
            .as_bytes()
            .to_vec();
        body.resize(1_048_576, b' ');
        let mut bytes =
            b"HTTP/1.1 200 OK\r\nContent-Length: 1048576\r\nConnection: close\r\n\r\n".to_vec();
        bytes.extend(body);
        let html = read_search_body(local_response(bytes).await).await.unwrap();
        assert_eq!(html.len(), 1_048_576);
        assert_eq!(extract_ddg_results(&html).unwrap(), vec!["Café 🧪 & tea"]);
    }
}
