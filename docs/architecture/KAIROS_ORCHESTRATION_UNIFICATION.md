# KAIROS: Hybrid Agentic OS Orchestration Unification

**Mission:** To finalize the architectural unification of One Human Corp's Agentic OS. KAIROS consolidates fragmented task management, enforces strict OHC-SIP mesh compliance, and bridges ephemeral execution with durable semantic memory.

---

## 1. The Brain: Unified Shared Task List
KAIROS v6 mandates the `shared_tasks_decomposition` table as the single source of truth for all swarm activities.

### 1.1 Master Schema Consolidation
All legacy tables (`shared_tasks`, `swarm_tasks`, `kairos_shared_tasks`) are to be migrated into the unified schema:
- **Table:** `shared_tasks_decomposition`
- **Primary Key:** `id` (UUID/TEXT)
- **Multi-Tenancy:** `organization_id` (Mandatory)
- **States:** `PENDING`, `ASSIGNED`, `EXECUTING`, `REVIEW`, `SUCCESS`, `FAILED`.
- **Concurrency:** PostgreSQL utilizes `FOR UPDATE SKIP LOCKED`. SQLite utilizes transaction-level locking.

### 1.2 State Machine & Audit Logs
Every transition MUST be recorded in `state_machine_transitions`.
- **Traceability:** Map every state change to an `agent_id` and a `reason`.
- **Hybrid Sync:** In Standalone mode, transitions are recorded locally and escalated to the Cloud via the `SyncEscalator`.

---

## 2. The Nerves: OHC-SIP Compliant Teammate Mesh
The Teammate Mesh is the real-time nervous system of the swarm. V6 enforces strict compliance with the Swarm Intelligence Protocol (SIP).

### 2.1 Mesh Gateway Enforcement
The `MeshAPI` (`/api/mesh/broadcast`) must validate all incoming payloads against the OHC-SIP schema:
```json
{
  "agent_id": "spiffe://ohc.internal/org/{org}/agent/{id}",
  "channel": "mesh:tasks | mesh:coordination | mesh:presence",
  "event_type": "TASK_CREATED | TASK_TRANSITION | AGENT_HEARTBEAT",
  "data": { ... }
}
```
If any field is missing or malformed, the gateway MUST return `400 Bad Request`.

---

## 3. The Memory: AutoDream Pipeline
Memory is no longer ephemeral. AutoDream V6 ensures that every task completion triggers a semantic consolidation event.

### 3.1 Consolidation Loop
1. **Completion:** Task reaches `SUCCESS` state.
2. **Extraction:** AutoDreamWorker extracts logs and checkpoints from `swarm_checkpoints`.
3. **Vectorization:** LLM generates a summary and embeddings.
4. **Persistence:** Upsert into `autodream_memories_master`.

---

## 4. Aesthetic Mandate: The Premium Feel
Every UI component interacting with KAIROS must adhere to the OHC Visual Excellence Mandate.

### 4.1 Glassmorphism Standard
```css
.premium-card {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
}
```

### 4.2 Typography
- **Headings:** `Outfit`, sans-serif.
- **Body:** `Inter`, sans-serif.

---

## 5. Implementation Roadmap (UltraPlan)
The following tasks have been decomposed into GitHub issues for the Swarm:

1. **Migration (P0):** [backend] Unify Task State Machine to Master Decomposition Schema (Issue #5894)
   - *Objective:* Consolidate `shared_tasks` and `swarm_tasks` into `shared_tasks_decomposition`.
2. **Middleware (P1):** [backend] Enforce OHC-SIP Compliance in Teammate Mesh Gateway (Issue #5895)
   - *Objective:* Validate `/api/mesh/broadcast` payloads for strict SIP fields.
3. **Audit (P1):** [backend] Integrate Sub-Agent Job Lifecycle into State Machine Audit Logs (Issue #5896)
   - *Objective:* Bridge `QueueManager` events to the `StateMachine` transitions.
4. **Dashboard (P2):** [frontend] Implement Glassmorphism-First Swarm Dashboard (Issue #5897)
   - *Objective:* Build the premium KAIROS command center with 20px blur and Outfit typography.
