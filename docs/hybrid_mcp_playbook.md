<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# Interactive Hybrid MCP RAG Playbook

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Welcome to the Hybrid MCP RAG Protocol</h2>
  <p>The OHC Hybrid Agentic OS seamlessly bridges local SQLite execution with scalable cloud PostgreSQL orchestration through the <strong>Hybrid MCP RAG Protocol</strong>.</p>
</div>

## Core Architecture

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>The system synchronizes local SQLite RAG state to the cloud Postgres orchestration engine via OHC-SIP.</p>
  <div style="background: rgba(0,0,0,0.3); padding: 1rem; border-radius: 8px; margin-top: 1rem;">
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
  </div>
</div>

## Sync Mechanics

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>Standalone clients use an internal sync daemon to synchronize local RAG contexts with the Cloud Gateway over mutually authenticated TLS.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>POST /api/mcp/rag/sync
Authorization: Bearer &lt;YOUR_JWT_TOKEN&gt;

{
  "records": [
    { "id": "123", "context": "user preference", "status": "pending" }
  ]
}
</code></pre>
</div>

</div>
