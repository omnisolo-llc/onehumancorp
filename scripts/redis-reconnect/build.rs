use ohc_rust_source_extract::{Selector, extract, sha256};
use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("../..");
    let mut output = String::new();
    let mut proofs = Vec::new();
    let source = fs::read(root.join("src/server/queue.rs")).unwrap();
    for (kind, name, trait_name) in [
        ("struct", "Job", None),
        ("trait", "TaskQueue", None),
        ("struct", "RedisTaskQueue", None),
        ("impl", "RedisTaskQueue", None),
        ("impl", "RedisTaskQueue", Some("TaskQueue")),
    ] {
        let mut selector = Selector::new(kind, name);
        selector.impl_trait = trait_name.map(str::to_string);
        let selected = extract(&source, &selector).unwrap();
        output.push_str(std::str::from_utf8(&source[selected.start..selected.end]).unwrap());
        output.push('\n');
        proofs.push(
            serde_json::json!({"source":"src/server/queue.rs", "kind":kind, "name":name,
            "trait":trait_name,"start":selected.start,"end":selected.end,
            "source_sha256":selected.source_sha256,"selection_sha256":selected.selection_sha256}),
        );
    }
    for file in [
        "Cargo.toml",
        "src/server/queue.rs",
        "src/server/utils/cache.rs",
        "src/server/services/cache_invalidator.rs",
        "src/server/utils/Cargo.toml",
        "src/server/omnisolo/Cargo.toml",
        "src/server/omnisolo/build.rs",
        "src/server/omnisolo/mod.rs",
        "src/proto/interop.proto",
        "scripts/redis-reconnect/build.rs",
        "scripts/redis-reconnect/probe.rs",
        "scripts/redis-reconnect/test.rs",
        "scripts/redis-reconnect/fault.rs",
        "scripts/redis-reconnect/invalidator_dependencies.rs",
        "scripts/redis-reconnect/Cargo.toml",
        "scripts/redis-reconnect/Cargo.lock",
        "scripts/rust-source-extract/Cargo.toml",
        "scripts/rust-source-extract/src/lib.rs",
    ] {
        println!("cargo:rerun-if-changed={}", root.join(file).display());
        proofs.push(serde_json::json!({"source":file,"source_sha256":sha256(&fs::read(root.join(file)).unwrap())}));
    }
    let mut protos: Vec<_> = fs::read_dir(root.join("src/proto"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "proto"))
        .collect();
    protos.sort();
    for proto in protos {
        let source = proto.strip_prefix(&root).unwrap().to_str().unwrap();
        println!("cargo:rerun-if-changed={}", proto.display());
        proofs.push(
            serde_json::json!({"source":source,"source_sha256":sha256(&fs::read(&proto).unwrap())}),
        );
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(out.join("queue.rs"), output).unwrap();
    fs::copy(
        root.join("src/server/services/cache_invalidator.rs"),
        out.join("cache_invalidator.rs"),
    )
    .unwrap();
    fs::write(
        out.join("source-manifest.json"),
        serde_json::to_vec_pretty(&proofs).unwrap(),
    )
    .unwrap();
}
