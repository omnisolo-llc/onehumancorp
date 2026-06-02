<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Document: Omni-Context Sub-agent Routing

**Author:** Principal Product Researcher & Oracle (L7) / TPM Agent
**Status:** Approved
**Date:** 2026-03-20

## 1. Objective
To leapfrog existing agentic orchestration frameworks (e.g., Claude Code, OpenClaw) by implementing "Omni-Context Sub-agent Routing." This feature seamlessly bridges the gap in project grounding by automatically injecting critical project-level context (such as `AGENTS.md` or `CLAUDE.md`) into the Swarm Intelligence Protocol Database (`agent_missions` table) precisely at the time of task delegation.

## 2. Motivation
Currently, sub-agents spawned by orchestrating agents must proactively discover and parse grounding files to align with the project's macro-context. This approach introduces latency, heightens the risk of hallucination (if grounding files are missed), and degrades operational efficiency. By natively injecting this context into the initial task payload, OHC ensures that newly instantiated sub-agents possess immediate, deterministic awareness of project guidelines without explicitly executing "read file" tools.

## 3. Architecture

The feature hooks into the `DelegateMission` function of the orchestration layer (`src/server/sip.rs`).

When an orchestrating agent identifies the need for a sub-agent and delegates a task:
1.  **Context Interception:** The `DelegateMission` routine checks if a `ContextRoot` is configured.
2.  **File Reading:** If configured, it sequentially searches the context root for `AGENTS.md` and `CLAUDE.md`.
3.  **Payload Injection:** The content of the first found grounding file is appended to the `task.Content` field with the strict prefix `[SYSTEM GROUNDING]:\n`.
4.  **Database Storage:** The enriched task payload is serialized and inserted into the `agent_missions` table.

### 3.1 Diagram

```mermaid
graph TD
    A[Orchestrating Agent] -->|Identifies Need| B(Task Generator)
    B --> C{Context Injector}
    C -->|Reads| D[(AGENTS.md / CLAUDE.md)]
    C -->|Injects Grounding| E[(OHC-SIP DB: agent_missions)]
    E -->|Instantly Routed| F[Sub-Agent Executor]
    F -->|Executes with Zero Latency| G[Task Completion]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,F,G premium;
    class C,D,E premium;
```

## 4. Technical Implementation Details
*   **SIPDB Modification:** The `SIPDB` struct in `src/server/sip.rs` requires a `ContextRoot` string field to identify where to search for grounding files.
*   **Injection Logic:** Inside `DelegateMission`, implement a fallback check for `AGENTS.md`, then `CLAUDE.md`. If either is found, append its content.
*   **Database Schema:** No schema changes are required to the `agent_missions` table, as the payload modification occurs prior to JSON serialization.

## 5. Security & Privacy
The injection mechanism is read-only against the local workspace and occurs strictly server-side within the trusted k8s orchestrator environment. No external APIs are called during context injection, adhering to the "Fail Closed" security mandate.

## 6. Alternatives Considered
*   **Ad-Hoc File Reads:** Relying on agents to read `AGENTS.md` via MCP tools. *Rejected* due to latency and alignment drift.
*   **Continuous Vector DB Sync:** Indexing project files into a vector store for semantic search. *Rejected* for primary grounding because it lacks deterministic enforcement. Omni-Context Routing guarantees the agent sees the exact rules.

</div>
