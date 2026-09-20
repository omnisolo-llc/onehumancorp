# Developer setup and ESLint remediation

## Scope

Adds `make init` and `make doctor` for the native Rust/Tauri/Node developer environment, and resolves the previously reported 597 ESLint errors without changing the ESLint rules, ignored source scope, dependency lockfiles or CI workflow gates. The checkout already contained partial bootstrap/type-cleanup work when this continuation began; those changes were preserved, completed and verified. Baseline commit: `92ea601465a5386f5980377eaf388310f4d0088d` on `fix/native-cross-platform-release`.

## Developer commands

```sh
make init
# Use project-local tools directly, including Homebrew keg paths on macOS:
source target/dev-tools/env.sh
make doctor
make lint
make test
```

`make init INIT_ARGS=--plan` previews without installing or writing. `--yes` approves the described system-package operations; `--no-system` never runs sudo/Homebrew and requires the native libraries to exist; `--force` reinstalls locked npm trees instead of reusing their success stamps.

The initializer reads Rust/Node versions from repository pins. It installs rustfmt/Clippy without changing the global Rust default, links the selected Node/npm/npx locally, installs all four locked npm dependency trees including OpenCode, fetches locked Cargo dependencies, creates an isolated Python environment and installs/launch-checks Playwright Chromium. Native library installation is supported for Debian/Ubuntu/WSL2 and macOS/Homebrew. Versioned official Node archive checksums were independently compared with the official `SHASUMS256.txt` for all six entries in the shared manifest.

Dependency success stamps include both npm manifests, Node version, operating system and architecture. Invalid downloads, incomplete installs and unexpected non-symlink tool files fail explicitly. The bootstrap does not edit shell profiles, create application secrets, grant Docker permissions, start business services or provision a production environment. `make doctor` explicitly disables implicit rustup toolchain installation and Cargo network access while probing readiness.

Git, Make and a usable Python are bootstrap prerequisites. Test/release Python tooling needs Python 3.11+ with venv support. Docker must already be installed and running locally with Compose and Buildx. Effective Docker contexts are checked before daemon access; `DOCKER_CONTEXT` overrides `DOCKER_HOST`, and remote endpoints are rejected. Homebrew and Apple command-line tools require prior installation on macOS. Native Windows release tooling, Android SDK/NDK and signing/notarization credentials remain release-specific; WSL2 is the supported full POSIX test environment on Windows.

## Source changes

Shared finite business, builder and agent-feed types replace unbounded `any` in application state and rendering. Unknown JSON is narrowed where its shape is not established. Error handling uses unknown-safe message extraction. Test transports use typed responses and partial SDK/database doubles instead of blanket `any` casts. Unused bindings/imports and dead assignments were removed while retaining test discovery and business assertions. No new lint/typecheck suppression directives were added.

The stricter types and complete tests also identified several concrete behavioral gaps:

- Rate-limited offline operations previously swallowed the rate-limit exception before deciding whether to clear queued actions. Five action-family regressions now verify that the queue is retained on HTTP 429.
- Offline mutation timestamps accept the existing numeric/ISO inputs, normalize ISO values before persistence, and reject invalid dates instead of persisting a malformed timestamp.
- The leaderboard renderer passed a numeric score into a string-only HTML escaper. It now stringifies the numeric score and still escapes untrusted names; the new route test checks both behaviors.
- Legacy approval history is normalized into the feed's activity shape. Missing timestamps are shown as unrecorded, not replaced with an invented time.
- A poll-branding test ended in `expect(true)` and depended on a production-only global testing hook. The hook was removed; the test now exercises the plan boundary and checks the actual checkbox/paywall/embed behavior.
- Voice/changelog mocks reused a real, single-use `Response` body across fetches. Each request now receives its own response, preserving the assertions about success, stale callbacks and rendered content.

## Completed verification

The final complete validation invocation returned exit code 0. A fingerprint of **2,222 source/configuration inputs** was unchanged during the run. An earlier invocation was interrupted by replacement of the WebCodex runner; it was not counted as a successful final run, and the checks were rerun.

| Command / suite | Result |
| --- | --- |
| `make lint-node` | ESLint with `--max-warnings 0`, web TypeScript and CLI TypeScript all passed |
| Root native/script suite | 69 passed, no failures or skips |
| Complete frontend Vitest suite | 1,542 tests passed across 343 files |
| Complete CLI suite | 59 tests passed across 13 files |
| Complete desktop UI unit suite | 15 tests passed across 4 files |
| `make test-contracts` | All 16 deployment/security/source-contract groups passed |
| `make build-web` | Fresh Next production compilation and source-bound standalone packaging passed |
| Bootstrap regressions | 13 passed, covering plan/pins, download validation, repeatability, platform stamps, local Docker boundaries, Python requirements and non-installing doctor probes |
| `npm run test:e2e -- --list` | 1,688 browser tests discovered in 453 files; discovery is not execution |
| `git diff --check` | Passed after removing only newly introduced blank-line trailing whitespace |

Wrapper suites invoke some Python tests, so these counts must not be added into a purported unique-test total. Validation logs and the source snapshot are in ignored `target/init-lint-review/verified/`; they are local execution evidence, not files to publish as release assets.

## Evidence limitations

The bootstrap's elevated package-install path was not executed against the user's host or clean Windows/macOS machines. `make doctor` correctly reported the existing host's missing `ayatana-appindicator3-0.1` prerequisite before initialization; no sudo installation or permission changes were made by this review. Successful bootstrap unit tests or a passing plan are not a claim that this uninitialized host is ready.

This change does not certify the full Rust workspace, real-stack browser execution, Docker/Kind deployment execution, signed cross-platform packages or a hosted CI time budget. Their existing required gates remain intact. The successful production web build and Node tests are not substitutes for those separate results.
