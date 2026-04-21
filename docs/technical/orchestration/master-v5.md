<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS: The Hybrid Agentic OS Master Architecture (V5)

**Mission:** To provide a robust, autonomous, and aesthetically superior backbone for the One Human Corp Swarm. KAIROS orchestrates complex human goals into parallel, self-healing agent workflows across Cloud-Native (Postgres/Redis) and Standalone (SQLite) environments.

---

## 1. The KAIROS Triad

KAIROS rests on three immutable pillars that ensure "Absolute Autonomy":

### 1.1 The Brain: Unified Shared Task List
A distributed state machine that serves as the source of truth for all swarm activities. It uses the `shared_tasks_decomposition` master schema.
- **Cloud Mode**: Leverages PostgreSQL `FOR UPDATE SKIP LOCKED` for high-concurrency task claiming.
- **Standalone Mode**: Degrades to SQLite with application-level mutexes and transaction-based status transitions.
- **Schema**: Supports Parent/Child relationships (Epics/Tasks) and JSON-serialized `dependencies` and `payload` fields.

### 1.2 The Nerves: Teammate Mesh (Centrifuge & Redis)
A highly available, low-latency communication layer for realtime agent coordination.
- **Hybrid Broker**: Uses `RedisMeshBroker` (via `rueidis`) for Cloud and `LocalMeshBroker` (in-memory channels) for Standalone.
- **Channels**:
  - `mesh:tasks`: Broadcasts task state changes (CREATED, CLAIMED, COMPLETED).
  - `mesh:presence`: Heartbeats and capability advertisements (`AdvertiseCapabilities`).
- **Events**: Standardized JSON payloads for zero-friction interoperability.

### 1.3 The Memory: AutoDream Memory Consolidation
A long-term state consolidation pipeline that vectors ephemeral session logs into durable semantic memory.
- **Pipeline**: Asynchronously polls `OHC_MEMORY_DIR` and DB logs to generate embeddings via Minimax/OpenAI.
- **Persistence**: Stores in `autodream_memories` (pgvector for Cloud, local file-backed or standard SQLite for Standalone).
- **Consolidation**: "Dreams" about past sessions to create optimized, low-token context for future tasks.

---

## 2. Advanced Orchestration Logic

### 2.1 Distributed State Machine Tracking
Every entity in KAIROS (Tasks, Missions, Sub-agents) follows a strict state transition model managed by the `StateMachine` service in `srcs/server/orchestration/statemachine/`.
- **Transitions**: `PENDING` → `ASSIGNED` → `EXECUTING` → `REVIEW` → `SUCCESS/FAILED`.
- **Audit Logs**: Every transition is recorded in `state_machine_transitions` with an `agent_id` and `reason` for full-spectrum observability.
- **Consistency**: Distributed locks (Redis) or SQLite transactions prevent race conditions during state forks.

### 2.2 Sub-Agent Orchestration Queue
For tasks requiring dynamic scaling, KAIROS implements a scalable background queue (`srcs/server/orchestration/queue/`).
- **Isolation**: Each sub-agent is spawned in an isolated environment with a narrow, task-specific context.
- **Queuing**: Inspired by BullMQ/Celery, supporting `attempts`, `max_attempts`, and `backoff` logic.
- **Resource Management**: Strictly enforced VRAM and token quotas per sub-agent to prevent runaway compute costs.

---

## 3. Hybrid Resilience & Zero-Lock Scaling

KAIROS is designed to "degrade gracefully" without losing integrity.
- **Offline Sync**: When Standalone nodes reconnect to the Cloud, the `SyncEscalator` synchronizes local `shared_tasks_decomposition` and `autodream_memories` using LWW-Element-Set CRDTs.
- **Identity**: Zero-trust authorization via SPIFFE/SPIRE IDs (`spiffe://ohc.internal/org/{org}/agent/{id}`). No hardcoded secrets.

---

## 4. Visual Excellence Mandate
Any UI component interacting with KAIROS MUST adhere to the OHC Premium aesthetic:
- **Glassmorphism**: `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03);`
- **Typography**: `font-family: 'Outfit', 'Inter', sans-serif;`
- **Animation**: Fluid, 60fps transitions (e.g., `PulseAnimation`) for state changes.

---

## 5. Execution Sequence (UltraPlan)
1. **Goal Ingestion**: CEO submits a high-level mission via the Dashboard.
2. **Decomposition**: KAIROS decomposes the mission into a DAG of `shared_tasks_decomposition`.
3. **Delegation**: Agents claim tasks via `FOR UPDATE SKIP LOCKED`.
4. **Execution & Checkpointing**: Agents save periodic states to `swarm_checkpoints` for session recovery.
5. **Memory Consolidation**: AutoDream vectorizes findings.
6. **Completion**: Human-in-the-Loop approval for high-risk final results.

</div>
