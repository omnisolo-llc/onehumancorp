<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🔬 OHC Market Research Report: Unified Harness Architecture (Bwrap, Proxy, AST)

## Title
Implement Unified Bwrap Sandboxing, Egress Proxy, and AST Validation Engine

## Problem Statement
OHC’s Agent Harness currently uses rudimentary regex to filter shell commands (`bash_sandbox.go`) and executes processes with the same host permissions and network access as the orchestrator. Without robust execution boundaries, agents risk modifying critical host files (sandbox escape) or performing unauthorized network egress. Conversely, market leaders like Claude Code isolate shell commands via unprivileged namespaces (`bwrap`), employ dynamic system-call blocking (`seccomp-bpf`), intercept all network traffic via local HTTP/SOCKS proxies, and use abstract syntax tree (AST) validation for granular command-level restriction.

## Research Report
### Deep Audit: Leaked Claude Code (v2.1.88)
Claude Code encapsulates agent terminal execution inside an `@anthropic-ai/sandbox-runtime` using several critical primitives:

1. **Bubblewrap (`bwrap`) OS Sandbox:** On Linux, it strictly partitions the filesystem using `bwrap`. It applies `--ro-bind` (read-only) for sensitive paths and `/`, and `--bind` exclusively for working directories. Non-existent deny paths are mitigated via empty ghost mount-points.
2. **Network Egress Proxying:** Instead of broad network access, a local `socat` HTTP/SOCKS proxy is injected into the namespace. The proxy monitors and evaluates all egress against `allowedDomains` dynamically, pausing execution to prompt the human if access is required for unknown URLs.
3. **AST Validation:** Commands aren't just strings; they are parsed via `shell-quote` or Tree-sitter. It explicitly detects dangerous behaviors (like malicious `>` redirections, UNC paths on Windows, and carriage return injections via `$IFS`) that regexes miss.
4. **Seccomp Filters:** It blocks unsafe operations at the kernel level by applying generated BPF filters (`seccomp`), explicitly denying operations like Unix Domain socket creation to avoid proxy bypassing.

### Comparative Table: OHC vs Market

| Feature Capability | OHC Current State | Market Standard (Claude Code) | Gap Impact |
| :--- | :--- | :--- | :--- |
| **Command Execution** | Raw `exec.Command` | Wrapped via `SandboxManager` & AST Validator | 🚨 Critical (P0) |
| **OS Sandboxing** | None (Host Default) | Bubblewrap namespaces & Seccomp filters | 🚨 Critical (P0) |
| **Network Governance**| Broad Host Egress | Task-scoped SOCKS/HTTP MITM Proxy | 🟡 High (P1) |

## Design Doc
We propose the **Unified Agent Worktree Harness (UAWH)** in `src/server/harness/`, which integrates the KAIROS Orchestrator to the host securely:

1.  **AST Parser (`parser.go`)**: Pre-flight command validation leveraging an AST parser (like `mvdan.cc/sh/v3/syntax`) to block obfuscated injections, subshells, and redirections.
2.  **OS Sandbox Runner (`bwrap.go`)**: Wraps `bwrap` execution. Enforces `--ro-bind / /` globally and specific `--bind <workspace> <workspace>` access. Drops namespaces (`--unshare-all`).
3.  **Network Proxy & Proxy Telemetry (`proxy.go`)**: Sets up a local `socat`/Go-based proxy. Injects `HTTP_PROXY` inside the `bwrap` jail to evaluate network requests.
4.  **Security Observability (`telemetry.go`)**: Binds blocked AST evaluations and blocked network egress to Prometheus via OpenTelemetry (`ohc_harness_security_violation_total`).

### Architecture Diagram

```mermaid
graph TD
    A[Agent Command Request] --> B{AST Policy Parser}
    B -- Parse Failure / Deny --> C[Reject & Log Telemetry]
    B -- Passed --> D{bwrap Sandbox Runner}

    subgraph Secure Execution Jail
        D -->|Wraps Exec| E[Bubblewrap Namespace]
        E -.->|Enforce ro-bind /| F[FS Isolator]
        E -.->|Enforce Seccomp| G[Syscall Blocker]
        E -.->|Route Net| H[Local MITM Proxy]
        H -->|Denied| C
        H -->|Allowed| I((Internet))
        E --> J[Agent Process Execution]
    end

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F,G,H,I,J premium;
```

## Implementation Prompt
**Role:** Implementer Agent
**Task:** Build the core execution engine of the Unified Agent Worktree Harness.

1.  **AST Parser Module**: In `src/server/harness/parser.go`, implement a `ValidateCommand(cmd string) error` function using `mvdan.cc/sh/v3/syntax`. Parse the command and return an error if the AST contains redirection nodes (`>`) or subshell executions (`$()`).
2.  **Bwrap Runner**: In `src/server/harness/bwrap.go`, implement a `RunInSandbox(ctx context.Context, cmd string, workspace string, allowNet bool) error` adapter. Construct the `bwrap` command with flags: `--unshare-all`, `--ro-bind / /`, `--bind <workspace> <workspace>`. Add `--share-net` if network is allowed.
3.  **Network Proxy Adapter**: In `src/server/harness/proxy.go`, implement a local HTTP MITM proxy that inspects network requests and only passes them through if the requested domain is explicitly allowed by the agent config.
4.  **Metrics Integration**: Add OpenTelemetry counters `ohc_harness_security_violation_total` and increment when `ValidateCommand` fails or proxy requests are blocked. Ensure `telemetry.RedactInterfacePII` sanitizes inputs.
5.  **Testing**: Write comprehensive Go unit tests (`parser_test.go`, `bwrap_test.go`, `proxy_test.go`) achieving 100% test coverage. Supply malicious strings to `parser_test.go` to ensure they are trapped.

## Priority
P0

## Estimated Scope
Large

</div>
