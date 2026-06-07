#[test]
fn production_catalog_source_does_not_use_fake_credentials() {
    let source = include_str!("catalog.rs");

    for (prefix, suffix) in [("dummy", "token"), ("dummy", "key"), ("dummy", "secret")] {
        let needle = format!("{prefix}_{suffix}");
        assert!(
            !source.contains(&needle),
            "catalog metadata must not be built with fake credential literal: {needle}"
        );
    }
}
