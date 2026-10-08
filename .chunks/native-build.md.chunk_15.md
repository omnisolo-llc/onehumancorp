inal gate start, including waits between dependent jobs. It excludes the initial queue before the first job and the final reporting/upload itself. Missing/inconsistent pagination, wrong run/attempt, unfinished predecessor jobs and invalid timestamps fail closed. Runs above 60 minutes fail the performance check; failed functional jobs never become successful performance evidence. Markdown and JSON reports are uploaded as `ci-performance-<attempt>` and shown in the Actions summary. Cache mode is labeled as requested, not claimed to be a hit.

### Local observations on 2026-09-19

| Workload | Observed result | Cache and resource limits |
|---|---|---|
| Fresh Next production build and source-validated packaging | Passed, 54 seconds | `.next` absent; installed npm dependencies reused. Not a clean-runner CI measurement. |
| Backend + agent + worker executable build | Passed, 220.72 seconds; peak process-group RSS 4,722.75 MiB | Dependency artifacts reused; main server artifact absent after a failed bounded attempt; Cargo jobs 1, incremental off, `MALLOC_ARENA_MAX=2`. |
| Earlier dependency compilation attempt | Failed after 733 seconds at an imposed 6 GiB **virtual-address-space** ceiling | Not proof that resident memory exceeded 6 GiB. Replaced with measured RSS-based protection; do not report this failure as a passed cold baseline. |
| Browser discovery | 1,688 tests across 453 files | Listing only, not execution; no Docker, model keys or running database required for discovery. |

A clean build means a fresh source checkout can pass the full declared gates; a warm invocation, successful binary build or a configured timeout alone cannot certify that. See the remediation ledger for outstanding lint and runtime acceptance work.

## Browser and provider verification

Normal Rust and Next builds expose no demo-seeding, mock-inbox, approval simulation,
or reputation simulation HTTP routes. This applies to debug builds as well as
release builds; there is no environment vari