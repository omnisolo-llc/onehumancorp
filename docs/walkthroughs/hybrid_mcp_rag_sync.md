<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid MCP RAG Protocol Walkthrough

Welcome to the walkthrough for the **Hybrid MCP RAG Protocol**. This mechanism seamlessly synchronizes Standalone SQLite offline memory states with the massive multi-tenant Cloud PostgreSQL orchestration database, ensuring persistent Swarm Intelligence.

## 1. Architectural Overview

```mermaid
graph TD
    A[Standalone Mode] -->|Private Local State| B(SQLite DB)
    B -.->|Background Sync via OHC-SIP| C{Sync Engine}
    C -->|Aggregated Insights| D(PostgreSQL DB)
    D -->|Global Context| E[Cloud Swarm Orchestration]

    style A fill:#003366,stroke:#333,stroke-width:2px,color:#fff
    style B fill:#006699,stroke:#333,stroke-width:2px,color:#fff
    style D fill:#0099cc,stroke:#333,stroke-width:2px,color:#fff
    style E fill:#00ccff,stroke:#333,stroke-width:2px,color:#111
```

## 2. Sync Lifecycle

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <strong>Data Extraction & Queueing</strong>
  <p>Local Standalone Agents extract insights and record them into the local SQLite `rag_memories` table with `sync_status = 'pending'`. The Lightweight Go Daemon periodically checks this queue.</p>
</div>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <strong>Secure Cloud Escrow</strong>
  <p>The payload is TLS-encrypted and securely shipped to the Cloud API Gateway. Once the Postgres layer processes the vectors using `pgvector`, the Standalone client receives a confirmation, updating the record to `sync_status = 'synced'`.</p>
</div>

</div>
