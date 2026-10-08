# Native build migration and audit remediation ledger

## Standard release tooling continuation — 2026-09-19

The [standard release-pipeline record](standard_release_pipeline_2026-09-19.md) documents the official Tauri action integration, unchanged cross-platform matrix, explicit target/locked builds, DMG notarization ordering, builder provenance, draft upload verification and the cargo-dist layout probe. The [release guide](../development/native-releases.md) is the operator contract. These implementation/script-test results do not supersede the remaining full-CI, signed-platform and real-runtime acceptance gates below.

## Measured cleanup continuation — 2026-09-19

See [the measured native-build cleanup record](native_build_measurements_2026-09-19.md) for commands, cache conditions, source qualifications and regression evidence. Empty-output backend compilation completed in **8m 33.94s**, its exact unchanged rerun in **1.87s**, and a fresh Node proxy build in **31.40s** without stderr warnings. The refreshed frontend suite passed **1,516 tests across 340 files in 3m 30.82s**, with a stable source digest; a fresh web build then completed in **29.21s**. The selected thirteen tenant-cost/header regressions passed, but a concurrent Rust edit invalidated that invocation's final-source fingerprint. The Linux Tauri debug/no-bundle build passed in **6m 42s**; **59 native CI-script tests** passed. These are local measurements, not hosted full-CI certification.

This pass fixes unsafe shared-temporary-file cleanup, non-atomic cross-filesystem fallback, blocking environment-test locks, an unused AVIF encoder dependency, stale custom-target browser inputs, tooltip timer disposal and typed test/API helper issues. The full **30-minute required CI** target remains unmet until every strict quality/test/deployment gate passes on final source. The new record tracks repository-wide lint debt; older completion statements below must not override that limitation. The refreshed full ESL