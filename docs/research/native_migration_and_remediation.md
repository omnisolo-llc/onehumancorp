[Output truncated for brevity]

ts only the application-owned atomic-write namespace and configured runtime memory directory, requires regular `.tmp` files older than one hour, preserves fresh/non-temporary files and rejects directory/file symlinks. **Five focused tests passed**, including foreign-file preservation, failed replacement and symlinks. No host-wide cleanup or other-project deletion is part of this change.

### Dependency graph

Removed the unused root `image` dependency and the pricing image crate's unused default AVIF encoder graph (`ravif`/`rav1e`). In the installed image crate, AVIF decoding requires `avif-native`, which was not enabled. Explicit features retain existing decoders, Rayon and WebP output. Cargo.lock was reconciled offline without a general update. **Twelve compression tests passed**, including PNG/JPEG/GIF/BMP/WebP input fixtures, WebP output and resize dimensions. This does not add or remove previously enabled AVIF decoding.

### Environment-test isolation

Five tests held blocking module-local mutexes across async work while mutating the global environment. Such locks cannot protect other modules or libraries. The test-only `src/server/harness/ambient_test.rs` runs an exact named test in a child with explicit environment and a dummy canary, a bounded Tokio runtime, a parent deadline and child reaping. A distinct completion exit code prevents an empty `--exact` selection from reporting false success. **All five isolation tests passed**; production environment filtering and actual provider credentials are untouched.

### Frontend/test cleanup

Reviewed onboarding, pricing, plan, cost-dashboard, morning-briefing, reply-card, WebSocket and sync-manager code now uses typed mocks, `RequestInit`, `Location`, concrete approval shapes and real `Response` objects where required. Assertions and initializer effects were preserved. Constant Playwright fixtures no longer need unused async setup arguments. The eleven specifically cleaned frontend/fixture files passed strict ESLint; that is not a whole-repository lint pass.

Tooltip tests now use fake-clock boundary checks instead of several seconds of wall-clock sleeps: show at 500 ms, hide after 2,000 ms, and cancellation/movement behavior. A former resize `expect(true)` now checks actual positioning before/after the 150 ms debounce with the newest viewport. A new unmount case caught a production timer-disposal defect; cleanup now cancels the pending scroll timer. **Ten tests passed in 416 ms of test execution**, with stronger assertions rather than reduced coverage.

### Native Node execution

Moved the maintained framework entrypoint from `src/middleware.ts` to `src/proxy.ts`, preserving the auth evaluator, explicit environment selection, private failure responses, canonical-origin redirects and cookie deletion. The storefront share-card route now runs in Node, preserves SVG escaping/cache semantics, and does not echo malformed input in errors/logs.

Official migration guidance checked on 2026-09-19: [Middleware to Proxy](https://nextjs.org/docs/messages/middleware-to-proxy), [Edge runtime deprecation](https://nextjs.org/docs/messages/edge-runtime-deprecated). No Next upgrade or downloaded codemod was used. **288 authentication/share-card tests passed** across sixteen files; the subsequent production build had empty stderr. Internal `middlewareCore` is a generic evaluator name, not a deprecated file convention.

### Cache and test-runner contracts

The browser runner now honors `CARGO_TARGET_DIR`; it previously could execute stale default-directory binaries despite a successful custom-target Cargo build. Cases cover default/relative/absolute directories, Windows executable names, blank-value rejection and keeping build-only settings out of the application environment.

CI-script assertions preserve the measured compiler settings: CI incremental compilation disabled, Linux allocator arenas bounded, line-table debugging, 256 codegen units and test debug/overflow checks enabled. Existing role/OS/toolchain/dependency cache isolation and trusted-writer restrictions remain. Cache hits accelerate compilation; they never stand in for executed tests.

The storefront Rust no-op test was replaced with cache-control, tenant tags, content-hash changes and header-injection checks. Calendar/Shopify mapping was simplified without dropping entries. Private HTTP/transport helper errors are boxed without changing wire status/body; recorded-command/message-handler types and test configuration literals are explicit. Final-source verification of these latest changes must be recorded, not inferred from older passes.

## Additional tenant-cost exposure found during cleanup

The original billing-service fix did not cover every cost display. Source review found global totals or agent-only lookup still used by `services/dashboard/service.rs`, `services/agent/service.rs`, `api/agent_metrics.rs` and `billing.rs::Tracker::summary`. Two businesses sharing an agent identifier could therefore still expose or mix costs outside the corrected billing RPC.

Those paths now use tenant-scoped snapshots, with totals and per-agent entries derived from the same snapshot where possible. The agent-manager mobile response no longer silently substitutes zero cost/tokens. Dashboard and agent-manager cache keys have new tenant-scoped version namespaces, preventing reuse of older Redis entries populated with global data; hire/fire/delegate invalidate both full/mobile entries in the new namespace. Missing/padded tenant identity fails before cache lookup. The generic Tracker summary honors its tenant argument instead of ignoring it. Agent-manager join failures now return unavailable errors rather than panicking.

Regression coverage checks identical agent IDs across tenants, full/mobile totals, stale legacy cache entries, invalidation after additional costs, unknown tenant summaries and authorization boundaries. All **13 selected cost/display/header tests passed** in `round2-tenant-visibility-final.log`. However, a separate Rust worktree edit landed during that invocation, so its full-source fingerprint check correctly exited 1 rather than certifying the current tree. This is a passing test checkpoint, not a stable-final-source result. An earlier version of the agent-metrics test exposed a fixture error: the default tariff was zero, making both compared costs zero. The corrected test supplies explicit unequal 125/900-cent costs and asserts the exact own-tenant value; no production tariff or isolation assertion was relaxed.

### Current runtime verification boundary

The seven `native_business_regression` browser cases passed against the newly compiled custom-target backend and new Node proxy package: unpriced intake, deterministic priced proposals, invalid deposits, unavailable payments, invoice amount/currency consistency, cross-business visibility, and owner-only spending limits. Browser execution took 16.8 seconds, excluding container startup/builds. This result precedes the final tenant-display patches and must not be presented as validation of an untested later binary.

The Linux Tauri build passed after fixing the test invocation's PATH to expose the already installed Cargo toolchain. It proves native debug compilation with the fresh Node resources, not signed installer validation, a GUI launch, mobile parity or a new deployment. The refreshed native CI-script suite passed **59 tests**, including compiler-timing artifact capture and custom-target browser inputs. All thirteen preserved deployment/security contract groups also passed.

## Gates still requiring a passing result

**No clean full CI run has been established.** Full ESLint initially found 942 errors. The refreshed full snapshot (`round2-eslint-latest.json`) found **723 errors across 294 files**, with zero warnings: 359 explicit-any, 266 unused-variable, 49 CommonJS-import, 21 empty-block, 11 unnecessary-escape, 10 unused-assignment, five rest-parameter and two const-preference diagnostics. This includes other preserved worktree edits; do not attribute every reduction to one patch. These remain actual failed required checks, not an optional warning budget. No blanket ignores or rule relaxation were added.

The last completed strict Clippy checkpoint, before the additional tenant-display fixes, (`cargo clippy --locked --workspace --all-targets --keep-going --message-format=short -- -D warnings`) exited **101** with **87 main-server/test diagnostics**: 70 library diagnostics and 17 additional test diagnostics. The earlier Calendar/Shopify/NATS/worker/builtin diagnostics were corrected. Remaining server debt includes large transport error values, repeated branches, over-wide function argument lists, complex types, and eight always-true test assertions. These are actual failing release gates, not runner outages. `cargo clippy --fix` exiting zero is not a clean `-D warnings` result.

A subsequent compilation encountered an in-flight refactor mismatch in `api/sync.rs`: an early authorization error returned a tuple while a successful WebSocket upgrade returned an HTTP response. The narrow fix converts the error with `into_response()`; the final native validation log must confirm the corrected source. One earlier running validation was lost when the Runner process was replaced. No surviving owned compiler remained before retrying; lost outcomes were not counted as passed.

Full workspace execution, the complete browser matrix, signed multi-platform builds, provider sandbox verification and an actual hosted 30-minute CI attempt remain separate gates. The focused business E2E success and Linux desktop compilation above do not certify the whole matrix. Discovery counts and cached artifacts are not current E2E certification.

No commits, pushes, deployments, releases, customer messages or live inference/payment charges were made. Unverified commercial/customer evidence and the remaining audit findings stay visible in the main ledger.

| ID | Finding and source evidence | Required remediation / proof | Initial status |
|---|---|---|---|
| F01 | Current code implements a one-way pipeline where `auditor.rs` uses an `event_pipeline` that forwards accounted events to `export_rx`, and `hub.rs` writes them as metrics without calling `record_event` again. | One-way ingestion/accounting/export verified by `ingress_is_accounted_once_and_exports_do_not_feed_back` regression test | Closed |
| F02 | `services/billing/service.rs:49-85` uses global snapshots for an organization response | Auth-derived tenant; reject mismatch/blank identity; tenant+agent isolation tests | Closed |
| F03 | `pricing/budget.rs:49-84` increments before reporting over-limit | Atomic reservation before spend; settle/release/replay/restart/concurrency checks; invalid/overflow amounts fail closed | Closed |
| F04 | Model paths disagree on usage; proposal adapter returns default usage; proxy forwards streams | Preserve actual provider counts, model/request identity and missing-usage state; no invented free usage | Open |
| F05 | Current telemetry/cost reports are not an invoice-grade meter | Durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit | Blocked |
| F06 | `tool_integrations.rs` returns 501/usable:false for secure connection | Verified supported-provider connection with encrypted storage, tenant binding, revoke/refresh behavior; unsupported providers remain explicitly unavailable | Verified |
| F07 | `proposals.rs:825-845` creates fixed $5,000 scope/deposit regardless of inquiry | Input/approved-business-rule driven draft; deterministic validated amounts; no unauthorized commitments or fabricated scope | Closed |
| F08 | `invoice.rs:45-48` and booking helpers fabricate checkout-looking URLs | Real provider session or explicit pending/unavailable state; persist provider IDs, validate money, idempotent retries | Closed |
| F09 | Receivables code logs a drafted reminder before implementing draft/delivery | Persist real draft; distinguish draft/sent/delivered; dedupe and stop on payment/cancel/revocation | Closed |
| F10 | Tauri packages exported Next assets despite blanket legacy claims | Rebuild actual frontend assets; no stale checked-in export used as release proof | Closed |
| F11 | Named full-journey tests only delegate to a smoke helper | Preserve smoke coverage; add actual mutation/state/provider-boundary acceptance tests without live credentials | Blocked |
| F12 | Simulation, unknown provider outcome and approval paths can look like completion | Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths | Open |
| F13 | API key, consumer plan and native-client subscription are distinct | Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference | Open |
| F14 | No measured representative serving costs or owner outcomes | Workload/cost instrumentation and repeatable benchmark/export; do not claim interviews, customer acceptance, real costs or competitive advantage without evidence | Verified blocked outcome due to missing prerequisites and owner economic/metric data. |
| F15 | Existing commerce/fulfillment assets and varying owner stories contradict premature exclusive segment | Preserve modules; keep reusable workflow/owner evidence and commercial decisions separate from engineering readiness | Closed |

## Completion rules

An item becomes implemented only after its production path is changed. A new library with no caller is not completion. An item becomes verified only after the declared tests actually run. Static checks, unit tests, boundary doubles, provider sandbox verification and live owner evidence are different levels. External account approval, code signing, live-provider cost reconciliation and owner interviews remain external verification requirements unless actually performed. No item is silently dropped; partial work and blockers remain visible here.
