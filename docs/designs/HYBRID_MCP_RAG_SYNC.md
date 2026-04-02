<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.08); font-family: 'Outfit', 'Inter', sans-serif; color: #ffffff; border-radius: 12px; padding: 24px; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# 🌐 OHC-HA Design Doc: Hybrid MCP RAG Protocol & State Sync

**Author**: Principal Product Architect & KAIROS Orchestrator
**Date**: 2026-04-02
**Status**: Finalized

## 1. Executive Summary

Competitors such as Claude Code, OpenClaw, and Replit Agent force users into a binary choice: trade privacy for cloud-scale execution, or trade scalability for local privacy. The **Hybrid Architecture (OHC-HA)** uniquely positions OHC to offer both.

This design document formalizes the architecture for the **Offline-to-Cloud State Sync for Swarm Memories**, establishing a unified **"Hybrid MCP RAG Protocol"**. By bridging local SQLite standalone capabilities with multi-tenant PostgreSQL orchestration, OHC enables secure private execution with massive parallel cloud escalation.

## 2. The Architectural Disruption

### 2.1 Private RAG with Cloud-Scale Routing
- **Private Execution**: Highly sensitive datasets are processed natively on the standalone Desktop mode via SQLite, without network transmission.
- **Cloud Escalation**: When compute-intensive tasks are requested, the local MCP agent flags the context. A local daemon strips PII and syncs the sanitized embeddings and mission requirements to the K8s multi-tenant cloud (`agent_missions`).

### 2.2 Component Orchestration Flow
```mermaid
graph TD
    A[Standalone Desktop (SQLite)] -->|Private RAG & Local Execution| B(Local MCP Agent)
    B -->|Task Requires Scaled Compute| C{OHC-SIP Cloud Sync Daemon}
    C -->|Sanitized Payload Injection| D[(Cloud Postgres: agent_missions)]
    D -->|K8s Pod Orchestration| E[Multi-Tenant Cloud Swarm]
    E -->|Computed Results| C
    C -->|Sync Back| A

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,E premium;
    class C,D premium;
```

## 3. Database Schema & State Tracking

To implement the Teammate Mesh dependencies and State Machine tracking for this protocol, we will introduce a distributed lock mechanism and the following schema changes:

**`swarm_truth_embeddings` & `agent_missions` Additions:**
- `sync_status` `VARCHAR`: Enum denoting the state (`LOCAL_ONLY`, `PENDING_SYNC`, `SYNCING`, `SYNCED`).
- `cloud_escalation_flag` `BOOLEAN`: User-approved boundary indicating whether it's safe to upload sanitized data.

**State Machine Tracking**:
- A distributed lock (using `rueidis` in Cloud mode) will ensure only one cluster node processes incoming sync mutations.
- In Local Standalone mode, standard SQLite transactions with optimistic locking will coordinate the sync daemon.

## 4. Teammate Mesh Integration

The synchronized data powers the **Teammate Mesh** across boundaries. The `Sync Daemon` communicates to the Hub over WebSockets/gRPC.

### API Contract (Draft):
```json
// POST /api/v1/sync/rag-state
{
  "mission_id": "uuid",
  "sanitized_context": "Refactored UI to use Glassmorphism CSS",
  "embedding_vector": "[0.123, 0.456, ...]",
  "priority": "P1"
}
```

## 5. Visual Excellence & UX

All frontend presentation components representing the synchronization state will strictly utilize the OHC Premium Aesthetic:
- `backdrop-filter: blur(20px) saturate(1.213);`
- Smooth 12px border radii with subtle `rgba(255, 255, 255, 0.08)` borders.

## 6. Next Steps
The mission file for the Implementer agent has been created in the Swarm Queue. The Implementer will pick up the task to execute the schema migrations and sync daemon implementation.

</div>
