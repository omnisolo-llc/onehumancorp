# Native CI quality cleanup implementation plan

> Execute inline with the executing-plans and test-driven-development workflows. This continues the owner-authorized migration; do not start another frontend or build system.

**Goal:** Make the native build and required quality gates reproducible, repair actual failures, and measure rather than assume the CI time budget.

**Architecture:** Preserve the existing Cargo + Next/Node + Tauri composition, the complete Make acceptance gates, and same-run artifact consumption. Prefer dependency/type/fixture repairs and explicit ownership over suppression, broad rewrites or repeated clean builds.

**Tech stack:** Rust 1.95.0; Node 22.22.1; Cargo; npm; Tauri; Vitest; Playwright; GitHub Actions.

**Spec:** `docs/development/native-build.md`, `docs/research/native_migration_and_remediation.md`, and the current owner instruction. Baseline is HEAD c3716d0875df6403322af4fb47d9f56f9042af3c plus preserved dirty work; HEAD alone does not identify these sources.

## Global constraints and rulings

- Preserve all implemented business modules, provider isolation, authority checks and meaningful test discovery/assertions.
- No lint-rule relaxation, test allowlist, swallowed failure, cached-test-success substitution, or time-budget increase to achieve green.
- No broad cache/source deletion, live credentials/provider requests, deployment, publication or push.
- One heavy Rust validation at a time on the shared host. Observe existing jobs before retries; do not interfere with another project's processes.
- Ruling: X was not numerically specified. Keep the already implemented **30-minute full Linux required-CI** limit. Aim for **10-minute warm-cache core/PR feedback** without using it to replace any required lane. Cold compilation, full tests, queue/setup, and releases remain distinct measurements.
- Ruling: this is a continuation of an existing approved native architecture, not a newly commissioned design/feature backlog. Record remaining external and product gaps rather than inventing evidence.

## Review focus

Check stale source/artifact reuse; tenant and money semantics changed by cleanup; side effects of unused initializers; async test isolation/deadlocks; unexpected skipped/zero-test results. Mechanical rewrites must be exact, guarded, inspectable and followed by compilation and tests.

## Task 1 — recover and inventory (read-only)

- [x] Read current instructions, migration ledger, Make targets and native CI graph; preserve newer work rather than replaying historical patches.
- [x] Establish responsive runner, branch and working tree. Fresh ESLint inventory: 807 messages in 330 files (428 explicit-any, 279 unused-variable, 51 require-import, 49 other).
- [ ] Capture current strict Clippy diagnostics in `target/validation/round3-clippy.jsonl`; inspect source and production callers before changing interfaces.

## Task 2 — repair native Rust quality and regression defects

Files: exact Cargo diagnostic paths; canonical modules and their existing tests. No package/test exclusions.

- [ ] Use the failing strict Clippy result as the baseline; repair safe duplicated branches and naming/type issues while retaining public compatibility where necessary.
- [ ] Replace vacuous assertions with observable production behavior, not a renamed or suppressed assertion.
- [ ] Run `cargo fmt --all -- --check` and `cargo clippy --offline --locked --workspace --exclude app --all-targets --keep-going -- -D warnings` on final source.
- [ ] Run focused changed tests, then `make test-backend`; record counts and any infrastructure prerequisite separately.

## Task 3 — repair JavaScript/TypeScript quality

Files: diagnostics in `target/validation/round3-eslint.json`, existing component/API/test files, no changes to ESLint rule coverage.

- [ ] Resolve unused bindings without removing initializer effects, hooks or protocol fields; use AST-derived exact guarded edits and review the diff.
- [ ] Replace dynamic `any` values with concrete model/request/mock types and unknown-input narrowing. Do not disguise `any` behind a type alias.
- [ ] Replace CommonJS imports in TypeScript with appropriate ESM/static types without invalidating test mocks.
- [ ] Run `make lint-node`, `make test-node` and `make test-contracts`; fix behavior failures, then rebuild source-validated web output.

## Task 4 — CI acceptance and evidence

Files: `.github/workflows/ci.yml`, `.github/actions/setup-native/action.yml`, native runner/contract tests and this ledger, only where new evidence identifies a defect.

- [ ] Review the critical path for duplicate compilations/downloads; preserve every required lane and cache trust boundary.
- [ ] Verify Tauri lint/test/build, fresh business browser regressions and complete test discovery/execution. Failure or unavailable prerequisites cannot be reported as passed.
- [ ] Record exact commands, source conditions, cold/warm timings, test counts and outstanding failures in the remediation ledger. A configured timeout or a local core build is not hosted-CI certification.

## Progress

Initial recovery is complete. The earlier symlink and unknown-outcome invoice issues have already been addressed in newer work. Prior local measurements (8m33.94s empty-output backend and 31.40s fresh web build) are historical source-qualified evidence, not results of this pass. Final acceptance remains open until the checks above return successful terminal results.
