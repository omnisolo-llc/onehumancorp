ld/CI script tests: **33 passed, 0 skipped**, including eight underlying image-archive tests, timing failures, complete browser discovery and Make command propagation. `target/validation/current-all-scripts.log`.
- Additional safe help/voice rendering tests: **5 passed**, including syntax checks of the repaired scripts. These are DOM/unit checks, not packaged desktop installation evidence.
- PostgreSQL CI contract and its behavioral anti-bypass tests: passed after the CI graph/refactoring changes; the actual PostgreSQL isolation suite still needs its normal runtime job.
- Full browser discovery: **1,688 tests / 453 files**, exit 0; `target/validation/browser-discovery.log`. This is enumeration only.
- Current-profile focused Rust regression run: execution result must be appended from the completed job; do not infer it from successful compilation or a previous run.
- Complete `make lint`, `make test`, Tauri packaging, actual production image/deployment suites, provider sandbox and hosted CI timing are not certified by the above partial results.


Date: 2026-09-18. Source baseline: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20` on `fix/bazel-modernization-and-cleanup`, plus existing research/documentation changes. This ledger implements the user's instruction to migrate from Bazel to native Rust/Cargo, Tauri and Node.js, then address every finding in [the detailed audit](business_capability_and_usage_economics_audit.md). Existing business logic, platform support, permission boundaries and useful tests must not be removed just to make migration green.

- Sentinel Tenant/Client Isolation Audit: Verified no active unmitigated CRITICAL local data exposure vulnerabilities or multi-tenant leakage confirmed in immediate local code paths during this run. No active unaddressed isolation violations in standard endpoints.

## Evidence recorded before changes

The audit records owner anecdotes, current product comparisons and provider-access distinctions separately from source findin