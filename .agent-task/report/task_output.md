issue_title: "Implement Shared Task List Decomposition for KAIROS in TaskDbService"
issue_description: |
  # [Architect] Implement Shared Task List Decomposition for KAIROS

  ## Problem Statement
  The swarm needs a central, distributed tracking system to coordinate efforts and avoid duplicate work when acting upon high-level feature requests. Currently, agents operate in silos and lack a unified "Brain" to decompose UltraPlans into actionable nodes. KAIROS orchestrates the agent team by decomposing high-level feature requests into actionable tasks within a distributed Shared Task List, preventing race conditions during task claiming.

  While `TaskDecompositionService` in `src/server/orchestration/tasks.rs` manages tasks for the `shared_tasks_decomposition` table, the existing `TaskDbService` in `src/server/orchestration/tasks_db.rs` (which operates on the `shared_tasks` table) lacks dependency checking (DAG evaluation) during task claiming and does not notify the Teammate Mesh upon task assignment.

  ## Research Report
  The Shared Task List handles the complex Directed Acyclic Graph (DAG) dependencies for agentic workflows, orchestrating tasks across both Cloud-Native (PostgreSQL + Redis) and Standalone (SQLite) modes. The Shared Task List relies on database-backed state machines to prevent race conditions during task claiming.

  An audit of `src/server/orchestration/tasks_db.rs` reveals that `claim_task` grabs any `PENDING` task without checking if its prerequisites (stored in the `dependencies` JSON array) are `COMPLETED`. Furthermore, it fails to emit a mesh event upon task assignment, isolating agents from the overall swarm state.

  ## Design Doc
  ### Shared Task List Architecture (DAG & Mesh Extension)
  To ensure tasks are executed in the correct order, the claim queries must be updated to enforce DAG constraints:
  - **PostgreSQL**: The query using `FOR UPDATE SKIP LOCKED` must be updated to use `NOT EXISTS` coupled with `json_array_elements_text(shared_tasks.dependencies::json)` to ensure all parent tasks are evaluated.
  - **SQLite**: The query using simulated distributed locking must similarly use `NOT EXISTS` coupled with `json_each(shared_tasks.dependencies)`.

  ### Teammate Mesh Integration
  Once a task is successfully claimed, the service must encode the `SharedTask` to Protobuf (via `.into_proto()`) and use the `TeammateMesh` (`publish_with_ack`) to broadcast the payload to the `"task_claimed"` topic. This ensures realtime agent coordination.

  ### Mobile UX Flow (375px First)
  *   **Owner Feed**: The owner sees a real-time progress indicator of the UltraPlan on their 375px device. When a sub-task is claimed, the Teammate Mesh event instantly updates the UI, reflecting the agent working on it without requiring a manual refresh.

  ### AI Agent Integration Points
  *   **Worker Agents**: Agents will only receive tasks whose prerequisites are complete, avoiding hallucinations caused by missing context.
  *   **Manager Agent**: Can listen to `"task_claimed"` events on the Teammate Mesh to monitor the swarm's velocity.

  ## Implementation Prompt
  Implement the Shared Task List decomposition and DAG dependencies within `src/server/orchestration/tasks_db.rs` and its associated tests.

  - **Target Outcome**: The `TaskDbService` respects task dependencies and integrates with the `TeammateMesh`.
  - **CUJ**: A complex UltraPlan with a dependency graph is submitted. Worker Agents attempt to claim tasks. The database guarantees that tasks with pending dependencies cannot be claimed. When an eligible task is claimed, a `task_claimed` event is published to the Teammate Mesh.
  - **Acceptance Criteria**:
    - Modify `TaskDbService` to include an `Arc<dyn crate::orchestration::mesh::TeammateMesh>`.
    - Update `claim_task` (for both PostgreSQL and SQLite) to enforce DAG dependencies ensuring parent tasks in the JSON array are `COMPLETED`.
    - On a successful claim, encode the task to proto and broadcast it on the `"task_claimed"` topic via the mesh.
    - Update `src/server/orchestration/tasks_db_test.rs` to pass a `DummyMesh` into `TaskDbService::new`.
    - Add a new test case (`test_tasks_db_claim_task_sqlite_with_dependencies`) to verify that tasks with uncompleted dependencies are blocked.
  - Do NOT modify the `SharedTask` struct definition; use `serde_json::to_vec` or protobuf encoding natively as appropriate to emit the event payload.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []