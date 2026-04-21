<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid MCP RAG Protocol: Visual Walkthrough

This guide details the architectural flow of the Hybrid MCP RAG Protocol, which bridges Standalone SQLite and Cloud PostgreSQL.

## 1. Data Flow

The Hybrid Architecture uses a background daemon to synchronize states between Local SQLite and Cloud Postgres.

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

## 2. Synchronization Mechanism

The `Sync Daemon` periodically queries the local SQLite DB for rows where `sync_status = 'pending'`, batches them, and sends them via an encrypted SPIFFE/SPIRE payload to the Cloud API Gateway.

</div>
