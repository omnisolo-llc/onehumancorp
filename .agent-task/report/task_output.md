issue_title: "[architecture] Implement High-Performance Multi-Tenant Job Queue"
issue_description: |
  # Problem Statement
  OneHumanCorp relies heavily on autonomous agents working asynchronously. Whether it's The Promoter generating marketing copy, The Manager reconciling multi-channel inventory conflicts, or the Finance Agent drafting invoices, these actions must be queued, executed, and retried reliably. Currently, the system has a basic queue structure defined in `src/server/queue.rs` (`InMemJobQueue`), but this does not scale to thousands of tenants in a cloud deployment. When multiple pods or worker nodes spin up, we risk duplicate job execution or job starvation unless we implement a robust distributed queue. The cloud implementation must use a PostgreSQL `SKIP LOCKED` pattern, enforcing strict tenant isolation through RLS to prevent cross-tenant data leaks during job processing.

  # Research Report
  - **The "Lost Job" Problem:** Small business owners expect absolute reliability. If Carlos the handyman clicks "Approve Quote", and the job to send that email gets dropped due to a pod restart, Carlos loses a client.
  - **PostgreSQL `SKIP LOCKED`:** This is the industry-standard approach for building reliable, high-concurrency job queues without the overhead of maintaining a separate messaging system like Kafka or RabbitMQ. It allows multiple workers to poll the `ohc_job_queue` table concurrently; if one worker locks a row, others instantly skip it and grab the next available job.
  - **Multi-Tenancy Requirements:** OHC's core design mandates row-level security. The job queue table (`ohc_job_queue`) currently has RLS policies, but the queue dequeuing logic must be strictly context-aware. A worker must only be able to dequeue jobs for the tenant it is authorized to operate on, or the background worker must assume the correct tenant role before executing the job.

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App
      participant API as OHC API
      participant DB as PostgreSQL (ohc_job_queue)
      participant Worker1 as Background Worker A
      participant Worker2 as Background Worker B

      App->>API: Trigger Agent Action (e.g. Generate Quote)
      API->>DB: INSERT INTO ohc_job_queue (status='PENDING')

      loop Every Second
          Worker1->>DB: SELECT * FROM ohc_job_queue WHERE status='PENDING' FOR UPDATE SKIP LOCKED LIMIT 1
          Worker2->>DB: SELECT * FROM ohc_job_queue WHERE status='PENDING' FOR UPDATE SKIP LOCKED LIMIT 1
      end

      DB-->>Worker1: Returns Job ID 123 (Locks it)
      DB-->>Worker2: Skips Job ID 123, Returns Job ID 124 (Locks it)

      Worker1->>Worker1: Execute Job 123
      Worker1->>DB: UPDATE ohc_job_queue SET status='COMPLETED' WHERE id=123
  ```

  ## Mobile UX Flow (375px)
  While the queue is a backend mechanism, its reliability directly impacts the mobile UX.
  1. The user taps "Generate Quote" on their phone.
  2. The UI instantly shows an optimistic "Drafting Quote..." state.
  3. The job enters the queue. If the user loses internet connection immediately after, the job is still safely queued in the backend.
  4. Once the background worker completes the job, a real-time notification or WebSocket event updates the mobile app to show the finished draft.

  ## AI Agent Integration
  - **Task Delegation:** When the Orchestrator identifies a complex, multi-step goal, it decomposes it into smaller `Job` records.
  - **Retry Semantics:** If the LLM API (e.g., Gemini) times out, the worker updates the job status to `FAILED`, increments `retry_count`, and sets `next_retry_at` using exponential backoff. The `SKIP LOCKED` query explicitly filters by `next_retry_at <= NOW()`.

  # Implementation Prompt
  Implement the PostgreSQL `SKIP LOCKED` backend for the `TaskQueue` trait defined in `src/server/orchestration/queue/queue.rs`.

  **Outcome:** The system should be able to run multiple worker instances concurrently against a shared PostgreSQL database. They must safely dequeue jobs without race conditions or deadlocks.

  **Acceptance Criteria:**
  1. Create a `PgTaskQueue` struct implementing `TaskQueue`.
  2. The `dequeue` method must use a raw SQL query with the `FOR UPDATE SKIP LOCKED` clause.
  3. Ensure the dequeuing respects exponential backoff (`next_retry_at <= CURRENT_TIMESTAMP`).
  4. Write a concurrent unit test that spins up 5 mock workers attempting to pull from a pool of 100 jobs, verifying that exactly 100 jobs are executed with 0 duplicates.
  5. The execution context must securely pass the `tenant_id` so the worker operates under the correct RLS policy.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []