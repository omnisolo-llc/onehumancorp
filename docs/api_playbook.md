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
  <strong>POST <code>/api/mesh/v2/broadcast</code></strong>
  <p>Advanced routing for State Machine events. Enables directed broadcast across specific CentrifugeNode channels with priority scheduling.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>{
  "topic": "state_machine.transition",
  "priority": "high",
  "payload": {
    "entity_id": "uuid-1234",
    "to_state": "EXECUTING"
  }
}</code></pre>
</div>

### 2.3 Sub-Agent Queue
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <strong>POST <code>/api/queue/subagent</code></strong>
  <p>Enqueues a background job directly to the Sub-Agent Orchestration Queue. Supports Celery/BullMQ-style priority and retry semantics.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>{
  "job_type": "vector_embedding",
  "retries": 3,
  "payload": {
    "document_id": "doc-5678"
  }
}</code></pre>

  <h4 style="margin-top: 1rem;">Queue Orchestration Flow</h4>
  <div style="background: rgba(0,0,0,0.3); padding: 1rem; border-radius: 8px; margin-top: 1rem;">
    ```mermaid
    sequenceDiagram
        participant API as OHC API
        participant DB as State Machine (PG/SQLite)
        participant Queue as Sub-Agent Queue
        participant Worker as Sub-Agent

        API->>Queue: POST /api/queue/subagent
        Queue->>DB: Record Task (PENDING)
        Worker->>Queue: Poll/Subscribe
        Worker->>DB: FOR UPDATE SKIP LOCKED
        DB-->>Worker: Lock Acquired (EXECUTING)
        Worker->>API: Complete Task
        API->>DB: Update State (COMPLETED)
    ```
  </div>
</div>

## 3. Client Integrations

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>Whether you are developing against the <strong>Local SQLite SIPDB</strong> or the <strong>Cloud Postgres/Redis</strong> stack, the REST API interface remains identical. Standalone desktop applications proxy requests seamlessly directly to the local backend runner.</p>
  <p>To verify the backend health programmatically:</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>GET /api/health</code></pre>
</div>

</div>
