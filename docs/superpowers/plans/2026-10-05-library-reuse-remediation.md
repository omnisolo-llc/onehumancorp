# Library reuse remediation

**Goal:** Resolve the concrete library-reuse audit defects with testable, bounded changes; retain domain policy and established architecture.

**Authorization:** User requested implementation and a passing PR on 2026-10-05. Parent coordinates integration/merge and separately owns UI #40124/#40150/#40153. No deployment, live sends, charges, credential creation, or provider mutation.

**Base:** `405733038a8541b5cdaab70f88ad8b5e91a30c52`; only `.automator/research_report.json` differs from reviewed `ee926e0a4aaa179b86ac23d6e6070b81e30e63a8`.

**Spec:** Library report `libfile_88c2d04ea83081919263cb6996412596`, version 0; local `/workspace/review-output/summary-draft.md` plus the six detailed appendices. These are review evidence, not implementation/test certification.

**Architecture / stack:** Preserve Cargo/Rust 1.95, Axum/Tonic, SQLx, Redis, Reqwest, Next/React, Tauri and canonical npm. Prefer current installed public libraries. Keep tenant identity, authority, budgets, receipts, idempotency, unknown-outcome reconciliation, offline semantics and source-bound packaging.

## Coordination and acceptance

Six isolated worktrees have disjoint source ownership. Each worker first reproduces with meaningful failing tests, then implements and runs focused parity/negative tests. No worker publishes or merges. Root owns integration, shared root Cargo manifest/lock reconciliation and aggregate acceptance. Workers may propose exact dependency edits, but must not update shared root locks independently. One heavy Cargo invocation globally until resources are measured; no worker starts one without root coordination. No weakening assertions, discovery or required gates. No UI overlapping edits until active PR inventory is resolved.

Each handoff must include commit/diff, changed paths, RED/GREEN commands with exact results, relevant API/documentation sources, licenses/advisories for newly selected dependencies, remaining uncertainty, and a review package. Independent review follows every batch; then `make lint`, `make test`, full required hosted checks on exact final source. Missing prerequisites remain blockers. Parent performs the coordinated merge only after acceptance.

## Finding disposition

“Prerequisite” means a tracked compatibility step before implementation, not a dismissed defect. “Keep” applies to a justified design choice, not an untested claim that code is correct.

| Finding | Disposition | Owner / bounded work and tests |
|---|---|---|
| B1 durable Redis claims | Prerequisite then fix | Backend: establish producer/consumer contract and persisted payload compatibility; crash/restart, poison, delay, tenant and duplicate tests before activation. Do not invent missing business execution. |
| B2 Redis reconnect | Fix after money | Backend: existing ConnectionManager/RedisPool, separate blocking/command connections; reconnect and non-replay tests. PubSub explicitly resubscribes. |
| B3 exact TaxJar amounts | Fix first | Backend: exact JSON decimal boundary, checked integer cents, explicit precision policy; 0.29, overflow, negative and extra-precision tests in real quote/TaxJar harness. |
| B4 Moka cache | Keep current until parity decision | Backend: freshness/tags/SWR compatibility and measured cost prerequisite; no blanket cache rewrite. |
| B5 SQL migrations | Prerequisite | Backend: baseline deployed SQLite/MySQL schemas and forward-only migration fixtures; preserve existing PostgreSQL migrator and domain SQL. |
| Backend tenant workflow adjacency | Fix with ownership tests | Backend: authenticated tenant/object lookup and confirmation; atomic durable-state transitions tested before claiming workflow reliability. |
| S1 raw harness HTTP | Fix | Security: existing Reqwest, no proxy/redirects, loopback policy, bounded streams, timeout and cancellation; chunked/close/status/malformed and oversized fixtures. |
| S2 MCP transport | Prerequisite then fix | Security: official rmcp adapter with protocol/legacy-SSE compatibility and process cleanup; no silent endpoint migration or activation of dormant tools. |
| S3 JWE session codec | Fix after UI overlap resolution | Frontend: existing jose, old/new cross-decrypt fixtures, exact header/context/algorithm/size/rotation/expiry policy and generic failures. |
| S4 credential envelope | Prerequisite then fix | Security: persisted ciphertext inventory, versioned backward-read/forward-write and stable key source; separate digest from encryption; no implicit migration/data loss. |
| S5 typed JWK | Fix first | Security: installed jsonwebtoken JwkSet, RS256 signing eligibility, mixed keys and negative algorithm/use/key_ops/issuer tests. |
| S6 bounded JSON RPC lines | Fix first | Security: tokio-util codec, max frame, fail pending/reap child on oversize; cancellation and dialect parity. |
| S7 dormant proxy/executor | Keep pending caller evidence | Security: do not rewrite inactive APIs speculatively. |
| Reverse tunnel identity adjacency | Fix | Security: bind request to verified AuthInfo and tenant/agent, reconnect ownership tests; preserve Tonic. |
| FE01 receipt-backed quote approval | Fix with UI owner | Frontend: compare active UI branches, approval/rejection/unknown receipt tests; no simulated completion. |
| FE02 accessible primitives | Fix after parity design | Frontend: bounded Radix dialog/tooltip pilot, focus/Escape/ARIA/mobile tests; no global redesign. |
| FE03 polling races | Fix | Frontend: bounded cancellation/nonoverlap first, logout/tenant/stale/unmount tests; query library only if lifecycle reuse warrants cost. |
| FE04 unused UI dependencies | Fix | Frontend: prove no callers, remove unused direct packages and correct dev scope; canonical npm locks, bundle/type/test evidence. |
| FE05 form state/schema | Prerequisite | Frontend: selective RHF and version-consistent schema only with strictness/semantic validation parity; no blanket forms migration. |
| FE06 App Router metadata/zoom | Fix with UI owner | Frontend: route metadata and accessible zoom without overlap or product redesign. |
| FE07 fake product/payment labels | Fix with UI owner | Frontend: truthful failure/confirmation labels with source-backed states and tests. |
| Frontend public assets | Prerequisite | Frontend: URL compatibility inventory before deleting historical files. Keep Zustand, PowerSync/offline journal, DOMPurify and supported payment/documentation clients. |
| Platform Node/Tauri advisories | Fix first | Platform: same-major patch policy, all official Node digests, native resource and source-proof tests; root owns Cargo lock integration. |
| Platform Url-only Reqwest | Fix | Platform: tauri::Url reuse, remove only direct unused HTTP dependency, native compile/parity tests. |
| Platform updater mobile gating | Fix | Platform: desktop target dependency/initialization/capabilities; mobile and desktop configuration tests. |
| CLI selection/Markdown/RPC SDK | Keep until requirement | Platform: preserve restricted formatter, small fetch wrapper, protected endpoint and native integrity/readiness policy. No library merely for dependency count. |
| CLI test dependency scope | Fix | Platform: move test-only dependencies to dev, install/type/test proof. |
| T1 Rust source extraction | Fix | Tooling: tree-sitter byte-span parser, exact attrs/impl/source hashes, ERROR/missing/ambiguous rejection; valid comments/chars/raw strings and malformed regression fixtures. |
| T2 dependency/lock policy | Fix incrementally | Tooling + root: feature-aware cargo advisories/licenses/sources and actual npm/Python trees; shared tomllib validation preserving specialty; no arbitrary audit overrides. |
| T3 workflow YAML scanner | Fix | Tooling: existing PyYAML compose, style/locations/duplicates/merge policy; preserve OHC semantic/security coverage. |
| T4 nextest | Keep current runner | Tooling: only migrate on documented isolation/discovery/doc-test parity and measured benefit. Preserve CI #40076 balancing. |
| INT1 Twilio form parsing | Fix first | Integrations: form_urlencoded, preserve raw signature and duplicate policy; Unicode, plus, equals, malformed/empty and signed input tests. |
| INT2 unsafe Twilio replay | Fix first | Integrations: one mutating attempt, bounded deadline, valid SID receipts and typed unknown outcomes; disconnect-after-read, stall, 5xx, 4xx, malformed 2xx tests. |
| INT3 output schemas | Fix after provider parity | Integrations: schemars/serde_path_to_error matching derives and actual Vec caller; provider subset, semantics and bounded correction tests. |
| INT4 metrics lifecycle | Fix | Integrations: existing OTel SDK/exporter opt-in startup and bounded shutdown; in-memory export and redaction/cardinality tests; never replace billing ledger. |
| INT5 HTML extraction | Fix | Integrations: mature parser and bounded WebSearch body; entities/quoted delimiters/malformed markup and WebFetch limit parity. |
| INT6 Gmail MIME | Prerequisite then fix | Integrations: lettre builder/base64 with verified authenticated sender; no invented From, live sends or unsupported activation. |
| INT7 meeting-provider fake URLs | Fix truthful outcomes | Integrations: typed required receipts, no fabricated success or dormant-provider activation. |

## Initial execution batch

1. Backend: B3 only; propose exact shared manifest changes separately.
2. Integrations: INT1 and INT2 only, including actual caller error propagation.
3. Security: S5 and S6, then reverse-tunnel identity if independent and reproduced.
4. Platform: same-major Node maintenance, Url reuse, updater target gating and CLI dev-only cleanup; give root Cargo update set.
5. Tooling: T3 plus T1 design/RED fixtures; preserve specialized source proof.
6. Frontend: inspect active UI PR ownership; only nonoverlapping S3/FE03/FE04 after confirming boundaries.

Subsequent adapters follow once initial batch evidence is reviewable. Every concrete finding remains in this ledger until fixed or justified with a specific prerequisite/keep decision. Shared locks, wider protocol and persisted-format changes receive independent review before integration.

## Review focus

Lossy money conversion; provider replay after unknown outcome; raw signed body preservation; tenant/algorithm/identity confusion; unbounded allocation; process cleanup; persisted-format compatibility; dependency feature/toolchain/platform fit; exact final-source test evidence. Estimate benefits as fewer custom edge-case paths and known defects; do not invent performance/bundle savings.
