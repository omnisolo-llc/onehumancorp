e defects: an orphaned brace sequence in a historical neighborhood contract, escaped template delimiters in the legacy voice script, and malformed interpolated HTML in the legacy help widget. Syntax fixes preserve the contract assertions. Help message/link/video rendering now uses safe DOM text and validated destinations instead of interpolated HTML/inline handlers; regression tests cover executable URLs, credential-bearing links and malicious titles. Voice no longer supplies a localStorage-derived workload identity or treats a response without transcription as a prepared action.

A first bounded Rust build failed at an imposed **6 GiB virtual-address-space limit** after 733 seconds. That was a validation setup limit, not measured resident-memory exhaustion. The subsequent monitored build passed in **220.72 seconds**, using cached dependencies and compiling the main server artifact, with **4,722.75 MiB peak process-group RSS**. Both use one Cargo build job and disabled incremental. The latter used `MALLOC_ARENA_MAX=2`. Neither is an untouched cold-runner CI baseline.

One isolated test setup was blocked by the host's enforced bwrap execution policy before tests ran; a separate read-only/no-network Docker validation setup was used without disabling host security. Its first attempt lacked `/etc/alternatives` for the system `cc` symlink and failed before tests; the corrected setup includes only that public system-tool directory. These failures are not counted as passing tests.

### Verification journal

- Native production Next build: passed in 54 seconds after removing stale symlinks; `.next` absent, installed npm dependencies reused. `target/validation/current-web-build.log`.
- Native backend/agent/worker binaries: passed in 220.72 seconds, dependency-warm/main-crate rebuild; RSS details in `target/validation/current-native-build-rss.json` and matching log.
- Full Rust formatting: passed after formatting the actual workspace, not by excluding files.
- Root native/bui