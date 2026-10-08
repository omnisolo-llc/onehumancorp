OS: `redis`). Its runner
starts a disposable loopback process and verifies the server PID before destructive
connection tests; shared Redis services and supplied URLs are rejected. Run
`bash scripts/redis-reconnect/fetch.sh` then
`python3 scripts/focused_ci_gate.py redis-reconnect`. Native contracts enforce the
fixture ownership and required CI wiring without running those connection tests.

Full gates require GNU Make, the pinned Rust/Node toolchains, Python 3 with PyYAML, native Tauri/WebKit/GTK development libraries for the host, a running Docker daemon and Playwright Chromium plus its OS dependencies (`npx --no-install playwright install --with-deps chromium`). Install locked npm dependencies at root, `src/ui/next`, and `src/cli`. See the native CI setup action for platform package lists. Mobile SDK, signing and physical-device checks remain separate release verification, not implied by a host workspace pass.

## Setup and boundaries

Install Rust with rustup and the pinned Node release. Ensure `$HOME/.cargo/bin` is on PATH on Unix. Use `npm ci` at the repository root, `npm --prefix src/ui/next ci` and `npm --prefix src/cli ci`; none should update a lockfile. Use `--locked` with Cargo in automation. Proto generation uses the workspace build scripts and vendored protoc; do not invoke deleted Bazel targets.

Standalone backend startup requires explicitly supplied `OMNISOLO_AGENT_TOKEN` and `OMNISOLO_AGENT_AUTH_KEY` (at least 32 bytes), using the operator's existing secret-management mechanism. Missing or invalid configuration fails before database initialization; the server never inserts development credentials. `OMNISOLO_AGENT_AUTH_DISABLED` is rejected by production binaries, and SPIFFE mode remains unavailable until verified peer extraction is implemented. The isolated browser runner supplies its own test-only credentials. Cluster mode does not create or overwrite agent credentials.

The default Cargo members are the backend and harness worker. `app` is the Tau