# Required CI head repair — 2026-09-20

Baseline: main `15a913e27462c939e804c1863d5e4d659e74c93d`.

This PR originally experimented with 32 isolated single-worker browser shards. That topology has been superseded by the newer repair tree adopted here: browser qualification is capped at 8 concurrent shards so the suite still covers the complete discovered Playwright inventory while leaving runner capacity for Rust, images, security, desktop, Docker, Kind, and other pull requests.

The current qualification contract remains fail-closed: hosted evidence is bound to the exact source SHA, run ID, and attempt; missing shards, duplicate or missing tests, skipped/interrupted tests, retries hiding first-attempt failures, and incomplete reports fail qualification. Complete browser discovery remains source-driven rather than a smoke allowlist.

The 30-minute full-CI budget is unchanged. Full Cargo workspace tests, strict lint/type checks, Next production build, Tauri, PostgreSQL tenant isolation, Docker/Kind probes, shared production images, and complete real-stack browser qualification remain required.

This document is a repair-history note only. Current workflow files and their contract tests are authoritative.
