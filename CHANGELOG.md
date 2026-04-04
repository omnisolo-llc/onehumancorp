# Changelog

## [0.2.9] - 2026-04-03

### Added
- Cloud Mode: Introduced enhanced multi-tenant metrics and dynamic horizontal scaling adjustments for K8s pod deployments.
- Standalone Mode: Improved local SQLite fallback for seamless offline-to-cloud synchronization and robust single-user privacy controls.
- Core: Deployed Swarm Intelligence Protocol (OHC-SIP) telemetry upgrades across all agent missions.

### Changed
- Refined build processes and synchronized SemVer (0.2.9) across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.
- Promoted stability patches from Local desktop beta directly into Cloud staging.

### Fixed
- Stabilized offline data persistence under heavy swarm loads in desktop binaries.

## [0.2.8] - 2026-04-03

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.

### Changed
- Coordinated promotions across Cloud staging and Local desktop beta builds.
- Synchronized version configurations across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.

### Fixed
- Stabilized hybrid test scenarios ensuring both PostgreSQL (Cloud) and SQLite (Standalone) compatibilities.
