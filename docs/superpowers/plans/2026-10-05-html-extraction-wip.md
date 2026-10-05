# INT5 HTML extraction migration — WIP, do not merge

This checkpoint is **not ready for integration**. It replaces the two custom HTML
scanners with `dom_query = 0.27.0` and bounds WebSearch response bytes, but a
pathological-input benchmark found an unacceptable synchronous parsing cost.
The 1 MiB input limit alone does not bound CPU work in the HTML5 tree builder.

## Implemented contract

- WebFetch retains title/script/style text, normalizes whitespace, separates DOM
  text nodes with spaces, and keeps its network guards, 1 MiB input limit and
  10,000-Unicode-character display limit.
- WebSearch selects actual `.result__snippet` elements in DOM order, concatenates
  descendant text without adding spaces, trims only the edges, skips empty
  matches, and returns the first five. Its HTTP reader rejects oversized declared
  or streamed bodies before DOM allocation and reports read failures as errors.
- HTML5 parsing decodes entities and repairs markup. Comments and inert template
  contents are excluded. Excluding template contents is an explicitly approved
  migration difference. This is not a sanitizer or prompt-injection defense.

## Regression evidence (2026-10-05 UTC, Rust 1.95.0)

A focused harness includes the unchanged production WebFetch, WebSearch,
network-policy, Pydantic adapter and core types source, plus the exact Tool and
ToolExecutor definitions. Tests use local HTTP servers; no provider was called.

Before implementation: **38 passed, 14 failed**, 31.796 seconds including build.
Failures demonstrate attribute leakage, undecoded entities, incorrect snippet
selection, missing template exclusion, and accepted oversized/truncated bodies.
After implementation with a copied repository lockfile: **52 passed**, 23.242
seconds including cold parser dependencies, 0.18 seconds in tests. Strict Clippy
passed in 23.587 seconds. This is focused evidence, not a full workspace gate.

Evidence paths in the execution workspace:

- `/tmp/html-contract/red.log`, `source-manifest.red.json`
- `/tmp/html-contract/green-root-lock.log`, `clippy-root-lock.log`
- `/tmp/html-contract/source-manifest.json` SHA-256
  `ba56d1555f103c610d532f2c6ad13ce9586503acb727b17a72b065e94d7abf8d`
- `/tmp/html-contract/probe.rs`, `run_cost.py`, `cost-debug.json`

## Blocking resource measurement

A separate process runs the exact extraction functions, first WebFetch and then
WebSearch, with a five-second kill deadline. The following measurements use
**unoptimized** dependencies and are observations, not release-build performance
claims. RSS is process peak, not a precise DOM heap allocation count.

| Input | Bytes | Fetch | Search | Peak RSS |
| --- | ---: | ---: | ---: | ---: |
| Repeated flat paragraphs/entities | 1,048,570 | 926 ms | 784 ms | 31,584 KiB |
| Repeated misnested formatting | 1,048,564 | 921 ms | 951 ms | 43,124 KiB |
| Repeated matching snippets | 1,048,576 | 436 ms | 439 ms | 28,560 KiB |
| Unclosed nested `<div>` | 4,095 | 52 ms | 48 ms | 9,984 KiB |
| Unclosed nested `<div>` | 16,380 | 727 ms | 762 ms | 9,984 KiB |
| Unclosed nested `<div>` | 65,535 | Killed after 5 s before first parse completed | — | 9,984 KiB |
| Unclosed nested `<div>` | 1,048,575 | Killed after 5 s before first parse completed | — | 9,984 KiB |

The scaling demonstrates a real migration risk. Moving this work to
`spawn_blocking` and timing out its future would not cancel the CPU work and is
not an acceptable fix. A subsequent design must impose explicit conservative
complexity budgets using the mature parser's public tokenizer/tree-builder APIs,
including unfinished tokens and raw text, and prove ordinary fixture parity.
Do not silently lower the existing input/display limits or fall back to the old
scanner after a budget failure.

## Dependency review

The [official version metadata](https://crates.io/api/v1/crates/dom_query/0.27.0)
reports MIT, Rust 1.75 MSRV, not yanked, published 2026-03-17, checksum
`521e380c0c8afb8d9a1e83a1822ee03556fc3e3e7dbc1fd30be14e37f9cb3f89`.
The source archive license agrees. The direct tools dependency disables default
features (unused markdown), atomic, mini_selector and optional hashbrown remain
disabled. The package and its dependency closure were already locked through the
desktop graph; this adds backend compilation and DOM allocation. No shared lock
change is included in this checkpoint. Repository-wide license/advisory and full
workspace acceptance remain with the integration owner.
