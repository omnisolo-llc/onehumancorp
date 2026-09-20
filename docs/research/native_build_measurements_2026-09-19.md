# Native cleanup and measured build performance

Date: 2026-09-19. Repository: OneHumanCorp, branch `fix/bazel-modernization-and-cleanup`, HEAD `c3716d0875df6403322af4fb47d9f56f9042af3c` plus the existing uncommitted migration and this cleanup. HEAD alone does **not** identify the tested dirty source. This record supplements [the remediation ledger](native_migration_and_remediation.md), not a claim that all audit findings are complete.

## Budgets and measured results

Use **10 minutes as the core Linux backend compilation goal**, **30 minutes as the full required Linux CI target**, and **15 minutes as the cached full-CI stretch target**. The latter two remain targets, not demonstrated GitHub Actions results. A successful compiler command is not a passing `make lint`, `make test`, signed release matrix or hosted CI attempt. Do not raise timeouts, exclude tests or disable warnings to manufacture a passing result.

The local host uses Linux x86_64, Rust 1.95.0 and Node 22.22.1, with approximately 15 GiB usable memory and other workloads. `/usr/bin/time -v` measures elapsed command time and maximum reported process RSS, **not summed memory for the entire process group**. Setup, dependency downloads, cache transfer and CI scheduling are not included unless stated.

| Measurement | Cache/source condition | Elapsed | Peak reported RSS | Result |
|---|---|---:|---:|---|
| Backend, agent, worker and probe | New empty `target/ci-cold-20260919-r2`; downloaded dependencies present; offline; two compiler jobs; no incremental cache | 8m 33.94s | 5,011,056 KiB | Exit 0 |
| Exact unchanged backend rerun | Same command/output directory; no intervening Rust changes | 1.87s | 155,752 KiB | Exit 0 |
| Complete frontend unit suite | Installed dependencies; two workers; before the subsequent proxy convention change | 3m 19.78s | 252,576 KiB | 1,506 passed across 338 files; zero failed/skipped/todo |
| Fresh Next build/package | Existing Next compilation cache; actual compiler invocation | 30.97s | 954,176 KiB | Exit 0; identified deprecated middleware/Edge runtime warnings |
| Fresh Next rebuild after Node proxy cleanup | Existing Next compilation cache; new proxy and Node share-card route | 31.40s | 1,442,444 KiB | Exit 0, empty stderr; source-bound package generated |
| Earlier complete frontend checkpoint | Node proxy/share-card source before further typed tests | 3m 35.57s | 250,392 KiB | 1,509 passed across 339 files; zero failed/skipped/todo |
| Refreshed complete frontend suite | Source digest identical before/after; two workers | 3m 30.82s | 250,968 KiB | 1,516 passed across 340 files; zero failed/skipped/todo |
| Refreshed source-bound web package | Fresh compiler invocation after further worktree changes | 29.21s | 1,188,152 KiB | Exit 0; compiler source proof and package validation succeeded |
| Linux Tauri debug/no-bundle | Fresh verified web package; installed desktop SDKs; one compiler job; dependency rebuild | 6m 42.00s | 1,301,772 KiB | Exit 0; Cargo's compilation portion was 6m 34s |
| Later backend changed-package rebuild | Same native target; Calendar/Shopify/worker cleanup, before final tenant-display patches | 4m 13.75s | 3,494,436 KiB | Exit 0, no compiler warnings |


The backend result proves a clean **compiler-output** build below ten minutes on this host. It does not prove a completely cold hosted runner, release/LTO compilation, or the entire CI pipeline below ten minutes. Later small Calendar/Shopify mapper and test-typing edits require final-source compilation; do not silently relabel the benchmark as a measurement of a later untested source revision.

Evidence: `target/validation/round2-*.time`, `round2-web-tests.json`, `round2-web-tests-final.json`, `round2-web-closeout.json`, WebCodex command records, and `target/ci-cold-20260919-r2/cargo-timings/`. The refreshed frontend suite's stable source digest is `7a0c281e4eba1ff0a82920c4b1f97c0b6d13d0d1919e5f8059c805919f314953`. Generated logs and outputs are not source and must not be committed as evidence that future revisions pass.

### Reproduction

Choose a **new** directory instead of deleting another build cache. With dependencies available:

```sh
mkdir target/ci-cold-20260919-r2
CARGO_TARGET_DIR=target/ci-cold-20260919-r2 \
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 MALLOC_ARENA_MAX=2 \
cargo build --offline --locked \
  -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker \
  --bins --timings
```

Use another new directory name when the example already exists. Run the exact command again without source edits for unchanged-cache timing. Dependency/network-cold measurements require a disposable environment and separate setup/download accounting; never wipe the shared host registry, other projects or active validation directories.

## Fixes and regression evidence

### Atomic files and owned cleanup

`src/server/utils/fs.rs` previously staged atomic writes on the shared temporary filesystem and used a non-atomic copy fallback after `EXDEV`. It now stages an exclusively created randomized file beside the destination, syncs it and renames it. Failed replacement removes staging and preserves the original; plain relative filenames work correctly.

The cleanup helper previously selected generic `.tmp`, `.tmp.rs`, `.tmp_py_*.py` and `test_*.log` names across the shared host temporary directory. Filenames do not prove ownership. It now visits only the application-owned atomic-write namespace and configured runtime memory directory, requires regular `.tmp` files older than one hour, preserves fresh/non-temporary files and rejects directory/file symlinks. **Five focused tests passed**, including foreign-file preservation, failed replacement and symlinks. No host-wide cleanup or other-project deletion is part of this change.

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
