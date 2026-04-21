<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🔬 OHC Hybrid Agent Harness: Visual Walkthrough

Welcome to the **Hybrid Agent Harness** visual walkthrough. This document outlines the architectural components of OHC's execution sandboxing layer, illustrating how we achieve true zero-trust autonomy.

## 1. The Core Architecture

OHC uses a robust `SandboxManager` that bridges KAIROS orchestration with OS-native primitives, drawing inspiration from leading implementations.

- **OS-Level Sandboxing (`bwrap`):** On Linux, the harness heavily leverages `bwrap` to spawn tightly restricted child processes with explicit `allowRead` and `denyWrite` directives.
- **Network Proxy Interception:** Every execution forces traffic through a localized HTTP/SOCKS MITM proxy to drop unauthorized API calls.
- **Semantic AST Parsing:** The Bash execution tool intercepts and validates compound commands or redirections, preventing escapes before they even reach the shell.

## 2. Harness Execution Flow

This diagram illustrates how a task request moves through the orchestration layer and into the secured sandbox.

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

## 3. Parallel Workspaces & Git Worktrees

To allow horizontal sub-agent scaling locally without filesystem collisions, the harness implements **Parallel Sprint Workspaces**. When an agent forks, it receives an isolated `git worktree`.

```mermaid
graph TD
    O[KAIROS Hub] -->|Spawns| PW[Parallel Workspace Harness]
    O -->|Spawns| VH[Verification Harness]
    PW -->|Git Worktree| WT(Isolated Worktree)
    VH -->|Read Only Mount| RO(Source Code)
    VH -->|RW Mount| Ephemeral(/tmp)

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class O,PW,VH,WT,RO,Ephemeral premium;
```

## 4. MCP & Memory Directory (MemDir) Integration

Agents maintain persistence through locally configured Memory Directories (`.ohc/memory/auto`), while interactions with external tools flow exclusively through the Model Context Protocol (MCP) bridge to ensure cloud-to-local hybrid synchrony.

</div>
