<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# Teammate Mesh API Playbook

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Welcome to the Teammate Mesh</h2>
  <p>The OHC Hybrid Agentic OS uses the Teammate Mesh for real-time inter-agent communication, backing WebSockets/SSE with Redis Pub/Sub in Cloud-Native mode or a local event bus in Standalone Desktop mode.</p>
</div>

## Endpoints

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <h3>1. Broadcast Event</h3>
  <p>Broadcasts an event to a specific channel within the mesh.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>POST /api/mesh/broadcast
Authorization: Bearer &lt;YOUR_JWT_TOKEN&gt;

{
  "channel": "mesh:tasks",
  "event_type": "TASK_CLAIMED",
  "agent_id": "Implementer-1",
  "payload": {
    "task_id": "123e4567-e89b-12d3-a456-426614174000",
    "timestamp": "2026-04-05T22:45:00Z"
  }
}
</code></pre>
</div>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <h3>2. Stream Events</h3>
  <p>Connects to the SSE stream to receive events for subscribed channels.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>GET /api/mesh/stream?channels=mesh:tasks,mesh:presence
Authorization: Bearer &lt;YOUR_JWT_TOKEN&gt;
</code></pre>
  <p><strong>Response Stream:</strong></p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>data: {"channel": "mesh:tasks", "event_type": "TASK_CLAIMED", "agent_id": "Implementer-1", "payload": {...}}

data: {"channel": "mesh:presence", "event_type": "HEARTBEAT", "agent_id": "Scribe-1", "payload": {...}}
</code></pre>
</div>

</div>
