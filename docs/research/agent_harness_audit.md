<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.05); color: #fff; border-radius: 12px; padding: 24px; border: 1px solid rgba(255,255,255,0.1);">

# 🔬 OHC Market Research: Agent Harness Architecture Analysis
**Target:** Leaked Claude Code (v2.1.88), OpenClaw, Gstack
**Analyst:** Principal Product Researcher & Oracle (L7)

## Executive Summary
This report analyzes the core architectural patterns and "Agent Harness" design of leading open source and leaked agent repositories (Claude Code, OpenClaw, Gstack) and compares them against One Human Corp's (OHC) current hybrid architecture (OHC-HA).
The objective is to identify critical gaps and define actionable missions to elevate the OHC platform towards absolute autonomy and aesthetic excellence.

## 1. Claude Code: Interactive Terminal Harness & Sandboxing

The Claude Code repository (v2.1.88) utilizes a robust Agent Harness designed for stateful, interactive, and highly sandboxed execution:

*   **Interactive Harness (`interactiveHelpers.tsx`, `dialogLaunchers.tsx`)**: Uses React (`.tsx` files) to render terminal UIs (via `ink`). This provides a rich, structured visual experience in a CLI environment.
*   **Unified Tool Registry (`Tool.ts`, `tools.ts`)**: A highly modularized tool system. The presence of 44 subdirectories under `tools/` indicates a massive surface area of capabilities. Tools are strongly typed and self-describing.
*   **Sandbox Adapter (`sandbox-adapter.ts`)**: Wraps `@anthropic-ai/sandbox-runtime` to intercept, isolate, and monitor interactions between the LLM and the host OS. Uses strict read/write permission mappers and intercepts network traffic (`sandboxAskCallback`).
*   **Cost & Resource Tracking (`cost-tracker.ts`)**: Built-in mechanisms to track token usage and API costs in real-time within the harness.

## 2. OpenClaw: The Plugin Harness Registry

OpenClaw implements a dynamic **Agent Harness Plugin** architecture (`sdk-agent-harness.md`):

*   **Harness Registry**: It allows registering different low-level executors for prepared OpenClaw agent turns.
*   **Dynamic Resolution**: The harness is selected after resolving the provider and model (e.g., falling back to a built-in PI harness if a native native coding-agent server fails).
*   **Codex Harness (`codex-harness.md`)**: A bundled app-server harness specifically for executing native threads, compaction, and app-server execution, while OpenClaw still owns the visible transcript mirror and tools.

## 3. Gstack: Parallel Workspace Isolation

Gstack's "Conductor" implements a different paradigm for harness isolation:

*   **Parallel Sprint Workspaces**: To enable horizontal agent scaling, Conductor runs 10-15 parallel agent sprints by isolating each session in its own temporary workspace (similar to `git worktree`), preventing file collisions between agents.

## 4. OHC vs. Market Reality (Comparative Analysis)

| Feature | OHC Hybrid Architecture | Market Leader (Claude/OpenClaw/Gstack) | Gap Priority |
| :--- | :--- | :--- | :--- |
| **Harness Flexibility** | Direct execution | OpenClaw's dynamic Harness Plugin Registry | **Medium**: OHC needs a flexible registry for different execution backends. |
| **Tool Modularity** | Fragmented | Claude's Unified Tool Registry (UTR). | **High**: OHC needs a unified, heavily typed tool registry. |
| **Terminal/CLI UX** | Basic CLI | Claude's Interactive Terminal Harness (React/Ink). | **High**: Develop a "Premium" aesthetic CLI for Standalone mode. |
| **Cost Awareness** | Backend/billing focused | Claude's real-time in-harness cost tracking. | **Medium**: Agents must be aware of their own burn rate. |
| **Parallel Isolation** | Shared file system | Gstack's Parallel Workspace Isolation. | **High**: OHC needs `git worktree` isolation for swarm scaling. |

## 5. Architectural Gap Visualization

```mermaid
graph TD
    subgraph Market Standards
        C_Harness[Interactive Harness] --> C_Cost[Cost Telemetry]
        C_Harness --> C_UTR[Unified Tool Registry]
        O_Harness[Harness Plugin Registry] --> O_Codex[Native App Server]
        G_Harness[Conductor] --> G_Worktree[Parallel Workspaces]
    end

    subgraph OHC Future Architecture
        T_Registry[Harness Plugin Registry]
        T_Registry --> T_ITH[Interactive Terminal Harness - ITH]
        T_ITH --> T_Cost[Real-Time Cost Telemetry]
        T_ITH --> T_UTR[Unified Tool Registry - UTR]
        T_Registry --> T_PW[Parallel Workspace Harness]
        T_PW --> T_WT(Isolated Worktrees)
    end

    Market Standards -.->|Design Inspiration| OHC Future Architecture
```

## 6. Actionable Missions (GitHub Issues)

The following missions have been extracted and submitted to the OHC GitHub repository (`onehumancorp/mono/issues`):

1.  **Mission 1: [backend] Implement Unified Tool Registry (UTR) with Strong Typing** (Created: #5884)
    *   Goal: Standardize how agents discover and execute tools, inspired by Claude Code's `Tool.ts`.
2.  **Mission 2: [harness] Implement Interactive Terminal Harness (ITH) with React/Ink for Standalone Mode** (Created: #5885)
    *   Goal: Create a visually stunning, interactive CLI experience for Standalone mode, rivaling web UIs.
3.  **Mission 3: [telemetry] Inject Real-time Token and Cost Tracking into Execution Loop** (Created: #5886)
    *   Goal: Agents must track their own resource consumption in real-time, matching the `cost-tracker.ts` capability.

</div>
