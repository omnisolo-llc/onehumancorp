issue_title: "[feature] Implement High-Performance Background Job Queue"
issue_description: |
  ## Problem Statement
  For non-technical owners like **Maya (baker)** and **Nora (agency principal)**, background automation is critical to the OHC promise. When Maya receives an Instagram DM order while sleeping, the Customer Assistant needs to draft a reply, update inventory, and possibly request a Stripe deposit asynchronously. If a third-party API (like Stripe or Meta) has a transient failure, the owner should not experience missing data, failed follow-ups, or unhandled errors.

  Currently, while OHC envisions a rich multi-agent background orchestration system, the infrastructure lacks a robust, natively multi-tenant, durable job queue. If the background system fails, Maya misses her custom cake order, and Nora's invoice reminders don't go out.

  The opportunity is to implement a high-performance background job queue leveraging PostgreSQL `SKIP LOCKED` (as envisioned in the OHC backend architecture) combined with a dead-letter queue (DLQ) and exponential backoff retry mechanisms. This provides the invisible reliability engine needed for AI agent coordination.

  ## Research Report

  ### Market Mapping & Competitor Discovery
  - **Shopify & Wix**: Rely on massive proprietary asynchronous event buses (Kafka/SQS) with heavily engineered retry patterns. Their complexity is too high for a single-binary/standalone deployment model.
  - **Sidekiq (Ruby) / Celery (Python) / Oban (Elixir)**: Oban represents the ideal architectural model—using PostgreSQL for durable, transaction-safe job queueing with `SKIP LOCKED`. This avoids the need for a separate message broker (like RabbitMQ) and keeps operational simplicity high, which is a core OHC requirement (especially in standalone/SQLite modes, though this task focuses on the Postgres `SKIP LOCKED` pattern for cloud-native mode).
  - **Current OHC State**: The backend uses Go with PostgreSQL. A structured background queue using `SKIP LOCKED` ensures at-least-once delivery, tenant isolation within the database transaction boundary, and prevents concurrent worker collisions.

  ### Design Doc
  - **Architecture**:
    - **Queue Table**: `ohc_jobs` table in PostgreSQL.
    - **Columns**: `id`, `tenant_id` (for row-level security / isolation), `queue_name`, `payload` (JSONB), `status` (pending, processing, completed, failed), `run_at` (timestamp for deferred/scheduled jobs), `attempts`, `max_attempts`, `last_error`, `created_at`, `updated_at`.
    - **Worker Loop**: A background Goroutine task polls the `ohc_jobs` table.
    - **Dequeue Query**: `UPDATE ohc_jobs SET status = 'processing', updated_at = NOW() WHERE id = (SELECT id FROM ohc_jobs WHERE status = 'pending' AND run_at <= NOW() ORDER BY run_at ASC FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING *;`
    - **Dead Letter Queue (DLQ)**: Jobs exceeding `max_attempts` are moved to a `status = 'failed'` state (the DLQ), which can be inspected by administrators or surfaced as "Failed Automations" to the owner in plain language.
  - **Mobile UX Flow**: This is a backend architectural component. It powers the "Invisible" agent work. The user experience is that Maya sees "Agent drafted a reply" in her unified feed, completely unaware of the durable queue guaranteeing it.
  - **AI Agent Integration Points**: AI tasks (e.g., "Draft Proposal", "Sync Inventory", "Generate Daily Briefing") are enqueued here. Agents can enqueue subsequent jobs, enabling complex chained workflows.

  ### Implementation Prompt
  Implement a robust PostgreSQL-backed job queue using the `SKIP LOCKED` pattern in the Go backend.
  - Create the database migration for the `ohc_jobs` table, ensuring `tenant_id` is present for multi-tenancy.
  - Build a Go service module (`src/server/queue` or similar) that provides `Enqueue` and `Dequeue` functionality.
  - Implement a Goroutine-based worker loop that continuously polls and processes jobs using the Context package for cancellation.
  - Ensure jobs support exponential backoff for retries and are marked as failed (DLQ) after a configurable number of attempts.
  - The implementation must include 100% unit test coverage for the queue logic, verifying that concurrent workers do not process the same job twice (`SKIP LOCKED` behavior).
  - Do NOT prescribe the exact Go structs or function signatures; let the implementer design the optimal internal abstractions.

  ### Priority
  P0

  ### Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
