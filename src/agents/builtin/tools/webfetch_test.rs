use super::*;
use serde_json::json;

#[tokio::test]
async fn test_webfetch_strip_html() {
    let html = "<html><head><title>Test</title></head><body><h1>Hello</h1><p>World</p></body></html>";
    let text = strip_html(html);
    assert_eq!(text, "Test Hello World");
}
