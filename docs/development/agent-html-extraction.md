# Bounded HTML extraction for agent tools

WebFetch and WebSearch use `dom_query 0.27.0` with the public `html5ever 0.38.0`
tokenizer and tree builder. The parser owns HTML syntax, character references,
raw text, namespaces, malformed-markup repair and template handling. The adapter
in `src/agents/builtin/tools/html_parser.rs` applies conservative work limits.

WebFetch preserves title, script and style text, collapses whitespace, separates
DOM text nodes with spaces, and displays at most 10,000 Unicode characters.
WebSearch selects actual `.result__snippet` class members in DOM order,
concatenates descendant text, trims the edges, skips empty matches and returns
at most five results. Comments and inert template contents are excluded. This
is text extraction; executable content is retained as text under the existing
policy, and parsing does not establish that retrieved instructions are trusted.

Both HTTP paths retain a 1 MiB response-byte limit. WebFetch keeps its existing
DNS/SSRF checks, pinned addresses and per-redirect validation. WebSearch checks
both declared and streamed size before parsing and propagates truncated-body
errors. Response decoding can expand the retained UTF-8 string: one MiB of invalid
bytes becomes three MiB of replacement characters. The parser preserves that
case rather than imposing a smaller limit on the decoded representation.

## Parser budgets

| Budget | Limit |
| --- | ---: |
| UTF-8 feed chunk | 512 bytes |
| Fed bytes without a completed token | 4,096 bytes |
| Tokens, including parse errors and EOF | 32,768 |
| Start tags | 4,096 |
| Attributes on one emitted tag | 64 |
| Live tree-builder handle references | 128 |
| Accumulated live handles plus their DOM attribute counts, per token | 131,072 |
| Character-token bytes multiplied by live handle count | 16 MiB work units |

Live references are obtained through `TreeBuilder::trace_handles`; document,
open elements, active formatting, head, form and context references count, with
duplicates retained. The work meter reads current attributes from the DOM without
cloning them. This accounts for formatting reconstruction and attributes merged
onto an existing body/html element. Character work is charged before forwarding,
including work later flushed from table text. Handle limits remain independent
of attribute counts.

The unfinished-token check protects tokenizer work before a tag, comment or
character reference is emitted. Parse errors, EOF and empty text do not count as
progress. A trailing suffix after the last completed token can add one chunk,
so at most 4,608 bytes of unfinished markup can reach the production tokenizer.
This also bounds duplicate-attribute processing before the per-tag attribute
count is available. Raw text that emits characters continues normally.

A limit returns an explicit recoverable complexity error. The caller receives no
partial text or snippets. After rejection the sink skips further tree work, the
driver stops after the bounded feed returns, and the parser is dropped without
an EOF tree flush. Parsing runs synchronously within these work budgets.
Accepted inputs retain the upstream tree-builder results, scripting-disabled option,
foreign-namespace callback, script/raw-text/plaintext state transitions and
encoding-indicator driver behavior.

## Verification record

The focused native harness imports the complete production modules and the exact
production Tool/ToolExecutor definitions. The initial budget RED was **55 passed,
8 failed**. The first adapter exposed a separate formatting-attribute resource
amplification; its additional RED was **63 passed, 2 failed**. The final source
passed **65 tests, 0 failed, 0 ignored**, plus strict all-target Clippy. The parity
test compares 28 upstream DOM fixtures at 11 selected UTF-8 chunk sizes: **308
serialization comparisons**. Existing network, response-size, Unicode display,
script/style, template and five-result assertions remain enabled.

Twenty-five separate-process debug probes exercise deep markup, formatting
repair, tables and deferred table text, many/long attributes, merged body
attributes, incomplete tags/comments/entities, script text and decoding
expansion. All had the expected accepted result or explicit complexity error;
none crashed or reached the five-second external kill deadline. The largest
observed single extraction was **213.728 ms** and process peak RSS was **16,320
KiB**. These are local unoptimized measurements, not a universal latency or heap
allocation guarantee. The combined formatting/attribute case decreased from
**84,896 KiB and 1,271.230 ms** before attribute weighting to **9,984 KiB and
109.660 ms** afterward. A second case exercises reconstruction after a smaller
initial formatting set, rather than only rejecting during seeding.

Execution-workspace receipts are under `/tmp/html-budget-contract/`:
`red.log`, `amplification-red.log`, `final-green.log`, `final-clippy.log`,
`cost-amplification-before.json`, `cost-debug.json`, `final-runs.json` and
`final-source-manifest.json`. Each final run records its exact command, status,
log digest, prepared-source check and stable post-run source digest. The resource
probe includes the exact extraction bodies and complete parser source; its
binary hash is bound to both build and execution. A blocked Cargo invocation
was stopped before compilation and is retained as `amplification-interrupted.*`;
it is excluded from RED/GREEN evidence.

The focused source manifest SHA-256 is
`81ab808274d01b3f9c31a8cd0a74842797a700f2c3f1f0f72f074d8d83b844e4`;
the resource probe binary SHA-256 is
`d5bd5254c117045ab3df9dc6bc7025cba01a7131217d7087b6979763d0ec8f53`.

## Integration prerequisites

The tools crate directly declares the already-locked parser packages.
`dom_query` uses `default-features = false`, is MIT-licensed and declares Rust
1.75; `html5ever` is MIT OR Apache-2.0 and declares Rust 1.71.0. The integration
owner must reconcile the tools package's dependency edges in the authoritative
root lockfile before running `--locked` workspace gates. Focused registry
package/version/source/checksum identities match that lock; no registry upgrade
was performed here. Preserve applicable dependency notices.

The focused receipt does not certify the entire tools crate, full `make lint` /
`make test`, desktop packaging or real external searches. Those remain separate
native acceptance gates. The earlier unbounded parser WIP commit `000838758`
remains excluded from the implementation branch's ancestry.
