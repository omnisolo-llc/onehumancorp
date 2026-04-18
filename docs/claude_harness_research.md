---
status: PENDING
---
# [research] Architect Claude-Class Agent Harness & Sandbox Telemetry

Parent: #5450

## Problem Statement
The current OHC agent execution harness lacks fine-grained sandbox control, sophisticated permission systems, and comprehensive execution telemetry, making it less secure and observable than state-of-the-art frameworks like Claude Code.

## Research Report
**Target**: Claude Code Leaked Source (`/tmp/claude-code`)
*   **Sandbox Architecture**: Relies on a `SandboxManager` that enforces `network` and `filesystem` rules via `@anthropic-ai/sandbox-runtime`.
*   **Permission System**: Uses a dynamic `tengu_sandbox_disabled_commands` system for bypassing sandbox execution, combined with a `bashPermissions` and `bashSecurity` layer that heavily analyzes commands with `tree_sitter` AST parsing before allowing execution.
*   **Telemetry**: Employs robust `tengu_*` event logging (e.g., `tengu_bash_security_check_triggered`, `tengu_bash_tool_command_executed`) to track the lifecycle and potential breaches of the sandbox.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

### 1. Harness & Sandbox Adapter
*   Implement a rigorous adapter around `SandboxManager` similar to Claude Code's `src/utils/sandbox/sandbox-adapter.ts`.
*   Define explicit configurations for `NetworkRestrictionConfig` and `FsWriteRestrictionConfig`.

### 2. AST-Based Security Parsing
*   Integrate a Tree-sitter AST parser before command execution to dynamically analyze bash commands for `tengu_bash_security_check_triggered` events.

### 3. Telemetry Integration
*   Emit specific execution metrics (e.g., `harness_tool_executed`, `harness_sandbox_bypass_attempted`) to our OpenTelemetry framework.

</div>

## Implementation Prompt
Implementer Agent: Please implement the Claude-Class Agent Harness & Sandbox Telemetry.

1. Implement `srcs/server/sandbox/manager.go` adapting the Go equivalent of `SandboxManager`.
2. Add AST-based shell parsing in `srcs/server/sandbox/security.go` to block destructive commands.
3. Integrate OpenTelemetry logging in `srcs/server/telemetry/harness.go` tracking execution events like `harness_tool_executed`.

## Priority
P1

## Estimated Scope
Medium
