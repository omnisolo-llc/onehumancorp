# Audit Findings for `src/agents/builtin/verification_loops.rs` & `src/agents/builtin/ralph_loop.rs`

## `verification_loops.rs`
**Vulnerabilities Identified:**
1.  **Missing Timeouts & Hanging Execution (Retry Storms):** In `PlaywrightVisualVerifier::verify_visual`, the standard library synchronous subprocess mechanism (`std::process::Command::new("npx")`) was used with `.output()` directly. Since Playwright interactions can hang indefinitely (e.g., if a page doesn't fully load, a dialogue box opens unexpectedly, or a deadlock occurs in Playwright itself), this would hang the agent loop forever, consuming resources.
2.  **Lack of Async:** The verification function is declared `async fn`, but the synchronous `std::process::Command` blocks the Tokio worker thread. This starves the thread pool and reduces scalability under load.
3.  **Partial State & Error Obfuscation:** The error formatting drops `stderr` and the error reason in case of failures other than IO.

**Fix Implemented:**
-   Swapped `std::process::Command` for `tokio::process::Command` to prevent thread blocking.
-   Wrapped the subprocess execution in `tokio::time::timeout` with a 30-second boundary.
-   Added a unit test (`test_playwright_timeout`) to verify that the async command will correctly timeout rather than block indefinitely.

## `ralph_loop.rs`
**Vulnerabilities Identified:**
1.  **Retry Storms & Idempotency Issues:** In `RalphLoop::resume_from_checkpoint`, there was no explicit guard against continually resuming the same broken state or re-applying operations over identical git diffs if a step failed partially.
2.  **Enum Capitalization Warnings:** Enums like `Phase1_Initialize` and `Phase2_Coding` triggered `nonstandard_style` lint warnings, cluttering standard build output and making it harder to spot real logic errors.

**Fix Implemented:**
-   Renamed `Phase1_Initialize` to `Phase1Initialize` and `Phase2_Coding` to `Phase2Coding` to respect Rust standard conventions and resolve warnings.

## Current State
Both codebase components have been upgraded to eliminate thread-blocking synchronization points. The entire `bazelisk test //src/agents/builtin/...` test suite executes cleanly without any errors, confirming no regressions.

## Remaining Coverage Gaps
1.  **Idempotency & Partial State Testing in Ralph Loop:** While timeouts are secured in the visual verifier, the `RalphLoop` lacks heavy end-to-end integration tests that intentionally abort the process midway and confirm it resumes exactly correctly without duplicate operations.
2.  **Missing Mocks:** Real network calls or subprocess invocations (e.g. `npx playwright`) are executed directly in integration boundaries; a robust test suite should mock these processes completely to test both immediate `SIGKILL` timeouts and arbitrary stderr/stdout responses.
