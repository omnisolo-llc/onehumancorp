seful on memory-constrained/shared hosts; `.cargo/config.toml` defaults to two jobs. Set `CARGO_INCREMENTAL=0` for bounded-space batch builds. A large native cold compile still has a real cost; this migration removes Bazel analysis and duplicate build plumbing, not all Rust compilation.

Before deleting generated caches, confirm their exact project-relative path, that they contain no tracked files, and that no compiler owns them. Do not delete another project's caches. Keep test logs under `target/validation` and record elapsed time, toolchain, workload and whether a cache was warm. Do not claim a speedup from comparing a warm native run to a cold Bazel run.

## CI critical path and measured time budget

The safety ceiling is **X = 60 minutes for the complete Linux CI required gate**, with a **15-minute warm-cache stretch target**. A provisional **10-minute core-build target** covers backend, Next and Tauri compilation; it does not substitute for the complete test/security/deployment gate. These are acceptance targets, not achieved CI percentiles. Desktop/mobile signing, publishing and other release platforms are separate workflows and are not represented as fitting this budget.

The complete-CI budget was initially relaxed from 30 to 35 minutes on 2026-10-04; the owner subsequently requested a one-hour ceiling for complete CI and browser execution. Normal browser work still targets 15–20 minutes per physical runner; the hour is a safety ceiling, not a performance target. Individual test limits, retries and required checks are unchanged. The final reporting job retains its separate five-minute timeout for report collection and upload; it does not run the preceding CI workloads. Historical measurements retain their original budgets.

The CI graph now separates work that can run independently:

- Rust executable build publishes this run's backend, agent, worker and mTLS probe. The headless test/lint job and desktop test/lint job partition the complete Rust workspace w