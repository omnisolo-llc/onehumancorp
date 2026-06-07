issue_title: "Implement High-Scale Background Job Queue for the Operations Agent"
issue_description: |
  # Implement High-Scale Background Job Queue for the Operations Agent

  ## Title
  High-Scale Background Job Queue for AI Departments

  ## Problem Statement
  Small business owners need their "Operations Agent" to handle background tasks reliably (e.g., processing order fulfillments, sending out appointment reminders, retrying webhook failures). Currently, there is a risk that temporary failures or scale spikes (like Fatima getting 50 lunch pre-orders at once) might result in dropped processes because there's no resilient, centralized background job processing system. A small business owner will lose trust immediately if an AI agent drops an order due to a timeout.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify & Stripe:** Use highly resilient background queue architectures with exponential backoff and idempotency guarantees.
  - **Wix:** Utilizes internal event-driven architectures to process actions asynchronously without blocking the user interface.
  - **OHC Architecture Gap:** While OHC defines an AI Job Queue using PostgreSQL `SKIP LOCKED`, there needs to be a robust Go implementation for a background job processor that ensures tenant isolation, robust retries with exponential backoff, and dead-letter queues, keeping the Operations Agent fully reliable.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[API Layer] -->|Enqueue Job| B(PostgreSQL Jobs Table)
      B -->|SKIP LOCKED| C[Job Worker Pool Go]
      C -->|Process| D{AI Department Handlers}
      D -->|Success| E[Mark Completed]
      D -->|Failure < Max Retries| F[Schedule Retry exp. backoff]
      D -->|Failure >= Max Retries| G[Dead-Letter Queue]
  ```

  ### Mobile UX Flow
  - Users do not see the background queue directly.
  - They see optimistic UI updates ("Processing...") with eventual consistency notifications via the app (e.g., "Maya, the customer has been notified").
  - On the backend, failed jobs will simply retry invisibly, preserving the "magic" experience for the user.

  ### AI Agent Integration Points
  - Operations Agent: Will rely on this queue for async tasks like order processing.
  - Customer Success Agent: Enqueuing email/DM responses to send reliably.

  ### Key Design Decisions
  - **PostgreSQL `SKIP LOCKED`**: We use Postgres over external brokers like Redis/RabbitMQ to simplify infrastructure for the SaaS deployment while achieving robust transactional guarantees.
  - **Exponential Backoff**: Prevents hammering external APIs during outages.

  ## Implementation Prompt
  **To the Implementer:**
  Implement a reliable PostgreSQL-backed background job queue for OHC using the `SKIP LOCKED` pattern in Go.
  The solution must include:
  1. A Postgres table schema for jobs, ensuring tenant isolation (`tenant_id`), payload storage, retry counts, and status tracking.
  2. A Go worker pool that queries jobs using `FOR UPDATE SKIP LOCKED`.
  3. Exponential backoff retry logic (up to 3 retries).
  4. Dead-letter queue mechanism for permanently failed jobs.
  Ensure the implementation is covered by 100% unit tests and integrates well with our Bazel build system.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
