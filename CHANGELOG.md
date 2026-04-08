# Changelog

## [0.2.9] - 2026-04-08

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.
- Scaling (Cloud): Distributed State Machine for rigorous tracking of agent transitions.
- Privacy/Offline (Standalone): AutoDream data pipeline updates and robust SQLite mutex updates for standalone backend scaling.

## [0.2.8] - 2026-04-03

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.

### Changed
- Coordinated promotions across Cloud staging and Local desktop beta builds.
- Synchronized version configurations across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.

### Fixed
- Stabilized hybrid test scenarios ensuring both PostgreSQL (Cloud) and SQLite (Standalone) compatibilities.
