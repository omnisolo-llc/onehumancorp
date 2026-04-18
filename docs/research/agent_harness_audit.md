<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# OHC Oracle Research Report: Agent Harness & Execution Isolation

**Date:** 2026-04-18
**Target Analyzed:** Claude Code (v2.1.88)
**Focus Area:** Agent Harness Environment, Sandboxing, Shell Execution Lifecycle

## 1. Executive Summary
This research investigates the operational harness of *Claude Code*, a leading CLI-based AI agent, to identify structural gaps in OHC's local and hybrid execution models. The findings highlight immediate opportunities to harden OHC's execution environment using deep OS-level sandboxing, granular AST bash parsing, and robust network proxies.

## 2. Competitive Architectural Analysis

Claude Code uses a sophisticated **Sandbox Adapter** wrapping an external `@anthropic-ai/sandbox-runtime` package. This layer intercepts, isolates, and monitors all interactions between the LLM and the host OS.

### OS-Level Sandboxing & Telemetry
- **Bubblewrap (`bwrap`) Integration:** On Linux, Claude uses `bwrap` to spawn tightly restricted child processes. It dynamically mounts permitted read/write paths and bind-mounts `/dev/null` over denied paths.
- **Seccomp Filters:** Dynamically generated BPF filters restrict available system calls.
- **Network Telemetry and Proxying:** A local custom HTTP/SOCKS MITM proxy intercepts all network traffic, evaluating requests against allow/deny lists.

### Shell Execution Strategies
- **Semantic Bash Sandboxing:** Instead of blindly running strings, `BashTool` performs AST-based semantic analysis, blocking dangerous patterns like `>()`, obfuscated variables, and dangerous redirection (`2>nul`).
- **Stateful REPL Simulation:** Agents need context across commands. Claude writes environment (`declare -p`) and path snapshots (`pwd -P`) to temporary files, `source`ing them before new commands.
- **TMPDIR Jailing:** Every spawned shell process uses an overridden, securely permissioned (`0700`) temporary directory specific to that session. Security flags like `shopt -u extglob` are explicitly injected.

## 3. OHC vs. Market Reality

| Feature | OHC (Current State) | Market Standard (Claude Code) | Gap Impact |
| :--- | :--- | :--- | :--- |
| **Command Execution** | Raw `exec.Command` | Wrapped via `ShellProvider` & AST Validation | High |
| **State Persistence** | Stateless per command | Snapshot/Restore (`source` + `pwd`) | High |
| **Sandboxing & Isolation** | Host Default | OS-level `bwrap` with Seccomp & `0700` TMPDIR | Critical |
| **Network Interception** | Allowed (Host Network) | Hooked via Local Proxy | Medium |

## 4. Architectural Blueprint

```mermaid
graph TD;
    A[KAIROS Orchestrator] -->|Dispatch Task| B(Hybrid Agent Harness);
    B -->|Enforce read/write| C{Bubblewrap Sandbox};
    C --> D[Sub-Agent Execution];
    D -->|Network Req| E(Local Intercept Proxy);
    E -->|Allowed| F((Internet/Intranet));
    E -->|Denied| G[Drop & Log Telemetry];
    D -->|Stdout/Stderr| H[OpenTelemetry Span Exporter];
    H --> I[(OHC Central Database)];
    G --> I;

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F,G,H,I premium;
```

</div>
