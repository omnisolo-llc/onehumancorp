<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Realtime Teammate Mesh APIs
The API endpoints for KAIROS coordination:
- `POST /api/mesh/broadcast`: Broadcast an event to a mesh channel.
- Events must be JSON compliant with `agent_id`, `action`, `status`.

## Mesh Event Payload Contract
All Teammate Mesh payloads must be JSON and structured to OHC-SIP compliance:
```json
{
    "agent_id": "sub_agent_xyz123",
    "action": "TaskCompleted",
    "status": "success",
    "payload": { "task_id": "task_abc", "result": "..." }
}
```

</div>
