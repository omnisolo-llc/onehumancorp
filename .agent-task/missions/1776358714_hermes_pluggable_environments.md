---
status: PENDING
agent: NONE
---

# Title: [research] Implement Pluggable Execution Environments for Agent Harness

## Problem Statement
The OHC Agent Harness currently executes bash commands via a static `sandbox.go` implementation. To scale efficiently across our Cloud-Native and Standalone modes, we need the ability to seamlessly swap the underlying execution environment (e.g., Local OS, Docker, Remote serverless) while maintaining strict security constraints.

## Research Report
Based on an analysis of the Hermes Agent codebase, they utilize an abstract `BaseEnvironment` to plug in different execution backends like `LocalEnvironment`, `DockerEnvironment`, and serverless providers (Modal, Daytona). Additionally, they perform robust environment variable scrubbing (`_sanitize_subprocess_env`) to prevent secrets from leaking into subprocesses. For local isolation, they override the `$HOME` directory per agent profile to sandbox tool configs (like `~/.gitconfig` or `~/.npmrc`).

## Design Doc
1. **Architecture**: Introduce a new `ExecutionEnvironment` interface in `srcs/server/bash_sandbox/` to abstract command execution.
2. **Implementations**: Refactor the current `Sandbox` into a `LocalEnvironment` struct implementing the interface.
3. **Security**: Add an environment variable blocklist (scrubbing `OHC_API_KEY`, `GH_TOKEN`, OpenTelemetry keys) applied before `exec.CommandContext` runs.
4. **Isolation**: Add support for overriding the `$HOME` environment variable to a dedicated `.agent-home/` directory for Standalone mode.

## Implementation Prompt
Implementer Agent: Open `srcs/server/bash_sandbox/sandbox.go`.
1. Define an `ExecutionEnvironment` interface with `ExecuteContext(ctx context.Context, command string, workDir string) (string, error)`.
2. Ensure the existing `Sandbox` struct implements this.
3. Update `ExecuteContext` to strip sensitive environment variables (e.g. `OTEL_EXPORTER_OTLP_HEADERS`, `GITHUB_TOKEN`) by setting `cmd.Env`.
4. Modify `cmd.Env` to set `HOME` to a temporary agent workspace directory.
5. Update tests in `srcs/server/bash_sandbox/sandbox_test.go` to verify environment variables are correctly scrubbed and `HOME` is overridden.

## Priority
P1

## Estimated Scope
Medium
