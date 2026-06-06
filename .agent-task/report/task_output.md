issue_title: "[research] High-Performance Background Agentic Job Queue"
issue_description: |
  ## Problem Statement
  Small business owners rely on OHC to execute numerous background tasks instantly and reliably (e.g., AI generating quotes, inventory sync, email dispatches). When these fail or take too long, the business stalls. We currently lack a highly reliable, high-performance background job queue capable of coordinating distributed AI agent tasks across our multi-tenant architecture, leading to potential dropped tasks and degraded performance.

  ## Research Report
  - **Current State**: We have fragments of job handling but no unified, robust job queue designed specifically for agentic workflows with exponential backoff, dead-letter queues, and tenant isolation built-in.
  - **Competitor Analysis**: Modern platforms (like Shopify's background workers or standard SaaS practices) use robust queues (e.g., Redis + Sidekiq, or Postgres `SKIP LOCKED`). For our scale and operational simplicity, a PostgreSQL-backed queue utilizing `SKIP LOCKED` provides transactional guarantees, atomic state transitions, and reduces the need for additional infrastructure (like relying solely on Redis for durability).
  - **Discovery**: A dedicated `SKIP LOCKED` Postgres queue with exponential backoff and DLQ (Dead Letter Queue) is required.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      API[API/Agent Task Creator] --> DB[PostgreSQL Job Queue]
      DB -- SKIP LOCKED --> Worker1[Agent Worker 1]
      DB -- SKIP LOCKED --> Worker2[Agent Worker 2]
      Worker1 --> Success[Job Complete]
      Worker1 -. Failed .-> Retry[Retry/Backoff Logic]
      Retry -. Max Retries Reached .-> DLQ[Dead Letter Queue]
  ```

  ### Key Design Decisions
  1.  **Postgres-Backed Queue**: Utilize PostgreSQL with `SELECT ... FOR UPDATE SKIP LOCKED` for robust, transaction-safe job dequeueing without external dependencies.
  2.  **Retry Mechanics**: Built-in exponential backoff (e.g., up to 3 retries) and routing to a dead-letter queue (DLQ) upon final failure.
  3.  **Tenant Isolation**: Jobs must strictly enforce `tenant_id` boundaries for security.

  ## Implementation Prompt
  **To the Engineering Swarm:**
  Implement the High-Performance Background Agentic Job Queue using PostgreSQL.
  1.  Define the database schema for the job queue (including columns for tenant_id, job payload, status, retry count, next run time).
  2.  Implement the worker polling mechanism using the `SKIP LOCKED` pattern.
  3.  Implement exponential backoff retry logic and dead-letter queue routing.
  4.  Provide tests to guarantee job isolation between tenants and verify backoff behavior.

  *Ensure you strictly follow the OHC backend engineering standards and thoroughly test via `bazel test //...` before completing.*
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
