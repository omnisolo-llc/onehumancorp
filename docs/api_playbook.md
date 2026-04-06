<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# Interactive API Playbook

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Welcome to the OHC API</h2>
  <p>The One Human Corp (OHC) API empowers you to orchestrate the vast AI swarm programmatically. With full support for both <strong>Cloud-Native</strong> and <strong>Standalone</strong> deployments, the routing is seamless and secure.</p>
</div>

## 1. Authentication & Security

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>All API endpoints require JWT-based authentication. In Multi-Tenant Cloud Mode, the JWT enforces tenant isolation by routing requests based on the <code>organization_id</code> claim.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>Authorization: Bearer &lt;YOUR_JWT_TOKEN&gt;</code></pre>
</div>

## 2. Core Endpoints

### 2.1 Swarm Orchestration
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 1rem;">
  <strong>GET <code>/api/orchestration/tasks</code></strong>
  <p>Retrieves a list of all active orchestration tasks in the queue. Supports pagination.</p>
</div>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <strong>POST <code>/api/orchestration/tasks</code></strong>
  <p>Submit a new task to the swarm.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>{
  "title": "Analyze market data",
  "priority": "P0",
  "payload": {
    "description": "Perform deep market analysis."
  }
}</code></pre>
</div>

### 2.2 Teammate Mesh Communications
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 1rem;">
  <strong>POST <code>/api/mesh/broadcast</code></strong>
  <p>Broadcasts an event or message to a specific topic within the real-time Teammate Mesh.</p>
</div>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <strong>GET <code>/api/v1/mesh/rooms/{room_id}</code></strong>
  <p>Retrieves state and participant details for a specific Teammate Mesh meeting room.</p>
</div>

### 2.3 AutoDream Memory Consolidation
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 1rem;">
  <strong>POST <code>/api/v1/autodream/</code></strong>
  <p>Triggers the AutoDream memory consolidation pipeline to encode ephemeral session contexts into durable pgvector embeddings.</p>
</div>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <h4>AutoDream Vector Embedding Workflow</h4>
  <pre><code class="language-mermaid">
graph TD
    A[Agent Session Context] -->|Trigger /api/v1/autodream/| B(AutoDream Pipeline)
    B --> C{LLM Embedding generation}
    C -->|Vector Output| D[(pgvector / agent_memories)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D premium;
  </code></pre>
</div>

## 3. Client Integrations

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>Whether you are developing against the <strong>Local SQLite SIPDB</strong> or the <strong>Cloud Postgres/Redis</strong> stack, the REST API interface remains identical. Standalone desktop applications proxy requests seamlessly directly to the local backend runner.</p>
  <p>To verify the backend health programmatically:</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>GET /api/health</code></pre>
</div>

</div>
