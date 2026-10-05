use ohc_rust_source_extract::{Selector, extract, sha256};
use serde_json::{Value, json};
use std::{collections::HashSet, env, fs};

fn run() -> Result<Value, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        return Err("usage: ohc-rust-source-extract FILE KIND NAME [--module a::b] [--impl-type TYPE] [--impl-trait TRAIT] [--expect-source-sha256 HASH]".into());
    }
    let mut selector = Selector::new(&args[1], &args[2]);
    let mut expected_hash = None;
    let mut seen = HashSet::new();
    for pair in args[3..].chunks(2) {
        if pair.len() != 2 || !seen.insert(&pair[0]) {
            return Err("missing flag value or duplicate flag".into());
        }
        match pair[0].as_str() {
            "--module" => selector.modules = pair[1].split("::").map(str::to_owned).collect(),
            "--impl-type" => selector.impl_type = Some(pair[1].clone()),
            "--impl-trait" => selector.impl_trait = Some(pair[1].clone()),
            "--expect-source-sha256" => expected_hash = Some(&pair[1]),
            other => return Err(format!("unknown flag: {other}")),
        }
    }
    let source = fs::read(&args[0]).map_err(|error| format!("{}: {error}", args[0]))?;
    if expected_hash.is_some_and(|expected| expected != &sha256(&source)) {
        return Err("source fingerprint does not match expected SHA-256".into());
    }
    let selected = extract(&source, &selector)?;
    let enclosing: Vec<Value> = selected.enclosing.iter().map(|span| {
        json!({"start": span.start, "end": span.end, "sha256": sha256(&source[span.start..span.end])})
    }).collect();
    Ok(json!({
        "start": selected.start,
        "end": selected.end,
        "source_sha256": selected.source_sha256,
        "selection_sha256": selected.selection_sha256,
        "enclosing": enclosing,
        "text": std::str::from_utf8(&source[selected.start..selected.end]).expect("validated UTF-8"),
    }))
}

fn main() {
    match run() {
        Ok(proof) => println!("{proof}"),
        Err(error) => {
            eprintln!("Rust source extraction: {error}");
            std::process::exit(1);
        }
    }
}
