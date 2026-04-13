---
status: PENDING
agent: Researcher
priority: P1
---

# Title: Integrate Hybrid Code Execution MCP Server

## Problem Statement
OHC agents need to execute code securely to accomplish complex tasks, such as data analysis, building software, or running tests. In Cloud-Native Mode (K8s), this requires a highly sandboxed, ephemeral environment (e.g., restricted pods, Firecracker microVMs, or gVisor) to prevent malicious or accidental code execution from compromising the host or other tenants. In Standalone Desktop Mode (SQLite/Local), execution can be performed within a local Docker container or, with explicit user permission, directly on the host machine. Without a unified interface, agents cannot execute arbitrary logic consistently across both environments.

## Research Report
- **Market Context**: Platforms like Replit Agent operate entirely in a cloud-hosted containerized environment. Claude Code relies on local host execution. There is no major agentic OS providing a unified execution protocol bridging isolated cloud runtimes and local host execution.
- **OHC Requirement**: A "Hybrid Code Execution MCP Proxy" that provides tools for agents to run shell commands, python scripts, and other code safely.
- **Tooling Discovery**: A dedicated MCP adapter wrapping an interface like `mcp.CodeExecutionProvider`.
- **Security & Multi-Tenancy**: The MCP must validate that agents only execute code within their authenticated tenant's isolated sandbox in `OHC_MULTITENANT` mode. Resource limits (CPU/Memory/Time) must be strictly enforced.

## Design Doc
- **Module Path**: `srcs/server/tools/codeexecmcp`
- **Architecture**: A new Go module within `srcs/server/tools/codeexecmcp`. Tools should include `execute_code`, `run_shell_command`.
- Introduce `mcp.CodeExecutionProvider`. In `OHC_MULTITENANT=true`, use `CloudExecutionProvider` (scales via K8s/Firecracker). In `OHC_STANDALONE=true`, use `LocalExecutionProvider` (local Docker/host).

## Implementation Prompt
Hello Implementer agent!
1. Create a new directory `srcs/server/tools/codeexecmcp`.
2. Abstract the code execution logic behind an interface `mcp.CodeExecutionProvider`.
3. Create a unified MCP server in `srcs/server/tools/codeexecmcp` that implements `ListTools` and `CallTool`.
4. Ensure tenant isolation and resource limits are enforced in cloud mode.
5. Provide tests that achieve >90% coverage.

## Estimated Scope
Medium
