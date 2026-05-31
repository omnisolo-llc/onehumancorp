issue_title: "[Architecture] High-Performance Agentic Background Job Queue"
issue_description: |
  # High-Performance Agentic Background Job Queue

  ## Problem Statement
  OneHumanCorp's core promise is that "AI agents do the heavy lifting invisibly." However, our background job processing system—critical for asynchronous agent tasks, large batch processing, long-running workflow orchestration, and resilient retries—lacks a robust, multi-tenant aware, edge-capable, and highly observable architecture. Currently, tasks are either spawned directly (which lacks durability and retries), pushed to a generic Postgres queue without advanced multi-tenant scaling, or lack deep integration with our open telemetry standards and agent lifecycle. As OHC scales to handle thousands of concurrent background jobs for tasks like AI website generation, social media post scheduling, bulk customer email syncs, and financial report generation across various business types (physical, services, subscriptions), we need a queue architecture that guarantees Zero-Drop operations, respects strict multi-tenant row-level security, and supports localized edge execution where necessary.

  ## Research Report
  - **Market Context**: Platforms like Shopify utilize mature, distributed job queues (e.g., Sidekiq, Resque, or custom internal systems) that allow millions of background jobs to be processed efficiently, retried automatically, and isolated to prevent noisy neighbor problems. Wix and Squarespace similarly rely on background orchestration for long-running tasks like bulk media processing and SEO analysis.
  - **Codebase Audit Findings**:
    - `src/server/builder/jobs.rs` shows an explicit comment: `// Instead of simple spawn, we should use a job queue... In a real implementation this would enqueue to a PostgreSQL table for processing via SKIP LOCKED pattern.`
    - Existing implementations in `src/server/orchestration/queue` (e.g., `pg_queue.rs`, `sqlite_queue.rs`, `redis_queue.rs`) provide basic functionality but need a comprehensive architectural upgrade to support dynamic agent department workflows, strict multi-tenant context propagation, Dead Letter Queues (DLQ), and integration with OpenTelemetry/Prometheus metrics (some of which are started in `telemetry/mod.rs` but need formalization).
    - Latency benchmarks (`latency_bench.rs`) indicate ongoing efforts to measure queue performance, highlighting its critical role in system latency.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  flowchart TD
      API[API/Agent Handlers] -->|Enqueue| MQ[Multi-Tenant Agent Job Queue]
      MQ -->|SKIP LOCKED| Workers[Agentic Workers]
      Workers -->|Update State| DB[(PostgreSQL)]
      Workers -->|Emit Metrics| OTel[OpenTelemetry/Prometheus]
      Workers -->|Failures| DLQ[Dead Letter Queue]
      DLQ -->|Retry Logic| MQ

      subgraph Workers Layer
      Op[Operations Agent]
      Mkt[Marketing Agent]
      Fin[Finance Agent]
      CS[CS Agent]
      end
      Workers --> Op
      Workers --> Mkt
      Workers --> Fin
      Workers --> CS
  ```

  ### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ QueueJob : owns
      Department ||--o{ QueueJob : processes
      QueueJob ||--o{ JobError : tracks

      QueueJob {
          uuid id PK
          uuid tenant_id FK
          string department_id FK
          string status "QUEUED, PROCESSING, COMPLETED, FAILED"
          jsonb payload
          int attempts
          timestamp run_after
          timestamp locked_until
          timestamp created_at
      }

      JobError {
          uuid id PK
          uuid queue_job_id FK
          string error_message
          timestamp occurred_at
      }
  ```

  ### UI Wireframes
  - Not directly applicable, this is a backend capability.

  ### Mobile UX Flow
  - **Zero-Touch Backgrounding**: The non-technical business owner (e.g., Maya or Carlos) never sees "Processing" spinners for long tasks. They are immediately returned to the UI (optimistic UI update) while the Agentic Queue processes the task (e.g., generating 100 product descriptions) and sends a real-time notification (via WebSocket/Push) upon completion.

  ### AI Agent Integration Points
  - **Department Routing**: Jobs are tagged with their functional department (`Operations`, `Marketing`, `Finance`, etc.), allowing specialized workers to consume them based on available AI capabilities or VRAM/Token requirements.

  ### Key Design Decisions
  - **Multi-Tenant Isolation**: Every job must carry the `tenant_id` and execute under a scoped database context. Uses SKIP LOCKED for maximum concurrent throughput.

  ## Implementation Prompt
  Implement a highly robust, multi-tenant background job queue system using the `PostgreSQL SKIP LOCKED` pattern as the core durable storage.

  1.  **Data Model**: Define or refine the `sub_agent_jobs` table to include strict `tenant_id` partitioning, `agent_role`, `status` (QUEUED, PROCESSING, COMPLETED, FAILED), `payload` (JSONB), `attempts`, `run_after`, `locked_until`, and `error_log`.
  2.  **Queue Interface Update**: Enhance the `TaskQueue` trait in `src/server/orchestration/queue/queue.rs` to support robust enqueueing (single and batch), dequeueing with specific role/department targeting, explicit completion, and failure handling with exponential backoff routing to a Dead Letter Queue state.
  3.  **Worker Implementation**: Develop a scalable worker pool that polls the Postgres queue using `SELECT ... FOR UPDATE SKIP LOCKED`. Ensure workers automatically inherit the `tenant_id` context for all database operations to maintain multi-tenant isolation.
  4.  **Observability Integration**: Ensure all enqueue, dequeue, completion, and failure events emit high-fidelity metrics to OpenTelemetry (e.g., time-in-queue, processing duration, failure rates per department).
  5.  **Refactor Existing Spawns**: Identify critical areas currently using direct async spawns (like `enqueue_publish_site_job` in `builder/jobs.rs`) and convert them to use the new robust Postgres queue.
  6.  **Testing**: Write comprehensive unit tests and Playwright E2E tests simulating a background AI job (e.g., bulk product import or AI social post generation) from the owner's perspective, verifying the optimistic UI and subsequent background completion.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
