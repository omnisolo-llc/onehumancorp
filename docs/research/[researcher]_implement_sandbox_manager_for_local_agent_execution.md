## Title
[researcher] Implement Sandbox Manager for Local Agent Execution

## Problem Statement
In OHC Standalone Desktop Mode, agents currently execute shell commands directly on the user's host machine via `exec.Command` without intermediate security filtering, validation, or pausing for human permission. This is a severe gap compared to leading market products (like AI coding assistant) which use a robust `SandboxManager` adapter to govern filesystem/network access and intercept violations.

## Research Report
Based on an architectural audit of the  AI coding assistant execution harness:
- AI coding assistant uses an adapter (`sandbox-adapter.ts`) wrapped around `@assistant provider-ai/sandbox-runtime`.
- It enforces `FsReadRestrictionConfig`, `FsWriteRestrictionConfig`, and `NetworkRestrictionConfig`.
- It provides a telemetry stream for `SandboxViolationEvent`s.
- It leverages a `SandboxAskCallback` to pause execution and request human permission when encountering risky patterns (e.g., modifying system files).

## Design Doc
1.  **Interface Definitions (`srcs/server/orchestration/sandbox.go`)**:
    -   Define an `OHCSandboxManager` interface mirroring these capabilities.
    -   Create `SandboxConfig` (for Fs/Network restrictions) and `ViolationEvent`.
2.  **Implementation (`srcs/server/orchestration/local_sandbox.go`)**:
    -   Implement the manager specifically for Standalone Mode.
    -   Integrate with existing agent invocation logic so that all commands flow through this manager.
3.  **Human-in-the-Loop Callback (`srcs/server/orchestration/sandbox_ask.go`)**:
    -   Expose a mechanism (e.g., via KAIROS state machine or WebSockets) to pause execution when a policy violation occurs, triggering an approval prompt in the UI.

## Implementation Prompt
Implement a Go-based `OHCSandboxManager` in `srcs/server/orchestration/`.
1. Create the interface and basic config structs (`SandboxConfig`, `ViolationEvent`) in `sandbox.go`.
2. Implement `local_sandbox.go` which provides a wrapper around `exec.CommandContext`. It should accept a `SandboxConfig` and return a `ViolationEvent` if limits are exceeded (for now, simply check command strings against a hardcoded deny-list of system directories).
3. Wire this into the agent execution loop so that if a violation occurs, the agent pauses and the event is logged via OpenTelemetry.
4. Write comprehensive unit tests for `local_sandbox.go` mocking various valid and violating commands.
5. Ensure 100% test coverage.

## Priority
P0

## Estimated Scope
Medium
