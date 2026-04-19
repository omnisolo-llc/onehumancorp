<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OHC Interactive API Playbook

**Version:** 1.1.0
**Target Audience:** Orchestration Engineers, Internal Integrators & Human CEOs

## 1. Introduction
The One Human Corp (OHC) API Playbook provides an interactive reference for the core components of the Hybrid Agentic OS. It outlines key REST endpoints, integration strategies, and the Hybrid API architecture.

## 2. Authentication & AuthZ

The system enforces a **Zero Secrets** policy, relying entirely on SPIFFE/SPIRE for identity and authentication across all deployments.

For local development and testing, an ephemeral token can be used.

**Headers:**
```http
Authorization: Bearer <SPIFFE_TOKEN>
X-OHC-Dev-Token: <OHC_DEV_TOKEN>  # (Optional, local development only)
```

## 3. Core Endpoints

### 3.1 KAIROS Sub-Agent Queue Orchestration

**Endpoint:** `POST /api/queue/subagent`
Enqueues a sub-agent task into the distributed queue.

**Payload:**
```json
{
  "parent_task_id": "task_12345",
  "payload": {
    "instruction": "Verify the styling tokens in the frontend."
  },
  "scheduled_at": "2026-04-06T12:00:00Z"
}
```

**Response (202 Accepted):**
```json
{
  "queue_id": "queue_9876",
  "status": "ENQUEUED"
}
```

### 3.2 Teammate Mesh v2 (Centrifuge)

**Endpoint:** `POST /api/mesh/v2/broadcast`
Broadcasts a validated state machine event over structured Centrifuge channels.

**Payload:**
```json
{
  "channel": "mesh:tasks",
  "event_type": "TASK_TRANSITION",
  "data": {
    "task_id": "task_12345",
    "previous_state": "PENDING",
    "new_state": "IN_PROGRESS"
  }
}
```

### 3.3 Agents List

**Endpoint:** `GET /api/agents`
Returns a list of all configured agents within the OHC swarm.

## 4. Standalone vs. Cloud Routing

The OHC API routes dynamically based on the active OHC Hybrid Architecture mode.

### Cloud-Native Mode
- Queue requests are routed to Rueidis ZSETs backed by K8s pods.
- Sub-agent coordination uses Redis Pub/Sub channels.

### Standalone Desktop Mode
- Queue requests fall back to an application-level mutexed SQLite instance.
- Sub-agent coordination happens in-memory via direct event passing (graceful degradation).

### Thin Client Mode
- UI forwards all API requests securely to configured cloud API endpoints via OAuth.

## 5. Code Snippets & Testing Instructions

**Testing with cURL (Local Development):**
```bash
# Get list of agents
curl -X GET "http://localhost:8080/api/agents" \
  -H "X-OHC-Dev-Token: <your_dev_token>"

# Broadcast an event
curl -X POST "http://localhost:8080/api/mesh/v2/broadcast" \
  -H "Content-Type: application/json" \
  -H "X-OHC-Dev-Token: <your_dev_token>" \
  -d '{
    "channel": "mesh:test",
    "event_type": "PING",
    "data": {}
  }'
```

**Interactive Swagger Docs:**
For real-time testing, navigate to `/api/docs` in your local setup, which exposes the Swagger/OpenAPI portal and integrates with WebSockets.

</div>
