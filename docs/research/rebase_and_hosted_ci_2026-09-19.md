# Rebase, branch publication and hosted CI findings

Review date: September 19, 2026, America/Los_Angeles (the captured Actions timestamps are September 20 UTC).

## Branch and preservation

The active work is `fix/native-cross-platform-release`, tracking the same branch on `omnisolo-llc/onehumancorp`. Fetching origin established that the latest main was `c3716d0875df6403322af4fb47d9f56f9042af3c`. Both existing migration commits already descended from that commit; `git rebase origin/main` therefore completed with "Current branch ... is up to date." No force push or main-branch rewrite was necessary.

The initial HEAD was `2a8a369c7eeb7603fa7454c5436630a7299912d7`, following migration commit `9d5b520692d951f9ea21db25008df71501e724aa`. Forty-six pending continuation files were inspected and preserved in a binary patch and SHA-256 inventory under `.git/ohc-rebase-backups/20260920T045039Z`, with local backup branch `backup/native-before-rebase-20260920T045039Z`. No untracked build artifacts were staged. Those continuation changes and the missing verbose-packaging flag were committed as `45b9ec666a2df33e09e33fa68d5b1b79248d3cd7` and pushed normally to the feature branch.

A fresh CI invocation, [35490208206](https://github.com/omnisolo-llc/onehumancorp/actions/runs/35490208206), was dispatched explicitly against that pushed commit. Existing runs [35488712642](https://github.com/omnisolo-llc/onehumancorp/actions/runs/35488712642) (CI) and [35488716604](https://github.com/omnisolo-llc/onehumancorp/actions/runs/35488716604) (build-only Release) target the older `2a8a369c7` revision; their results are not certification of the follow-up fixes in this document. The superseded Release run was cancelled after its concrete failures were captured; cancellation is not a passing result.

## What the hosted runners actually established

On `45b9ec666`, completed green jobs included dependency auditing, PostgreSQL tenant isolation, backend compilation, the Next production build, and Linux Tauri compilation. The Node job correctly remained failed because repository-wide ESLint reported **599 errors across 251 files**. Its independent typechecks and complete unit-test stages nevertheless executed successfully: **1,527 frontend tests in 341 files**, **59 CLI tests in 13 files**, **15 desktop-UI tests in 4 files**, and **66 native script tests**. The preserved deployment/security source contracts also passed. These counts refer to this specific historical run, not every later revision.

On the older build-only release, both Intel and Apple Silicon desktop packaging and both Linux backend archive jobs completed successfully. Other release rows failed or had not completed when reviewed. A matrix definition, successful Cargo check, or one working installer does not establish a complete release.

## Reproduced failures and corrections in this follow-up

### 1. Container startup could not locate PostgreSQL migrations

Both Kind and Compose logs reported `Source(Os { code: 2, kind: NotFound, ... })` immediately after the server started migrations. The runtime image sets `/data` as its working directory and copies SQL to `/src/server/migrations`, while `src/server/db.rs` opened `src/server/migrations` relative to the working directory.

The server now uses `sqlx::migrate!("./src/server/migrations")` to embed the exact SQL and checksums into the executable. Root `build.rs` tracks migration-directory changes so newly added migrations invalidate cached binaries on stable Rust. `.gitignore` explicitly permits that build script; merely having an ignored local build file would not fix hosted CI.

The Rust regression `embedded_migrations_match_every_repository_version_and_checksum` passed after compiling the actual server library. It compares every bundled migration's version, description, SQL and checksum against the repository. The focused invocation ran one test and explicitly filtered other library tests; it is not a full Rust-suite result. A subsequent hosted container deployment must confirm the startup correction in the produced image.

### 2. Real OpenCode integration tests lacked their executable

The older native Rust job passed 25 tests and failed three in `server_harness --test opencode_http`: `launches_real_pinned_server_with_bounded_health_and_session_lifecycle`, `real_pinned_server_executes_a_prompt_through_openai_responses`, and `explicit_shutdown_reaps_managed_process_before_removing_isolated_home`. All failed to spawn `opencode`; this was a missing test prerequisite, not a reason to ignore the tests.

`.github/test-tools/package.json` and its npm lock now pin `opencode-ai` to the existing supported version, `1.18.15`, including integrity-checked platform dependencies. The native Rust job installs that isolated dependency tree, verifies the actual executable version, and adds its bin directory to the job PATH. No customer key or live provider account is supplied. The integration suite uses its existing local provider fixture.

The actual pinned executable was installed locally, and **all 28 OpenCode integration tests passed**, including the three hosted failures. The workflow prerequisite contract also passed. The public version is not automatically upgraded and no global developer installation is replaced.

### 3. A shared browser helper attempted obsolete passwordless authentication

Hosted browser error contexts show authenticated dashboard pages while `currentAppSmoke` waited for an `Email or Username` login field. Its callers had already authenticated a seeded actor through `loginAs`; the helper then navigated to `/login` and tried to enter a hardcoded prototype identity. The real application correctly redirects an authenticated session to the dashboard, so that input never appears.

The helper now navigates to the dashboard using the actor's existing real session and explicitly asserts the authenticated URL. It no longer switches to a prototype identity. Existing page and business assertions remain, and the previously unused leaderboard empty-state result is now asserted rather than silently allowing neither data nor empty state. A regression guards against restoring the obsolete authentication flow. Full browser acceptance is still required; fixing the login obstruction does not validate all the subsequent feature assertions.

## Remaining failures are not waived

- The complete ESLint gate remains mandatory. The 599-error inventory on the initial pushed revision consists of explicit-any, unused-variable, empty-block, unused-assignment, rest-parameter and const diagnostics. Passing TypeScript compilation does not waive them.
- Browser shards on the older run reached real test failures before exhausting their job budgets. Beyond the shared authentication bug, a guardrail test received the product's explicit "not implemented" response where the test expects a successful evaluation. An assistant test also uses an invalid mixed CSS/text selector and a retired prototype path. Do not replace required feature outcomes with unconditional success or increase a timeout to disguise missing behavior.
- The older Rust suite stopped at OpenCode's missing executable. Its now-passing focused test does not prove that every later integration suite passed; run the complete headless and desktop gates.
- Linux AppImage packaging reported a linuxdeploy failure, and Windows MSI packaging reported a WiX `light.exe` failure. The verbose flag is now retained so a fresh packaging run can expose the real cause. The Windows backend Cargo lane separately uses PowerShell to avoid Git Bash shadowing Microsoft's linker.
- Android release signing is blocked by missing `ANDROID_KEYSTORE_BASE64`, `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS`, and `ANDROID_KEY_PASSWORD` secrets. No substitute production signing identity was generated and no signing check was disabled. This does not block the ordinary Linux CI workflow, but it blocks complete signed-release acceptance.
- The complete required CI run has not met the 30-minute target. A successful subset, a queued job, or a cancelled superseded run is not a green required gate.

Raw logs and synthetic browser diagnostic artifacts are stored locally under ignored `target/ci-followup/`, not published as application source. No main merge, tag push, public release, production deployment, provider charge or customer operation was performed by this follow-up.
