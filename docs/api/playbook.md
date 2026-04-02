<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OHC API Playbook: Interactive Reference

This playbook covers the essential REST and gRPC API endpoints for interacting with the OHC Agentic Platform.

## Authentication

All endpoints are secured via SPIFFE/SPIRE zero-trust principles. Ensure your client provides a valid JWT.

### `POST /api/orgs/register`
Provisions a new organization in multi-tenant mode.
**Payload:**
```json
{
  "id": "acme",
  "name": "Acme Corp",
  "domain": "acme.com"
}
```

## Agent Management

### `GET /api/agents`
Retrieves a list of active agents within the current tenant scope.

### `POST /api/agents/hire`
Requests a new agent capability. This triggers the dynamic tool registration via MCP.

## Teammate Mesh

### Centrifuge Realtime Sync
Channels:
- `mesh:tasks`: Global task coordination.
- `mesh:ultraplan:<plan_id>`: Deliberation cycle realtime updates.
- `meeting:<meeting_id>`: Transcript sync.

</div>
