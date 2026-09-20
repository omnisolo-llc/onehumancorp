# Native development, testing and build caches

The [2026-09-19 measured cleanup record](../research/native_build_measurements_2026-09-19.md) records an **8m 33.94s empty-output backend build**, **1.87s unchanged rerun**, and **31.40s fresh Node build**. Dependency downloads/toolchains were already available; this is not completely cold hosted CI. Keep the **10-minute core backend compilation goal** distinct from the **30-minute full required-CI target**. Repository-wide lint and full execution gates remain mandatory.

Use the same `CARGO_TARGET_DIR` with Cargo and `npm run test:e2e`; the browser runner honors custom output directories instead of using old default-directory binaries. Web output remains at `target/native-web` and must pass its source/platform/Node/lockfile checks.

Updated 2026-09-19. The supported build is Cargo/Rust + Next.js/Node + Tauri, not Bazel. Root `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.node-version`, and the root/Next/CLI npm lockfiles are authoritative.

## Canonical quality gates

Run `make lint` and `make test` from the repository root before accepting an implementation. GNU Make invokes the native tools; failures stop the target and return a nonzero exit status, including with `make -j`. These targets never silently skip an unavailable tool or test lane.

- `make test`: all Cargo workspace members (including Tauri), Node script tests, Next Vitest, CLI Vitest with coverage, desktop UI Vitest, native contracts and the complete real-stack Playwright discovery. E2E builds fresh Rust binaries and Next output first.
- `make lint`: rustfmt check, Clippy for all workspace targets with warnings denied, ESLint for handwritten JS/TS, and Next/CLI TypeScript checks. Generated output and dependency directories are excluded from ESLint, not source tests. Existing lint debt is a failure, not an automatic exemption.
- Focused lanes: `make test-rust`, `make test-backend`, `make test-node`, `make test-contracts`, `make test-e2e`, `make lint-rust`, `make lint-node`. `make test-backend` excludes Tauri and cannot certify the full suite.
- `make test-e2e E2E_ARGS='--workers=1'` adjusts browser concurrency. Filters are diagnostic only; do not claim full acceptance for filtered runs. `make build-web` builds just the maintained web package.

Full gates require GNU Make, the pinned Rust/Node toolchains, Python 3 with PyYAML, native Tauri/WebKit/GTK development libraries for the host, a running Docker daemon and Playwright Chromium plus its OS dependencies (`npx --no-install playwright install --with-deps chromium`). Install locked npm dependencies at root, `src/ui/next`, and `src/cli`. See the native CI setup action for platform package lists. Mobile SDK, signing and physical-device checks remain separate release verification, not implied by a host workspace pass.

## Setup and boundaries

Install Rust with rustup and the pinned Node release. Ensure `$HOME/.cargo/bin` is on PATH on Unix. Use `npm ci` at the repository root, `npm --prefix src/ui/next ci` and `npm --prefix src/cli ci`; none should update a lockfile. Use `--locked` with Cargo in automation. Proto generation uses the workspace build scripts and vendored protoc; do not invoke deleted Bazel targets.

The default Cargo members are the backend and harness worker. `app` is the Tauri package and is excluded only from focused headless checks. Full `make test` and `make lint` include it and require native desktop dependencies.

```sh
cargo check --locked --workspace --exclude app --all-targets
cargo test --locked -p server_services_billing
cargo test --locked -p server_pricing
cargo test --locked -p server_harness
cargo test --locked -p server_integrations_stripe
cargo test --locked --workspace --exclude app
npm run test:contracts
npm run typecheck:web
npm run test:web
```

Focused checks accelerate iteration; they do not replace the complete regression suite. Test output must include nonzero executed tests where tests are expected. No `--pass-with-no-tests`, hidden failures, disabled assertions or changed business expectations merely to obtain green output.

## Build once, consume real artifacts

```sh
cargo build --locked -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins
npm run build:web
npm run desktop:build -- --debug --no-bundle
npx --no-install playwright install chromium
npm run test:e2e -- --workers=1
```

`build:web` compiles the maintained Next application and only then creates a source-bound proof and standalone package in `target/native-web`. Tauri owns a loopback Node server for this package. The authenticated Next server routes are retained; a static HTML export would remove necessary application behavior. The packaged executable is a separate official Node distribution, verified against the pinned release checksums and shipped with its license—not a copied CI `process.execPath` that might depend on shared host libraries.

The package manifest binds source, dependency lock, build ID, Node version, OS and architecture. Packaging refuses stale output even when dependency locks are unchanged. Environment files and build caches are excluded from standalone, public and static content. Never merge old artifacts into a new bundle. `OMNISOLO_PREBUILT_WEB` may reuse a validated artifact from the same workflow/source/platform, not an arbitrary cached directory.

The Rust API runs separately, either locally or on a configured HTTPS host. Desktop owns only its packaged Node process; it must not claim to provision or supervise a missing Rust backend. Mobile builds point at explicit HTTPS `OMNISOLO_MOBILE_WEB_URL` and do not bundle a desktop Node runtime. Platform SDKs, signing keys, store enrollment and actual device/install tests remain separate release prerequisites.

## CI cache design

`.github/actions/setup-native/action.yml` configures **dependency-only** caches by OS, architecture, runner image and job role. Rust-cache supplies the installed-compiler, Cargo manifest/lock, configuration and compiler-environment hashes, plus compatible dependency fallback behavior. A second manual manifest hash is not added to the job key. The `ohc-native-dependencies-v1` namespace retires previous caches containing workspace binaries. Source-bound workspace crates, installed toolchain executables, incremental graphs and failed builds are not saved; application/test binaries travel only as same-run artifacts. Cargo always revalidates fingerprints, and restored dependencies are not test results. Backend, desktop, release and cross-target jobs do not share incompatible target caches.

This follows [rust-cache's documented dependency-cache behavior](https://github.com/Swatinem/rust-cache#cache-details) and avoids relying on updating an immutable cache with newer application binaries; [GitHub requires a new key to change cached contents](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching). The first run in the new namespace is expected to miss. Measure restore/save duration and total cache size before claiming a CI speedup.

npm caches compressed dependency downloads, not a reusable `node_modules` tree. Each job installs only its declared locked dependency trees: `root` for consumers of prebuilt desktop/browser artifacts, `web` for the root + Next build, or `all` for root + Next + CLI quality checks. The scope is part of the cache key. The full Node quality lane still tests all three areas; dependency minimization does not remove a test. The Next cache contains compiler cache only; the complete application build runs on every source revision. Built executables and web output are passed between jobs as artifacts, not confused with dependency caches.

Only successful trusted main/tag runs may save the applicable compiler cache; pull requests restore but do not write shared trusted caches. Signing material, provider keys, `.env`, test sessions and live databases must never enter caches or artifacts. Cache entries need version/role changes when the build contract changes. Avoid saving failed builds or unbounded local caches into CI.

## Disk, memory and iteration speed

This migration review found a 16 GB `target/debug/incremental` directory on a full 244 GB host disk. Removing only that generated cache recovered space without deleting source or compiled dependencies. Never use a broad `git clean` or remove a whole worktree to address a build cache issue.

Development incremental compilation stays available. Dev/test profiles explicitly share 256 codegen units and line-table debugging; test debug assertions and overflow checks remain enabled. Disabling incremental must not silently change a large crate to the non-incremental default codegen-unit count. The test profile disables incremental object graphs; the CI setup explicitly exports `CARGO_INCREMENTAL=0` because these graphs are not persisted. `CARGO_BUILD_JOBS=1` is useful on memory-constrained/shared hosts; `.cargo/config.toml` defaults to two jobs. Set `CARGO_INCREMENTAL=0` for bounded-space batch builds. A large native cold compile still has a real cost; this migration removes Bazel analysis and duplicate build plumbing, not all Rust compilation.

Before deleting generated caches, confirm their exact project-relative path, that they contain no tracked files, and that no compiler owns them. Do not delete another project's caches. Keep test logs under `target/validation` and record elapsed time, toolchain, workload and whether a cache was warm. Do not claim a speedup from comparing a warm native run to a cold Bazel run.

## CI critical path and measured time budget

The engineering target is **X = 30 minutes for the complete Linux CI required gate**, with a **15-minute warm-cache stretch target**. A provisional **10-minute core-build target** covers backend, Next and Tauri compilation; it does not substitute for the complete test/security/deployment gate. These are acceptance targets, not achieved CI percentiles. Desktop/mobile signing, publishing and other release platforms are separate workflows and are not represented as fitting this budget.

The CI graph now separates work that can run independently:

- Rust executable build publishes this run's backend, agent, worker and mTLS probe. The headless test/lint job and desktop test/lint job partition the complete Rust workspace without making the desktop lane rebuild every backend crate.
- The production Next build publishes promptly. Root/web/CLI/legacy-desktop Node tests, typechecks and lint run in an independent **required** job; a passing build cannot bypass them.
- Four browser shards consume the same source-validated web/binary artifacts and may run concurrently. They retain the complete browser discovery, not a smoke allowlist.
- One production Docker build produces the server/agent image layers. Kind and Compose both load that same run's archive and still execute their full deployment checks. `scripts/native-images.py` checks the source fingerprint, tar checksum, image tags and loaded image IDs; caches and arbitrary local images are not accepted as current build evidence. Both deployment suites reuse the compiled mTLS probe rather than installing another Rust toolchain and recompiling it.
- PostgreSQL tenant-isolation tests remain independently required under the non-superuser application role. `CI Required` fails on failed/cancelled or unexpectedly skipped builds, tests, lint, dependency audit and security lanes.

`workflow_dispatch` accepts `cold_cache: true`. It disables restored Rust, npm-download, Next and Docker build caches; freshly compiled layers may still be reused **within the same run**. Hosted toolchain downloads/base operating-system images are outside this project-cache definition. This input does not remove any acceptance check and is not an instruction to dispatch a workflow without authorization.

The final gate fetches the complete job list for the **exact run attempt** with read-only Actions permission. `scripts/ci-performance.py` records elapsed time from the earliest job start to the final gate start, including waits between dependent jobs. It excludes the initial queue before the first job and the final reporting/upload itself. Missing/inconsistent pagination, wrong run/attempt, unfinished predecessor jobs and invalid timestamps fail closed. Runs above 30 minutes fail the performance check; failed functional jobs never become successful performance evidence. Markdown and JSON reports are uploaded as `ci-performance-<attempt>` and shown in the Actions summary. Cache mode is labeled as requested, not claimed to be a hit.

### Local observations on 2026-09-19

| Workload | Observed result | Cache and resource limits |
|---|---|---|
| Fresh Next production build and source-validated packaging | Passed, 54 seconds | `.next` absent; installed npm dependencies reused. Not a clean-runner CI measurement. |
| Backend + agent + worker executable build | Passed, 220.72 seconds; peak process-group RSS 4,722.75 MiB | Dependency artifacts reused; main server artifact absent after a failed bounded attempt; Cargo jobs 1, incremental off, `MALLOC_ARENA_MAX=2`. |
| Earlier dependency compilation attempt | Failed after 733 seconds at an imposed 6 GiB **virtual-address-space** ceiling | Not proof that resident memory exceeded 6 GiB. Replaced with measured RSS-based protection; do not report this failure as a passed cold baseline. |
| Browser discovery | 1,688 tests across 453 files | Listing only, not execution; no Docker, model keys or running database required for discovery. |

A clean build means a fresh source checkout can pass the full declared gates; a warm invocation, successful binary build or a configured timeout alone cannot certify that. See the remediation ledger for outstanding lint and runtime acceptance work.

## Browser and provider verification

The native E2E runner starts isolated PostgreSQL/Valkey containers, the real Rust binaries and the newly built web package. It seeds only its own database and reconstructs an environment without production database or provider credentials. CI shards the complete browser-spec discovery. Per-test filters are available for local diagnosis without changing CI discovery.

Provider-boundary test servers can supply deterministic external contracts; the internal UI/API/database must remain real. Label those results as contract verification, not live provider or customer evidence. Billing/provider failure tests must not make real charges. Signing, public publishing, infrastructure changes and customer actions require explicit applicable authority.

## Migration status

See `docs/research/native_migration_and_remediation.md` for the detailed finding-to-code-to-test ledger. A source fix, a passing unit test, a running sandbox flow and a live owner outcome are different evidence levels. Keep those distinctions in release notes and research.
