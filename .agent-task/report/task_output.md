issue_title: "[Architecture] Distributed Agent Execution and Orchestration"
issue_description: |
  # Mission Queue Protocol: Agent Orchestration and Visual Workflow Scaling

  ## Problem Statement
  The current platform includes a built-in agent harness and a visual workflow engine (`visual_workflow.rs`) for no-code agent assembly. While the current execution path works for a single machine or basic test use case, it lacks the multi-node scaling and durability needed to support thousands of active SMB owners.
  When Maya (the baker) gets a surge of custom cake requests over Instagram or Carlos (the handyman) has agents quoting multiple complex jobs, the agent orchestration needs to queue, distribute, and track these workflows across a cluster without losing state or blocking other owner operations. We need a way to execute agent plans with distributed durability, parallel execution tracking, and proper tenant isolation on PostgreSQL and Redis (via the KAIROS Orchestration layer).

  ## Research Report
  - **Platform Context:** OHC is built for single business owners and operators (Maya, Carlos, Priya) who expect immediate insight and trust the system to coordinate background work efficiently.
  - **Current Implementation:** Found in `src/agents/builtin/visual_workflow.rs` and `src/server/harness/executor.rs`. The basic parallel fan-out/fan-in mechanisms (ParallelFork, ParallelJoin) exist, but they lack a durable state machine backend for resuming after crashes or migrating between worker nodes.
  - **Market Context:** Modern execution engines like Temporal or AWS Step Functions use event sourcing and database-backed queues. A robust architecture needs to apply these patterns via the PostgreSQL `SKIP LOCKED` job queue and Redis Redlock for cross-node coordination.
  - **Key Gap:** Missing distributed state checkpoints, tenant-isolated message bus queues for agent workflows, and proper recovery mechanisms if an agent worker dies while executing a parallel fork.

  ## Design Doc
  ### Mobile UX Flow
  - Maya opens her OHC app (375px viewport) and sees a "Workflow Status" indicator in her command center (e.g., "Agents: 2 active drafting quotes").
  - Behind the scenes, the workflow UI doesn't block. It is reactive to the distributed workflow state.
  - Any failed agent tasks automatically show an owner-friendly error card ("We couldn't reach the supplier, click to retry") rather than a technical log.

  ### AI Agent Integration Points
  - Operations Agent triggers workflows via the visual workflow router.
  - System must capture partial state (episodic memory) at every step in `ParallelFork` to allow the AutoDream pipeline to embed knowledge correctly.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[API Router /visual_workflow/run] -->|Enqueue| B[Postgres Job Queue SKIP LOCKED]
      B --> C[Worker Node 1]
      B --> D[Worker Node 2]
      C -->|Redlock| E[(Redis)]
      C -->|Update State| F[(Postgres Tenant DB)]
      C -.-> G[ParallelFork]
      G --> H[Agent Step A]
      G --> I[Agent Step B]
      H --> J[ParallelJoin]
      I --> J
  ```

  ### Core Decisions
  - **Tenant Isolation:** Every queued step must carry `tenant_id` to strictly limit data access via Row Level Security (RLS).
  - **Durability:** Every state transition in `ParallelFork` and `ParallelJoin` must commit to Postgres before continuing, preventing duplicate billing or ghost operations.

  ## Implementation Prompt
  Implementer: Refactor the visual workflow engine to support durable, distributed execution.
  1. Define a durable state schema for workflow steps that maps to the existing job queue pattern.
  2. Implement checkpointing for `ParallelFork` and `ParallelJoin` so that if a worker crashes, another worker can safely resume the workflow using `SKIP LOCKED`.
  3. Expose these state updates to the UI so owners can see plain-language progress on active tasks.
  4. Write comprehensive integration tests simulating node failure during a parallel fork.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
