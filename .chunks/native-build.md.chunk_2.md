red-CI ceiling**. Repository-wide lint and full execution gates remain mandatory.

Use the same `CARGO_TARGET_DIR` with Cargo and `npm run test:e2e`; the browser runner honors custom output directories instead of using old default-directory binaries. Web output remains at `target/native-web` and must pass its source/platform/Node/lockfile checks.

Updated 2026-09-19. The supported build is Cargo/Rust + Next.js/Node + Tauri, not Bazel. Root `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.node-version`, and the root/Next/CLI npm lockfiles are authoritative.

## Canonical quality gates

Run `make lint` and `make test` from the repository root before accepting an implementation. GNU Make invokes the native tools; failures stop the target and return a nonzero exit status, including with `make -j`. These targets never silently skip an unavailable tool or test lane.

- `make test`: all Cargo workspace members (including Tauri), Node script tests, Next Vitest, CLI Vitest with coverage, desktop UI Vitest, native contracts and the complete real-stack Playwright discovery. E2E builds fresh Rust binaries and Next output first.
- `make lint`: rustfmt check, Clippy for all workspace targets with warnings denied, ESLint for handwritten JS/TS, and Next/CLI TypeScript checks. Generated output and dependency directories are excluded from ESLint, not source tests. Existing lint debt is a failure, not an automatic exemption.
- Focused lanes: `make test-rust`, `make test-backend`, `make test-node`, `make test-contracts`, `make test-e2e`, `make lint-rust`, `make lint-node`. `make test-backend` excludes Tauri and cannot certify the full suite.
- `make test-e2e E2E_ARGS='--workers=1'` adjusts browser concurrency. Filters are diagnostic only; do not claim full acceptance for filtered runs. `make build-web` builds just the maintained web package.

The Redis reconnect focused gate additionally requires `redis-server` and `redis-cli`
on PATH (Debian/Ubuntu: `redis-server redis-tools`; mac