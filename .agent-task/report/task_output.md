issue_title: "Implement High-Scale Cross-Channel Agent Action Engine"
issue_description: |
  # Mission Brief
  The OneHumanCorp "Teammate" AI philosophy mandates that our system act as an invisible, intelligent work assistant that resolves operational friction rather than merely presenting chat interfaces. The `agent_feed` currently exists as a primitive asynchronous queue wrapper, but it lacks the necessary data modeling, integration with distributed locks (Redis Redlock), multi-tenant isolation schemas, and full API integration required to enable our target personas (like Maya, Carlos, and Priya) to confidently dispatch critical actions (e.g., booking management, pricing rule execution, or sending customer responses).

  This task aims to implement a comprehensive, scalable, and secure "Agent Action Engine" built on top of the Agent Feed. It will serve as the coordination plane for department AI workers, executing state-mutating actions reliably.

  # Problem Statement
  Currently, actions suggested by the LLM are poorly modeled and largely untracked, making multi-channel operations uncoordinated and dangerous. If Priya is managing an inventory sync failure, or Carlos needs an automated quote sent, the system must guarantee the action executes precisely once and preserves the transaction state.

  # Research Report
  - **Competitor Gap:** Platforms like Shopify and Wix utilize simplistic webhook handlers and basic cron jobs. The OHC architecture requires high-scale distributed background job capabilities.
  - **Core Requirement:** Ensure multi-tenant safety and idempotency. The queue must support distributed locks via Redis.
  - **Observation:** `src/server/services/agent_feed/service.rs` uses a simplistic LLM reasoning prompt. It should decouple task ingestion from execution via robust PostgeSQL job queues.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[API Router] -->|Enqueue Action| B(Agent Feed Repo)
      B --> C[(PostgreSQL Job Table)]
      C -->|SKIP LOCKED Dequeue| D[Worker Pool]
      D --> E{Redis Distributed Lock}
      E -->|Acquire| F[Execute LLM / Mutate State]
      F -->|Update Status| C
  ```

  ### Components
  1.  **PostgreSQL Multi-Tenant Job Queue:** Use a `SKIP LOCKED` query pattern to fetch `AgentFeedItem`s with `lifecycle_state = 'PENDING_APPROVAL'` or `lifecycle_state = 'QUEUED'`.
  2.  **Distributed Lock (Redis):** Before an agent executes a mutation on a business resource (e.g., quoting a customer or updating a booking), it MUST acquire a Redlock `ohc:lock:{tenant_id}:agent_feed:{action_id}`.
  3.  **API Layer:** Implement API routes (`/api/agent-feed`) in `src/server/services/agent_feed/mod.rs` to fetch feed items and approve them.

  # Implementation Prompt
  - **Objective:** Implement the `Agent Feed API` and the robust asynchronous worker queue mechanism for the Agent Action Engine.
  - **Acceptance Criteria:**
      - Create API routes in `src/server/services/agent_feed/mod.rs` for listing agent feed items (`GET /api/agent-feed`) and approving an action (`PATCH /api/agent-feed/{id}/approve`).
      - Enhance the `AgentFeedService` to support transitioning an item from `PENDING_APPROVAL` to `APPROVED` and subsequently processing it.
      - Ensure you add Redis Distributed Lock usage in the processing flow.
      - Add comprehensive unit tests.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
