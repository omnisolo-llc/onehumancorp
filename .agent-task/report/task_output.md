issue_title: "[Architecture Gap] Centralized Agent Execution and Task Queue Pipeline"
issue_description: |
  # Research Report: Centralized Agent Execution and Task Queue Pipeline

  ## Problem Statement
  OneHumanCorp's vision depends heavily on asynchronous, agentic execution ("The agent feed"). Currently, we do not have a robust architectural pattern for queuing, executing, and monitoring background AI agent tasks across our system. Background workflows (like email generation, inventory tracking, social media responses) lack a resilient, multi-tenant capable event-driven job queue. If a long-running LLM generation fails due to a rate limit or a network error, there is no standardized dead-letter queue or retry backoff system.

  ## 1. Market Context & Scaling Discovery (Track 1)
  - **Codebase Audit:** We have basic server frameworks in Go (`src/server/`) and AI components (`src/agents/`), but we are missing a mature scheduling/job queue implementation.
  - **Competitor Systems:**
    - *Shopify:* Uses high-throughput job queues for thousands of webhooks and automated workflows per second.
    - *Temporal / Celery / Oban:* Standard architectures for distributed async tasks use robust backing stores (e.g., PostgreSQL with `SKIP LOCKED` or Redis) to maintain state, handle retries, and decouple fast API handlers from slow jobs.
  - **The Gap:** OHC needs a robust async job processing system for agent operations that fits into our multi-tenant architecture.

  ## 2. Selected Architecture Deep Dive (Track 2)
  We need an **Agent Task Execution Pipeline** utilizing a PostgreSQL-backed job queue for strong persistence and multi-tenancy, and Redis for distributed locking to prevent duplicate task execution in concurrent environments.

  ### Data Model & Invariants
  - **`AgentJob` Entity:** Represents a pending, executing, or failed task.
    - `id` (UUID)
    - `tenant_id` (UUID) - Mandatory for row-level security.
    - `job_type` (Enum/String: e.g., 'DraftEmailResponse', 'ReconcileInventory')
    - `payload` (JSONB) - Context necessary for the AI.
    - `status` (Enum: 'Pending', 'Running', 'Completed', 'Failed')
    - `attempts` (Int)
    - `next_run_at` (Timestamp)
    - `locked_by` (String) - Identifier of the worker holding the lease.
  - **Distributed Locks:** Redis Redlock pattern `ohc:lock:{tenant_id}:{job_id}` for short-lived leases during execution.
  - **AI Department Coordination:** When an event occurs (e.g., webhook received), it writes a fast `AgentJob` to DB. A pool of workers queries pending jobs via `SELECT ... FOR UPDATE SKIP LOCKED`, executes the LLM logic, handles retries/backoff, and publishes the result.

  ### Visualizing the Queue (Mermaid Representation)
  ```mermaid
  erDiagram
      Tenant ||--o{ AgentJob : has
      AgentJob {
          UUID id
          UUID tenant_id
          String job_type
          JSONB payload
          String status
          Int attempts
          Timestamp next_run_at
          String locked_by
      }
      AgentJob ||--o{ JobLog : generates
  ```
  ```mermaid
  sequenceDiagram
      participant API
      participant DB
      participant Worker
      participant LLM
      API->>DB: Insert AgentJob
      Worker->>DB: SELECT FOR UPDATE SKIP LOCKED
      Worker->>LLM: Execute Job Payload
      LLM-->>Worker: Result
      Worker->>DB: Update AgentJob (Completed/Failed)
  ```

  ## 3. Technical Integrity & Mobile-First Review (Track 3)
  - **UX/UI:** The user does not interact with the queue directly. Instead, the UI subscribes to SSE/WebSockets or polls an endpoint to see "Agent is drafting..." and then receives the finished Draft. On a 375px mobile screen, this translates to smooth real-time Action Cards popping into the Agent Feed without the app blocking the main UI thread.
  - **Security:** Strict row-level multi-tenancy (`tenant_id`) ensures workers never leak context between businesses. Identity must be propagated into the worker context.

  ## 4. Implementation Prompt (Track 4)

  **Feature Name:** Agent Task Execution Pipeline (Background Job Queue)

  **Target Persona:** Maya the Baker (who expects her agent to gracefully queue up Instagram DM responses without dropping them during traffic spikes).

  **Outcome:** A robust, multi-tenant background job processing system integrated into the Go backend that reliably handles long-running AI tasks with exponential backoff and dead-lettering.

  **Critical User Journey (CUJ):**
  1. A webhook from a customer DM arrives.
  2. The API instantly returns 200 OK and asynchronously enqueues a `DraftResponse` job.
  3. A background worker picks up the job, queries the LLM, and successfully drafts the message.
  4. The job is marked Completed, and an event is sent to Maya's mobile feed.
  5. (Error case): If the LLM rate limits, the worker records the failure, increments the attempt counter, and schedules a retry using exponential backoff.

  **Acceptance Criteria for Implementer:**
  - Create the multi-tenant PostgreSQL schema for background jobs (including indices for efficient polling).
  - Implement a job dequeue logic utilizing `FOR UPDATE SKIP LOCKED`.
  - Provide a Go interface for defining "Job Handlers" (e.g., an `EmailHandler`, an `InventoryHandler`).
  - Implement an execution loop with configurable worker pools and retry policies.
  - No specific UI required, but include a robust test proving jobs are isolated by tenant, retried on failure, and execute successfully.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
