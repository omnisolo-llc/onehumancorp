# Changelog

## [0.2.9] - 2026-04-06

### Added
- Cloud Scaling: Optimized multi-tenant PostgreSQL queries and background synchronization queues.
- Standalone Privacy/Offline: Improved SQLite file locking configurations and buffered telemetry syncing.

### Changed
- Refined multi-environment promotion CI/CD workflow testing for hybrid deployments.

## [0.2.8] - 2026-04-03

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.

### Changed
- Coordinated promotions across Cloud staging and Local desktop beta builds.
- Synchronized version configurations across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.

### Fixed
- Stabilized hybrid test scenarios ensuring both PostgreSQL (Cloud) and SQLite (Standalone) compatibilities.
