# 🔬 LibSQL MCP for Distributed Edge SQLite Synchronization

## Problem Statement
OHC's Hybrid Architecture relies heavily on seamless data transition between Cloud-Native Mode (PostgreSQL) and Standalone Desktop Mode (SQLite). Currently, local-only SQLite deployments lack built-in, frictionless replication to edge nodes or cloud fallbacks without complex application-level queueing. This gap creates latency and availability risks for agents operating in edge environments with intermittent connectivity.

## Research Report
LibSQL (the open-contribution fork of SQLite by Turso) provides native edge replication and a distributed architecture while maintaining 100% compatibility with SQLite. By integrating LibSQL as an MCP tool, OHC agents can dynamically provision, manage, and query distributed LibSQL databases. This allows Standalone agents to seamlessly push local memory and state to an edge replica (e.g., Turso) when online, providing an automatic, ultra-low-latency cloud fallback without requiring a heavy PostgreSQL footprint.

## Design Doc
1. **MCP Interface**: Create `srcs/server/integrations/libsql/provider.go`.
2. **Architecture Updates**:
    - Add a `LibSQLIntegration` struct implementing the standard `Integration` interface.
    - Provide tools for agents to configure replication URLs, check replication lag, and perform edge-sync validations.
    - Expose telemetry metrics for LibSQL sync status.
3. **Registration**: Add LibSQL to `srcs/server/integrations/catalog.go`.

## Implementation Prompt
Hello Implementer agent! Your mission is to:
1. Create a new package `srcs/server/integrations/libsql/` containing `provider.go`.
2. Implement the `Integration` interface for LibSQL, exposing `Metadata()` and `WizardSteps()`.
3. Integrate the new provider into `srcs/server/integrations/catalog.go`.
4. Write comprehensive tests in `provider_test.go` and create appropriate Bazel targets (`BUILD.bazel`) for the package.
5. Verify functionality by running `bazelisk test //srcs/server/integrations/libsql/...`.

## Priority
P2

## Estimated Scope
Medium
