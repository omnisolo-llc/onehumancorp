use ohc_rust_source_extract::{Selector, extract, sha256};
use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    let mut generated = String::new();
    let mut proof = Vec::new();
    for (file, items) in [
        (
            "src/server/domain/repository/models.rs",
            vec![("struct", "Quote")],
        ),
        (
            "src/server/api/quotes.rs",
            vec![
                ("struct", "TenantAuthority"),
                ("impl", "TenantAuthority"),
                ("struct", "CreateQuoteRequest"),
                ("struct", "QuoteLineItemRequest"),
                ("function", "validate_line_item_references"),
                ("function", "validate_create_references"),
                ("function", "create_quote"),
            ],
        ),
    ] {
        println!("cargo:rerun-if-changed={}", root.join(file).display());
        let source = fs::read(root.join(file)).unwrap();
        for (kind, name) in items {
            let selected = extract(&source, &Selector::new(kind, name))
                .unwrap_or_else(|error| panic!("{file}: {kind} {name}: {error}"));
            generated.push_str(std::str::from_utf8(&source[selected.start..selected.end]).unwrap());
            generated.push('\n');
            proof.push(serde_json::json!({
                "source": file, "kind": kind, "name": name,
                "start": selected.start, "end": selected.end,
                "source_sha256": selected.source_sha256,
                "selection_sha256": selected.selection_sha256,
            }));
        }
    }
    for file in [
        "Cargo.toml",
        "Cargo.lock",
        "scripts/quote-taxjar/Cargo.toml",
        "scripts/quote-taxjar/Cargo.lock",
        "scripts/quote-taxjar/probe.rs",
        "scripts/quote-taxjar/build.rs",
        "scripts/quote-taxjar/money_test.rs",
        "scripts/quote-taxjar/provider_test.rs",
        "src/server/api/quote_taxjar.rs",
        "src/server/api/quote_taxjar_test.rs",
        "src/server/integrations/taxjar/Cargo.toml",
        "src/server/integrations/taxjar/mod.rs",
        "src/server/integrations/taxjar/money.rs",
        "src/server/integrations/taxjar/client.rs",
        "src/server/integrations/taxjar/provider.rs",
        "src/server/services/quoting/mod.rs",
        "src/server/workers/quote_generation_worker.rs",
        "scripts/rust-source-extract/Cargo.toml",
        "scripts/rust-source-extract/src/lib.rs",
    ] {
        println!("cargo:rerun-if-changed={}", root.join(file).display());
        proof.push(serde_json::json!({"source": file, "source_sha256": sha256(&fs::read(root.join(file)).unwrap())}));
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("quote_handler.rs"), generated).unwrap();
    fs::write(
        out.join("source-manifest.json"),
        serde_json::to_vec_pretty(&proof).unwrap(),
    )
    .unwrap();
}
