# Changelog

## [0.2.9] - 2026-04-09

### Added
- Hybrid Local-to-Cloud State Sync MCP Proxy integration for multi-tenant scalability.
- Strict Opt-in Telemetry Enforcement for Hybrid Privacy in Standalone mode.

### Changed
- Updated environment tests to use `t.Setenv()` for clean isolation.
- Coordinated promotions across Cloud staging and Local desktop beta builds.

## [0.2.8] - 2026-04-03

### Added
- Hybrid SemVer version bump for Cloud server pods and Standalone desktop binaries.

### Changed
- Coordinated promotions across Cloud staging and Local desktop beta builds.
- Synchronized version configurations across `MODULE.bazel`, `package.json`, and `srcs/app/pubspec.yaml`.

### Fixed
- Stabilized hybrid test scenarios ensuring both PostgreSQL (Cloud) and SQLite (Standalone) compatibilities.
