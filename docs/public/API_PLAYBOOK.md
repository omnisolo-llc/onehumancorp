<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# OHC Hybrid API Playbook

**Version:** 1.0.0
**Target Audience:** Orchestration Engineers & Human CEOs

## 1. Introduction
The One Human Corp (OHC) API is the central nervous system of the Agentic OS. It bridges the gap between Cloud-Native Kubernetes clusters and Standalone Desktop deployments via the **Swarm Intelligence Protocol (OHC-SIP)**.

## 2. Authentication (Zero Secrets)
All endpoints require a valid SPIFFE/SPIRE identity token or an OIDC JWT. We do not use static API keys.

**Example Request:**
```bash
curl -X GET https://api.ohc.local/v1/agents/status \
  -H "Authorization: Bearer <JWT_OR_SVID>" \
  -H "X-OHC-Tenant-ID: org_acme_123"
```

## 3. Core Endpoints

### 3.1 Task Delegation
Delegate a subtask to an autonomous agent. The Hub handles provisioning and VRAM quota enforcement.

**Endpoint:** `POST /api/v1/tasks/delegate`

**Payload:**
```json
{
  "target_role": "swe",
  "instruction": "Implement the new billing module according to docs/features/billing.",
  "parent_thread_id": "thread_8f92a"
}
```

**Response (200 OK):**
```json
{
  "task_id": "task_99b1x",
  "status": "PROVISIONING",
  "assigned_agent": "agent_swe_004"
}
```

### 3.2 Dynamic Scaling
Adjust the number of concurrent agents for a specific role in real-time.

**Endpoint:** `POST /api/v1/scale`

**Payload:**
```json
{
  "role": "sales_rep",
  "count": 5
}
```

### 3.3 Hybrid RAG Sync
Synchronize local SQLite context to the cloud Postgres orchestration engine.

**Endpoint:** `POST /api/missions/sync`
**Headers:**
- `X-OHC-Conflict-Resolution: force-local`

**Payload:**
```json
{
  "missions": [
    {
      "id": "mission_local_01",
      "status": "COMPLETED",
      "context": "..."
    }
  ]
}
```

## 4. Webhooks & SSE
Real-time state changes are pushed via Server-Sent Events (SSE).

**Endpoint:** `GET /api/v1/stream`

Events emitted:
- `AgentHired`
- `AgentFired`
- `TaskCompleted`
- `QuotaExhausted`

## 5. Visualizing the Flow
```mermaid
graph TD
    Client[Human CEO / External Tools] --> API[OHC Gateway]
    API --> Auth{SPIFFE / OIDC}
    Auth -->|Valid| Hub[Orchestration Hub]
    Auth -->|Invalid| 401[401 Unauthorized]
    Hub --> K8s[K8s Operator]
    Hub --> Agents[Swarm Intelligence]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Client,API,Auth,Hub,401,K8s,Agents premium;
```

</div>
