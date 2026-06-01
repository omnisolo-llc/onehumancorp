issue_title: "[Architecture] High-Performance Background Job Queue and Distributed Ledger Engine"
issue_description: |
  # Architecture Gap: High-Performance Background Job Queue & Universal Ledger

  ## Problem Statement
  OneHumanCorp (OHC) empowers non-technical business owners like Maya (the baker) and Fatima (the food cart operator) to run their operations effortlessly, relying heavily on background "AI agents" performing tasks invisibly (e.g. coordinating inventory, responding to IG DMs, managing financial records, sending out quotes). As the tenant base scales, managing background automation strictly through ad-hoc service workers or basic DB-level pub/sub falls apart.

  OHC currently lacks a foundational, robust, high-performance background job queue system capable of guaranteed execution, distributed cross-agent coordination (distributed locking), and a strongly consistent tenant-isolated universal ledger for tracking state transitions across departments (Sales, Finance, Operations). Without this, Maya's "vegan cake" orders might get lost if an AI worker crashes, or Carlos might double-book his handyman services.

  ## Research Report
  - **Shopify & Wix**: Utilize massively distributed job queues (like Sidekiq/Kafka) to decouple storefront operations from background fulfillment and notification syncs, ensuring that even under flash-sale loads, no orders or webhooks are lost.
  - **Stripe**: Employs an immutable ledger architecture combined with stringent idempotency controls, allowing multiple isolated microservices to coordinate financial updates reliably.
  - **OHC Architecture Context**: The backend is Go/Rust with PostgreSQL and Redis. The required missing capability is a robust, tenant-isolated background job queue system using `SKIP LOCKED` in PostgreSQL combined with Redis-based distributed locking (Redlock) for cross-department coordination. This is essential for the "Zero Drop" operations promise.

  ## Design Doc
  - **Data Model (Universal Ledger & Job Queue)**:
    - `ohc_job_queue`: `id`, `tenant_id`, `job_type`, `payload` (JSONB), `status` (PENDING, PROCESSING, COMPLETED, FAILED), `retry_count`, `next_retry_at`. (Strict RLS on `tenant_id`).
    - `ohc_universal_ledger`: Immutable append-only log recording state changes across AI departments (e.g., "Marketing posted to IG", "Finance logged deposit").
  - **Architecture Mechanism**:
    - **PostgreSQL `SKIP LOCKED`**: Workers poll the `ohc_job_queue` using `SELECT ... FOR UPDATE SKIP LOCKED` to dequeue jobs concurrently without deadlocks.
    - **Redis Redlock**: Distributed locks (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`) to prevent race conditions when multiple agents try to modify the same resource (e.g., updating Maya's cake inventory).
    - **Exponential Backoff**: Built into the worker retry loop.
    - **Dead-letter Queue**: For permanently failed jobs after max retries.
  - **AI Department Integration**:
    - "The Operations Manager" agent pushes fulfillment jobs to the queue.
    - "The Accountant" agent listens to the ledger for payment clearance events.
  - **Mobile UX Flow**:
    - The non-technical user never sees the queue. They simply see optimistic UI updates (e.g., "Order placed"). The background queue ensures eventual consistency and triggers real-time mobile notifications via Firebase/APNs upon job completion.

  ## Implementation Prompt
  **Mission**: Implement the core High-Performance Background Job Queue and Distributed Ledger infrastructure.
  - **CUJ**: A tenant (e.g., Maya) receives a new custom cake order via the storefront. The system must enqueue a sequence of background jobs: 1) Deduct inventory, 2) Send confirmation email, 3) Notify the Operations agent.
  - **Acceptance Criteria**:
    - Create the database schemas for the job queue and universal ledger with row-level security enabled.
    - Implement a Go/Rust worker pool that dequeues jobs using PostgreSQL `SKIP LOCKED`.
    - Implement a Redis Redlock mechanism for cross-agent coordination.
    - Ensure jobs are retried with exponential backoff on failure.
    - Write unit and Playwright/API E2E tests validating that jobs are processed reliably under concurrency without data loss. All `bazelisk test //...` must pass.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
