# Swarm Operations Analysis Report: Hybrid Telemetry Review & Observability Gap Analysis

## 1. Hybrid Telemetry Review

### Cloud-native vs Standalone Discrepancies
Based on reviewing the `src/server/telemetry` module and the database access patterns, a core disparity exists in the execution profile of Standalone vs Cloud telemetry persistence.

**Standalone Mode Inefficiencies**:
- In Standalone SQLite-backed mode, high-frequency telemetry metrics (such as memory-bound trace serializations via `json.Marshal`) execute synchronously. This creates artificial latency on the main thread, delaying Swarm event processing.
- The single-writer db locking in SQLite introduces contention spikes when multiple SubAgents try to record metrics simultaneously (observed via `ohc_sqlite_lock_contention_total`). This blocks Swarm execution unnecessarily compared to the K8s deployed Postgres.

**Cloud-native Inefficiencies**:
- Relying entirely on Postgres `SKIP LOCKED` polling for Swarm orchestration works but comes with constant network overhead. JetStream NATS is needed to close the gap on low-latency pub-sub event delivery (see `[backend]_nats_hybrid_event_mesh.md`).

## 2. Observability Gap Analysis

Existing observability effectively tracks `AgentTokenUsageTotal`, `TaskProcessingLatency`, and database contention (`postgresLockContentionCounter` / `sqliteLockContentionCounter`). However, critical gaps exist:

- **Agent Approval Wait Time**: The newly designed "Draft-for-Review" approval workflow (`[architecture]_ai_agent_department.md`) defines high-risk actions pending human review. However, there is no histogram metric (e.g., `ohc_agent_approval_wait_time_seconds`) tracking the user friction duration between the pending state and the owner's 1-tap approval.
- **Offline Sync Duration**: Telemetry lacks measurement for exactly how long the delta queue takes to sync once an offline standalone node reconnects to the cloud.
- **NATS Hybrid Mesh Metrics**: The planned event mesh requires metrics (`ohc.nats.messages_published`, `ohc.nats.messages_received`) to effectively measure hybrid event throughput.

## 3. Bottleneck Hunting

Analysis points to three primary bottlenecks:
1. **Agent State Contention**: In Cloud Mode, multi-step agent task transitions heavily content on row locks (`shared_tasks` table). The distributed Redis locking (`ohc:lock:{tenant_id}...`) alleviates some contention, but `TaskClaimContentionTotal` indicates deadlocks and retries.
2. **Synchronous Local Telemetry**: SQLite-backed standalone mode directly persists telemetry points without buffering, punishing overall system latency.
3. **Queue Polling Overheads**: Polling for state vs. a Push-based NATS hybrid mesh causes higher idle CPU and delayed event triggers.

## 4. Swarm Health Assessment
The AI swarm operates effectively but its performance diverges widely between Cloud and Standalone environments. Multi-tenant K8s handles high-throughput concurrency reasonably well, albeit with database lock contention when tasks share the same resource state. In Standalone mode, single-writer limits (SQLite) and lack of asynchronous telemetry buffering artificially constrict performance.

## 5. Cost Efficiency Analysis
- AI Agents are fetching significant past interactions into the `autodream_memories` unified memory model. The high context token usage without semantic pruning inflates LLM costs.
- The `TokenBurnRatePredicted24h` gauge provides visibility, but a "Context Window Optimizer" strategy is recommended to aggressively prune context for simple Operations or Customer Success tasks.

## Next Steps / Proposed Issues

The following detailed issue briefs have been formulated based on this analysis:

### 1. NATS Hybrid Event Mesh Gap Analysis (P1)
**Problem**: Missing low-latency, scalable messaging backbone for cross-environment workflows.
**Design**: Implement `NatsProvider` in `src/server/integrations/`. Cloud Mode uses NATS JetStream, Standalone Mode uses an embedded NATS server acting as a leaf node.
**Prompt**: "Implement the NATS Event Mesh module in `src/server/integrations/nats/`. Ensure metrics (`ohc.nats.messages_published`, `ohc.nats.messages_received`) are instrumented."

### 2. Telemetry: Asynchronous SQLite Buffering (P2)
**Problem**: In Standalone mode, high-frequency metric persistence operations execute synchronously, blocking swarm processing.
**Design**: Extend the standalone telemetry provider to include a buffered event channel and a background sync daemon (`src/server/telemetry/sync_daemon.go` or similar) that flushes to SQLite in batches.
**Prompt**: "Implement asynchronous metric buffering in the Standalone telemetry configuration within `src/server/telemetry/`. Create a background worker that buffers high-frequency metrics using an event channel. Batch writes and flush them asynchronously to SQLite."

### 3. Observability for Agent Draft-for-Review Approvals (P1)
**Problem**: The "Draft-for-Review" approval workflow has no metric tracking how long tasks remain in a pending state awaiting human approval.
**Design**: Introduce a new `Float64Histogram` named `ohc_agent_approval_wait_time_seconds` in `src/server/telemetry`. Instrument KAIROS orchestration to record the delta.
**Prompt**: "Add a new `Float64Histogram` metric named `ohc_agent_approval_wait_time_seconds` in `src/server/telemetry/` to monitor the 'Draft-for-Review' workflow. Instrument the task state transitions in `src/server/orchestration/`."
