# Native build migration and audit remediation ledger

## Standard release tooling continuation — 2026-09-19

The [standard release-pipeline record](standard_release_pipeline_2026-09-19.md) documents the official Tauri action integration, unchanged cross-platform matrix, explicit target/locked builds, DMG notarization ordering, builder provenance, draft upload verification and the cargo-dist layout probe. The [release guide](../development/native-releases.md) is the operator contract. These implementation/script-test results do not supersede the remaining full-CI, signed-platform and real-runtime acceptance gates below.

## Measured cleanup continuation — 2026-09-19

See [the measured native-build cleanup record](native_build_measurements_2026-09-19.md) for commands, cache conditions, source qualifications and regression evidence. Empty-output backend compilation completed in **8m 33.94s**, its exact unchanged rerun in **1.87s**, and a fresh Node proxy build in **31.40s** without stderr warnings. The refreshed frontend suite passed **1,516 tests across 340 files in 3m 30.82s**, with a stable source digest; a fresh web build then completed in **29.21s**. The selected thirteen tenant-cost/header regressions passed, but a concurrent Rust edit invalidated that invocation's final-source fingerprint. The Linux Tauri debug/no-bundle build passed in **6m 42s**; **59 native CI-script tests** passed. These are local measurements, not hosted full-CI certification.

This pass fixes unsafe shared-temporary-file cleanup, non-atomic cross-filesystem fallback, blocking environment-test locks, an unused AVIF encoder dependency, stale custom-target browser inputs, tooltip timer disposal and typed test/API helper issues. The full **30-minute required CI** target remains unmet until every strict quality/test/deployment gate passes on final source. The new record tracks repository-wide lint debt; older completion statements below must not override that limitation. The refreshed full ESLint snapshot still reports **723 errors across 294 files**. The original F02 repair also required additional tenant-scoping corrections in dashboard, agent-manager, agent-metrics and Tracker summaries; those paths and cache-key migration are documented in the measured record. Neither a passing targeted suite nor a cache hit establishes full green CI.

## Current continuation: 2026-09-19

The working branch is `fix/bazel-modernization-and-cleanup`, HEAD `c3716d0875df6403322af4fb47d9f56f9042af3c`, with substantial uncommitted migration/application work. The original register below is retained as the initial acceptance inventory; this section records current evidence and supersedes its initial-status column. No commit, push, hosted-CI dispatch, deployment, live-provider call or customer charge is implied.

### Build acceptance and cleanup

**Full green CI is not established.** The explicit target is **30 minutes to the complete Linux CI required gate**, with a **15-minute warm-cache stretch target**. The actual job-timestamp reporter and cache-disabled manual option are implemented; no hosted run of this uncommitted revision has been executed. Never describe the configured timeout as an achieved build duration.

| ID | Current implementation / evidence | Remaining acceptance |
|---|---|---|
| M01 | Cargo workspace and locked dependencies; pinned Rust 1.95.0. Removed residual Bazel conditionals and duplicate inline billing compilation; persistence tests now exercise canonical modules rather than a second copied module graph. Full Rust formatting passed. Native backend/agent/worker binaries compiled successfully. | Complete current-source Rust test and Clippy results, including Tauri; no whole-workspace green claim from binary compilation. |
| M02 | Removed two tracked dangling Bazel launcher symlinks from `src/ui/next/bin`. A fresh Next build then passed and packaged validated standalone output in 54 seconds with npm dependencies already installed. Node remains required for server routes. | Rebuild after subsequent UI edits; final Tauri/runtime and browser acceptance. |
| M03 | Canonical `make lint` / `make test` remain complete. CI partitions headless and desktop Rust lint/tests; Node lint/tests run independently of the web build and remain required. All 1,688 browser tests in 453 files now enumerate after fixing fixture imports and duplicate Playwright resolution. | Discovery is not execution. Full Rust/browser and all Node quality gates remain necessary. |
| M04 | OS/architecture/role/compiler-aware caches; explicit Node dependency scopes; positive trusted-event cache-save rules; no PR cache publication. Cold-cache control disables project-cache restoration. | Real fresh-runner cache hit/miss and runtime measurements; signed release cache isolation is configuration-reviewed, not release-certified. |
| M05 | Native archives/desktop/mobile/Docker workflows retained. Production Docker layers built once and reused by both Kind and Compose; source/checksum/tag/loaded-ID checks protect image reuse. Same-run mTLS probe reused. | Actual Docker release-image and deployment-suite execution on this revision; signing/install/device validation remains outstanding. |
| M06 | Active native README/AGENTS/Automator instructions and Make targets replace Bazel. Obsolete launcher symlinks are guarded by a regression test. | Keep future automation aligned with current Make gates and truthful evidence; do not restore a stale legacy command from historical reports. |
| M07 | Local timing logs, Cargo timing HTML and RSS measurement captured under `target/validation` / `target/cargo-timings`. CI produces attempt-bound JSON/Markdown timing evidence and fails above 30 minutes. | No measured hosted cold/warm percentiles yet; initial queue and final reporting time are explicitly excluded. |

### Findings-to-implementation status

| Finding | Production changes already present | Evidence level / remaining gap |
|---|---|---|
| F01: repeated usage accounting | One-way ingress/accounting/export pipeline replaces re-enqueueing; canonical billing workspace crate is used by the backend. | Regression checks assert one count, queue drain and closure. Historical focused suites passed; current-profile rerun is recorded separately. |
| F02: global totals in tenant summary | Billing reads scoped tenant/agent totals; absent, blank and mismatched authenticated organization identities are rejected. | Same-agent-name/two-tenant regression tests exist; not a claim that every legacy reporting endpoint is audited. |
| F03: budget increments before denial | Checked pricing budget and durable usage reservations, settlement/cancel/replay state; tenant limits cannot be lowered below outstanding exposure. | SQLite concurrency/restart/replay cases exist. All inference-path coverage and provider-bound maxima remain separate verification; no universal hard-dollar guarantee is claimed. |
| F04: missing or inconsistent model usage | Provider-native capture preserves known quantities and unknown states; local generation parses actual model counts; proposal adapter no longer substitutes a default zero-usage result. | Metered routes only. Other model, tool, embedding and summarization paths still require complete inventory and reconciliation. |
| F05: telemetry not invoice-grade | Integer micro-unit durable records bind tenant/task/attempt/provider/model/payer/rate revision; duplicate provider receipts cannot be charged twice; customer-direct usage is not debited as managed inference. | Platform compute/idle/support allocation, provider-invoice reconciliation and payment collection are not completed by this ledger. Customer payment collection remains disabled on the new usage API. |
| F06: unusable connection flow | Encrypted, tenant/provider-bound vault; supported OpenAI API/Stripe connection controls and read-only verification, revocation, recheck and stale-verification states. | Google Workspace OAuth lifecycle and wider connector support remain unverified/incomplete. A native subscription is not supported merely by having an API key field. No live credentials were tested. |
| F07: fabricated proposal terms | Intake preserves inquiry and validates owner-supplied line items/deposit with checked integer arithmetic. Missing pricing creates `NEEDS_PRICING`; optional unselected items are excluded from committed totals. | Real-stack persistence/isolation cases added; complete browser execution remains outstanding. |
| F08: fabricated checkout links | Invoice creation returns a draft without invented Stripe IDs/URLs. Real Stripe session client validates returned evidence and stable operation identity. Mercado Pago placeholder returns unavailable instead of a generated URL. | Full provider sandbox replay/payment event reconciliation, persisted provider receipts across every workflow and all business transitions remain outstanding. |
| F09: fictitious receivable reminders | Removed an unused module that merely logged a draft. The mounted worker persists actual source-grounded drafts and retires stale/paid-invoice drafts. | Drafting is not sending/delivery. Approval/revocation and provider delivery must be verified on the actual send path before claiming the full loop. |
| F10: stale exported desktop assets | Tauri packages freshly built, source-bound Next standalone assets and a pinned official Node runtime. `.env` exclusions apply to standalone, public and static content. Removed unused plaintext-key/placeholder IPC handlers. | Final Tauri build/install tests and resource startup on every platform are still required. Existing user config files were not deleted. |
| F11: smoke tests labeled full journey | Added seven real-stack business regression cases; preserved the full browser suite. Discovery now works without starting services and uses one Playwright runtime across nested npm trees. | 1,688 discovered tests are not 1,688 passed tests. Several older journeys reference obsolete static prototypes and require real behavior repair rather than removal. |
| F12: false completion/authority | Invoice-context and job-generation stubs no longer invent completed invoices; generic status changes cannot manufacture payment/delivery. Walk-up route is mounted, validates input/tenant and no longer succeeds after storage/model failure. Legacy voice no longer constructs tenant identity headers. | These are specific fixes, not certification of all simulation/approval paths. Unsupported workflows remain explicitly unavailable. |
| F13: BYOK versus subscriptions | API proxy rejects unsupported subscription-relay modes; verified tenant OpenAI keys bind to the provider origin and do not fall back to another payer after revocation. | Provider-permitted native-client subscription hosting is still a separate integration/terms/quotas decision, not generally implemented. |
| F14: economics/owner outcomes | Workload usage records and build/resource timing available; research keeps costs, owner correction time and actual outcome evidence separate. | No representative customer-serving cost, owner interviews, willingness-to-pay result or paid-retention result has been measured in this work. |
| F15: premature exclusive segment | Existing commerce, fulfillment and service modules preserved; earlier exclusive agency segment and fixed-price targets remain suspended. | Customer selection requires owner evidence, not a green build or competitor feature list. |

### Additional defects found during this pass

The full lint gate exposed **1,368 initial ESLint errors** across 486 files and **99 Rust files requiring formatting**. Rust formatting was repaired. ESLint's supported safe autofixes removed 44 errors; a subsequent inventory still had 1,324 errors, predominantly explicit `any`, unused values and TypeScript `require` imports. That count preceded the separately repaired parser/security defects and must not be reported as a final count. Rules and source tests were not excluded or weakened to obtain success.

Three JavaScript/TypeScript parser failures were concrete code defects: an orphaned brace sequence in a historical neighborhood contract, escaped template delimiters in the legacy voice script, and malformed interpolated HTML in the legacy help widget. Syntax fixes preserve the contract assertions. Help message/link/video rendering now uses safe DOM text and validated destinations instead of interpolated HTML/inline handlers; regression tests cover executable URLs, credential-bearing links and malicious titles. Voice no longer supplies a localStorage-derived workload identity or treats a response without transcription as a prepared action.

A first bounded Rust build failed at an imposed **6 GiB virtual-address-space limit** after 733 seconds. That was a validation setup limit, not measured resident-memory exhaustion. The subsequent monitored build passed in **220.72 seconds**, using cached dependencies and compiling the main server artifact, with **4,722.75 MiB peak process-group RSS**. Both use one Cargo build job and disabled incremental. The latter used `MALLOC_ARENA_MAX=2`. Neither is an untouched cold-runner CI baseline.

One isolated test setup was blocked by the host's enforced bwrap execution policy before tests ran; a separate read-only/no-network Docker validation setup was used without disabling host security. Its first attempt lacked `/etc/alternatives` for the system `cc` symlink and failed before tests; the corrected setup includes only that public system-tool directory. These failures are not counted as passing tests.

### Verification journal

- Native production Next build: passed in 54 seconds after removing stale symlinks; `.next` absent, installed npm dependencies reused. `target/validation/current-web-build.log`.
- Native backend/agent/worker binaries: passed in 220.72 seconds, dependency-warm/main-crate rebuild; RSS details in `target/validation/current-native-build-rss.json` and matching log.
- Full Rust formatting: passed after formatting the actual workspace, not by excluding files.
- Root native/build/CI script tests: **33 passed, 0 skipped**, including eight underlying image-archive tests, timing failures, complete browser discovery and Make command propagation. `target/validation/current-all-scripts.log`.
- Additional safe help/voice rendering tests: **5 passed**, including syntax checks of the repaired scripts. These are DOM/unit checks, not packaged desktop installation evidence.
- PostgreSQL CI contract and its behavioral anti-bypass tests: passed after the CI graph/refactoring changes; the actual PostgreSQL isolation suite still needs its normal runtime job.
- Full browser discovery: **1,688 tests / 453 files**, exit 0; `target/validation/browser-discovery.log`. This is enumeration only.
- Current-profile focused Rust regression run: execution result must be appended from the completed job; do not infer it from successful compilation or a previous run.
- Complete `make lint`, `make test`, Tauri packaging, actual production image/deployment suites, provider sandbox and hosted CI timing are not certified by the above partial results.


Date: 2026-09-18. Source baseline: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20` on `fix/bazel-modernization-and-cleanup`, plus existing research/documentation changes. This ledger implements the user's instruction to migrate from Bazel to native Rust/Cargo, Tauri and Node.js, then address every finding in [the detailed audit](business_capability_and_usage_economics_audit.md). Existing business logic, platform support, permission boundaries and useful tests must not be removed just to make migration green.

## Evidence recorded before changes

The audit records owner anecdotes, current product comparisons and provider-access distinctions separately from source findings. It does not establish willingness to pay, a winning segment, actual per-workflow cost, universal subscription access or production readiness. The previous fixed $99 offer and exclusive agency segment remain suspended hypotheses. This implementation is not permission to charge customers, contact prospects, publish or deploy.

The observed build delay was a focused pricing-budget Bazel invocation timing out during analysis at 180 seconds, before any test result. The existing Cargo workspace contains 59 packages, including the backend, builtin agent, integration libraries and the Tauri app. Rust 1.95.0 is installed in the host's user Cargo directory, which was absent from the tool execution PATH. Native compilation must be measured; merely removing Bazel does not guarantee instantaneous cold builds.

## Migration acceptance register

| ID | Required result | Initial status |
|---|---|---|
| M01 | Pin toolchain; coherent Cargo workspace/lockfile; headless tests do not compile the Tauri CLI or WebView | In progress |
| M02 | Native Node install, frontend build/test/typecheck, fresh exported assets wired into Tauri | In progress |
| M03 | Native targeted Rust checks/tests and complete scheduled regression lanes; no weakened checks | In progress |
| M04 | CI caches keyed by toolchain, OS/architecture, dependencies and build role; untrusted PRs cannot publish caches/secrets | In progress |
| M05 | Native release archives, desktop/mobile packaging, Docker build and documentation workflows, preserving signing and verification | In progress |
| M06 | Active developer scripts and OHC Automator prompts stop requiring Bazel; historical reports remain clearly historical | In progress |
| M07 | Before/after verification records: actual commands, exit status, tests executed, durations, remaining platform/environment blockers | In progress |

## All audit findings and acceptance evidence

| ID | Finding and source evidence | Required remediation / proof | Initial status |
|---|---|---|---|
| F01 | Current code implements a one-way pipeline where `auditor.rs` uses an `event_pipeline` that forwards accounted events to `export_rx`, and `hub.rs` writes them as metrics without calling `record_event` again. | One-way ingestion/accounting/export verified by `ingress_is_accounted_once_and_exports_do_not_feed_back` regression test | Closed |
| F02 | `services/billing/service.rs:49-85` uses global snapshots for an organization response | Auth-derived tenant; reject mismatch/blank identity; tenant+agent isolation tests | Closed |
| F03 | `pricing/budget.rs:49-84` increments before reporting over-limit | Atomic reservation before spend; settle/release/replay/restart/concurrency checks; invalid/overflow amounts fail closed | Closed |
| F04 | Model paths disagree on usage; proposal adapter returns default usage; proxy forwards streams | Preserve actual provider counts, model/request identity and missing-usage state; no invented free usage | Open |
| F05 | Current telemetry/cost reports are not an invoice-grade meter | Durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit | Closed |
| F06 | `tool_integrations.rs` returns 501/usable:false for secure connection | Verified supported-provider connection with encrypted storage, tenant binding, revoke/refresh behavior; unsupported providers remain explicitly unavailable | Closed |
| F07 | `proposals.rs:825-845` creates fixed $5,000 scope/deposit regardless of inquiry | Input/approved-business-rule driven draft; deterministic validated amounts; no unauthorized commitments or fabricated scope | Closed |
| F08 | `invoice.rs:45-48` and booking helpers fabricate checkout-looking URLs | Real provider session or explicit pending/unavailable state; persist provider IDs, validate money, idempotent retries | Open |
| F09 | Receivables code logs a drafted reminder before implementing draft/delivery | Persist real draft; distinguish draft/sent/delivered; dedupe and stop on payment/cancel/revocation | Closed |
| F10 | Tauri packages exported Next assets despite blanket legacy claims | Rebuild actual frontend assets; no stale checked-in export used as release proof | Open |
| F11 | Named full-journey tests only delegate to a smoke helper | Preserve smoke coverage; add actual mutation/state/provider-boundary acceptance tests without live credentials | Open |
| F12 | Simulation, unknown provider outcome and approval paths can look like completion | Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths | Open |
| F13 | API key, consumer plan and native-client subscription are distinct | Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference | Open |
| F14 | No measured representative serving costs or owner outcomes | Workload/cost instrumentation and repeatable benchmark/export; do not claim interviews, customer acceptance, real costs or competitive advantage without evidence | Open |
| F15 | Existing commerce/fulfillment assets and varying owner stories contradict premature exclusive segment | Preserve modules; keep reusable workflow/owner evidence and commercial decisions separate from engineering readiness | Open |

## Completion rules

An item becomes implemented only after its production path is changed. A new library with no caller is not completion. An item becomes verified only after the declared tests actually run. Static checks, unit tests, boundary doubles, provider sandbox verification and live owner evidence are different levels. External account approval, code signing, live-provider cost reconciliation and owner interviews remain external verification requirements unless actually performed. No item is silently dropped; partial work and blockers remain visible here.
