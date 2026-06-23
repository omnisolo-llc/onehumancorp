use super::*;
use serde_json::json;

#[tokio::test]
async fn test_webfetch_strip_html() {
    let html = "<html><head><title>Test</title></head><body><h1>Hello</h1><p>World</p></body></html>";
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
