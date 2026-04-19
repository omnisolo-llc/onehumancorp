<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1);">

# 🔬 OHC Oracle Research Report: Agent Harness & Execution Isolation

**Date:** 2026-04-18
**Target Analyzed:** Claude Code (v2.1.88)
**Focus Area:** Agent Harness Environment, Sandboxing, Shell Execution Lifecycle

## 1. Executive Summary
This research investigates the operational harness of *Claude Code*, a leading CLI-based AI agent, to identify structural gaps in OHC's local and hybrid execution models. The findings highlight immediate opportunities to harden OHC's execution environment using OS-level sandboxing.

## 2. Competitive Architectural Analysis

Claude Code uses a sophisticated **Sandbox Adapter** wrapping an external `@anthropic-ai/sandbox-runtime` package. This layer intercepts, isolates, and monitors all interactions between the LLM and the host OS.

### OS-Level Sandboxing & Telemetry
- **Bubblewrap (`bwrap`) Integration:** On Linux, Claude uses `bwrap` to spawn tightly restricted child processes. It dynamically mounts permitted read/write paths (`allowRead`, `denyWrite`) and bind-mounts `/dev/null` over denied paths.

### Memory & Context Management
- **Memory Directory Pattern:** State is handled via a dedicated Memory Directory pattern (`memdir.ts`).

### Tooling Integration
- **Native MCP:** Claude Code relies entirely on the Model Context Protocol (MCP) as its core integration layer.

## 3. OHC vs. Market Reality

| Feature | OHC (Current State) | Market Standard (Claude Code) | Gap Resolution |
| :--- | :--- | :--- | :--- |
| **Sandboxing Isolation** | OS-level generic boundaries | Strict FS/Network per-command rules (`bwrap`) | Integrate scoped sandbox policies |
| **Tool Integration** | Bespoke Interfaces | Native MCP (Model Context Protocol) | Implement manager |
| **State Sharing** | Centralized OHC-SIP | File-based memory directory (`memdir.ts`) | Consider adding local memdir |

## 4. Architectural Blueprint

```mermaid
graph TD;
    A[KAIROS Orchestrator] -->|Dispatch Task| B(Hybrid Agent Harness);
    B -->|Enforce read/write| C{Bubblewrap Sandbox};
    C --> D[Sub-Agent Execution];

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D premium;
```

</div>
