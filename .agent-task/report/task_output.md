issue_title: "[Architectural Design] - Autonomous Event Ingestion & Notification Bus (EventBridge)"
issue_description: |
  ## Problem Statement
  OHC aims to differentiate itself from competitors like Shopify and Wix by replacing passive dashboards with "Invisible AI Automation." The Agent Feed (detailed in `agent_feed_deep_dive.md`) is central to this, pushing action cards directly to the owner. Currently, OHC lacks a unified Event Ingestion Pipeline (EventBridge) to reliably ingest webhooks (from external sources like Instagram or Stripe), internal state changes (e.g., `ProductCreated`), and scheduled jobs, and route them to the relevant LLM-powered agents (like The Ambassador or The Promoter). Without a resilient message bus, the proactive, AI-driven notification UX cannot function effectively.

  ## Research Report
  - **Competitor Analysis:**
    - Shopify relies on third-party apps for advanced automations or its Sidekick chatbot, which advises rather than acts.
    - AI-Native builders (like Durable) focus on site generation but lack operational workflow automation.
    - OHC's unique value proposition is the "Agent Feed," which requires robust, event-driven backend systems.
  - **OHC Specific Needs:**
    - **Event Ingestion:** Must handle diverse sources (Instagram DMs, Stripe payments, Inventory updates).
    - **Resilience:** Needs guaranteed delivery and graceful retries for asynchronous worker processing.
    - **Agent Routing:** Must route events to the correct specialized agents (Operations, Sales/Ambassador, Promoter) for intent classification and RAG-based context building.
    - **Mobile-First UX Integration:** Processed events must result in Action Cards pushed to the mobile viewport (375px) via WebSockets or similar real-time mechanisms.

  ## Design Doc
  ### High-Level Architecture
  1.  **Ingress API & Webhook Layer (Go/Rust)**:
      - Exposes generic and vendor-specific webhook endpoints (`/api/webhooks/instagram`, `/api/webhooks/stripe`).
      - Validates payloads (signatures, Zero Trust) and normalizes them into standard OHC internal events (e.g., `MessageReceived`, `PaymentSucceeded`, `ProductCreated`).
  2.  **Distributed Event Bus (Redis Pub/Sub or Kafka / PostgreSQL SKIP LOCKED)**:
      - Normalized events are published to a central message bus or queue.
      - Ensures tenant isolation by tagging all events with `tenant_id`.
  3.  **Agent Dispatcher (Asynchronous Workers)**:
      - Background workers consume events from the bus.
      - **Intent & Context Resolution:** The dispatcher routes the event to the appropriate Agent (e.g., The Ambassador for messages, The Promoter for new products).
      - Agents use LLMs (Gemini Pro) to classify intent and query tenant data (RAG) to build context and draft responses.
  4.  **Notification & Approval Delivery**:
      - The generated draft/action is converted into an "Action Card".
      - The Action Card is persisted (e.g., `agent_feed` table) and pushed to the client (mobile app/browser) via WebSockets for real-time visibility.

  ### Mobile UX Impact (375px First)
  - The Event Ingestion Pipeline is entirely invisible to the user.
  - The tangible outcome is the immediate appearance of Action Cards in the mobile Agent Feed (e.g., "New product detected! Schedule a post?").
  - Ensures a seamless, latency-free notification experience.

  ### AI Agent Integration
  - **The Ambassador:** Subscribes to `MessageReceived` events. Drafts replies based on RAG context.
  - **The Promoter:** Subscribes to `ProductCreated` events. Drafts social media posts based on product metadata.

  ### Data Model (Conceptual)
  - `ohc_events` (event_id, tenant_id, source, event_type, payload, created_at)
  - `agent_feed_items` (item_id, tenant_id, agent_type, status [pending_approval, approved, discarded], action_payload, created_at)

  ## Implementation Prompt
  **Task for Implementer Agent:**
  Design and implement the core backend infrastructure for the Autonomous Event Ingestion & Notification Bus (EventBridge).
  1. Create the database schemas for normalized events (`ohc_events`) and feed action cards (`agent_feed_items`) ensuring strict row-level security by `tenant_id`.
  2. Implement an Ingress API module in Go/Rust to receive, validate (mock signature for now), and normalize incoming webhooks.
  3. Implement a robust queueing mechanism (e.g., utilizing PostgreSQL `SKIP LOCKED` or Redis) to reliably distribute events to asynchronous worker agents.
  4. Build a dispatcher loop that pulls events, routes them to a stubbed agent logic, and generates a corresponding record in `agent_feed_items`.
  5. Provide 100% unit test coverage for the ingestion, queueing, and dispatching logic, proving that events are correctly mapped to tenant feeds without data leakage.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
