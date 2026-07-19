issue_title: "Implement High-Performance Multi-Tenant Event Mesh for Agent Handoffs"
issue_description: |
  ## Title
  Implement High-Performance Multi-Tenant Event Mesh for Agent Handoffs

  ## Problem Statement
  OHC requires seamless, invisible coordination between multiple AI agents (The Ambassador, The Promoter, The Manager, The Accountant). Currently, agent communication and trigger mechanisms are fragmented, leading to delayed responses, dropped contexts, and race conditions (e.g., an inventory update during a customer DM about availability). For our personas (Maya the Baker, Carlos the Handyman), this results in inaccurate auto-replies or missed fulfillment alerts. A centralized, strongly isolated, multi-tenant event mesh is necessary to reliably route business events to the appropriate AI departments in real-time.

  ## Research Report
  Our analysis of competitive architectures (Shopify, Stripe, and modern microservices patterns) reveals that point-to-point webhook handling is insufficient for agentic orchestration. Shopify uses EventBridge extensively; however, OHC's unique requirement is that events must trigger *autonomous agent workflows* (e.g., an `InventoryDepleted` event triggers both The Operations Agent for reordering and The Customer Success Agent to update pending carts). We need a unified event mesh that guarantees delivery, enforces tenant boundaries, and maintains an immutable audit log for AI decision-making.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[API / Webhooks / UI] -->|Publish Event| B[Event API Gateway]
      B --> C[PostgreSQL Outbox Table]
      C -->|CDC / Polling| D[Redis Streams Mesh]
      D --> E{Agent Event Router}
      E -->|Route| F[The Ambassador Queue]
      E -->|Route| G[The Manager Queue]
      E -->|Route| H[The Promoter Queue]
      F --> I[Agent Execution Worker]
      G --> J[Agent Execution Worker]
      H --> K[Agent Execution Worker]
      I/J/K -->|Record Decision| L[Agent Mission Log DB]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  *   **Owner View:** This is a backend architectural feature, but its outcome surfaces in the Agent Feed.
  *   **Mobile Experience:** When an event fires (e.g., "New VIP Customer Order"), the Event Mesh triggers The Ambassador. Within seconds, a card appears on the 375px viewport: "VIP Order Received. Drafted a thank you text. [Send Now] [Edit]". The speed and reliability of this notification depend entirely on the Event Mesh.

  ### AI Agent Integration Points
  *   **The Operations Agent (The Manager):** Subscribes to `OrderPlaced`, `InventoryLow`, `BookingRequested`.
  *   **The Customer Success Agent (The Ambassador):** Subscribes to `MessageReceived`, `CartAbandoned`, `OrderDelivered`.
  *   **The Marketing Agent (The Promoter):** Subscribes to `ProductCreated`, `FiveStarReviewReceived`.
  *   All agents publish `ActionProposed` or `ActionExecuted` events back to the mesh.

  ### Key Design Decisions
  1.  **Transactional Outbox Pattern:** To ensure database writes (e.g., an order) and event publication are atomic, events will first be written to an `agent_events_outbox` table in PostgreSQL within the same transaction.
  2.  **Redis Streams:** A background worker will relay events from the outbox to Redis Streams, providing high-throughput, reliable delivery to agent consumer groups.
  3.  **Strict Tenant Isolation:** Every event payload MUST include `tenant_id`. Consumer groups must validate this tenant ID before processing.

  ## Implementation Prompt
  **User-Facing Outcome:** Business owners experience instantaneous, coordinated actions from their AI staff. When a customer buys the last cake, the storefront updates, the owner gets a restock suggestion, and any active DMs about that cake are intercepted—all seamlessly coordinated.
  **CUJ & Acceptance Criteria:**
  1.  Implement the `agent_events_outbox` table with strict RLS (`tenant_id`).
  2.  Build the Go service to relay outbox entries to Redis Streams.
  3.  Implement consumer groups for at least two AI departments (e.g., Operations and Customer Success).
  4.  Provide Playwright E2E tests simulating a multi-agent scenario: Trigger a `ProductSoldOut` event and verify that two different mock agents process the event and update the database accordingly, asserting the final state via the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
