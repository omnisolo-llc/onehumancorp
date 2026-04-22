<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# OneHumanCorp (OHC) Interactive API Playbook

Welcome to the definitive guide for integrating with the OneHumanCorp (OHC) "Hybrid Agentic OS" backend.

## 1. Authentication & AuthZ

OHC enforces strict Zero-Trust authentication utilizing SPIFFE/SPIRE for internal communications and OIDC for external client integrations.

*   **External API Calls**: Standard `Authorization: Bearer <token>` header containing a valid OIDC JWT.
*   **Internal Microservices/Agents**: Must wrap endpoints with `auth.RequireRole("system", ...)` and provide valid SPIFFE credentials.
*   **Tenant Isolation**: For any cloud-mode request, tenant isolation is enforced automatically via Row Level Security (RLS) in PostgreSQL matching the `tenant_id` claim in the authentication context.

## 2. Core Endpoints

### 2.1 Orchestration & Tasks

*   **Claim Task**: `POST /api/v1/tasks/claim`
    *   **Description**: Claims a pending task for processing.
    *   **Payload Example**: `{"agent_id": "agent_swe_007", "role": "swe"}`
*   **Complete Task**: `POST /api/v1/tasks/{task_id}/complete`
    *   **Description**: Marks a previously claimed task as completed.
    *   **Payload Example**: `{"agent_id": "agent_swe_007", "outcome_summary": "Task complete."}`

### 2.2 Teammate Mesh

*   **Broadcast Event**: `POST /api/mesh/broadcast`
    *   **Description**: Broadcasts a realtime payload across the KAIROS Orchestrator.
    *   **Payload Schema**: Root-level `agent_id`, `action`, `status` (as per `MeshMessage`), and `data` (JSON).

### 2.3 AutoDream Agent Memory

*   **Sync Memory**: `POST /api/v1/autodream/sync`
    *   **Description**: Triggers sync of agent session logs into the pgvector consolidated memory database.
*   **Query Memory**: `POST /api/v1/autodream/query`
    *   **Description**: Query the consolidated memory using vector search.

## 3. Standalone vs. Cloud Routing

OHC operates seamlessly across two distinct runtime environments, dynamically handled via the Hybrid File System MCP Provider and Hybrid RAG syncing.

*   **Cloud Mode**: The default for K8s deployments. Scalable PostgreSQL with pgvector, Redis for distributed locking (Redlock), and GCS for blobs.
*   **Standalone Mode**: Engaged when `OHC_STANDALONE=true` is present in the environment. Falls back gracefully to SQLite + FTS5, internal polling queues, and MinIO/local filesystem.

*API routing automatically accounts for mode degradation, maintaining consistent client-facing contracts.*

## 4. Code Snippets & Testing Instructions

### Code Snippets

**Go: Publishing to Teammate Mesh**
```go
reqBody, _ := json.Marshal(map[string]interface{}{
    "agent_id": "demo_agent_1",
    "action": "status_update",
    "status": "PROCESSING",
    "data": {"progress": 50},
})
http.Post("http://127.0.0.1:8080/api/mesh/broadcast", "application/json", bytes.NewBuffer(reqBody))
```

### Testing Instructions

1.  **Start the Local Environment**:
    Run `export PATH=$PATH:$HOME/go/bin && bazelisk run //srcs/app:start > /tmp/flutter_web.log 2>&1 &`
2.  **Verify Backend Health**:
    Run `curl http://127.0.0.1:8080/api/health` to confirm the environment is responsive.

</div>
