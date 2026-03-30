# Design Doc: Modular Capability Plugin Mesh & Next-Gen Aesthetics

**Author(s):** Antigravity, Principal Product Architect & Visionary (L7)
**Status:** Approved
**Last Updated:** 2026-03-28

## 1. Overview & Vision
The "One Human Corp" Agentic OS is transitioning from static Skill Blueprints to a dynamic, decentralized **Capability Plugin Mesh**. This addresses the architectural bottleneck of hardcoded tool schemas that previously hindered 100% agent autonomy and rapid organizational scaling.

Coupled with this structural overhaul, we are mandating the **Next-Generation "Premium Feel" Design System** to ensure the platform’s interface visually communicates fluidity, depth, and enterprise-grade reliability.

## 2. The Architectural Bottleneck: Static Tool Registration
Previously, agent capabilities were statically defined and deeply coupled to specific MCP tool schemas. If a new business domain required a custom tool, human intervention was required to re-deploy the MCP Gateway with the updated schemas. This prevented agents from autonomously discovering and utilizing new tools.

### 2.1 The Solution: Capability Plugin Mesh
The **Capability Plugin Mesh** abstracts capabilities into standalone Kubernetes services that expose a standardized `CapabilityManifest`.

When a new capability is deployed to the cluster, the MCP Gateway dynamically discovers it. Agents, via the Switchboard, query the Gateway for capabilities matching their current task's intent, seamlessly binding to the new tool at runtime without human intervention or cluster re-deployment.

## 3. Database Schema Updates (OHC-SIP)
To support the Plugin Mesh, the Swarm SQLite DB (`swarm_memory` and related tables) will be extended to track capability state and semantic embeddings for discovery:

*   **`capability_plugins`**: Tracks registered capability endpoints, their health status, and associated MCP schemas.
*   **`swarm_memory_embeddings`**: Stores vector representations of capabilities to allow agents to perform semantic search during discovery (e.g., "I need a tool to query Postgres").

## 4. Next-Generation Design System Tokens
The "Premium Feel" Aesthetic Mandate requires the entire OHC frontend to adopt the following design tokens. This visual language hides infrastructure complexity behind an elegant, glass-like interface.

*   **Core Structure (Glassmorphism):**
    *   `backdrop-filter: blur(15px) saturate(180%)`
    *   `background: rgba(255, 255, 255, 0.05)`
*   **Definition & Border:**
    *   `border: 1px solid rgba(255, 255, 255, 0.1)`
*   **Typography:**
    *   `font-family: 'Outfit', 'Inter', sans-serif`
*   **Interaction:**
    *   Smooth, easing transitions for data population to visually represent the asynchronous nature of agent capability binding.

## 5. Architectural Flow

```mermaid
graph TD
    %% Capability Providers
    P1[New Service: Data Analyst Tool]
    P2[New Service: Web Scraper]

    %% Discovery Layer
    subgraph Plugin Mesh
        P1 -- Publishes Manifest --> Gateway[MCP Gateway]
        P2 -- Publishes Manifest --> Gateway
    end

    %% Persistence Layer
    Gateway -- Syncs State --> DB[(OHC Central Database)]
    DB -- Stores Schemas --> T1[capability_plugins]
    DB -- Stores Embeddings --> T2[swarm_memory_embeddings]

    %% Agent Execution
    Agent[Autonomous Agent] -- Intent Query --> Gateway
    Gateway -- Returns Matched Tools --> Agent
    Agent -- Dynamically Executes --> P1

    %% User Interface
    UI[CEO Dashboard] -- Renders Status via Glassmorphism --> DB

    %% Styling Note
    classDef premium fill:rgba(255,255,255,0.05),stroke:rgba(255,255,255,0.1),stroke-width:1px,backdrop-filter:blur(15px);
    class UI premium;
```

## 6. Execution Playbook (Handoffs)
This architectural update triggers immediate handoff missions via the `agent_missions` table:

1.  **Backend Engineering (`backend_dev`)**: Implement the `capability_plugins` and `swarm_memory_embeddings` tables. Update the MCP Gateway to support dynamic capability discovery.
2.  **Frontend Engineering (`ui_dev`)**: Refactor the Next.js/Flutter application to exclusively utilize the defined Glassmorphism design tokens.
3.  **Design (`visualizer`)**: Produce high-fidelity mockups of the Capability Plugin Dashboard reflecting the new aesthetic constraints.
