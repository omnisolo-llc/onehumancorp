<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🔬 OHC Market Research Report: Agent Harness Isolation & AST Validation

## Title
Implement Unified Bwrap Sandboxing & AST Bash Validation Engine

## Problem Statement
The OHC Agentic OS requires absolute autonomy safely. Currently, OHC's local agent harness lacks robust boundary constraints when executing shell commands, allowing agents broad access to the host's filesystem and network. This exposes the host to unintended modification and network exfiltration. Competitive products (e.g., Claude Code) utilize deep, OS-level namespaces and abstract syntax tree (AST) validation to enforce zero-trust execution. To achieve safe Swarm Intelligence, OHC must adopt an airtight execution harness natively integrated with our KAIROS orchestration engine.

## Research Report
### Target Analyzed: Leaked Claude Code (v2.1.88)
Claude Code implements a sophisticated isolation architecture via its `@anthropic-ai/sandbox-runtime`.

**1. OS-Level Isolation (`bwrap`)**
On Linux, the harness exclusively executes agent commands within `bwrap` (Bubblewrap) namespaces.
- It dynamically computes read-only and write permissions, mapping `/` as read-only (`--ro-bind`) and bind-mounting specific workspaces as read-write (`--bind`).
- It drops all unneeded namespaces (`--unshare-all`).

**2. Syscall Blocking (Seccomp-BPF)**
- The harness dynamically generates and applies a `seccomp-bpf` filter to the child process. Crucially, this blocks Unix domain socket creation, preventing the agent from establishing IPC channels to bypass network isolation.

**3. Network Interception & Proxying**
- Network traffic is forcefully routed through a local `socat` HTTP/SOCKS proxy daemon (`--share-net`).
- The proxy acts as a MITM layer, evaluating all HTTP/TCP connections against a dynamic allowlist. Unknown domains trigger a user-prompt (AskCallback).

**4. AST-Based Security Pre-Flight**
- Before execution, Claude Code's `BashTool` parses the raw bash string using a Tree-sitter AST or `shell-quote` equivalent.
- This allows it to reliably block destructive shell injections, such as `>()`, backslash-escaped operators, UNC paths, and quote desynchronizations—which simple regexes fail to catch.

### Comparative Matrix: OHC vs Market

| Feature Capability | OHC Current State | Market Standard (Claude Code) | Gap Impact |
| :--- | :--- | :--- | :--- |
| **Command Execution** | Raw `exec.Command` | Wrapped via `SandboxManager` & AST Validator | 🚨 Critical (P0) |
| **OS Sandboxing** | None (Host Default) | Bubblewrap namespaces & Seccomp filters | 🚨 Critical (P0) |
| **Network Governance**| Broad Host Egress | Task-scoped SOCKS/HTTP MITM Proxy | 🟡 High (P1) |

## Design Doc

### Proposed Architecture: The Unified Hybrid Harness (UHH)
We will introduce the **Unified Hybrid Harness** in `src/server/harness/`. It acts as the execution layer between the KAIROS Orchestrator and the host OS.

1.  **AST Policy Engine (`src/server/harness/parser.go`)**:
    *   A pre-flight validation phase. All bash commands will be parsed into an AST using an AST library (e.g., `mvdan.cc/sh/v3/syntax`).
    *   Validators will explicitly check for blocked subshell executions, redirects, and unauthorized alias usage.
2.  **OS Sandbox Runner (`src/server/harness/bwrap.go`)**:
    *   A Go adapter that wraps standard commands in `bwrap`.
    *   Enforces `--ro-bind / /` and restricts write access via `--bind` strictly to the agent's task workspace.
    *   Unshares network and PID namespaces by default.
3.  **Network Proxy Daemon (`src/server/harness/proxy.go`)**:
    *   Spawns a local HTTP/SOCKS proxy.
    *   Dynamically sets `HTTP_PROXY` inside the `bwrap` environment, proxying all egress traffic against a `TaskEgressPolicy` struct.
4.  **Telemetry Hook**:
    *   Every blocked AST check or intercepted network request emits an OpenTelemetry metric (`ohc_harness_security_violation_total`).

### Architecture Diagram

```mermaid
graph TD
    A[KAIROS Orchestrator] -->|Command Request| B(AST Policy Engine)
    B -->|Parse Failure / Deny| C[Reject & Log Telemetry]
    B -->|Passed| D{UHH OS Sandbox Runner}

    subgraph Secure Execution Sandbox
        D -->|Wraps Exec| E[bwrap Namespace]
        E -.->|Mount / as RO| F[Filesystem Isolator]
        E -.->|Route Net| G[Local MITM Proxy]
        G -->|Denied| C
        G -->|Allowed| H((Internet))
        E --> I[Sub-Agent Shell]
    end

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F,G,H,I premium;
```

## Implementation Prompt

**Role:** Implementer Agent
**Task:** Build the core execution engine of the Unified Hybrid Harness.

1.  **Create the AST Parser:** In `src/server/harness/parser.go`, implement a `ValidateCommand(cmd string) error` function using `mvdan.cc/sh/v3/syntax` to parse the command. If the AST contains redirection nodes (`>`) or subshell execution (`$()`), return an error.
2.  **Create the Bwrap Runner:** In `src/server/harness/bwrap.go`, implement `RunInSandbox(ctx context.Context, cmd string, workspace string, allowNet bool) error`.
    *   Construct a command slice starting with `bwrap`.
    *   Add flags: `--unshare-all`, `--ro-bind / /`, `--bind <workspace> <workspace>`.
    *   If `allowNet` is true, add `--share-net`.
    *   Execute the target `cmd` inside this wrapper.
3.  **Metrics Integration:** Add an OpenTelemetry counter `ohc_harness_security_violation_total`. Increment it inside `ValidateCommand` if validation fails. Ensure `telemetry.RedactInterfacePII` is applied to any logged payload.
4.  **Testing:** Provide 100% test coverage in `parser_test.go` and `bwrap_test.go`. Include tests that verify malicious bash strings (e.g., `echo test > /etc/passwd`) are rejected by the AST parser.

## Priority
P0

## Estimated Scope
Large

</div>
