use crate::builder::publication_json::decode_publication_json;
use serde_json::json;

#[test]
fn raw_publication_json_rejects_duplicate_decoded_keys_at_every_depth() {
    for raw in [
        r#"{"a":1,"a":2}"#,
        r#"{"nested":{"a":1,"\u0061":2}}"#,
        r#"[{"label":"first","label":"second"}]"#,
    ] {
        assert!(
            decode_publication_json(raw.as_bytes()).is_err(),
            "accepted duplicate keys in {raw}"
        );
    }
}

#[test]
fn raw_publication_json_rejects_unrepresentable_strings_and_keeps_real_unicode() {
    let valid =
        decode_publication_json("{\"text\":\"雪 🧪\\nCafé\",\"0\":false}".as_bytes()).unwrap();
    assert_eq!(valid, json!({"text":"雪 🧪\nCafé","0":false}));
    for raw in [
        r#"{"text":"\u0000"}"#,
        r#"{"\u0000":1}"#,
        r#"{"text":"\ud800"}"#,
        r#"{"\udfff":1}"#,
    ] {
        assert!(
            decode_publication_json(raw.as_bytes()).is_err(),
            "accepted {raw}"
        );
    }
}

#[test]
fn raw_numeric_admission_matches_the_shared_exact_literal_vectors() {
    let cases: serde_json::Value =
        serde_json::from_str(include_str!("raw-number-vectors.json")).unwrap();
    let mut failures = Vec::new();
    for case in cases.as_array().unwrap() {
        let literal = case["literal"].as_str().unwrap();
        let actual =
            decode_publication_json(format!("{{\"number\":{literal}}}").as_bytes()).is_ok();
        if actual != case["accepted"].as_bool().unwrap() {
            failures.push((literal, actual));
        }
    }
    assert!(failures.is_empty(), "raw numeric mismatches: {failures:?}");
}

#[test]
fn publication_json_has_a_bounded_nesting_depth() {
    let raw = format!("{}0{}", "[".repeat(33), "]".repeat(33));
    assert!(decode_publication_json(raw.as_bytes()).is_err());
    let valid = format!("{}0{}", "[".repeat(8), "]".repeat(8));
    assert!(decode_publication_json(valid.as_bytes()).is_ok());
}

#[test]
fn snapshot_digest_uses_rfc8785_utf16_keys_and_ecmascript_numbers() {
    use crate::builder::publication_store::{SiteSnapshot, prepare_snapshot};
    use sha2::{Digest, Sha256};
    let raw = r#"{"domain":null,"pages":[{"path":"/","title":"雪","seo_metadata":{"\ue000":1e-7,"😀":-0.0,"a":1.0},"blocks":[]}]}"#;
    let snapshot: SiteSnapshot =
        serde_json::from_value(decode_publication_json(raw.as_bytes()).unwrap()).unwrap();
    let canonical = r#"{"domain":null,"pages":[{"blocks":[],"path":"/","seo_metadata":{"a":1,"😀":0,"":1e-7},"title":"雪"}]}"#;
    assert_eq!(
        prepare_snapshot(&snapshot).unwrap().1,
        format!("{:x}", Sha256::digest(canonical.as_bytes()))
    );
}

#[test]
fn stored_snapshot_admission_rejects_nul_unsafe_integers_and_ambiguous_paths() {
    use crate::builder::publication_store::{SiteSnapshot, prepare_snapshot};
    let base = json!({"domain":null,"pages":[{"path":"/","title":"Reviewed","seo_metadata":{},"blocks":[]}]});
    for value in [json!(9007199254740992_u64), json!("nul\0text")] {
        let mut input = base.clone();
        input["pages"][0]["seo_metadata"]["value"] = value;
        let snapshot: SiteSnapshot = serde_json::from_value(input).unwrap();
        assert!(prepare_snapshot(&snapshot).is_err());
    }
    for path in ["/nested//page", "/trailing/"] {
        let mut input = base.clone();
        let mut page = input["pages"][0].clone();
        page["path"] = json!(path);
        input["pages"].as_array_mut().unwrap().push(page);
        let snapshot: SiteSnapshot = serde_json::from_value(input).unwrap();
        assert!(prepare_snapshot(&snapshot).is_err(), "accepted {path}");
    }
}
