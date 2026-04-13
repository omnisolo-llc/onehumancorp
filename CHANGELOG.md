# Changelog
## [0.2.9] - 2026-04-11

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.
- New cost feature: Hybrid Local-to-Cloud State Sync MCP Proxy (statesyncmcp).
- HybridHealthProbe for verification of hybrid-mode switching and local-to-cloud mission sync reliability.

### Changed
- Coordinated promotions across Cloud staging and Local desktop beta builds.
- Synchronized version configurations across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.
- Improved AutoDream Memory consolidation pipeline test coverage to >90%.

### Fixed
- Enforced strict opt-in telemetry for Hybrid Architecture Privacy in standalone mode.

## [0.2.8] - 2026-04-03

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.

### Changed
- Coordinated promotions across Cloud staging and Local desktop beta builds.
- Synchronized version configurations across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.

### Fixed
- Stabilized hybrid test scenarios ensuring both PostgreSQL (Cloud) and SQLite (Standalone) compatibilities.
