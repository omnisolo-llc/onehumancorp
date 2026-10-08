int snapshot still reports **723 errors across 294 files**. The original F02 repair also required additional tenant-scoping corrections in dashboard, agent-manager, agent-metrics and Tracker summaries; those paths and cache-key migration are documented in the measured record. Neither a passing targeted suite nor a cache hit establishes full green CI.

## Current continuation: 2026-09-19

The working branch is `fix/bazel-modernization-and-cleanup`, HEAD `c3716d0875df6403322af4fb47d9f56f9042af3c`, with substantial uncommitted migration/application work. The original register below is retained as the initial acceptance inventory; this section records current evidence and supersedes its initial-status column. No commit, push, hosted-CI dispatch, deployment, live-provider call or customer charge is implied.

### Build acceptance and cleanup

**Full green CI is not established.** The explicit target is **30 minutes to the complete Linux CI required gate**, with a **15-minute warm-cache stretch target**. The actual job-timestamp reporter and cache-disabled manual option are implemented; no hosted run of this uncommitted revision has been executed. Never describe the configured timeout as an achieved build duration.

| ID | Current implementation / evidence | Remaining acceptance |
|---|---|---|
| M01 | Cargo workspace and locked dependencies; pinned Rust 1.95.0. Removed residual Bazel conditionals and duplicate inline billing compilation; persistence tests now exercise canonical modules rather than a second copied module graph. Full Rust formatting passed. Native backend/agent/worker binaries compiled successfully. | Complete current-source Rust test and Clippy results, including Tauri; no whole-workspace green claim from binary compilation. |
| M02 | Removed two tracked dangling Bazel launcher symlinks from `src/ui/next/bin`. A fresh Next build then passed and packaged validated standalone output in 54 seconds with npm dependencies already installed. Node remains required