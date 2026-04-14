---
status: FAILED
Title: "KAIROS Orchestration: Unified Shared Task List & Architecture"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
The OHC Swarm lacks a cohesive orchestration mechanism for decomposing complex features. We need a "Shared Task List" distributed state machine, supported by a Teammate Mesh and AutoDream pipeline.

# Research Report
Based on `CLAUDE_OHC.md` and `README.md`, OHC uses PostgreSQL for cloud scaling and SQLite for standalone mode. For task tracking, `FOR UPDATE SKIP LOCKED` is optimal in Postgres. Teammate mesh requires Redis (`rueidis`) and `CentrifugeNode` for web-socket pub/sub events. Memory should use pgvector for long-term consolidation.

# Design Doc
**Shared Task List Schema:**
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    assigned_agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_shared_tasks_org_status ON shared_tasks(organization_id, status);
```

**Mesh Data Structure:**
```protobuf
message MeshEvent {
  string event_id = 1;
  string topic = 2;
  bytes payload = 3;
  int64 timestamp = 4;
}
```

# Implementation Prompt
You are an Implementer agent. Your task is to implement the KAIROS Triad.
1. Create `srcs/server/db/migrations/032_kairos_unified.sql` with the `shared_tasks` table.
2. Update `srcs/proto/hub.proto` with `MeshEvent`. Run Bazel proto gen.
3. In `srcs/server/orchestration/tasks_db.go`, implement a `ClaimTask` method using Postgres locking (`FOR UPDATE SKIP LOCKED`).
4. In `srcs/server/orchestration/hub.go`, integrate `CentrifugeNode` and Redis pub/sub for the Teammate Mesh transport.
5. Create `srcs/server/orchestration/autodream_pipeline.go` with an `AutoDreamWorker` daemon.
6. Write unit tests for data layer, mesh transport, and worker daemon. Verify with `bazelisk test //srcs/server/orchestration/...`.

# Visual Excellence Guidelines
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
