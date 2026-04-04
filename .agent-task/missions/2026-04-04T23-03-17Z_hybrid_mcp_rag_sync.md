<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# 🔬 RESEARCHER: [Hybrid MCP RAG] Offline-to-Cloud State Sync

**Priority**: P0
**Estimated Scope**: Large

## 1. Problem Statement
The current Agentic OS market is highly bifurcated. Competitors like Claude Code enforce local execution with limited global swarm knowledge, while OpenClaw and Replit Agent enforce pure cloud execution, alienating users with air-gapped or privacy-first requirements. One Human Corp (OHC) needs a definitive "Blue Ocean" disruption: **Hybrid MCP RAG State Sync**, enabling agents to seamlessly switch between local SQLite vector queries (Standalone) and multi-tenant PostgreSQL vector processing (Cloud-Native), synchronizing sanitized payloads for global execution.

## 2. Research Report
**Competitive Analysis: The "Local-to-Cloud Gap"**

| Feature Area | Claude Code | OpenClaw | Replit Agent | **OHC (Hybrid MCP RAG)** |
| :--- | :--- | :--- | :--- | :--- |
| **Execution Domain** | Local CLI only | Cloud K8s only | Cloud IDE only | **True Hybrid** (Local Desktop + Cloud Pods) |
| **Data Sovereignty** | High (Local files) | Low (Provider Cloud) | Low (Replit Cloud) | **Adaptive** (Private local, sanitized cloud) |
| **RAG Sync** | None (Siloed) | None (Siloed) | None (Siloed) | **OHC-SIP Synchronizer** |
| **Resilience** | API Dependent | API Dependent | API Dependent | **Graceful Degradation** (Local LLM fallback) |

Competitors force a binary choice: Privacy OR Scalability. OHC's unique value proposition is the ability to securely delegate sanitized task state from a local single-user SQLite RAG instance to the K8s cloud swarm.

## 3. Design Doc
The objective is to implement an `agent_missions` synchronization protocol that bridges the Local-First SQLite SIPDB and the Cloud-Native Postgres Orchestration Engine.

**Architecture:**
- **Local Standalone Mode**: Analyzes files using local MCP integration and stores vectors in local SQLite. When a task requires cloud burst compute, it sanitizes the prompt, drops PII/local paths, and tags the mission `status = 'BURSTING'`.
- **Cloud-Native Mode**: Receives the `BURSTING` payload via a secure endpoint authenticated with SPIFFE/SPIRE JWTs. The cloud Postgres database adopts the mission.
- **Protocol**: OHC-SIP Synchronization via gRPC or HTTP REST.

```mermaid
graph TD
    A[Standalone Desktop (SQLite)] -->|Private RAG & Execution| B(Local MCP Agent)
    B -->|Task Requires Swarm Compute| C{OHC-SIP Synchronizer}
    C -->|Sanitized RAG Payload| D[(Cloud Postgres: agent_missions)]
    D -->|K8s Pod Orchestration| E[Multi-Tenant Cloud Swarm]
    E -->|Computed Results Sync| C
    C -->|Sync Back via API| A

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,E premium;
    class C,D premium;
```

## 4. Implementation Prompt
Implementer Agent, execute the following instructions verbatim to build the Hybrid RAG Sync:

1. **Database Schema**: Add a new column `sync_status` to the `agent_missions` table in both PostgreSQL and SQLite migrations (e.g., in `srcs/server/db/migrations/`). The status should support states like `LOCAL`, `BURSTING`, and `SYNCED`.
   - *Note*: Use `CREATE TABLE IF NOT EXISTS` or unconditional `ALTER TABLE ADD COLUMN` for SQLite compatibility (do not use `IF NOT EXISTS` for `ALTER TABLE`).
2. **Synchronization Client**: In `srcs/server/orchestration/`, implement `HybridMCPRAGDaemon` in Go.
   - It should periodically poll `agent_missions` where `sync_status = 'BURSTING'`.
   - Implement exponential backoff retries for transient network unavailabilities during `sendToCloud`.
3. **Cloud Endpoint**: Create an API route (e.g., `/api/orchestration/hybrid-sync`) on the OHC Cloud API. Ensure distinct path prefixes.
   - Secure it with `auth.RequireRole("system", ...)`.
4. **Data Sanitization**: Before transmitting the payload to the cloud, explicitly sanitize all local data and PII using a new `redactPII` function.
5. **Testing**: Write unit tests for both the syncing client and receiving cloud endpoint. Do not mock network requests in frontend UI tests.
6. **Observability**: Expose Prometheus metrics (e.g., `missions_synced_total`, `sync_latency_seconds`) in `srcs/server/telemetry/telemetry.go`.

Ensure your execution has >90% test coverage and respects OHC Multi-tenancy isolation (`auth.ClaimsFromContext(r.Context()).OrganizationID`).

</div>