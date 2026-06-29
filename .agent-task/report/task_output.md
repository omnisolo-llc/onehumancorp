issue_title: "Implement High-Performance Multi-Tenant AI Background Job Queue"
issue_description: |
  # Research Report: Multi-Tenant AI Background Job Queue

  ## Problem Statement
  Small business owners rely on OHC’s AI agents to process operations invisibly (e.g., drafting customer replies, recovering missed leads). Currently, the system lacks a robust, multi-tenant background job queue dedicated to agent task orchestration. This creates bottlenecks, limits scale, and forces synchronous processing, meaning the owner might experience sluggish UI updates or dropped background tasks during traffic spikes.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Flow / Webhooks:** Utilizes robust background job processing (Sidekiq/Kafka) to trigger automations reliably.
  - **Wix Automations:** Queues events asynchronously to trigger emails/tasks, avoiding synchronous blocking.
  - **Stripe:** Uses idempotency and reliable queueing to handle webhooks and events even under heavy load.
  - **OHC Gap:** OHC needs a highly scalable, multi-tenant aware background job queue (using PostgreSQL `SKIP LOCKED` or Redis) specifically designed to ingest events (new DMs, new orders) and dispatch them to the correct AI agent capability with exponential backoff and dead-letter queues.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[API/Webhook Ingestion] --> B[Event Router]
      B --> C[(PostgreSQL Job Queue with SKIP LOCKED)]
      C --> D[Worker Pool 1: Triage]
      C --> E[Worker Pool 2: Customer Success Agent]
      C --> F[Worker Pool 3: Operations Agent]
      D --> G[Unified Customer Feed]
      E --> G
      F --> G
      G --> H[Owner Mobile App (375px)]
  ```
  ### Mobile UX Flow (375px)
  - The owner does not see the queue directly. Instead, they see instant updates in their "Action Required" feed as jobs are processed in the background.
  - Visual indicators (e.g., a subtle loading state or "Agent is drafting..." badge) provide feedback that background processing is active.

  ### AI Agent Integration
  - **Dispatcher Module:** A generic dispatcher that listens to the queue, determines the task type, and routes it to the correct AI capability's prompt architecture.
  - **Fault Tolerance:** If the LLM provider (Gemini/OpenAI) rate-limits or fails, the job remains in the queue and retries with exponential backoff.

  ## Implementation Prompt
  **Feature Name:** Multi-Tenant AI Agent Background Event Dispatcher
  **Target Persona:** Maya the Baker (receiving overnight DMs).
  **Outcome:** All incoming events (DMs, orders) are reliably queued and processed by AI agents in the background. The owner sees drafted replies or triaged tasks in their feed without UI lag.

  **Next Actions:**
  1. Implement a robust background job queue in PostgreSQL leveraging the `SKIP LOCKED` pattern. Ensure strict `tenant_id` isolation.
  2. Create worker processes that pull from this queue, execute the required AI agent tasks, and update the unified owner feed.
  3. Include error handling, retries, and a dead-letter queue.
  4. Integrate the queue with at least one existing workflow.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
