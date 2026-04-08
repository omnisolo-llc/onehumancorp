<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# Advanced Help Portal Walkthrough

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Mastering the Hybrid Agentic OS</h2>
  <p>Welcome to the Advanced Help Portal. This walkthrough covers seamless transitions between Standalone and Cloud-Native modes, as well as an in-depth look at the KAIROS Orchestration layer.</p>
</div>

## 1. Hybrid Mode Transitions

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>The One Human Corp (OHC) architecture degrades gracefully to an offline-first Standalone Mode. When reconnected, agents seamlessly sync their offline SQLite task history to the Cloud-Native Postgres cluster using the <strong>Hybrid MCP RAG Protocol</strong>.</p>
  <ul>
    <li><strong>Cloud-Native Mode:</strong> Utilizes <code>FOR UPDATE SKIP LOCKED</code> on Postgres for distributed horizontal scaling.</li>
    <li><strong>Standalone Mode:</strong> Relies on local mutexes and SQLite to preserve complete privacy without cloud exfiltration.</li>
  </ul>
</div>

## 2. KAIROS AutoDream Orchestration

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>The AutoDream Pipeline consolidates ephemeral agent task memory into persistent, searchable vector stores.</p>

<div markdown="1" style="background: rgba(0,0,0,0.3); padding: 1rem; border-radius: 8px; margin-top: 1rem;">
```mermaid
graph TD
    Agent[Agent Shared Memory] -->|Writes to .agent-task/memory| FS[File System]
    FS -->|Watched by| AutoDream[AutoDream Pipeline Worker]
    AutoDream --> Chunk[Chunk & Tokenize]
    Chunk --> Embed[Minimax/Cohere Embedding API]
    Embed --> VectorDB[(pgvector / Local SQLite)]
    VectorDB -->|RAG Sync| API[KAIROS Orchestration API]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Agent,FS,AutoDream,Chunk,Embed,VectorDB,API premium;
```
</div>
</div>

## 3. Teammate Mesh Communications

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>To support massive scale, the OS replaces bare WebSockets with a resilient <code>CentrifugeNode</code> layer backed by Redis Pub/Sub (<code>rueidis</code>). This guarantees high availability across worker nodes.</p>
  <p>For more details on connecting via the API, check out the <a href="../api/playbook.md">API Playbook</a>.</p>
</div>

</div>