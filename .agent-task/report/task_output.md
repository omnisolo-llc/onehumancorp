issue_title: "Implement High-Performance Agentic Background Job Queue"
issue_description: |
  # Problem Statement
  OneHumanCorp's platform heavily relies on background processing to fulfill its core promise: AI agents handle complexity invisibly. Currently, functions like AI-driven inventory syncing, quoting, email drafting, and financial reconciliation need a robust, scalable backend queue. If this system fails, is slow, or loses jobs, users like Maya (the baker) or Carlos (the handyman) experience delayed notifications, dropped orders, or incorrect inventory - directly hurting their small businesses. A highly resilient, multi-tenant background job queue is critical. We need a performant job queueing mechanism with strict multi-tenant isolation, priority processing, and dead-lettering capabilities, integrated seamlessly with PostgreSQL `SKIP LOCKED` semantics and Redis Redlock for cross-agent coordination.

  # Research Report
  - **Shopify:** Utilizes a highly robust custom asynchronous job processing system to ensure high-volume flash sales events do not result in dropped orders.
  - **Wix:** Employs event-driven architectures with background processing pipelines, though often abstracted heavily from their external APIs.
  - **Squarespace & GoDaddy:** Have standard worker-based background tasks to handle site generation and email sending asynchronously.
  - **OHC Specifics:** The OHC platform operates as a multi-tenant SaaS with PostgreSQL row-level security. A background queue must naturally respect this `tenant_id` barrier. The architecture should leverage `SKIP LOCKED` for efficient enqueue/dequeue operations over PostgreSQL, alongside exponential backoff for AI agents that might fail due to rate-limiting from Gemini/GPT-4 APIs.

  # Design Doc

  ## Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[API Layer / Frontend] -->|Enqueue Job| B(Job Queue Table in PostgreSQL)
      B -->|SKIP LOCKED Dequeue| C{Worker Pool}
      C -->|Process Task| D[AI Agent Departments]
      C -.->|Failure| E(Retry Queue with Backoff)
      E -.->|Max Retries Exceeded| F[Dead Letter Queue]
      D -->|Redlock Co-ordination| G[(Redis Lock)]
      D -->|Update Result| B
  ```

  ## UI / User Facing Impact (375px first)
  - **Invisible by Design:** This feature is primarily backend infrastructure. However, it affects the UI by ensuring operations like "Generating Storefront" or "Drafting Quote" are responsive.
  - **Dashboard Cards:** The UniFi-style dashboard will have a small, sleek indicator (e.g., a subtle pulsing dot or a translucent glass toast) showing "Agent working..." when background jobs are processing.
  - **Mobile UX Flow:** Maya initiates a bulk inventory update. Instead of blocking the UI, a non-intrusive banner appears: "Updating inventory...". The job queue handles it securely in the background. If it fails, a push notification alerts her.

  ## AI Agent Integration Points
  - Operations AI ("The Manager") places order fulfillment steps onto the queue.
  - Marketing AI ("The Promoter") enqueues social media posting tasks to be executed at optimal times.
  - Legal & Compliance AI uses the queue to asynchronously generate custom terms and conditions based on Maya's input.
  - The queue workers must inject the appropriate multi-tenant context (SPIFFE/SPIRE identity tokens + `tenant_id`) into the AI Agent harness before execution.

  ## Key Design Decisions
  1. **PostgreSQL SKIP LOCKED:** Chosen over external queues (like RabbitMQ or SQS) to maintain transactional integrity with the core application data. Enqueuing a job and updating an entity can happen in the same database transaction.
  2. **Multi-Tenant Isolation:** The worker process must explicitly set the `tenant_id` context (`SET app.current_tenant_id = '...'`) before processing the job to ensure row-level security policies apply even in background contexts.
  3. **Exponential Backoff:** Crucial for AI operations, which are prone to transient rate-limiting from external LLM providers.
  4. **Redis Redlock:** Used for distributed locking when multiple agents need to coordinate on the same resource (e.g., updating shared inventory).

  # Implementation Prompt
  Implement the High-Performance Agentic Background Job Queue. The user-facing outcome is a more reliable, responsive app where AI operations happen seamlessly without blocking the user.
  - Create the backend job queue engine using PostgreSQL and the `SKIP LOCKED` concurrency model.
  - Ensure all job executions are tightly scoped to the tenant requesting the job using row-level security context.
  - Implement retry logic with exponential backoff and a dead-letter queue.
  - Expose telemetry metrics (OpenTelemetry/Prometheus) for queue depth, processing latency, and failure rates per tenant.
  - Develop a full Playwright E2E Critical User Journey (CUJ) where a non-technical user (e.g., Carlos) initiates a bulk service update, the UI returns immediately, and the queue successfully processes it in the background.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
