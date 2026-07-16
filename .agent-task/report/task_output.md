issue_title: "Implement OHC Universal Event Bus and Multi-Tenant Async Queue"
issue_description: |
  ## Problem Statement
  Small business owners need an assistant that operates continuously and autonomously across different "Departments" (Sales, Support, Marketing, Operations). Currently, OHC's backend components handle logic sequentially or via brittle inline API calls. If Maya (the baker) receives 5 Instagram DMs simultaneously, or an abandoned cart triggers a follow-up, the system must process these events reliably without blocking the main API thread or losing events during transient network failures. We need a unified event mesh that routes incoming triggers to the appropriate AI Agent (e.g., the Ambassador, the Manager) asynchronously, ensuring perfect multi-tenant data isolation.

  ## Research Report
  - **Context:** OHC agents (The Ambassador, The Manager, The Quartermaster) must react to a variety of asynchronous events: incoming messages (Instagram/WhatsApp/Email), webhooks (Stripe payments, Shopify sync), state changes (inventory drops), and scheduled tasks (daily digests).
  - **Competitive Analysis:** Platforms like Shopify heavily rely on synchronous webhook broadcasting or third-party tools (Zapier, Klaviyo) which forces the business owner to pay "app taxes" and configure integrations.
  - **OHC Opportunity:** By embedding an autonomous event-driven nervous system into OHC's core architecture, we create the true "invisible AI" experience. Maya doesn't configure an integration; the Event Bus simply routes a "Missed DM" to the Ambassador agent to draft a reply.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Ingress: Webhooks/API] --> B(API Gateway)
      B --> C[Event Emitter Layer]
      C --> D[(PostgreSQL/Redis Event Queue)]
      D --> E[Multi-Tenant Worker Pool]
      E --> F{Event Router}
      F -->|intent=customer_support| G[The Ambassador Agent]
      F -->|intent=scheduling| H[The Manager Agent]
      F -->|intent=inventory_alert| I[The Quartermaster Agent]
      G --> J[Action Required Queue / Agent Feed]
      H --> J
      I --> J
  ```

  ### Core Mechanisms & Key Decisions
  - **Queue Technology:** Leverage PostgreSQL `SKIP LOCKED` (using a table like `agent_jobs` or `event_queue`) combined with Redis for low-latency pub/sub and distributed locking (Redlock). This ensures we do not overcomplicate the stack with Kafka while guaranteeing transactional safety within the tenant database.
  - **Multi-Tenant Isolation:** Every event and job in the queue MUST enforce row-level security or strict `tenant_id` filtering. Workers must acquire jobs exclusively for the tenant they are processing and must never cross-pollinate context.
  - **Idempotency & Retries:** Implement exponential backoff for failed agent execution. External agent API calls (e.g., to Gemini/OpenAI) must be wrapped in retry logic and circuit breakers.
  - **Agent Feed Integration:** The ultimate outcome of the worker processing is usually writing an "Action Card" to the unified Agent Feed for the owner to review on their 375px mobile app.

  ### Mobile UX Parity
  - **Visibility:** The UI does not expose the queue. However, the mobile dashboard must show non-intrusive, truthful state indicators (e.g., "The Ambassador is drafting 3 replies..." with a subtle pulsing animation) reflecting the queue's real-time depth for that tenant.

  ## Implementation Prompt
  **User-Facing Outcome:** When Maya receives three custom cake inquiries overnight, the system reliably captures them, queues them, and processes them through the AI agents before she wakes up. She simply opens the app and sees three beautifully formatted response drafts waiting for her approval.

  **Next Actions:**
  1. Define the PostgreSQL schema for the asynchronous job queue (`ohc_async_jobs`) including state, retries, and strict `tenant_id` bounds.
  2. Implement the `SKIP LOCKED` worker logic in Go/Rust (depending on the core service) to dequeue and route events to the correct agent module.
  3. Ensure distributed locking (Redis Redlock) is applied to prevent two workers from processing the same event.
  4. Create E2E Playwright tests simulating concurrent incoming webhooks to verify no data loss and correct agent feed population.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
