use super::*;

#[test]
fn ordinary_html_matches_upstream_dom_across_utf8_chunk_boundaries() {
    let fixtures: std::collections::BTreeMap<String, String> =
        serde_json::from_str(include_str!("html_parity_fixtures.json")).unwrap();
    for (name, html) in fixtures {
        let expected = dom_query::Document::from(html.as_str()).html().to_string();
        for chunk_bytes in [1, 2, 3, 4, 5, 7, 11, 31, 127, 511, 512] {
            assert_eq!(
                parse_html_chunked(&html, chunk_bytes)
                    .unwrap_or_else(|error| panic!("{name}, chunk {chunk_bytes}: {error}"))
                    .html()
                    .to_string(),
                expected,
                "{name}, chunk {chunk_bytes}"
            );
        }
    }
}

#[test]
fn rejects_deep_elements_and_templates() {
    for tag in ["<div>", "<template>", "<b class='unique'>"] {
        assert!(parse_html(&tag.repeat(256)).is_err(), "accepted {tag}");
    }
}

#[test]
fn bounds_total_start_tags_without_lowering_the_response_byte_limit() {
    assert!(parse_html(&"<br>".repeat(4096)).is_ok());
    assert!(parse_html(&"<br>".repeat(4097)).is_err());
}

#[test]
fn bounds_completed_attributes() {
    let attributes = |count: usize| {
        format!(
            "<p {}>text</p>",
            (0..count).map(|i| format!("a{i}='x' ")).collect::<String>()
        )
    };
    assert!(parse_html(&attributes(64)).is_ok());
    assert!(parse_html(&attributes(65)).is_err());
}

#[test]
fn bounds_unfinished_tokens_including_parse_errors_and_eof() {
    for html in [
        format!("<!--{}", "x".repeat(8192)),
        format!("<p title='{}", "x".repeat(8192)),
        format!("<p {}", "'".repeat(8192)),
        format!("<p {}", "a=x ".repeat(2048)),
        format!("&#{}", "1".repeat(8192)),
        format!("<script></{}", "script".repeat(2048)),
    ] {
        assert!(parse_html(&html).is_err());
    }
}

#[test]
fn bounds_token_errors_and_repeated_tree_work() {
    assert!(parse_html(&"\0".repeat(32_769)).is_err());
    let html = format!("{}{}", "<div>".repeat(64), "<p>x</p>".repeat(3000));
    assert!(parse_html(&html).is_err());
}

#[test]
fn bounds_character_work_in_deep_tree_and_pending_table_state() {
    for last_tag in ["<p>", "<table>"] {
        let html = format!("{}{last_tag}{}", "<div>".repeat(64), " x".repeat(200_000));
        assert!(parse_html(&html).is_err());
    }
}

#[test]
fn reconstructed_formatting_attributes_consume_tree_work() {
    let mut html = String::from("<p>");
    for index in 0..20 {
        html.push_str(&format!(
            "<b {}>",
            (0..64)
                .map(|attribute| format!("a{attribute}='{index}' "))
                .collect::<String>()
        ));
    }
    html.push_str("x</p>");
    // Closing the paragraph leaves distinct active formatting entries. Each new
    // paragraph reconstructs their DOM nodes and clones the attribute vectors.
    html.push_str(&"<p>x</p>".repeat(40));
    assert!(parse_html(&html).is_err());
}

#[test]
fn merged_body_attributes_consume_tree_work() {
    let html = (0..80)
        .map(|index| {
            format!(
                "<body {}>",
                (0..64)
                    .map(|attribute| format!("a{index}_{attribute}='x' "))
                    .collect::<String>()
            )
        })
        .collect::<String>();
    // HTML5 merges repeated body attributes onto the existing active node. The
    // limit must inspect current DOM attributes, not only each individual tag.
    assert!(parse_html(&html).is_err());
}

#[test]
fn one_mib_text_and_rawtext_are_not_unfinished_markup() {
    for (prefix, suffix) in [("<p>", "</p>"), ("<script>", "</script>")] {
        let html = format!(
            "{prefix}{}{suffix}",
            "x".repeat(1_048_576 - prefix.len() - suffix.len())
        );
        let document = parse_html(&html).unwrap();
        assert_eq!(
            document.root().text().len(),
            1_048_576 - prefix.len() - suffix.len()
        );
    }
}

#[test]
fn lossy_utf8_expansion_preserves_the_http_byte_limit() {
    let bytes = vec![0xff; 1_048_576];
    let decoded = String::from_utf8_lossy(&bytes);
    let document = parse_html(&decoded).unwrap();
    assert_eq!(document.root().text().chars().count(), 1_048_576);
}
