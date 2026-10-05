use ohc_rust_source_extract::{Selector, extract};
use sha2::{Digest, Sha256};

fn check_item(source: &str, kind: &str, name: &str, expected: &str) {
    let selected = extract(source.as_bytes(), &Selector::new(kind, name)).unwrap();
    assert_eq!(
        &source.as_bytes()[selected.start..selected.end],
        expected.as_bytes()
    );
    assert_eq!(
        selected.source_sha256,
        format!("{:x}", Sha256::digest(source))
    );
    assert_eq!(
        selected.selection_sha256,
        format!("{:x}", Sha256::digest(expected))
    );
}

#[test]
fn valid_rust_braces_do_not_truncate_items() {
    for body in [
        "// } comment\n    assert!(true);",
        "/* } /* nested { */ } */ assert!(true);",
        "let brace = '}'; let escaped = '\\'';",
        "let text = r###\"} \" {\"###;",
        "let text = br##\"} \" {\"##; let byte = b'}';",
        "let text = \"} \\\" {\";",
        "macro_rules! wrap { ($x:expr) => {{ $x }} } wrap!({ 1 });",
    ] {
        let item = format!("fn sample() {{\n    {body}\n}}");
        check_item(
            &format!("{item}\nfn after() {{}}\n"),
            "function",
            "sample",
            &item,
        );
    }
}

#[test]
fn exact_bytes_include_outer_attributes_docs_unicode_and_crlf() {
    let item =
        "/// Résumé\r\n#[cfg(feature = \"é\")]\r\n#[inline]\r\npub fn café() { let x = 'é'; }";
    check_item(
        &format!("// preceding unrelated comment\r\n{item}\r\n"),
        "function",
        "café",
        item,
    );
}

#[test]
fn comments_between_attributes_and_item_remain_exact() {
    let item = "#[inline]\n// } explanatory comment\nfn sample() {}";
    check_item(item, "function", "sample", item);
}

#[test]
fn declarations_with_braces_in_const_generic_type_select_full_item() {
    check_item(
        "struct Sized<const N: usize = { 1 + 1 }> { x: [u8; N] }",
        "struct",
        "Sized",
        "struct Sized<const N: usize = { 1 + 1 }> { x: [u8; N] }",
    );
    check_item(
        "const VALUE: [u8; { 1 + 1 }] = [1, 2];",
        "const",
        "VALUE",
        "const VALUE: [u8; { 1 + 1 }] = [1, 2];",
    );
}

#[test]
fn module_identity_disambiguates_same_names_and_preserves_enclosing_context() {
    let source = "#[cfg(test)] mod one { pub fn same() {} } mod two { pub fn same() { panic!() } }";
    let mut selector = Selector::new("function", "same");
    assert!(
        extract(source.as_bytes(), &selector)
            .unwrap_err()
            .contains("not found")
    );
    selector.modules = vec!["one".into()];
    let selected = extract(source.as_bytes(), &selector).unwrap();
    assert_eq!(&source[selected.start..selected.end], "pub fn same() {}");
    assert_eq!(selected.enclosing.len(), 1);
    assert_eq!(
        &source[selected.enclosing[0].start..selected.enclosing[0].end],
        "#[cfg(test)] mod one { pub fn same() {} }"
    );
}

#[test]
fn impl_identity_keeps_inherent_and_trait_methods_distinct() {
    let source = "#[cfg(test)] impl Item { #[inline] fn same(&self) {} } impl Trait for Item { fn same(&self) { panic!() } }";
    let mut selector = Selector::new("function", "same");
    selector.impl_type = Some("Item".into());
    let selected = extract(source.as_bytes(), &selector).unwrap();
    assert_eq!(
        &source[selected.start..selected.end],
        "#[inline] fn same(&self) {}"
    );
    assert_eq!(
        &source[selected.enclosing[0].start..selected.enclosing[0].end],
        "#[cfg(test)] impl Item { #[inline] fn same(&self) {} }"
    );
    selector.impl_trait = Some("Trait".into());
    let selected = extract(source.as_bytes(), &selector).unwrap();
    assert_eq!(
        &source[selected.start..selected.end],
        "fn same(&self) { panic!() }"
    );
}

#[test]
fn entire_impl_keeps_its_attributes_and_members() {
    let item = "#[cfg(test)] impl<'a> Item<'a> { pub fn value(&self) -> &str { self.0 } }";
    check_item(item, "impl", "Item<'a>", item);
}

#[test]
fn named_type_enum_trait_mod_and_static_items_are_supported() {
    for (source, kind, name) in [
        ("type Id = u64;", "type", "Id"),
        ("enum Kind { One, Two }", "enum", "Kind"),
        ("trait Read { fn read(&self); }", "trait", "Read"),
        ("mod child;", "mod", "child"),
        ("static FLAG: bool = true;", "static", "FLAG"),
    ] {
        check_item(source, kind, name, source);
    }
}

#[test]
fn duplicate_cfg_items_and_duplicate_impl_methods_are_ambiguous() {
    let source = "#[cfg(a)] fn same() {} #[cfg(b)] fn same() {}";
    assert!(
        extract(source.as_bytes(), &Selector::new("function", "same"))
            .unwrap_err()
            .contains("ambiguous")
    );
    let mut selector = Selector::new("function", "same");
    selector.impl_type = Some("Item".into());
    assert!(
        extract(
            b"impl Item { fn same() {} } impl Item { fn same() {} }",
            &selector
        )
        .unwrap_err()
        .contains("ambiguous")
    );
}

#[test]
fn parse_error_or_missing_node_anywhere_rejects_entire_source() {
    for source in [
        "fn good() {} fn broken( {",
        "fn good() {} fn missing() { let x = 1 }",
        "fn good() {} @@@",
    ] {
        assert!(
            extract(source.as_bytes(), &Selector::new("function", "good"))
                .unwrap_err()
                .contains("syntax"),
            "{source}"
        );
    }
}

#[test]
fn macros_and_nested_functions_cannot_masquerade_as_top_level_items() {
    for source in [
        "macro_rules! sample { () => { fn hidden() {} } }",
        "fn outer() { fn hidden() {} }",
    ] {
        assert!(
            extract(source.as_bytes(), &Selector::new("function", "hidden"))
                .unwrap_err()
                .contains("not found")
        );
    }
}

#[test]
fn unsupported_selector_invalid_utf8_and_missing_name_fail_closed() {
    assert!(
        extract(b"fn good() {}", &Selector::new("unknown", "good"))
            .unwrap_err()
            .contains("unsupported")
    );
    assert!(
        extract(b"fn good() {}\xff", &Selector::new("function", "good"))
            .unwrap_err()
            .contains("UTF-8")
    );
    assert!(
        extract(b"fn good() {}", &Selector::new("function", "gone"))
            .unwrap_err()
            .contains("not found")
    );
}
