<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid CRDT State Synchronization MCP: Visual Walkthrough

This guide details the architectural flow of the Hybrid CRDT State Synchronization Model Context Protocol (MCP), which enables offline-capable state convergence between Standalone SQLite and Cloud PostgreSQL.

## 1. Data Flow

The Hybrid Architecture uses CRDTs (Conflict-free Replicated Data Types) to resolve state changes when agents reconnect to the Cloud environment.

```mermaid
graph TD
    A[Standalone Mode] -->|Local Edits| B(SQLite DB)
    B -.->|crdt_push via MCP| C{Cloud MCP Gateway}
    C -->|crdt_merge| D(PostgreSQL DB)
    D -->|crdt_pull| E[Cloud Swarm Orchestration]

    style A fill:#003366,stroke:#333,stroke-width:2px,color:#fff
    style B fill:#006699,stroke:#333,stroke-width:2px,color:#fff
    style D fill:#0099cc,stroke:#333,stroke-width:2px,color:#fff
    style E fill:#00ccff,stroke:#333,stroke-width:2px,color:#111
```

## 2. Conflict Resolution Mechanism

When the `crdt_push` tool is invoked, the payload is validated against the multi-tenant boundary. If `OHC_MULTITENANT=true`, it checks the `organization_id` in the context to prevent cross-tenant mutations.

</div>
