issue_title: "Architectural Design: Resilient Multi-Tenant AI Job Orchestration & Fairness Queue"
issue_description: |
  # Architectural Design: Resilient Multi-Tenant AI Job Orchestration & Fairness Queue

  ## Problem Statement
  Currently, OHC agents lack a centralized, tenant-aware background job orchestration system with fairness guarantees. As a multi-tenant platform serving diverse owner/operator personas (like Maya, Carlos, Priya, Leo, Fatima, Nora, Jun), the absence of a proper queueing mechanism means that a sudden spike in requests from one tenant (e.g., a viral Instagram reel driving thousands of DMs to Maya's bakery) could monopolize AI processing resources, starving other tenants. Additionally, AI jobs (like generating drafts, intent classification, or report generation) are prone to transient API failures or rate limits from providers (OpenAI, Gemini). We need a robust job orchestrator that provides tenant-level isolation, exponential backoff, dead-lettering, and distributed lock coordination to ensure fair and reliable background work for all users.

  ## Research Report
  - **Competitive Landscape**: Modern AI-native platforms (e.g., Shopify Sidekick, Relevance AI, HubSpot Breeze) rely heavily on asynchronous, durable job queues with tenant isolation to scale background tasks.
  - **PostgreSQL SKIP LOCKED Pattern**: A proven architecture for implementing robust job queues using `SELECT ... FOR UPDATE SKIP LOCKED` allows multiple background workers to concurrently dequeue jobs without locking contention.
  - **Redis Redlock**: Distributed locking (e.g., `ohc:lock:{tenant_id}:{resource_type}:{resource_id}`) is essential to prevent multiple agents from acting on the same event simultaneously or causing race conditions (e.g., double-sending an email draft).
  - **Tenant Fairness**: Round-robin processing or weighted tenant priority queues ensure that no single high-volume tenant blocks others.

  ## Design Doc
  ### Architecture Details
  - **Queue Engine**: A PostgreSQL-backed job table using the `SKIP LOCKED` pattern.
  - **Data Model**:
    - `ai_jobs` table:
      - `id` (UUID, Primary Key)
      - `tenant_id` (UUID, Indexed)
      - `job_type` (Enum: `INTENT_CLASSIFICATION`, `DRAFT_RESPONSE`, `GENERATE_SUMMARY`, etc.)
      - `payload` (JSONB)
      - `status` (Enum: `PENDING`, `RUNNING`, `COMPLETED`, `FAILED`, `DEAD_LETTER`)
      - `attempts` (Integer)
      - `max_attempts` (Integer)
      - `next_retry_at` (Timestamp)
      - `error_details` (Text)
      - `created_at`, `updated_at`
  - **Multi-Tenant Fairness Strategy**: The dequeue query will rotate through active `tenant_id`s or use a CTE (Common Table Expression) to ensure a single tenant does not saturate the worker pool.
  - **Worker Coordination**:
    - Distributed Lock: Before a worker processes an event (e.g., a specific DM thread for `tenant_id`), it acquires a Redis Redlock to prevent duplicate processing.
    - Retry Strategy: Exponential backoff with jitter on transient AI provider failures. Jobs exceeding `max_attempts` are moved to a `DEAD_LETTER` state for review.

  ### Mobile UX Flow & AI Integration
  - The job queue operates entirely in the background. From a mobile UI perspective (375px), users only see the *results* of the queue via Action Cards in their Agent Feed (e.g., "Draft ready for approval").
  - AI agents subscribe to job types, retrieve context via RAG, and publish the results back to the feed or the database.

  ## Implementation Prompt
  **User-Facing Outcome:** The system should process background AI tasks fairly and reliably, ensuring that no tenant experiences lag or dropped tasks due to traffic spikes from other tenants. The backend must include a robust `ai_jobs` queue utilizing PostgreSQL `SKIP LOCKED` and Redis locks.

  **Critical User Journey (CUJ):**
  1. Multiple tenants receive varying bursts of incoming requests (e.g., Webhooks).
  2. The system enqueues these as `PENDING` AI jobs in the PostgreSQL table, tagged with `tenant_id`.
  3. The worker pool dequeues jobs ensuring fairness across tenants (no single tenant monopolizes the queue).
  4. Workers acquire Redis locks during processing to avoid duplicates.
  5. Failed jobs retry with exponential backoff; completely failed jobs enter the `DEAD_LETTER` queue.
  6. The owner/operator simply sees completed tasks (e.g., new Action Cards) in their Agent Feed without interruption.

  **Acceptance Criteria:**
  - Implement the `ai_jobs` PostgreSQL schema.
  - Implement a `JobDequeueService` in the backend that uses `SKIP LOCKED` and guarantees cross-tenant fairness.
  - Implement the Redis Redlock mechanism (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`) within the job worker flow.
  - Implement an exponential backoff retry mechanism.
  - Add extensive unit testing for the queue logic and E2E Playwright tests verifying the enqueue-to-feed flow using documented test credentials.
  - Ensure 100% test coverage for new/modified backend modules.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
