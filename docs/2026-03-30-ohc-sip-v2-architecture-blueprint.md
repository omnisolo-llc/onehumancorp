# Design Doc: OHC-SIP v2 Architecture & Aesthetic Evolution
**Author:** Principal Product Architect & Visionary (L7)
**Date:** 2026-03-30

## Objective
To build the world's most autonomous, aesthetically superior, and market-aware Agentic Operating System. One Human Corp (OHC) aims to empower a single human to orchestrate a vast swarm of AI agents.

## Core Values & Architectural Principles
- **Absolute Autonomy**: Agents propose and execute based on Vision and Market Reality.
- **Continuous Evolution**: Incorporating latest advancements from Claude Code, OpenClaw, Windsurf.
- **K8s Native & Bazel-First** engineering.
- **Database-Driven Orchestration**: \`agent_missions\`, \`swarm_memory\`, \`agent_status\`.

## Market-Leading Features (OHC-SIP v2)
1. **MCP Integration**: Dynamic tool registration and standardized tool use via MCP Gateways.
2. **Sub-Agent Isolation**: K8s container isolation boundaries per-agent logic.
3. **Hierarchical Memory**: Multi-layered state representations from `swarm_memory` to `swarm_memory_embeddings`.

## Aesthetic Excellence Mandate
Every interface and artifact must feel "Premium".

### OHC Design System Tokens
- **Backdrop Filter**: \`backdrop-filter: blur(20px) saturate(200%)\`
- **Background**: \`background: rgba(255, 255, 255, 0.03)\`
- **Border**: \`border: 1px solid rgba(255, 255, 255, 0.08)\`
- **Typography**: \`Outfit\`, \`Inter\`, sans-serif

### Premium Architecture Flow
```mermaid
graph TD
    classDef premium fill:rgba(255, 255, 255, 0.03),stroke:rgba(255, 255, 255, 0.08),stroke-width:1px,color:#fff,font-family:Outfit;

    A[Human Orchestrator] -->|Vision| B(OHC Central DB)
    B -->|OHC-SIP v2| C{Agent Swarm}

    C -->|Sub-Agent Isolation| D[Frontend Dev]
    C -->|Sub-Agent Isolation| E[Backend Dev]
    C -->|Sub-Agent Isolation| F[SRE Engineer]

    E -->|MCP Tooling| G(MCP Gateway)

    class A,B,C,D,E,F,G premium;
```

## Security Posture
- **Zero Secrets**: Rely entirely on SPIFFE/SPIRE for identity and auth.
- **Zero Traces**: Complete cleanup of all temporary tools and artifacts.

## New Agentic Protocol Schema (OHC-SIP v2)

### 1. `swarm_memory_embeddings` (Hierarchical Memory Extension)
```sql
CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
    memory_id TEXT PRIMARY KEY,
    context TEXT,
    vector_embedding BLOB,
    source_plugin TEXT,
    created_at DATETIME
);
```

### 2. `capability_plugins` (MCP Tool Discovery)
```sql
CREATE TABLE IF NOT EXISTS capability_plugins (
    plugin_id TEXT PRIMARY KEY,
    name TEXT,
    version TEXT,
    manifest_url TEXT,
    status TEXT,
    registered_at DATETIME
);
```
