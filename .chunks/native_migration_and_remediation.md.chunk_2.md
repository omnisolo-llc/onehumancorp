 for server routes. | Rebuild after subsequent UI edits; final Tauri/runtime and browser acceptance. |
| M03 | Canonical `make lint` / `make test` remain complete. CI partitions headless and desktop Rust lint/tests; Node lint/tests run independently of the web build and remain required. All 1,688 browser tests in 453 files now enumerate after fixing fixture imports and duplicate Playwright resolution. | Discovery is not execution. Full Rust/browser and all Node quality gates remain necessary. |
| M04 | OS/architecture/role/compiler-aware caches; explicit Node dependency scopes; positive trusted-event cache-save rules; no PR cache publication. Cold-cache control disables project-cache restoration. | Real fresh-runner cache hit/miss and runtime measurements; signed release cache isolation is configuration-reviewed, not release-certified. |
| M05 | Native archives/desktop/mobile/Docker workflows retained. Production Docker layers built once and reused by both Kind and Compose; source/checksum/tag/loaded-ID checks protect image reuse. Same-run mTLS probe reused. | Actual Docker release-image and deployment-suite execution on this revision; signing/install/device validation remains outstanding. |
| M06 | Active native README/AGENTS/Automator instructions and Make targets replace Bazel. Obsolete launcher symlinks are guarded by a regression test. | Keep future automation aligned with current Make gates and truthful evidence; do not restore a stale legacy command from historical reports. |
| M07 | Local timing logs, Cargo timing HTML and RSS measurement captured under `target/validation` / `target/cargo-timings`. CI produces attempt-bound JSON/Markdown timing evidence and fails above 30 minutes. | No measured hosted cold/warm percentiles yet; initial queue and final reporting time are explicitly excluded. |

### Findings-to-implementation status

| Finding | Production changes already present | Evidence level / remaining gap |
|---|---|---|
| F01: repeated usage accounting