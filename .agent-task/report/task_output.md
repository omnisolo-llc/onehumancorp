issue_title: "[Architecture] High-Performance Background Job Queue and Distributed Coordination"
issue_description: |
  # Background Job Queue and Distributed Coordination

  ## Problem Statement
  Currently, OHC lacks a robust, scalable background job queue that can safely orchestrate AI agents and long-running business operations (like bulk email sends, payment reconciliations, or generating weekly advisory reports) across multiple tenants. Without this, operations are brittle, prone to timeouts, and lack proper retry logic and observability. We need a system that supports row-level tenant isolation, `SKIP LOCKED` patterns for PostgreSQL, and Redis-backed distributed locking to prevent agents from stepping on each other's toes (e.g., preventing two agents from trying to restock the same inventory item simultaneously).

  ## Research Report
  - **Competitor Analysis:** Shopify uses heavily optimized asynchronous workers (Sidekiq/Resque equivalents in Ruby) to handle webhook processing and massive background tasks. Wix relies on robust, distributed task execution systems. GoDaddy and Squarespace also implement job queues for asynchronous processing.
  - **OHC Architecture Fit:** OHC uses PostgreSQL and Redis. The `SKIP LOCKED` pattern in PostgreSQL is a highly effective way to implement a durable job queue, while Redis Redlock is an industry standard for distributed locking across instances.
  - **Persona Impact:** If Maya's automated Instagram DM replies fail silently due to a timeout, she loses a sale and trusts the platform less. A reliable background job queue ensures all AI actions are completed eventually and retried if needed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant API as API Server
      participant DB as PostgreSQL (Job Queue)
      participant Redis as Redis (Distributed Lock)
      participant Worker as Job Worker

      API->>DB: Insert Job (e.g., Send Promo Email)
      Worker->>DB: SELECT FOR UPDATE SKIP LOCKED
      DB-->>Worker: Return Job
      Worker->>Redis: Acquire Lock `ohc:lock:{tenant_id}:{job_type}:{resource_id}`
      Redis-->>Worker: Lock Acquired
      Worker->>Worker: Execute Job (AI generation, API calls)
      Worker->>DB: Update Job Status (Complete/Failed)
      Worker->>Redis: Release Lock
  ```

  ### Implementation Details
  - **Database Schema (PostgreSQL):** A `jobs` table with columns for `id`, `tenant_id` (RLS enforced), `status` (pending, processing, completed, failed), `payload` (JSONB), `retry_count`, `next_retry_at`, and standard timestamps.
  - **Distributed Locks:** Implement a Redlock-style distributed lock using Redis to coordinate cross-agent actions. Use the lock pattern: `ohc:lock:{tenant_id}:{resource_type}:{resource_id}`.
  - **Worker Process:** A dedicated background worker process (or an asynchronous pool within the Rust server) that polls the `jobs` table using `SELECT ... FOR UPDATE SKIP LOCKED` and processes tasks.
  - **AI Agent Integration:** AI departments (like Operations or Marketing) will enqueue jobs for long-running tasks instead of blocking HTTP requests.

  ## Implementation Prompt
  Implement a durable, high-performance background job queue using PostgreSQL and an associated distributed locking mechanism using Redis.
  1.  **Database Migration:** Create the `jobs` table with `tenant_id` for Row-Level Security, a `status` enum, and a JSONB `payload`.
  2.  **Job Enqueue/Dequeue Logic:** Implement Rust repository functions to enqueue jobs and dequeue jobs safely using the `SKIP LOCKED` pattern. Ensure worker retry logic with exponential backoff (up to 3 retries) and routing to a dead-letter state upon exhaustion.
  3.  **Distributed Lock Manager:** Create a Redis-backed locking service in Rust following the `ohc:lock:{tenant_id}:{resource_type}:{resource_id}` pattern to ensure exclusive execution for critical agent tasks.
  4.  **Worker Loop:** Implement a background worker task that continuously polls the queue, acquires the necessary lock, executes the job payload (simulated for now), updates the job status, and releases the lock.
  5.  **Observability:** Instrument the enqueue and process steps with OpenTelemetry tracing and Prometheus metrics to monitor queue depth and processing latency.

  **Acceptance Criteria:**
  - `jobs` table created with RLS enabled.
  - Rust functions for enqueueing and safe dequeueing (`SKIP LOCKED`) implemented.
  - Redis distributed locking service implemented.
  - A background worker loop processes jobs, respects retries, and properly handles locks.
  - Unit tests verify queue ordering, lock acquisition/release, and exponential backoff.
  - Full mobile parity and no friction introduced in user flows.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
