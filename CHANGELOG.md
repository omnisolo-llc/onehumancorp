# Changelog

## [0.2.9] - 2026-04-09

### Added
- KAIROS Orchestration layer featuring Shared Task List DAG and Sub-Agent queues.
- AutoDream Memory Pipelines for local vectorization and postgres RAG synchronization.
- Teammate Mesh APIs built on Redis Pub/Sub for hybrid task broadcasts.
- Pure-Go agent harness implementation replacing legacy Rust bindings.

### Changed
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.

### Fixed
- Stabilized telemetry privacy checks for strictly opt-in metrics in Standalone mode.
- Corrected distributed state machine concurrency edge cases in SQLite mode.

## [0.2.8] - 2026-04-03

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.

### Changed
- Coordinated promotions across Cloud staging and Local desktop beta builds.
- Synchronized version configurations across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.

### Fixed
- Stabilized hybrid test scenarios ensuring both PostgreSQL (Cloud) and SQLite (Standalone) compatibilities.
