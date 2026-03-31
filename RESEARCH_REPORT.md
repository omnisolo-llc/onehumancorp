
# Market Audit & OHC Unfair Advantage: Omni-Context Sub-agent Routing

**Author**: Principal Product Researcher & Oracle (L7)
**Date**: 1774890523

## Executive Summary

After auditing the leading agentic operating systems in the global market, we've identified key trends in multi-channel routing, project grounding, and context retention. OHC has an opportunity to leapfrog these capabilities with an "Unfair Advantage" we're calling **Omni-Context Sub-agent Routing**.

## Market Reality vs. OHC

This analysis compares OHC to Claude Code, OpenClaw, and OpenCode across core agent orchestration parameters.

| Feature Area | Claude Code | OpenClaw | OpenCode | **OHC Vision** |
| :--- | :--- | :--- | :--- | :--- |
| **Session Persistence** | File-based context (`CLAUDE.md`) | Event-driven cross-channel persistence | Project-level `AGENTS.md` grounding | **OHC-SIP Database-Driven** with continuous synchronization |
| **Sub-agent Delegation** | Sub-agents spawned ad-hoc | Route to specialized nodes | Specialized roles defined via project metadata | **Swarm-as-Code** deterministic routing via `agent_missions` |
| **Tool Execution** | MCP (Model Context Protocol) | Custom integrations | Command hooks | **Universal MCP Mesh** native to k8s/Bazel build logic |
| **Context Retrieval** | Explicit file read | Persistent event history | On-demand indexing | **Instant Ingestion** via Omni-Context Routing |

## The Next "Unfair Advantage"

### Identification of the Delta
While Claude Code and OpenCode depend on agents actively discovering and reading grounding files (`CLAUDE.md` and `AGENTS.md`), this introduces latency and potential alignment drift. OpenClaw provides robust event routing but lacks the strict architectural enforcement of Bazel-first environments.

### Mission Brief: Omni-Context Sub-agent Routing
OHC will bridge this gap by directly embedding project grounding context into the swarm database at the time of task delegation.

When an orchestrating agent creates a new mission for a sub-agent, the system will automatically inject instructions from standard grounding files (like `AGENTS.md`) directly into the `agent_missions` payload.

**Impact:**
- **Zero-Latency Grounding**: Any newly spawned sub-agent instantly has complete project-level context.
- **Perfect Alignment**: Reduces hallucination and drift since the ground truth is injected deterministically.
- **Aesthetic Integration**: Maintains the OHC database-driven architecture without forcing agents to execute boilerplate "read file" tool calls.

## Visualizing the Architecture

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

## Aesthetic Styling Tokens

To adhere to the **Aesthetic Excellence Mandate**, the above visualization and the presentation layer of this report utilize the following tokens:

```css
.ohc-card {
    backdrop-filter: blur(20px) saturate(200%);
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    font-family: 'Outfit', 'Inter', sans-serif;
    color: #ffffff;
    border-radius: 12px;
    padding: 24px;
}
```

## Validation & Feasibility

Technical feasibility has been verified. Modifying the `DelegateMission` method in `srcs/orchestration/sip.go` to optionally append file contents from a defined context root is achievable within the current k8s/Bazel environment.

This architecture proposal has been submitted to `product_architecture` via the Swarm Intelligence Protocol DB.
