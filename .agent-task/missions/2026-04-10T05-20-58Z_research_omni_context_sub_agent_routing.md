---
status: DONE
agent: Researcher
priority: P0
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 24px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Title: Omni-Context Sub-agent Routing

## Problem Statement
Current agentic systems (like Claude Code, Replit Agent, and OpenClaw) suffer from context latency and grounding drift during sub-agent delegation. Sub-agents must manually fetch project grounding files (`CLAUDE.md`, `AGENTS.md`) at instantiation, leading to increased time-to-first-token, excess token burn, and a high risk of architectural hallucination if files are skipped or misread. OHC can capitalize on its Hybrid Architecture to bridge this gap with zero-latency Omni-Context routing.

## Research Report
- **Market Reality:** Replit Agent and Claude Code depend on sub-agents to independently discover and fetch context via file system reads. OpenClaw relies on event routing but lacks strict enforcement for Bazel-native environments.
- **The Gap:** Fetching rules adds compute latency and increases cloud inference costs (unnecessary tool loops dedicated to reading context).
- **OHC's Advantage (Omni-Context Sub-agent Routing):** Instead of explicit file fetching, the KAIROS Orchestrator utilizes the Swarm Intelligence Protocol (OHC-SIP). When spawning a sub-agent, the orchestrator directly injects project grounding files into the `agent_missions` payload at the exact moment of task creation. Sub-agents begin reasoning immediately with complete, hermetic architectural awareness.

### Competitive Market Audit

| Feature Area | Claude Code / Replit Agent | OpenClaw | **OHC Vision (Omni-Context)** |
| :--- | :--- | :--- | :--- |
| **Grounding Strategy** | Explicit file read (Adds Latency) | Event-based state | **Pre-injected Database Payloads** |
| **Sub-agent Delegation** | Ad-hoc CLI spawning | Configured routing rules | **Swarm-as-Code `agent_missions` row creation** |
| **Architectural Alignment** | Vulnerable to context-window drops | Rigid custom schemas | **Hermetic, zero-latency Bazel-native context** |

```mermaid
graph TD
    A[KAIROS Orchestrator] -->|Delegates Task| B{Context Injector (SIPDB)}
    B -->|Reads Grounding| C[(AGENTS.md / CLAUDE_OHC.md)]
    B -->|Writes Mission+Context| D[(Postgres / SQLite: agent_missions)]
    D -->|Instantiates| E[Specialized Sub-Agent]
    E -->|Executes with Zero Latency| F[Task Completion]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,D,E,F premium;
    class B,C premium;
```

## Design Doc
- **Target Module:** `srcs/server/orchestration/sip.go` or `tasks.go` (specifically the sub-agent delegation flow).
- **Core Mechanism:** Extend the current `agent_missions` table injection logic to scan the project root for standard grounding files (e.g., `AGENTS.md`, `CLAUDE.md`, or specific OHC documentation like `CLAUDE_OHC.md`).
- **Data Injection:** Read the contents of identified grounding files and securely append them directly to the `payload` TEXT blob under a formalized namespace, e.g., `[SYSTEM GROUNDING]`.
- **Database Schema:** `agent_missions` (already includes `id`, `status`, `payload`, `created_at`). The injected context is stored entirely within the `payload` column, maintaining current DB structure but vastly expanding the context delivered per row.

## Implementation Prompt
Hello Implementer agent!
1. Please locate the `DelegateMission` or sub-agent spawning function in `srcs/server/orchestration/sip.go` (or `tasks.go`).
2. Implement an automatic context injection step: before a mission is saved to the `agent_missions` table, check the current workspace for grounding files (e.g., `AGENTS.md`, `CLAUDE.md`, `CLAUDE_OHC.md`).
3. If found, append their contents directly to the `payload` under a `[SYSTEM GROUNDING]` tag.
4. Ensure the modified payload is still correctly serialized/inserted into the SQLite (Standalone) or PostgreSQL (Cloud-native) `agent_missions` table.
5. Add rigorous unit testing to confirm context is properly appended to delegated missions, and achieve >90% coverage for the new feature.

## Priority
P0

## Estimated Scope
Medium

</div>
