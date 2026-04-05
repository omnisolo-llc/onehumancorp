---
status: DONE
agent: Implementer
---

# Title: KAIROS Orchestration: Implement UltraPlan Deliberation State Machine

## Problem Statement
The OHC Hybrid Architecture orchestrates vast swarms of AI agents. For complex architectural changes (like Auth Overhauls or Database Migrations), a simple single-agent plan is insufficient. We need an "UltraPlan Deliberation" engine that allows the Swarm to break down massive tasks, propose execution paths, critique them (Peer Review), and vote on the best architectural approach before executing. Currently, KAIROS lacks this advanced distributed state machine capability.

## Research Report
1. **UltraPlan Definition**: A multi-phase state machine spanning `PROPOSE -> CRITIQUE -> REVISE -> APPROVED -> EXECUTE`.
2. **Data Structure**: `shared_tasks` and `swarm_tasks` currently only support `PENDING`, `IN_PROGRESS`, `COMPLETED`, `FAILED`. We must introduce an `UltraPlan` abstraction (possibly using `swarm_tasks` with a specific payload schema) to manage multi-agent voting and deliberation cycles.
3. **Storage & Locks**: In Cloud Mode, deliberation votes and status changes must be strictly serialized using PostgreSQL `FOR UPDATE SKIP LOCKED` and transaction blocks to avoid race conditions when multiple agents critique simultaneously. In Standalone Mode, SQLite native table locks will enforce serialization via `pool.IsSQLite()`.
4. **Teammate Mesh**: Deliberation states (`CRITIQUE_SUBMITTED`, `PLAN_APPROVED`) must be broadcasted in real-time over the `mesh:coordination` Redis Pub/Sub channels (or local channels).

## Design Doc
1. **State Machine Additions**:
   Update task status enumerations or payload state machines to support `PROPOSAL_PENDING`, `DELIBERATION`, `REVISION_REQUIRED`, `APPROVED`.
2. **Deliberation Models**:
   Add a new `UltraPlan` struct in Go that links to a parent `swarm_task`:
   ```go
   type UltraPlan struct {
       TaskID       string
       Phase        string // e.g., DELIBERATION
       Critiques    []Critique
       Approvals    int
       TargetVotes  int
   }
   ```
3. **Database Handlers**:
   Create `srcs/server/orchestration/ultraplan.go` with functions like `SubmitCritique(ctx, taskID, agentID, critique)` and `ApprovePlan(ctx, taskID, agentID)`. Use explicit DB transactions and concurrency checks to prevent concurrent voting anomalies.
4. **Visual Excellence**:
   Deliberation history must be presented in the CEO Dashboard utilizing the OHC Premium Feel: `backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter', sans-serif`.

## Implementation Prompt
Hello Implementer agent! Please execute the UltraPlan Deliberation State Machine:
1. Review `srcs/server/orchestration/tasks.go` and add support for the `UltraPlan` state machine. Create `srcs/server/orchestration/ultraplan.go`.
2. Ensure you use `FOR UPDATE SKIP LOCKED` in PostgreSQL mode when tallying votes or adding critiques to prevent race conditions.
3. Use `to.db.IsSQLite()` to apply appropriate lock serialization for Standalone Desktop Mode.
4. Broadcast state transitions (`VOTE_CAST`, `PHASE_CHANGED`) via `Teammate Mesh` channels so the UI updates instantly.
5. Create a new DB migration for an `ultraplan_votes` table or simply store deliberation state safely in the `swarm_tasks.payload` JSONB column. If creating a migration, remember to update `srcs/server/db/BUILD.bazel`.
6. Write rigorous unit tests in `srcs/server/orchestration/ultraplan_test.go` (achieve >90% coverage) demonstrating that concurrent agent votes do not result in TOCTOU issues.
7. Verify all work with `bazelisk test //srcs/server/... --test_output=errors`.

## Priority
P0

## Estimated Scope
Large
