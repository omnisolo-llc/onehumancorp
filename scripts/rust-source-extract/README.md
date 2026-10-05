# Exact Rust source extraction

This development tool replaces syntax recognition in focused harness generators.
It selects one named item by kind, full inline-module path, and optional impl
identity, then returns an unchanged range of the original UTF-8 bytes. Outer
attributes and doc comments belong to that range; inner module/impl attributes
remain in their enclosing item. Selecting an impl returns that entire impl.
Inherent and trait impl methods have separate identities. Duplicate matches,
missing items, unsupported selections, invalid UTF-8, and parse ERROR/missing
nodes fail closed. Macro token contents are never treated as declarations.

The result records the original source SHA-256, selected range SHA-256, offsets,
and the enclosing module/impl byte ranges. The caller must still preserve its
route/mount/auth checks, compile the generated Rust, bind the helper and grammar
versions in its source manifest, compare every fragment before use, and check
all source fingerprints again after execution. Parsing does not certify macro
expansion, Rust types, authorization, or mounted behavior.

## Implementation plan

1. Record valid-Rust truncation in current scanner fixtures before implementation.
2. Exercise comments, nested comments, chars, raw/byte strings, macros, const
   generics, Unicode, CRLF, attrs, modules and impl identities against exact bytes.
3. Reject malformed syntax, missing nodes, missing names and ambiguous matches.
4. Implement shared Tree-sitter traversal using original byte ranges only.
5. Compare each proposed consumer's current selected fragments/digests, then
   migrate one consumer with its full existing source-drift and focused checks.
   Specialty harnesses remain unchanged until their selection policy is verified.

Dependencies reuse root-locked Tree-sitter 0.26.9 and Rust grammar 0.24.2 (MIT),
SHA-2 0.10.9 and serde_json 1.0.150 (MIT OR Apache-2.0). This is a build tool,
not a new application runtime dependency. No compiler or performance improvement
is claimed. API sources: [byte ranges and syntax nodes](https://tree-sitter.github.io/tree-sitter/using-parsers/2-basic-parsing.html),
[Rust grammar API](https://docs.rs/tree-sitter-rust/0.24.2/tree_sitter_rust/).

## Use from a verified consumer

As a build dependency, use `Selector::new("function", "create_quote")` and
`extract(&source_bytes, &selector)`, then copy
`&source_bytes[result.start..result.end]` unchanged. For methods, set
`selector.impl_type`; trait methods also require `selector.impl_trait`.
Set `selector.modules` to the complete inline-module path; the default selects
only file-level declarations. Impl type and trait selectors match exact source
text, including generic arguments. Attribute-conditioned duplicate declarations
are ambiguous and are rejected even if a particular Cargo configuration would
compile only one. Out-of-line modules are selectable as declarations; resolving
and fingerprinting their other files is the consumer's responsibility.

The CLI emits JSON containing the fragment text, byte offsets, SHA-256 hashes,
and enclosing ranges/hashes:

```sh
cargo run --locked -p ohc-rust-source-extract -- path/to/source.rs function name
cargo run --locked -p ohc-rust-source-extract -- path/to/source.rs function method --impl-type Type
```

`--expect-source-sha256 HASH` rejects a source that changed since the caller's
inventory. No generated file is written by this tool. A consumer can preserve
its existing output path and before/after source-manifest checks.
