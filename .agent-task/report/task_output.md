issue_title: "OHC Multi-Tenant Task Management & Queue Architecture"
issue_description: |
  # OHC Task Management & Background Queue Architecture

  ## Problem Statement
  OHC requires a robust, scalable, multi-tenant task execution platform capable of coordinating background tasks (like data sync, scheduled auto-replies, and scheduled report generation). Existing structures lack a standardized multi-tenant Job Queue abstraction that seamlessly isolates tenant boundaries, tracks failures, and ensures agentic tasks (like the Ambassador Agent's auto-reply generation) run efficiently and durably in the background.

  ## Research Report
  - **Shopify & Wix**: Rely heavily on distributed background jobs to scale. Shopify processes millions of asynchronous events per minute using Kafka and Redis queues to separate web request processing from background execution.
  - **Tencent Workbuddy**: Incorporates asynchronous task delegation where agents can schedule operations across varied systems without blocking the main workflow.
  - **OHC Missing Capability**: The codebase requires a generalized, durable Background Job Queue backed by Postgres with row-level tenant isolation, built on a `SKIP LOCKED` pattern, supported by exponential backoff and dead-letter queues.

  ## Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        WebClient[Flutter App] -->|API Request| APIServer[Go API Layer]
        APIServer -->|Enqueue Job| PostgresQueue[(Postgres Job Queue)]
        Worker[AI Worker Nodes] -->|Dequeue: SKIP LOCKED| PostgresQueue
        Worker -->|Execute| Agent[Ambassador/Promoter Agent]
        Worker -->|Failure| DLQ[(Dead Letter Queue)]
        Worker -->|Update Status| PostgresQueue
    ```
  - **Mobile UX**: The mobile app (375px width) will present background tasks as "Pending Assistant Work". Owners can view a cleanly styled translucent card showing a live progress bar and status text (e.g., "Drafting reply...", "Syncing inventory...").
  - **AI Agent Integration**: Agent departments will enqueue tasks into the distributed queue for asynchronous execution rather than running blocking generation synchronously during a web request.

  ## Implementation Prompt
  **Target Outcome**: Implement a durable background job execution system built over a Postgres `SKIP LOCKED` queue. The system must enforce multi-tenant isolation, allowing background workers to pick up, retry, and report the status of AI agent tasks (like drafting customer messages or running scheduled business analysis). Ensure the API layer can enqueue tasks quickly while AI worker nodes handle the long-running generation. No specific DB library is prescribed; design a clean repository interface for the queue. The mobile-facing API should support polling or subscribing to task status.

  **Critical User Journey (CUJ)**:
  1. Maya (Persona) requests a detailed historical inventory analysis via the OHC UI.
  2. The system enqueues the `inventory-analysis` job.
  3. Maya sees a "Analysis in progress..." card on her 375px display.
  4. The background worker picks up the job via `SKIP LOCKED`, performs the work, and updates the task status.
  5. Maya's UI reflects the completion and allows her to view the report.

  **Acceptance Criteria**:
  - Full mobile parity and responsiveness.
  - Implementation of the `SKIP LOCKED` postgres strategy (or a clearly documented equivalent if an existing service queue is adapted).
  - 100% unit test coverage for the queue enqueue/dequeue logic.
  - At least one E2E Playwright test proving a user can trigger and observe a background task's completion.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
