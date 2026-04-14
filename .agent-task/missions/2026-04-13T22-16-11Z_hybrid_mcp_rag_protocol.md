status: DONE
agent: jules
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">

# Mission: Hybrid MCP RAG Protocol (Phases 1-3)

**Problem Statement:** Over-reliance on pure cloud dependency vs strictly siloed local states across competitors limits hybrid flexibility. We need to bridge Standalone Mode (local SQLite) with Cloud Mode (Postgres/Redis) for private RAG context delegation, defining technical specifications, and validating synchronization.

**Research Report:**
- A review of `RESEARCH_REPORT_HYBRID.md` confirms that bridging local SQLite states with cloud Postgres orchestration is a "Blue Ocean" disruption.
- Competitors like Claude Code and Replit Agent do not offer Local-to-Cloud escalation.
- SQLite supports `JSON` while Postgres uses `JSONB`.
- SQLite concurrency is handled via explicit locks (`sync.Mutex`), whereas Postgres handles row-level concurrency natively via `FOR UPDATE SKIP LOCKED`.

**Design Doc:**
- **Data Fields to Synchronize**: Mission Payload, Status, Organization ID, Task ID, and Dependencies.
- **Payload Structure**:
  ```json
  {
    "mission_id": "uuid",
    "organization_id": "uuid",
    "status": "PENDING",
    "payload": {
      "rag_context": "..."
    }
  }
  ```
- **Go Interfaces (`srcs/server/orchestration/hybrid_sync/hybrid_sync.go`)**:
  ```go
  type MissionSynchronizer interface {
      SyncLocalToCloud(ctx context.Context, mission *AgentMission) error
  }
  ```

**Implementation Prompt:**
- Implement the `MissionSynchronizer` Go interface in the existing `srcs/server/orchestration/hybrid_sync/hybrid_sync.go` file that safely copies tasks from the local SQLite `agent_missions` to the Cloud Postgres DB.
- Ensure the synchronization script scrubs PII prior to moving data to the cloud.
- Implement a unit test in the existing `srcs/server/orchestration/hybrid_sync/hybrid_sync_test.go` file demonstrating how a mock mission is safely inserted in SQLite and synchronized to Postgres.

**Priority:** P1
**Estimated Scope:** Large
</div>
