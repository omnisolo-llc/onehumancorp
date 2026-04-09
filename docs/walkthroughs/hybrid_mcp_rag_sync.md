<div markdown="1" style="backdrop-filter: blur(20px); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 16px; padding: 24px; font-family: 'Outfit', 'Inter', sans-serif; color: #E0E0E0;">
<h1 style="font-weight: 600; color: #FFFFFF; margin-bottom: 16px;">Hybrid MCP RAG Protocol: Bridging Standalone SQLite to Cloud PostgreSQL</h1>

<p style="line-height: 1.6; color: #B0B0B0;">
The Hybrid Architecture (OHC-HA) seamlessly integrates offline, private local execution with robust, scalable cloud infrastructure. The Hybrid MCP RAG Protocol synchronizes the local SQLite RAG state to the cloud Postgres orchestration engine via OHC-SIP, allowing private execution locally with cloud escalation.
</p>

<h2 style="font-weight: 500; color: #FFFFFF; margin-top: 24px; margin-bottom: 12px;">Architecture Overview</h2>

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

<h2 style="font-weight: 500; color: #FFFFFF; margin-top: 24px; margin-bottom: 12px;">Data Flow</h2>
<ul style="line-height: 1.6; color: #B0B0B0;">
    <li>Standalone Agent extracts insights and stores them in local SQLite.</li>
    <li>Sync Daemon periodically queries for pending rows and batches them.</li>
    <li>Payload is encrypted and sent via mutually authenticated TLS (SPIFFE/SPIRE).</li>
    <li>Cloud Gateway validates and upserts into the multi-tenant Postgres DB.</li>
    <li>Gateway responds with success, and local Sync Daemon marks rows as synced.</li>
</ul>
</div>
